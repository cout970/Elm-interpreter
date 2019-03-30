use std::collections::HashMap;

use analyzer::Analyser;
use analyzer::expression_analyzer::backtrack_expr;
use analyzer::expression_analyzer::expr_tree_to_expr;
use analyzer::expression_analyzer::find_var_replacements;
use analyzer::expression_analyzer::rename_variables;
use analyzer::expression_analyzer::replace_vars_with_concrete_types;
use analyzer::expression_analyzer::type_from_expected;
use analyzer::pattern_analyzer::analyze_pattern;
use analyzer::pattern_analyzer::analyze_pattern_with_type;
use analyzer::PatternMatchingError;
use analyzer::type_helper::calculate_common_type;
use analyzer::type_helper::get_common_type;
use analyzer::type_helper::is_assignable;
use ast::*;
use ast;
use constructors::*;
use errors::TypeError;
use typed_ast::expr_type;
use typed_ast::LetEntry;
use typed_ast::TypedExpr;
use types::Value;
use util::build_fun_type;
use util::expression_fold::*;

impl Analyser {
    pub fn analyze_application(&mut self, fun: &Expr, arg: &Expr, app: &Expr) -> Result<TypedExpr, TypeError> {
        // example of variable type inference:
        // sum = (+) 1.5

        let function = self.analyze_expression(None, fun)?;
        // (+) : number -> number -> number


        if let Type::Fun(argument, result) = expr_type(&function) {
            // argument: number
            // result: number -> number

            let input = self.analyze_expression(Some(&argument), arg)?;
            // Float

            if !is_assignable(&argument, &expr_type(&input)) {
                return Err(TypeError::ArgumentsDoNotMatch(
                    span(arg), format!("Expected argument: {}, found: {}", argument, input),
                ));
            }

            let mut vars: HashMap<String, Type> = HashMap::new();
            find_var_replacements(&mut vars, &expr_type(&input), &argument);
            // vars: [number => Float], change number to float

            let output = replace_vars_with_concrete_types(&vars, &result);
            // Float

            backtrack_expr(&mut self.env, &vars, fun);
            // env: [number => Float]

            Ok(TypedExpr::Application(output, Box::new(function), Box::new(input)))
        } else {
            return Err(TypeError::NotAFunction(
                span(app),
                format!("Expected function found: {}, (in: {}, out: {})", function, fun, arg),
            ));
        }
    }

    pub fn analyze_expression_ref(&mut self, expected: Option<&Type>, span: Span, name: &String) -> Result<TypedExpr, TypeError> {
        let def = self.env.find_definition(name)
            .or_else(|| self.env.find_alias(name))
            .ok_or(TypeError::MissingDefinition(span, name.to_string()))?;

        let new_ty = if let Some(expected_ty) = expected {
            let new_ty = type_from_expected(&mut self.env, expected_ty, &def);
            self.env.replace(name, new_ty.clone());

            new_ty
        } else {
            if !self.env.is_local(name) {
                rename_variables(&mut self.env, &mut HashMap::new(), def)
            } else {
                def
            }
        };

        Ok(TypedExpr::Ref(new_ty, name.to_string()))
    }

    pub fn analyze_expression_lambda(&mut self, expected: Option<&Type>, span: Span, patterns: &Vec<Pattern>, expr: &Expr) -> Result<TypedExpr, TypeError> {
        let (tys, new_vars) = self.analyze_function_arguments(patterns, &None)?;

        self.env.enter_block();
        for (name, value) in &new_vars {
            if self.env.find_definition(name).is_some() {
                self.env.exit_block();
                return Err(TypeError::VariableNameShadowed(span, name.clone()));
            }

            self.env.add_definition(name, value.clone());
        }

        let result = self.analyze_expression(expected, expr);
        self.env.exit_block();

        let typed_expr = result?;

        let mut var = tys.clone();
        var.push(expr_type(&typed_expr));

        Ok(TypedExpr::Lambda(build_fun_type(&var), patterns.clone(), Box::new(typed_expr)))
    }

    pub fn analyze_expression_list(&mut self, span: Span, exprs: &Vec<Expr>) -> Result<TypedExpr, TypeError> {
        if exprs.is_empty() {
            return Ok(TypedExpr::List(Type::Tag("List".to_string(), vec![Type::Var(self.env.next_name())]), vec![]));
        }

        let first = self.analyze_expression(None, &exprs[0])?;
        let mut list_type = expr_type(&first);
        let mut children = vec![first];

        for i in 1..exprs.len() {
            let elem = self.analyze_expression(Some(&list_type), &exprs[i])?;
            let elem_type = expr_type(&elem);

            if !is_assignable(&list_type, &elem_type) {
                return Err(TypeError::ListNotHomogeneous(span, list_type, expr_type(&elem), i as u32));
            }

            if let Type::Var(_) = list_type {
                list_type = elem_type;
            }

            children.push(elem);
        }

        Ok(TypedExpr::List(Type::Tag("List".to_string(), vec![list_type]), children))
    }

    pub fn analyze_expression_let(&mut self, expected: Option<&Type>, span: Span, decls: &Vec<LetDeclaration>, expr: &Expr) -> Result<TypedExpr, TypeError> {
        let mut entries = vec![];
        self.env.enter_block();
        for decl in decls {
            match decl {
                LetDeclaration::Def(def) => {
                    match self.analyze_definition(def) {
                        Ok(def) => {
                            self.env.add_definition(&def.name, def.header.clone());
                            entries.push(LetEntry::Definition(def));
                        }
                        Err(e) => {
                            self.env.exit_block();
                            return Err(e);
                        }
                    }
                }
                LetDeclaration::Pattern(pattern, expr) => {
                    let (pat_ty, vars) = match analyze_pattern(&mut self.env, pattern) {
                        Ok(it) => it,
                        Err(e) => {
                            self.env.exit_block();
                            return Err(TypeError::InvalidPattern(span, e));
                        }
                    };

                    let typed_expr = match self.analyze_expression(Some(&pat_ty), expr) {
                        Ok(ty) => ty,
                        Err(e) => {
                            self.env.exit_block();
                            return Err(e);
                        }
                    };

                    if !is_assignable(&pat_ty, &expr_type(&typed_expr)) {
                        self.env.exit_block();
                        return Err(TypeError::DefinitionTypeAndReturnTypeMismatch);
                    }

                    for (name, ty) in vars {
                        self.env.add_definition(&name, ty);
                    }

                    entries.push(LetEntry::Pattern(pattern.clone(), typed_expr));
                }
            }
        }

        let res = self.analyze_expression(expected, expr);
        self.env.exit_block();
        let expr = res?;

        Ok(TypedExpr::Let(expr_type(&expr), entries, Box::new(expr)))
    }

    pub fn analyze_expression_record(&mut self, entries: &Vec<(String, Expr)>) -> Result<TypedExpr, TypeError> {
        let types: Vec<(String, TypedExpr)> = entries.iter()
            .map(|(name, expr)| self.analyze_expression(None, expr).map(|ty| (name.clone(), ty)))
            .collect::<Result<_, _>>()?;

        let own_type = Type::Record(
            types.iter()
                .map(|(name, expr)| (name.clone(), expr_type(expr)))
                .collect::<Vec<_>>()
        );

        Ok(TypedExpr::Record(own_type, types))
    }

    pub fn analyze_expression_record_access(&mut self, name: &String) -> Result<TypedExpr, TypeError> {
        let output = self.env.next_name();
        let input = self.env.next_name();

        let own_type = Type::Fun(
            Box::new(Type::RecExt(input, vec![
                (name.clone(), Type::Var(output.clone()))
            ])),
            Box::new(Type::Var(output)),
        );

        Ok(TypedExpr::RecordAccess(own_type, name.clone()))
    }

    pub fn analyze_expression_record_field(&mut self, expected: Option<&Type>, expr: &Expr, name: &String) -> Result<TypedExpr, TypeError> {
        // Expected expr to be a record with field [name] and type [expected]
        let expected_expr_ty = Type::Record(vec![
            (name.clone(), expected.cloned().unwrap_or(Type::Var(self.env.next_name())))
        ]);

        let record = self.analyze_expression(Some(&expected_expr_ty), expr)?;

        match &record {
            TypedExpr::Record(_, fields) => {
                if !fields.iter().any(|(f_name, _)| f_name == name) {
                    return Err(TypeError::ExpectingRecordWithName { record: record.clone(), name: name.clone() });
                }
            }
            _ => {
                return Err(TypeError::ExpectingRecordWithName { record: record.clone(), name: name.clone() });
            }
        }

        Ok(record)
    }

    pub fn analyze_expression_tuple(&mut self, items: &Vec<Expr>) -> Result<TypedExpr, TypeError> {
        let sub_items: Vec<TypedExpr> = items.iter()
            .map(|e| self.analyze_expression(None, e))
            .collect::<Result<_, _>>()?;

        let own_type = Type::Tuple(
            sub_items.iter().map(expr_type).collect::<Vec<_>>()
        );

        Ok(TypedExpr::Tuple(own_type, sub_items))
    }

    pub fn analyze_expression_record_update(&mut self, span: Span, name: &String, updates: &Vec<(String, Expr)>) -> Result<TypedExpr, TypeError> {
        // { x = 0 } => { a | x = 2 } => { x = 2 }
        let mut update_types = vec![];
        for (name, expr) in updates {
            update_types.push((name.clone(), self.analyze_expression(None, expr)?));
        }
        let own_type = Type::RecExt(
            name.clone(),
            update_types.iter().map(|(name, expr)| (name.clone(), expr_type(expr))).collect::<Vec<_>>(),
        );
        let rec_type = Type::Record(
            update_types.iter().map(|(name, expr)| (name.clone(), expr_type(expr))).collect::<Vec<_>>(),
        );

        // Record to update
        let ref_expr = self.analyze_expression(Some(&rec_type), &Expr::Ref(span, name.to_owned()))?;

        match expr_type(&ref_expr) {
            Type::Record(fields) => {
                for (field_name, _) in updates {
                    if !fields.iter().any(|(field, _)| field == field_name) {
                        return Err(TypeError::RecordUpdateUnknownField(
                            span,
                            format!("Field '{}' not found in record: {} of type: {}", field_name, name, ref_expr),
                        ));
                    }
                }
            }
            _ => {
                return Err(TypeError::RecordUpdateOnNonRecord(
                    span,
                    format!("Expecting record to update but found: {}", ref_expr),
                ));
            }
        }

        Ok(TypedExpr::RecordUpdate(own_type, Box::new(ref_expr), update_types))
    }

    pub fn analyze_expression_case(&mut self, expected: Option<&Type>, span: Span, expr: &Expr, branches: &Vec<(Pattern, Expr)>) -> Result<TypedExpr, TypeError> {
        let patterns_types = branches.iter()
            .map(|(p, _)| analyze_pattern(&mut self.env, p).map(|(ty, _)| ty))
            .collect::<Result<Vec<_>, PatternMatchingError>>()
            .map_err(|e| TypeError::InvalidPattern(span, e))?;

        let mut patterns_types_iter = patterns_types.iter();
        let mut patterns_type = patterns_types_iter.next().unwrap();

        while let Some(ty) = patterns_types_iter.next() {
            match get_common_type(patterns_type, ty) {
                Some(ty) => { patterns_type = ty; }
                None => {
                    return Err(TypeError::InvalidPattern(
                        span,
                        PatternMatchingError::ListPatternsAreNotHomogeneous(patterns_type.clone(), ty.clone()),
                    ));
                }
            }
        }

        let cond_type = self.analyze_expression(Some(patterns_type), expr)?;

        let mut iter = branches.iter();
        let mut branches = vec![];
        let (first_pattern, first_expr) = iter.next().unwrap();

        let first_type = {
            // check patterns for variables
            let (_, vars) = analyze_pattern_with_type(&mut self.env, first_pattern, expr_type(&cond_type))
                .map_err(|e| TypeError::InvalidPattern(span, e))?;

            // add variable to the environment
            self.env.enter_block();

            for (name, ty) in &vars {
                self.env.add_definition(name, ty.clone());
            }

            let result = self.analyze_expression(expected, first_expr);

            // reset environment
            self.env.exit_block();

            let case_expr = result?;
            let case_type = expr_type(&case_expr);

            branches.push((first_pattern.clone(), case_expr));
            case_type
        };

        while let Some((pattern, expression)) = iter.next() {
            // check patterns for variables
            let (_, vars) = analyze_pattern_with_type(&mut self.env, pattern, expr_type(&cond_type))
                .map_err(|e| TypeError::InvalidPattern(span, e))?;

            // add variable to the environment
            self.env.enter_block();

            for (name, ty) in &vars {
                self.env.add_definition(name, ty.clone());
            }

            let result = self.analyze_expression(Some(&first_type), expression);

            // reset environment
            self.env.exit_block();

            let ret = result?;

            if !is_assignable(&first_type, &expr_type(&ret)) {
                return Err(TypeError::CaseBranchDontMatchReturnType(ast::span(expression), "".to_string()));
            }

            branches.push((pattern.clone(), ret));
        }

        Ok(TypedExpr::Case(first_type, Box::new(cond_type), branches))
    }

    pub fn analyze_expression_if(&mut self, expected: Option<&Type>, span: Span, cond: &Expr, a: &Expr, b: &Expr) -> Result<TypedExpr, TypeError> {
        let cond = self.analyze_expression(Some(&type_bool()), cond)?;
        let true_branch = self.analyze_expression(expected, a)?;
        let false_branch = self.analyze_expression(expected, b)?;

        if !is_assignable(&type_bool(), &expr_type(&cond)) {
            return Err(TypeError::IfWithNonBoolCondition(span, format!("Expected Bool expression but found {}", cond)));
        }

        let branches = vec![expr_type(&true_branch), expr_type(&false_branch)];

        match calculate_common_type(&branches) {
            Ok(ty) => Ok(TypedExpr::If(ty.clone(), Box::new(cond), Box::new(true_branch), Box::new(false_branch))),
            Err((a, b)) => Err(TypeError::IfBranchesDoesntMatch(span, format!("True Branch: {}, False Branch: {}", a, b)))
        }
    }

    pub fn analyze_expression_chain(&mut self, expected: Option<&Type>, span: Span, exprs: &Vec<Expr>, ops: &Vec<String>) -> Result<TypedExpr, TypeError> {
        match create_expr_tree(exprs, ops) {
            Ok(tree) => self.analyze_expression(expected, &expr_tree_to_expr(tree)),
            Err(e) => {
                let msg = match e {
                    ExprTreeError::InvalidInput => format!("Invalid input"),
                    ExprTreeError::AssociativityError => format!("Associativity error"),
                    ExprTreeError::InternalError(msg) => format!("Internal error: {}", msg),
                };
                Err(TypeError::InvalidOperandChain(span, msg))
            }
        }
    }

    pub fn analyze_expression_literal(&self, expected: Option<&Type>, lit: &Literal) -> Result<TypedExpr, TypeError> {
        let value = match lit {
            Literal::Int(i) => {
                if let Some(expected) = expected {
                    if let Type::Tag(name, _) = expected {
                        match name.as_str() {
                            "Int" => Value::Int(*i),
                            "Float" => Value::Float(*i as Float),
                            _ => Value::Number(*i)
                        }
                    } else {
                        Value::Number(*i)
                    }
                } else {
                    Value::Number(*i)
                }
            }
            Literal::Float(i) => Value::Float(*i),
            Literal::String(i) => Value::String(i.clone()),
            Literal::Char(i) => Value::Char(*i),
        };

        Ok(TypedExpr::Const(value))
    }
}


#[cfg(test)]
mod tests {
    use analyzer::static_env::StaticEnv;
    use test_utils::Test;
    use util::StringConversion;

    use super::*;

    fn analyze_function(env: &mut StaticEnv, fun: &Definition) -> Result<Type, TypeError> {
        let expr = Analyser::from(env.clone()).analyze_definition(fun)?;
        Ok(expr.header)
    }

    fn analyze_expression(env: &mut StaticEnv, expected: Option<&Type>, expr: &Expr) -> Result<Type, TypeError> {
        let expr = Analyser::from(env.clone()).analyze_expression(expected, expr)?;
        Ok(expr_type(&expr))
    }

    #[test]
    fn check_ref() {
        let mut env = StaticEnv::new();
        env.add_definition("varName", type_int());
        let expr = analyze_expression(&mut env, None, &Test::expr("varName"));

        assert_eq!(Ok(type_int()), expr);
    }

    #[test]
    fn check_unit() {
        let expr = Test::expr("()");
        let mut env = StaticEnv::new();
        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Unit));
    }

    #[test]
    fn check_literal() {
        let expr = Test::expr("123");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Var("number".s())));
    }

    #[test]
    fn check_fun() {
        let expr = Test::expr("fun 123");
        let mut env = StaticEnv::new();

        env.add_definition("fun", Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Tag("Int".s(), vec![])),
        ));

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_if() {
        let expr = Test::expr("if True then 1 else 0");
        let mut env = StaticEnv::new();

        env.add_definition("True", Type::Tag("Bool".s(), vec![]));

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Var("number".s())));
    }

    #[test]
    fn check_lambda() {
        let expr = Test::expr("\\x -> 1");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Fun(
            Box::new(Type::Var("a".s())),
            Box::new(Type::Var("number".s())),
        )));
    }

    #[test]
    fn check_list() {
        let expr = Test::expr("[1, 2, 3]");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag(
            "List".s(), vec![Type::Var("number".s())],
        )));
    }

    #[test]
    fn check_bad_list() {
        let expr = Test::expr("[1, 2, 'a']");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Err(
            TypeError::ListNotHomogeneous(span(&expr), type_number(), type_char(), 2)
        ));
    }

    #[test]
    fn check_record() {
        let expr = Test::expr("{ a = 1, b = \"Hi\" }");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(
            Type::Record(vec![
                ("a".s(), Type::Var("number".s())),
                ("b".s(), Type::Tag("String".s(), vec![])),
            ])
        ));
    }

    #[test]
    fn check_operator_chain() {
        let expr = Test::expr("1 + 2");
        let mut env = StaticEnv::new();

        env.add_definition("+", Type::Fun(
            Box::new(Type::Var("number".s())),
            Box::new(Type::Fun(
                Box::new(Type::Var("number".s())),
                Box::new(Type::Var("number".s())),
            )),
        ));

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(
            Type::Var("number".s())
        ));
    }

    #[test]
    fn check_tuple() {
        let expr = Test::expr("(1, \"a\", ())");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(
            Type::Tuple(vec![
                Type::Var("number".s()),
                Type::Tag("String".s(), vec![]),
                Type::Unit,
            ])
        ));
    }

    #[test]
    fn check_record_update() {
        let expr = Test::expr("{ x | a = 0 }");
        let mut env = StaticEnv::new();

        // Type of x
        let record_type = Type::Record(vec![
            ("a".s(), Type::Var("number".s())),
            ("b".s(), Type::Var("number".s())),
        ]);

        // Type of expr
        let result_type = Type::RecExt("x".s(), vec![
            ("a".s(), Type::Var("number".s()))
        ]);

        env.add_definition("x", record_type.clone());

        let result = analyze_expression(&mut env, None, &expr);
        assert_eq!(result, Ok(result_type));
    }

    #[test]
    fn check_case() {
        let expr = Test::expr("case 0 of\n 0 -> \"a\"\n _ -> \"b\"");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag("String".s(), vec![])));
    }

    #[test]
    fn check_case2() {
        let expr = Test::expr("case 0 of\n 0 -> 1\n _ -> \"b\"");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Err(TypeError::CaseBranchDontMatchReturnType((24, 27), "".s())));
    }
}
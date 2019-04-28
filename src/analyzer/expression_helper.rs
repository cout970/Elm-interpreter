use std::collections::HashMap;

use analyzer::Analyzer;
use analyzer::inference::{tmp_map_pattern, tmp_map_patterns};
use analyzer::pattern_analyzer::analyze_pattern;
use analyzer::pattern_analyzer::analyze_pattern_with_type;
use analyzer::type_helper::calculate_common_type;
use analyzer::type_helper::get_common_type;
use analyzer::type_helper::is_assignable;
use analyzer::type_inference::expr_tree_to_expr;
use analyzer::type_inference::type_inference_backtrack_expr;
use analyzer::type_inference::type_inference_find_var_replacements;
use analyzer::type_inference::type_inference_rename_variables;
use analyzer::type_inference::type_inference_replace_vars_with_concrete_types;
use analyzer::type_inference::type_inference_type_from_expected;
use ast::*;
use ast;
use constructors::*;
use errors::PatternMatchingError;
use errors::TypeError;
use typed_ast::expr_type;
use typed_ast::LetEntry;
use typed_ast::TypedExpr;
use types::Value;
use util::build_fun_type;
use util::expression_fold::*;

impl Analyzer {
    pub fn analyze_application(&mut self, fun: &Expr, arg: &Expr, app: &Expr) -> Result<TypedExpr, TypeError> {
        // example of variable type inference:
        // sum = (+) 1.5

        let function = self.analyze_expression_helper(None, fun)?;
        // (+) : number -> number -> number


        if let Type::Fun(argument, result) = expr_type(&function) {
            // argument: number
            // result: number -> number

            let input = self.analyze_expression_helper(Some(&argument), arg)?;
            // Float

            if !is_assignable(&argument, &expr_type(&input)) {
                return Err(TypeError::ArgumentsDoNotMatch {
                    span: span(arg),
                    expected: argument.as_ref().clone(),
                    found: expr_type(&input).clone(),
                });
            }

            let mut vars: HashMap<String, Type> = HashMap::new();
            type_inference_find_var_replacements(&mut vars, &expr_type(&input), &argument);
            // vars: [number => Float], change number to float

            let output = type_inference_replace_vars_with_concrete_types(&vars, &result);
            // Float

            type_inference_backtrack_expr(&mut self.env, &vars, fun);
            // env: [number => Float]

            Ok(TypedExpr::Application((0, 0), output, Box::new(function), Box::new(input)))
        } else {
            return Err(TypeError::NotAFunction {
                span: span(app),
                function: expr_type(&function),
                input: fun.clone(),
                output: arg.clone(),
            });
        }
    }

    pub fn analyze_expression_ref(&mut self, expected: Option<&Type>, span: Span, name: &String) -> Result<TypedExpr, TypeError> {
        let def = self.env.find_definition(name)
            .or_else(|| self.env.find_alias(name))
            .ok_or_else(|| {
                TypeError::MissingDefinition { span, name: name.to_string() }
            })?;

        let new_ty = if let Some(expected_ty) = expected {
            let new_ty = type_inference_type_from_expected(&mut self.env, expected_ty, &def);
            self.env.replace(name, new_ty.clone());

            new_ty
        } else {
            if !self.env.is_local(name) {
                type_inference_rename_variables(&mut self.env, &mut HashMap::new(), def)
            } else {
                def
            }
        };

        Ok(TypedExpr::Ref((0, 0), new_ty, name.to_string()))
    }

    pub fn analyze_expression_lambda(&mut self, expected: Option<&Type>, span: Span, patterns: &Vec<Pattern>, expr: &Expr) -> Result<TypedExpr, TypeError> {
        let (tys, new_vars) = self.analyze_function_arguments(patterns, &None)?;

        self.env.enter_block();
        for (name, value) in &new_vars {
            if self.env.find_definition(name).is_some() {
                self.env.exit_block();
                return Err(TypeError::VariableNameShadowed { span, name: name.clone() });
            }

            self.env.add_definition(name, value.clone());
        }

        let result = self.analyze_expression_helper(expected, expr);
        self.env.exit_block();

        let typed_expr = result?;

        let mut var = tys.clone();
        var.push(expr_type(&typed_expr));

        Ok(TypedExpr::Lambda((0, 0), build_fun_type(&var), tmp_map_patterns(patterns), Box::new(typed_expr)))
    }

    pub fn analyze_expression_list(&mut self, span: Span, exprs: &Vec<Expr>) -> Result<TypedExpr, TypeError> {
        if exprs.is_empty() {
            return Ok(TypedExpr::List((0, 0), Type::Tag("List".to_string(), vec![Type::Var(self.env.next_name())]), vec![]));
        }

        let first = self.analyze_expression_helper(None, &exprs[0])?;
        let mut list_type = expr_type(&first);
        let mut children = vec![first];

        for i in 1..exprs.len() {
            let elem = self.analyze_expression_helper(Some(&list_type), &exprs[i])?;
            let elem_type = expr_type(&elem);

            if !is_assignable(&list_type, &elem_type) {
                return Err(TypeError::ListNotHomogeneous { span, list_type, item_type: expr_type(&elem), index: i as u32 });
            }

            if let Type::Var(_) = list_type {
                list_type = elem_type;
            }

            children.push(elem);
        }

        Ok(TypedExpr::List((0, 0), Type::Tag("List".to_string(), vec![list_type]), children))
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
                        Err(info) => {
                            self.env.exit_block();
                            return Err(TypeError::PatternMatchingError { span, info });
                        }
                    };

                    let typed_expr = match self.analyze_expression_helper(Some(&pat_ty), expr) {
                        Ok(ty) => ty,
                        Err(e) => {
                            self.env.exit_block();
                            return Err(e);
                        }
                    };

                    if !is_assignable(&pat_ty, &expr_type(&typed_expr)) {
                        self.env.exit_block();
                        return Err(TypeError::DefinitionTypeAndReturnTypeMismatch {
                            span: ast::span(expr),
                            expected: pat_ty.clone(),
                            found: expr_type(&typed_expr),
                        });
                    }

                    for (name, ty) in vars {
                        self.env.add_definition(&name, ty);
                    }

                    entries.push(LetEntry::Pattern(tmp_map_pattern(pattern), typed_expr));
                }
            }
        }

        let res = self.analyze_expression_helper(expected, expr);
        self.env.exit_block();
        let expr = res?;

        Ok(TypedExpr::Let((0, 0), expr_type(&expr), entries, Box::new(expr)))
    }

    pub fn analyze_expression_record(&mut self, entries: &Vec<(String, Expr)>) -> Result<TypedExpr, TypeError> {
        let types: Vec<(String, TypedExpr)> = entries.iter()
            .map(|(name, expr)| self.analyze_expression_helper(None, expr).map(|ty| (name.clone(), ty)))
            .collect::<Result<_, _>>()?;

        let own_type = Type::Record(
            types.iter()
                .map(|(name, expr)| (name.clone(), expr_type(expr)))
                .collect::<Vec<_>>()
        );

        Ok(TypedExpr::Record((0, 0), own_type, types))
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

        Ok(TypedExpr::RecordAccess((0, 0), own_type, name.clone()))
    }

    pub fn analyze_expression_record_field(&mut self, expected: Option<&Type>, expr: &Expr, name: &String) -> Result<TypedExpr, TypeError> {
        // Expected expr to be a record with field [name] and type [expected]
        let expected_expr_ty = Type::Record(vec![
            (name.clone(), expected.cloned().unwrap_or(Type::Var(self.env.next_name())))
        ]);

        let record = self.analyze_expression_helper(Some(&expected_expr_ty), expr)?;

        match &record {
            TypedExpr::Record(_, _, fields) => {
                match fields.iter().find(|(f_name, _)| f_name == name) {
                    Some((_, expr)) => {
                        Ok(TypedExpr::RecordField((0, 0), expr_type(expr), Box::new(record.clone()), name.clone()))
                    }
                    None => {
                        Err(TypeError::ExpectingRecordWithName { record: record.clone(), name: name.clone() })
                    }
                }
            }
            _ => {
                Err(TypeError::ExpectingRecordWithName { record: record.clone(), name: name.clone() })
            }
        }
    }

    pub fn analyze_expression_tuple(&mut self, items: &Vec<Expr>) -> Result<TypedExpr, TypeError> {
        let sub_items: Vec<TypedExpr> = items.iter()
            .map(|e| self.analyze_expression_helper(None, e))
            .collect::<Result<_, _>>()?;

        let own_type = Type::Tuple(
            sub_items.iter().map(expr_type).collect::<Vec<_>>()
        );

        Ok(TypedExpr::Tuple((0, 0), own_type, sub_items))
    }

    pub fn analyze_expression_record_update(&mut self, span: Span, name: &String, updates: &Vec<(String, Expr)>) -> Result<TypedExpr, TypeError> {
        // { x = 0 } => { a | x = 2 } => { x = 2 }
        let mut update_types = vec![];
        for (name, expr) in updates {
            update_types.push((name.clone(), self.analyze_expression_helper(None, expr)?));
        }
        let own_type = Type::RecExt(
            name.clone(),
            update_types.iter().map(|(name, expr)| (name.clone(), expr_type(expr))).collect::<Vec<_>>(),
        );
        let rec_type = Type::Record(
            update_types.iter().map(|(name, expr)| (name.clone(), expr_type(expr))).collect::<Vec<_>>(),
        );

        // Record to update
        let ref_expr = self.analyze_expression_helper(Some(&rec_type), &Expr::Ref(span, name.to_owned()))?;

        match expr_type(&ref_expr) {
            Type::Record(fields) => {
                for (field_name, _) in updates {
                    if !fields.iter().any(|(field, _)| field == field_name) {
                        return Err(TypeError::RecordUpdateUnknownField {
                            span,
                            field: field_name.to_string(),
                            record_name: name.to_string(),
                            record: ref_expr,
                        });
                    }
                }
            }
            _ => {
                return Err(TypeError::RecordUpdateOnNonRecord { span, expr: ref_expr });
            }
        }

        Ok(TypedExpr::RecordUpdate((0, 0), own_type, Box::new(ref_expr), update_types))
    }

    pub fn analyze_expression_case(&mut self, expected: Option<&Type>, span: Span, expr: &Expr, branches: &Vec<(Pattern, Expr)>) -> Result<TypedExpr, TypeError> {
        let patterns_types = branches.iter()
            .map(|(p, _)| analyze_pattern(&mut self.env, p).map(|(ty, _)| ty))
            .collect::<Result<Vec<_>, PatternMatchingError>>()
            .map_err(|info| TypeError::PatternMatchingError { span, info })?;

        let mut patterns_types_iter = patterns_types.iter();
        let mut patterns_type = patterns_types_iter.next().unwrap();

        while let Some(ty) = patterns_types_iter.next() {
            match get_common_type(patterns_type, ty) {
                Some(ty) => { patterns_type = ty; }
                None => {
                    let info = PatternMatchingError::ListPatternsAreNotHomogeneous(patterns_type.clone(), ty.clone());
                    return Err(TypeError::PatternMatchingError { span, info });
                }
            }
        }

        let cond_type = self.analyze_expression_helper(Some(patterns_type), expr)?;

        let mut iter = branches.iter();
        let mut branches = vec![];
        let (first_pattern, first_expr) = iter.next().unwrap();

        let first_type = {
            // check patterns for variables
            let (_, vars) = analyze_pattern_with_type(&mut self.env, first_pattern, expr_type(&cond_type))
                .map_err(|info| TypeError::PatternMatchingError { span, info })?;

            // add variable to the environment
            self.env.enter_block();

            for (name, ty) in &vars {
                self.env.add_definition(name, ty.clone());
            }

            let result = self.analyze_expression_helper(expected, first_expr);

            // reset environment
            self.env.exit_block();

            let case_expr = result?;
            let case_type = expr_type(&case_expr);

            branches.push((tmp_map_pattern(first_pattern), case_expr));
            case_type
        };

        while let Some((pattern, expression)) = iter.next() {
            // check patterns for variables
            let (_, vars) = analyze_pattern_with_type(&mut self.env, pattern, expr_type(&cond_type))
                .map_err(|info| TypeError::PatternMatchingError { span, info })?;

            // add variable to the environment
            self.env.enter_block();

            for (name, ty) in &vars {
                self.env.add_definition(name, ty.clone());
            }

            let result = self.analyze_expression_helper(Some(&first_type), expression);

            // reset environment
            self.env.exit_block();

            let ret = result?;

            if !is_assignable(&first_type, &expr_type(&ret)) {
                return Err(TypeError::CaseBranchDontMatchReturnType {
                    span: ast::span(expression),
                    expected: first_type,
                    found: expr_type(&ret),
                });
            }

            branches.push((tmp_map_pattern(pattern), ret));
        }

        Ok(TypedExpr::Case((0, 0), first_type, Box::new(cond_type), branches))
    }

    pub fn analyze_expression_if(&mut self, expected: Option<&Type>, span: Span, cond: &Expr, a: &Expr, b: &Expr) -> Result<TypedExpr, TypeError> {
        let cond = self.analyze_expression_helper(Some(&type_bool()), cond)?;
        let true_branch = self.analyze_expression_helper(expected, a)?;
        let false_branch = self.analyze_expression_helper(expected, b)?;

        if !is_assignable(&type_bool(), &expr_type(&cond)) {
            return Err(TypeError::IfWithNonBoolCondition { span, expr: cond });
        }

        let branches = vec![expr_type(&true_branch), expr_type(&false_branch)];

        match calculate_common_type(&branches) {
            Ok(ty) => Ok(TypedExpr::If((0, 0), ty.clone(), Box::new(cond), Box::new(true_branch), Box::new(false_branch))),
            Err((a, b)) => Err(
                TypeError::IfBranchesDoesntMatch { span, true_branch, false_branch }
            )
        }
    }

    pub fn analyze_expression_chain(&mut self, expected: Option<&Type>, span: Span, exprs: &Vec<Expr>, ops: &Vec<String>) -> Result<TypedExpr, TypeError> {
        match create_expr_tree(exprs, ops) {
            Ok(tree) => self.analyze_expression_helper(expected, &expr_tree_to_expr(tree)),
            Err(e) => {
                let msg = match e {
                    ExprTreeError::InvalidInput => format!("Invalid input"),
                    ExprTreeError::AssociativityError => format!("Associativity error"),
                    ExprTreeError::InternalError(msg) => format!("Internal error: {}", msg),
                };
                Err(TypeError::InvalidOperandChain { span, msg })
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

        Ok(TypedExpr::Const((0, 0), value.get_type(), value))
    }
}


#[cfg(test)]
mod tests {
    use analyzer::static_env::StaticEnv;
    use test_utils::Test;
    use util::StringConversion;

    use super::*;

    fn analyze_expression(analyzer: &mut Analyzer, expr: &Expr) -> Result<Type, TypeError> {
        let expr = analyzer.analyze_expression_helper(None, expr)?;
        Ok(expr_type(&expr))
    }

    #[test]
    fn check_ref() {
        let (expr, mut analyzer) = Test::expr_analyzer("varName");
        analyzer.env.add_definition("varName", type_int());
        let expr = analyze_expression(&mut analyzer, &expr);

        assert_eq!(Ok(type_int()), expr);
    }

    #[test]
    fn check_unit() {
        let (expr, mut analyzer) = Test::expr_analyzer("()");
        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Unit));
    }

    #[test]
    fn check_literal() {
        let (expr, mut analyzer) = Test::expr_analyzer("123");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Var("number".s())));
    }

    #[test]
    fn check_fun() {
        let (expr, mut analyzer) = Test::expr_analyzer("fun 123");

        analyzer.env.add_definition("fun", Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Tag("Int".s(), vec![])),
        ));

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_if() {
        let (expr, mut analyzer) = Test::expr_analyzer("if True then 1 else 0");

        analyzer.env.add_definition("True", Type::Tag("Bool".s(), vec![]));

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Var("number".s())));
    }

    #[test]
    fn check_lambda() {
        let (expr, mut analyzer) = Test::expr_analyzer("\\x -> 1");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Fun(
            Box::new(Type::Var("a".s())),
            Box::new(Type::Var("number".s())),
        )));
    }

    #[test]
    fn check_list() {
        let (expr, mut analyzer) = Test::expr_analyzer("[1, 2, 3]");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Tag(
            "List".s(), vec![Type::Var("number".s())],
        )));
    }

    #[test]
    fn check_bad_list() {
        let (expr, mut analyzer) = Test::expr_analyzer("[1, 2, 'a']");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Err(
            TypeError::ListNotHomogeneous { span: span(&expr), list_type: type_number(), item_type: type_char(), index: 2 }
        ));
    }

    #[test]
    fn check_record() {
        let (expr, mut analyzer) = Test::expr_analyzer("{ a = 1, b = \"Hi\" }");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(
            Type::Record(vec![
                ("a".s(), Type::Var("number".s())),
                ("b".s(), Type::Tag("String".s(), vec![])),
            ])
        ));
    }

    #[test]
    fn check_operator_chain() {
        let (expr, mut analyzer) = Test::expr_analyzer("1 + 2");

        analyzer.env.add_definition("+", Type::Fun(
            Box::new(Type::Var("number".s())),
            Box::new(Type::Fun(
                Box::new(Type::Var("number".s())),
                Box::new(Type::Var("number".s())),
            )),
        ));

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(
            Type::Var("number".s())
        ));
    }

    #[test]
    fn check_tuple() {
        let (expr, mut analyzer) = Test::expr_analyzer("(1, \"a\", ())");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(
            Type::Tuple(vec![
                Type::Var("number".s()),
                Type::Tag("String".s(), vec![]),
                Type::Unit,
            ])
        ));
    }
}
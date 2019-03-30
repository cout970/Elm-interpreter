use std::collections::HashMap;
use std::ops::Deref;
use std::str::Chars;

use analyzer::expression_analyzer::analyze_expression;
use analyzer::expression_analyzer::backtrack_expr;
use analyzer::expression_analyzer::expr_tree_to_expr;
use analyzer::expression_analyzer::find_var_replacements;
use analyzer::expression_analyzer::get_adt_type;
use analyzer::expression_analyzer::rename_variables;
use analyzer::expression_analyzer::replace_vars_with_concrete_types;
use analyzer::expression_analyzer::type_from_expected;
use analyzer::expression_analyzer::type_of_app;
use analyzer::function_analyzer::analyze_function;
use analyzer::pattern_analyzer::analyze_pattern;
use analyzer::pattern_analyzer::analyze_pattern_with_type;
use analyzer::pattern_analyzer::is_exhaustive;
use analyzer::static_env::StaticEnv;
use analyzer::type_helper::calculate_common_type;
use analyzer::type_helper::get_common_type;
use analyzer::type_helper::is_assignable;
use ast::Definition;
use ast::Expr;
use ast::Literal;
use ast::Pattern;
use ast::span;
use ast::Type;
use constructors::type_bool;
use errors::*;
use source::SourceCode;
use typed_ast::expr_type;
use typed_ast::TypedDefinition;
use typed_ast::TypedExpr;
use types::Function;
use types::Value;
use util::build_fun_type;
use util::create_vec_inv;
use util::expression_fold::create_expr_tree;
use util::qualified_name;
use util::StringConversion;

pub mod static_env;
pub mod module_analyser;
mod function_analyzer;
mod expression_analyzer;
mod dependency_sorter;
mod pattern_analyzer;
mod type_helper;

#[derive(Clone, Debug, PartialEq)]
pub enum PatternMatchingError {
    ListPatternsAreNotHomogeneous(Type, Type),
    UnknownOperatorPattern(String),
    UnknownAdtVariant(String),
    ExpectedListType(Type),
    ExpectedUnit(Type),
    ExpectedTuple(Pattern, Type),
    ExpectedRecord(Type),
    ExpectedAdt(String, Type),
    PatternNotExhaustive(Pattern),
    InvalidRecordEntryName(String),
    ExpectedLiteral(String, Type),
    TODO,
}


pub struct Analyser {
    env: StaticEnv
}

impl Analyser {
    pub fn new() -> Self {
        Analyser { env: StaticEnv::new() }
    }

    pub fn from(env: StaticEnv) -> Self {
        Analyser { env }
    }

    pub fn analyse_definition(&mut self, fun: &Definition) -> Result<TypedDefinition, TypeError> {
        println!("analyze_function: {}", fun.name);
        let name_seq = self.env.name_seq.save();

        // Extract function input types and argument variables
        let (argument_types, argument_vars) = self.analyze_function_arguments(&fun.patterns, &fun.header)?;
        self.env.enter_block();

        // Add function arguments
        for (arg_name, ty) in &argument_vars {
            self.env.add_definition(arg_name, ty.clone());
        }

        // Analyse function expression
        let return_expr = if let Some(ty) = &fun.header {
            // Register own type to be able to call the function recursively
            self.env.add_definition(&fun.name, ty.clone());

            // TODO use the number of defined arguments to obtain the [return_type]

            // Convert `a -> b -> c` into vec![a, b, c]
            let fn_types = unpack_types(ty);
            // Extract the last value, `c`
            let return_type = fn_types.last().expect("Expected last value to exist");

            // Analyse function body
            self.analyze_expression(Some(return_type), &fun.expr)
        } else {

            // Create a self function type with a variable as output type, we don't know
            // the return type yet, it must be inferred
            let self_type = create_vec_inv(&argument_types, Type::Var("z".s()));
            // Register own type to be able to call the function recursively
            self.env.add_definition(&fun.name, build_fun_type(&self_type));

            // Analyse function body
            self.analyze_expression(None, &fun.expr)
        };

        // If the expression analysis failed, we return the error
        let return_expr = match return_expr {
            Ok(expr) => expr,
            Err(e) => {
                // The environment must be restored
                self.env.exit_block();
                self.env.name_seq.restore(name_seq);
                return Err(e);
            }
        };

        // Extract or update the final function type
        let fun_type = if fun.header.is_none() {
            let mut final_arg_types: Vec<Type> = vec![];

            // Update argument variable with concrete types
            'outer: for arg in &argument_types {
                if let Type::Var(arg_var_name) = arg {

                    // search in local variables for the type of this variable,
                    // this is needed because the number of arguments and local variables can be different
                    for (name, ty) in &argument_vars {
                        if let Type::Var(local_var_name) = ty {
                            if local_var_name == arg_var_name {
                                if let Some(ty) = self.env.find_definition(name) {
                                    final_arg_types.push(ty);
                                    continue 'outer;
                                }
                            }
                        }
                    }

                    // TODO remove panics in the middle of code
                    panic!("Unable to find variable '{}' in {:?}, for function: {}", &arg, argument_vars, fun.name);
                } else {
                    final_arg_types.push(arg.clone());
                }
            }

            final_arg_types.push(expr_type(&return_expr));
            build_fun_type(&final_arg_types)
        } else {
            fun.header.clone().unwrap()
        };

        self.env.exit_block();
        self.env.name_seq.restore(name_seq);

        // TODO check fun type and expr type are the same

        Ok(TypedDefinition {
            header: fun_type,
            name: fun.name.clone(),
            patterns: fun.patterns.clone(),
            expr: return_expr,
        })
    }

    pub fn analyze_function_arguments(&mut self, patterns: &Vec<Pattern>, func_ty: &Option<Type>) -> Result<(Vec<Type>, Vec<(String, Type)>), TypeError> {
        let mut arguments: Vec<Type> = vec![];
        let mut argument_vars: Vec<(String, Type)> = vec![];

        let iter: Vec<(Option<Type>, &Pattern)> = match func_ty {
            Some(ty) => {
                let list = unpack_types(ty);

                if patterns.len() > list.len() {
                    eprintln!("patterns: {:?}", patterns);
                    eprintln!("list: {:?}", list);
                    return Err(TypeError::InvalidPatternAmount(list.len(), patterns.len()));
                }

                list.into_iter().zip(patterns).map(|(ty, pat)| (Some(ty), pat)).collect()
            }
            _ => {
                patterns.iter().map(|p| (None, p)).collect()
            }
        };

        for (ty, patt) in iter {
            if !is_exhaustive(patt) {
                // TODO
                return Err(TypeError::InvalidPattern((0, 0), PatternMatchingError::PatternNotExhaustive(patt.clone())));
            }

            let (ty, vars) = match ty {
                Some(ty) => {
                    // TODO
                    analyze_pattern_with_type(&mut self.env, patt, ty)
                        .map_err(|e| TypeError::InvalidPattern((0, 0), e))?
                }
                None => {
                    //TODO
                    analyze_pattern(&mut self.env, patt)
                        .map_err(|e| TypeError::InvalidPattern((0, 0), e))?
                }
            };

            arguments.push(ty);
            for pair in vars {
                argument_vars.push(pair);
            }
        }

        Ok((arguments, argument_vars))
    }


    pub fn analyze_expression(&mut self, expected: Option<&Type>, expr: &Expr) -> Result<TypedExpr, TypeError> {
        let typed_expr: TypedExpr = match expr {
            Expr::Ref(_, name) => {
                let def = self.env.find_definition(name)
                    .or_else(|| self.env.find_alias(name))
                    .ok_or(TypeError::MissingDefinition(span(expr), name.clone()))?;

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

                TypedExpr::Ref(new_ty, name.clone())
            }
            Expr::QualifiedRef(span, path, name) => {
                let full_name = qualified_name(path, name);

                self.analyze_expression(expected, &Expr::Ref(*span, full_name))?
            }
            Expr::Application(_, fun, arg) => self.analyze_application(&**fun, &**arg, expr)?,
            Expr::Lambda(_, patterns, expr) => {
                let (tys, new_vars) = self.analyze_function_arguments(patterns, &None)?;

                self.env.enter_block();
                for (name, value) in &new_vars {
                    if self.env.find_definition(name).is_some() {
                        self.env.exit_block();
                        return Err(TypeError::VariableNameShadowed(name.clone()));
                    }

                    self.env.add_definition(name, value.clone());
                }

                let result = self.analyze_expression(expected, expr);
                self.env.exit_block();

                let typed_expr = result?;

                let mut var = tys.clone();
                var.push(expr_type(&typed_expr));

                TypedExpr::Lambda(build_fun_type(&var), patterns.clone(), Box::new(typed_expr))
            }
            Expr::List(_, exprs) => {
                if exprs.is_empty() {
                    TypedExpr::List(Type::Tag("List".to_string(), vec![Type::Var(self.env.next_name())]), vec![])
                } else {
                    let first = self.analyze_expression(None, &exprs[0])?;
                    let mut list_type = expr_type(&first);
                    let mut children = vec![first];

                    for i in 1..exprs.len() {
                        let elem = self.analyze_expression(Some(&list_type), &exprs[i])?;
                        let elem_type = expr_type(&elem);

                        if !is_assignable(&list_type, &elem_type) {
                            return Err(TypeError::ListNotHomogeneous(
                                span(expr),
                                format!("List of '{}', but found element '{}' at index: {}", list_type, elem, i),
                            ));
                        }

                        if let Type::Var(_) = list_type {
                            list_type = elem_type;
                        }

                        children.push(elem);
                    }

                    TypedExpr::List(Type::Tag("List".to_string(), vec![list_type]), children)
                }
            }
            Expr::Let(_, decls, expr) => {
//                env.enter_block();
//                for decl in decls {
//                    match decl {
//                        LetDeclaration::Def(def) => {
//                            let def_ty = analyze_function(env, def);
//
//                            match def_ty {
//                                Ok(ty) => {
//                                    env.add_definition(&def.name, ty);
//                                }
//                                Err(e) => {
//                                    env.exit_block();
//                                    return Err(e);
//                                }
//                            }
//                        }
//                        LetDeclaration::Pattern(pattern, expr) => {
//                            let res = analyze_let_destructuring(env, pattern, expr);
//
//                            match res {
//                                Ok(vars) => {
//                                    for (name, ty) in vars {
//                                        env.add_definition(&name, ty);
//                                    }
//                                }
//                                Err(e) => {
//                                    env.exit_block();
//                                    return Err(e);
//                                }
//                            }
//                        }
//                    }
//                }
//                let res = self.analyze_expression(expected, expr);
//                env.exit_block();
//
//                res
                unimplemented!()
            }
            Expr::OpChain(_, exprs, ops) => {
                match create_expr_tree(exprs, ops) {
                    Ok(tree) => self.analyze_expression(expected, &expr_tree_to_expr(tree))?,
                    Err(_) => {
                        // TODO
                        return Err(TypeError::InvalidOperandChain(span(expr), format!("You cannot mix >> and << without parentheses")));
                    }
                }
            }
            Expr::Record(_, entries) => {
                let types: Vec<(String, TypedExpr)> = entries.iter()
                    .map(|(name, expr)| self.analyze_expression(None, expr).map(|ty| (name.clone(), ty)))
                    .collect::<Result<_, _>>()?;

                let own_type = Type::Record(
                    types.iter()
                        .map(|(name, expr)| (name.clone(), expr_type(expr)))
                        .collect::<Vec<_>>()
                );

                TypedExpr::Record(own_type, types)
            }
            Expr::RecordAccess(_, name) => {
                let output = self.env.next_name();
                let input = self.env.next_name();

                let own_type = Type::Fun(
                    Box::new(Type::RecExt(input, vec![
                        (name.clone(), Type::Var(output.clone()))
                    ])),
                    Box::new(Type::Var(output)),
                );

                TypedExpr::RecordAccess(own_type, name.clone())
            }
            Expr::RecordField(_, expr, name) => {
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

                record
            }
            Expr::Tuple(_, items) => {
                let sub_items: Vec<TypedExpr> = items.iter()
                    .map(|e| self.analyze_expression(None, e))
                    .collect::<Result<_, _>>()?;

                let own_type = Type::Tuple(
                    sub_items.iter().map(expr_type).collect::<Vec<_>>()
                );

                TypedExpr::Tuple(own_type, sub_items)
            }
            Expr::RecordUpdate(span, name, updates) => {
                // { x = 0 } => { a | x = 2 } => { x = 2 }
                let mut update_types = vec![];
                for (name, expr) in updates {
                    update_types.push((name.clone(), self.analyze_expression(None, expr)?));
                }
                let own_type = Type::RecExt(
                    name.clone(),
                    update_types.iter().map(|(name, expr)| (name.clone(), expr_type(expr))).collect::<Vec<_>>(),
                );

                // Record to update
                let record = self.analyze_expression(Some(&own_type), &Expr::Ref(*span, name.to_owned()))?;

                match &record {
                    TypedExpr::Record(_, fields) => {
                        for (field_name, _) in updates {
                            if !fields.iter().any(|(field, _)| field == field_name) {
                                return Err(TypeError::RecordUpdateUnknownField(
                                    *span,
                                    format!("Field '{}' not found in record: {} of type: {}", field_name, name, record),
                                ));
                            }
                        }
                    }
                    _ => {
                        return Err(TypeError::RecordUpdateOnNonRecord(
                            *span,
                            format!("Expecting record to update but found: {}", record),
                        ));
                    }
                }

                TypedExpr::RecordUpdate(own_type, Box::new(record), update_types)
            }
            Expr::Case(_, expr, branches) => self.analyze_case_expr(expected, &*expr, branches)?,
            Expr::If(_, cond, a, b) => {
                let cond = self.analyze_expression(Some(&type_bool()), cond)?;
                let true_branch = self.analyze_expression(expected, a)?;
                let false_branch = self.analyze_expression(expected, b)?;

                if !is_assignable(&type_bool(), &expr_type(&cond)) {
                    return Err(TypeError::IfWithNonBoolCondition(span(expr), format!("Expected Bool expression but found {}", cond)));
                }

                let branches = vec![expr_type(&true_branch), expr_type(&false_branch)];
                match calculate_common_type(&branches) {
                    Ok(ty) => TypedExpr::If(ty.clone(), Box::new(cond), Box::new(true_branch), Box::new(false_branch)),
                    Err((a, b)) => {
                        return Err(TypeError::IfBranchesDoesntMatch(span(expr), format!("True Branch: {}, False Branch: {}", a, b)));
                    }
                }
            }
            Expr::Unit(..) => {
                TypedExpr::Const(Value::Unit)
            }
            Expr::Literal(_, lit) => {
                let value = match lit {
                    Literal::Int(i) => Value::Number(*i),
                    Literal::Float(i) => Value::Float(*i),
                    Literal::String(i) => Value::String(i.clone()),
                    Literal::Char(i) => Value::Char(*i),
                };
                TypedExpr::Const(value)
            }
        };

        Ok(typed_expr)
    }

    fn analyze_application(&mut self, fun: &Expr, arg: &Expr, app: &Expr) -> Result<TypedExpr, TypeError> {
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

    fn analyze_case_expr(&mut self, expected: Option<&Type>, expr: &Expr, branches: &Vec<(Pattern, Expr)>) -> Result<TypedExpr, TypeError> {
        let patterns_types = branches.iter()
            .map(|(p, _)| analyze_pattern(&mut self.env, p).map(|(ty, _)| ty))
            .collect::<Result<Vec<_>, PatternMatchingError>>()
            .map_err(|e| TypeError::InvalidPattern(span(expr), e))?;

        let mut patterns_types_iter = patterns_types.iter();
        let mut patterns_type = patterns_types_iter.next().unwrap();

        while let Some(ty) = patterns_types_iter.next() {
            match get_common_type(patterns_type, ty) {
                Some(ty) => { patterns_type = ty; }
                None => {
                    return Err(TypeError::InvalidPattern(
                        span(expr),
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
                .map_err(|e| TypeError::InvalidPattern(span(expr), e))?;

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
                .map_err(|e| TypeError::InvalidPattern(span(expr), e))?;

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
                return Err(TypeError::CaseBranchDontMatchReturnType(span(expression), "".to_string()));
            }

            branches.push((pattern.clone(), ret));
        }

        Ok(TypedExpr::Case(first_type, Box::new(cond_type), branches))
    }
}

fn unpack_types(ty: &Type) -> Vec<Type> {
    let mut curr = ty.clone();
    let mut components = vec![];

    while let Type::Fun(a, b) = curr {
        components.push((*a).clone());
        curr = (*b).clone();
    }
    components.push(curr.clone());
    components
}

pub fn type_check_expression(env: &mut StaticEnv, expr: &Expr) -> Result<Type, TypeError> {
    analyze_expression(env, None, expr)
}

pub fn type_check_function(env: &mut StaticEnv, fun: &Definition) -> Result<Type, TypeError> {
    analyze_function(env, fun)
}

pub fn type_of_value(value: &Value) -> Type {
    match value {
        Value::Unit => {
            Type::Unit
        }
        Value::Number(_) => {
            Type::Var("number".s())
        }
        Value::Int(_) => {
            Type::Tag("Int".s(), vec![])
        }
        Value::Float(_) => {
            Type::Tag("Float".s(), vec![])
        }
        Value::String(_) => {
            Type::Tag("String".s(), vec![])
        }
        Value::Char(_) => {
            Type::Tag("Char".s(), vec![])
        }
        Value::List(items) => {
            if items.is_empty() {
                Type::Tag("List".s(), vec![Type::Var("a".s())])
            } else {
                Type::Tag("List".s(), vec![type_of_value(items.first().unwrap())])
            }
        }
        Value::Tuple(items) => {
            Type::Tuple(items.iter().map(|i| type_of_value(i)).collect())
        }
        Value::Record(items) => {
            Type::Record(items.iter().map(|(s, i)| (s.to_owned(), type_of_value(i))).collect())
        }
        Value::Adt(var_name, items, adt) => {
            get_adt_type(var_name, items, adt.clone())
        }
        Value::Fun { fun, args, .. } => {
            let fun_ty = type_of_function(fun.deref());

            strip_fun_args(args.len(), &fun_ty).clone()
        }
    }
}

fn type_of_function(fun: &Function) -> &Type {
    match fun {
        Function::External(_, _, ty) => ty,
        Function::Wrapper(_, _, ty) => ty,
        Function::Expr(_, _, _, ty) => ty,
    }
}

fn strip_fun_args(args: usize, ty: &Type) -> &Type {
    if args == 0 {
        return ty;
    }

    if let Type::Fun(_, ref output) = ty {
        strip_fun_args(args - 1, output)
    } else {
        ty
    }
}


#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn type_check1() {
// TODO
//        let ast = from_code_mod(include_bytes!("../../benches/data/type_check.elm"));
//        let info = InterModuleInfo::new();

//        let module_info = ModuleInfo {
//            path: vec![],
//            ast,
//            code: String::from(include_str!("../../benches/data/type_check.elm")),
//        };


//        let checked = analyze_module(&info, module_info).expect("Type error");
//        println!("{:?}", checked);
//        panic!();
    }
}
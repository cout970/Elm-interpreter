use std::collections::HashMap;
use std::sync::Arc;

use analyzer::function_analyzer::analyze_function;
use analyzer::function_analyzer::analyze_function_arguments;
use analyzer::function_analyzer::analyze_let_destructuring;
use analyzer::static_env::StaticEnv;
use analyzer::type_of_value;
use analyzer::TypeError::*;
use analyzer::TypeError;
use ast::*;
use types::Adt;
use types::Value;
use util::build_fun_type;
use util::expression_fold::create_expr_tree;
use util::expression_fold::ExprTree;
use util::qualified_name;
use util::StringConversion;
use analyzer::pattern_analyzer::analyze_pattern_with_type;
use analyzer::type_helper::is_assignable;
use analyzer::type_helper::calculate_common_type;

pub fn analyze_expression(env: &mut StaticEnv, expected: Option<&Type>, expr: &Expr) -> Result<Type, TypeError> {
    println!("analyze_expression {{ expected: {:?}, expr: {:?} }}", expected, expr);
//    println!("analyze_expression {{ env: {:?} }}", env);
    match expr {
        Expr::Ref(name) => {
            let def =
                env.find_definition(name)
                    .or_else(|| env.find_alias(name))
                    .ok_or(MissingDefinition(format!("Missing def {}", name)))?;


            if let Some(expected_ty) = expected {
                let new_ty = type_from_expected(env, expected_ty, &def);
                env.replace(name, new_ty.clone());

                Ok(new_ty)
            } else {
                if !env.is_local(name) {
                    let mut vars = HashMap::new();
                    Ok(rename_variables(env, &mut vars, def))
                } else {
                    Ok(def)
                }
            }
        }
        Expr::QualifiedRef(path, name) => {
            let full_name = qualified_name(path, name);

            analyze_expression(env, expected, &Expr::Ref(full_name))
        }
        Expr::Application(fun, arg) => {
            type_of_app(env, &**fun, &**arg)
        }
        Expr::Lambda(patterns, expr) => {
            let (tys, new_vars) = analyze_function_arguments(env, patterns, &None)?;

            env.enter_block();
            for (name, value) in &new_vars {
                if env.find_definition(name).is_some() {
                    env.exit_block();
                    return Err(VariableNameShadowed(name.clone()));
                }

                env.add_definition(name, value.clone());
            }

            let out = analyze_expression(env, expected, expr);
            env.exit_block();

            let mut var = tys.clone();
            var.push(out?);

            Ok(build_fun_type(&var))
        }
        Expr::List(exprs) => {
            if exprs.is_empty() {
                Ok(Type::Tag("List".s(), vec![Type::Var(env.next_name())]))
            } else {
                let mut first = analyze_expression(env, None, &exprs[0])?;

                for i in 1..exprs.len() {
                    let elem = analyze_expression(env, Some(&first), &exprs[i])?;

                    if is_assignable(&first, &elem) {
                        if let Type::Var(_) = first {
                            first = elem;
                        }
                    } else {
                        return Err(ListNotHomogeneous(
                            format!("List of '{}', but found element '{}' at index: {}", first, elem, i)
                        ));
                    }
                }

                Ok(Type::Tag("List".s(), vec![first]))
            }
        }
        Expr::Let(decls, expr) => {
            env.enter_block();
            for decl in decls {
                match decl {
                    LetDeclaration::Def(def) => {
                        let def_ty = analyze_function(env, def);

                        match def_ty {
                            Ok(ty) => {
                                env.add_definition(&def.name, ty);
                            }
                            Err(e) => {
                                env.exit_block();
                                return Err(e);
                            }
                        }
                    }
                    LetDeclaration::Pattern(pattern, expr) => {
                        let res = analyze_let_destructuring(env, pattern, expr);

                        match res {
                            Ok(vars) => {
                                for (name, ty) in vars {
                                    env.add_definition(&name, ty);
                                }
                            }
                            Err(e) => {
                                env.exit_block();
                                return Err(e);
                            }
                        }
                    }
                }
            }
            let res = analyze_expression(env, expected, expr);
            env.exit_block();

            res
        }
        Expr::OpChain(exprs, ops) => {
            match create_expr_tree(exprs, ops) {
                Ok(tree) => analyze_expression(env, expected, &expr_tree_to_expr(tree)),
                Err(_) => Err(InvalidOperandChain(format!("You cannot mix >> and << without parentheses"))),
            }
        }
        Expr::Record(entries) => {
            let types: Vec<(String, Type)> = entries.iter()
                .map(|(name, expr)| analyze_expression(env, None, expr).map(|ty| (name.clone(), ty)))
                .collect::<Result<_, _>>()?;

            Ok(Type::Record(types))
        }
        Expr::RecordAccess(_) => {
            Ok(Type::Fun(
                Box::new(Type::Var(env.next_name())),
                Box::new(Type::Var(env.next_name())),
            ))
        }
        Expr::RecordField(expr, name) => {
            let exp_ty = Type::Record(vec![
                (name.clone(), expected.map(|e| e.clone()).unwrap_or(Type::Var(env.next_name())))
            ]);
            let record = match analyze_expression(env, Some(&exp_ty), expr) {
                Ok(t) => t.clone(),
                Err(e) => { return Err(e); }
            };

            if let Type::Record(fields) = record {
                let field: Option<&Type> = fields
                    .iter()
                    .find(|(f_name, _)| f_name == name)
                    .map(|(_, f_type)| f_type);

                match field {
                    Some(t) => Ok(t.clone()),
                    None => Err(InternalError)
                }
            } else {
                Err(InternalError)
            }
        }
        Expr::Tuple(items) => {
            let types: Vec<Type> = items.iter()
                .map(|e| analyze_expression(env, None, e))
                .collect::<Result<_, _>>()?;

            Ok(Type::Tuple(types))
        }
        Expr::RecordUpdate(name, updates) => {
            let mut update_types = vec![];
            for (name, expr) in updates {
                update_types.push((name.clone(), analyze_expression(env, None, expr)?));
            }
            let exp_ty = Type::Record(update_types);

            let record_type = analyze_expression(env, Some(&exp_ty), &Expr::Ref(name.to_owned()))?;

            if let Type::Record(fields) = &record_type {
                for (field_name, _) in updates {
                    let found = fields.iter().any(|(field, _)| field == field_name);
                    if !found {
                        return Err(RecordUpdateUnknownField(
                            format!("Field '{}' not found in record: {} of type: {}", field_name, name, record_type)
                        ));
                    }
                }

                Ok(record_type.clone())
            } else {
                Err(RecordUpdateOnNonRecord(
                    format!("Expecting record to update but found: {}", record_type)
                ))
            }
        }
        Expr::Case(expr, branches) => {
            let cond_type = analyze_expression(env, None, expr)?;
            let mut iter = branches.iter();
            let (first_pattern, first_expr) = iter.next().unwrap();

            let first_type = {
                // check patterns for variables
                let (_, vars) = analyze_pattern_with_type(env, first_pattern, cond_type.clone())
                    .map_err(|e| TypeError::InvalidPattern(e))?;

                // add variable to the environment
                env.enter_block();

                for (name, ty) in &vars {
                    env.add_definition(name, ty.clone());
                }

                let result = analyze_expression(env, expected, first_expr);

                // reset environment
                env.exit_block();

                result?
            };

            while let Some((pattern, expression)) = iter.next() {

                // check patterns for varibles
                let (_, vars) = analyze_pattern_with_type(env, pattern, cond_type.clone())
                    .map_err(|e| TypeError::InvalidPattern(e))?;

                // add variable to the environment
                env.enter_block();

                for (name, ty) in &vars {
                    env.add_definition(name, ty.clone());
                }

                let result = analyze_expression(env, Some(&first_type), expression);

                // reset environment
                env.exit_block();

                let ret = result?;

                if !is_assignable(&first_type, &ret) {
                    return Err(CaseBranchDontMatchReturnType("".s()));
                }
            }

            Ok(first_type)
        }
        Expr::If(cond, a, b) => {
            let cond = analyze_expression(env, Some(&Type::Tag("Bool".s(), vec![])), cond)?;
            let true_branch = analyze_expression(env, expected, a)?;
            let false_branch = analyze_expression(env, expected, b)?;

            if !is_assignable(&Type::Tag("Bool".s(), vec![]), &cond) {
                return Err(IfWithNonBoolCondition(format!("Expected Bool expression but found {}", cond)));
            }

            let branches = vec![true_branch, false_branch];
            let ret_ty = calculate_common_type(&branches);
            match ret_ty {
                Ok(ty) => Ok(ty.clone()),
                Err((a, b)) => Err(
                    IfBranchesDoesntMatch(format!("True Branch: {}, False Branch: {}", a, b))
                )
            }
        }
        Expr::Unit => {
            Ok(Type::Unit)
        }
        Expr::Literal(lit) => {
            Ok(type_of_literal(lit, expected))
        }
    }
}

fn type_of_literal(lit: &Literal, expected: Option<&Type>) -> Type {
    match lit {
        Literal::Int(_) => {
            match expected {
                Some(ty) => {
                    match ty {
                        Type::Var(_) => Type::Var("number".s()),
                        Type::Tag(name, _) => {
                            match name.as_str() {
                                "Int" => Type::Tag("Int".s(), vec![]),
                                "Float" => Type::Tag("Float".s(), vec![]),
                                _ => Type::Var("number".s())
                            }
                        }
                        _ => Type::Tag("Int".s(), vec![])
                    }
                }
                _ => Type::Var("number".s())
            }
        }
        Literal::Float(_) => Type::Tag("Float".s(), vec![]),
        Literal::Char(_) => Type::Tag("Char".s(), vec![]),
        Literal::String(_) => Type::Tag("String".s(), vec![]),
    }
}

fn type_of_app(env: &mut StaticEnv, fun: &Expr, arg: &Expr) -> Result<Type, TypeError> {
    // example of variable type inference:
    // sum = (+) 1.5

    let function = analyze_expression(env, None, fun)?;
    // (+) : number -> number -> number

    if let Type::Fun(ref argument, ref result) = function {
        // argument: number
        // result: number -> number

        let input = analyze_expression(env, Some(argument), arg)?;
        // Float

        if is_assignable(&**argument, &input) {
            // true

            let mut vars: HashMap<String, Type> = HashMap::new();
            find_var_replacements(&mut vars, &input, argument);
            // vars: [number => Float], change number to float

            let output = replace_vars_with_concrete_types(&vars, result);
            // Float

            backtrack_expr(env, &vars, fun);
            // env: [number => Float]

            Ok(output)
        } else {
            Err(ArgumentsDoNotMatch(format!("Expected argument: {}, found: {}", argument, input)))
        }
    } else {
        Err(NotAFunction(format!("Expected function found: {}, (in: {}, out: {})", function, fun, arg)))
    }
}

fn expr_tree_to_expr(tree: ExprTree) -> Expr {
    match tree {
        ExprTree::Leaf(e) => e,
        ExprTree::Branch(op, left, right) => {
            Expr::Application(
                Box::new(Expr::Application(
                    Box::new(Expr::Ref(op)),
                    Box::new(expr_tree_to_expr(*left)),
                )),
                Box::new(expr_tree_to_expr(*right)),
            )
        }
    }
}

fn backtrack_expr(env: &mut StaticEnv, vars: &HashMap<String, Type>, expr: &Expr) {
    match expr {
        Expr::Unit => {}
        Expr::Literal(_) => {}
        Expr::RecordAccess(_) => {}
        Expr::Tuple(items) => {
            items.iter().for_each(|i| backtrack_expr(env, vars, i));
        }
        Expr::List(items) => {
            items.iter().for_each(|i| backtrack_expr(env, vars, i));
        }
        Expr::Record(items) => {
            items.iter().for_each(|(_, i)| backtrack_expr(env, vars, i));
        }
        Expr::RecordUpdate(_, items) => {
            items.iter().for_each(|(_, i)| backtrack_expr(env, vars, i));
        }
        Expr::RecordField(e, _) => {
            backtrack_expr(env, vars, e);
        }
        Expr::If(a, b, c) => {
            backtrack_expr(env, vars, a);
            backtrack_expr(env, vars, b);
            backtrack_expr(env, vars, c);
        }
        Expr::Lambda(_, e) => {
            backtrack_expr(env, vars, e);
        }
        Expr::Application(a, b) => {
            backtrack_expr(env, vars, a);
            backtrack_expr(env, vars, b);
        }
        Expr::Let(_, e) => {
            backtrack_expr(env, vars, e);
        }
        Expr::Case(e, items) => {
            backtrack_expr(env, vars, e);
            items.iter().for_each(|(_, i)| backtrack_expr(env, vars, i));
        }
        Expr::OpChain(exprs, ops) => {
            match create_expr_tree(exprs, ops) {
                Ok(tree) => backtrack_expr(env, vars, &expr_tree_to_expr(tree)),
                Err(_) => {}
            }
        }
        Expr::QualifiedRef(path, name) => {
            let full_name = qualified_name(path, name);

            backtrack_expr(env, vars, &Expr::Ref(full_name))
        }
        Expr::Ref(variable) => {
            match env.find_definition(variable) {
                Some(variable_type) => {
                    env.replace(variable, replace_vars_with_concrete_types(vars, &variable_type))
                }
                None => {}
            }
        }
    }
}

pub fn get_adt_type(name: &String, vars: &Vec<Value>, adt: Arc<Adt>) -> Type {
    let variant = adt.variants.iter().find(|var| &var.name == name).unwrap();

    let mut var_replacement: HashMap<String, Type> = HashMap::new();
    let value_types: Vec<Type> = vars.iter().map(|v| type_of_value(v)).collect();

    find_var_replacements(&mut var_replacement, &Type::Tuple(value_types), &Type::Tuple(variant.types.clone()));

    let final_types = adt.types.iter()
        .map(|ty| {
            var_replacement.get(ty).cloned().unwrap_or_else(|| Type::Var(ty.clone()))
        })
        .collect();

    Type::Tag(adt.name.clone(), final_types)
}

fn replace_vars_with_concrete_types(vars: &HashMap<String, Type>, ty: &Type) -> Type {
    match ty {
        Type::Var(name) => {
            vars.get(name).unwrap_or(ty).clone()
        }
        Type::Tag(name, sub) => {
            Type::Tag(name.clone(), sub.iter().map(|i| replace_vars_with_concrete_types(vars, i)).collect())
        }
        Type::Fun(a, b) => {
            Type::Fun(
                Box::from(replace_vars_with_concrete_types(vars, a)),
                Box::from(replace_vars_with_concrete_types(vars, b)),
            )
        }
        Type::Unit => {
            Type::Unit
        }
        Type::Tuple(sub) => {
            Type::Tuple(sub.iter().map(|i| replace_vars_with_concrete_types(vars, i)).collect())
        }
        Type::Record(sub) => {
            Type::Record(sub.iter().map(|(n, i)| (n.clone(), replace_vars_with_concrete_types(vars, i))).collect())
        }
        Type::RecExt(name, sub) => {
            Type::RecExt(name.clone(), sub.iter().map(|(n, i)| (n.clone(), replace_vars_with_concrete_types(vars, i))).collect())
        }
    }
}

// Float, number => [number => Float]
// Float -> Float, number -> number => [number => Float]
// number -> Float, number -> number => [number => Float]
fn find_var_replacements(vars: &mut HashMap<String, Type>, arg: &Type, fun_in: &Type) {
    match fun_in {
        Type::Var(name) => {
            if let Type::Var(_) = arg {
                if name != "number" {
                    vars.insert(name.clone(), arg.clone());
                }
            } else {
                vars.insert(name.clone(), arg.clone());
            }
        }
        Type::Tag(_, sub_in) => {
            if let Type::Tag(_, sub_arg) = arg {
                sub_in.iter().zip(sub_arg).for_each(|(i, a)| find_var_replacements(vars, a, i));
            }
        }
        Type::Fun(a_in, b_in) => {
            if let Type::Fun(a_arg, b_arg) = arg {
                find_var_replacements(vars, a_arg, a_in);
                find_var_replacements(vars, b_arg, b_in);
            }
        }
        Type::Unit => {}
        Type::Tuple(sub_in) => {
            if let Type::Tuple(sub_arg) = arg {
                sub_in.iter().zip(sub_arg).for_each(|(i, a)| find_var_replacements(vars, a, i));
            }
        }
        Type::Record(sub_in) => {
            if let Type::Record(sub_arg) = arg {
                // TODO entries may not be ordered
                sub_in.iter().zip(sub_arg).for_each(|((_, i), (_, a))| find_var_replacements(vars, a, i));
            }
        }
        Type::RecExt(_, sub_in) => {
            if let Type::RecExt(_, sub_arg) = arg {
                // TODO entries may not be ordered
                sub_in.iter().zip(sub_arg).for_each(|((_, i), (_, a))| find_var_replacements(vars, a, i));
            }
        }
    }
}

fn type_from_expected(env: &mut StaticEnv, expected: &Type, value: &Type) -> Type {
    match value {
        Type::Var(_) => {
            expected.clone()
        }
        Type::Tag(val_name, val_sub) => {
            if let Type::Tag(exp_name, exp_sub) = expected {
                if val_name == exp_name {
                    let sub = exp_sub.iter()
                        .zip(val_sub)
                        .map(|(e, v)| type_from_expected(env, e, v))
                        .collect::<Vec<Type>>();

                    Type::Tag(val_name.clone(), sub)
                } else {
                    value.clone()
                }
            } else {
                value.clone()
            }
        }
        Type::Fun(val_in, val_out) => {
            if let Type::Fun(exp_in, exp_out) = expected {
                Type::Fun(
                    Box::from(type_from_expected(env, exp_in, val_in)),
                    Box::from(type_from_expected(env, exp_out, val_out)),
                )
            } else {
                value.clone()
            }
        }
        Type::Unit => {
            value.clone()
        }
        Type::Tuple(val) => {
            if let Type::Tuple(exp) = expected {
                Type::Tuple(
                    exp.iter().zip(val)
                        .map(|(e, v)| type_from_expected(env, e, v))
                        .collect::<Vec<Type>>()
                )
            } else {
                value.clone()
            }
        }
        Type::Record(val) => {
            if let Type::Record(exp) = expected {
                Type::Record(
                    exp.iter().zip(val)
                        .map(|((ne, e), (_, v))| (ne.clone(), type_from_expected(env, e, v)))
                        .collect::<Vec<(String, Type)>>()
                )
            } else {
                value.clone()
            }
        }
        Type::RecExt(val_name, val) => {
            if let Type::RecExt(exp_name, exp) = expected {
                if val_name == exp_name {
                    Type::Record(
                        exp.iter().zip(val)
                            .map(|((ne, e), (_, v))| (ne.clone(), type_from_expected(env, e, v)))
                            .collect::<Vec<(String, Type)>>()
                    )
                } else {
                    value.clone()
                }
            } else {
                value.clone()
            }
        }
    }
}

fn rename_variables(env: &mut StaticEnv, vars: &mut HashMap<String, String>, ty: Type) -> Type {
    match ty {
        Type::Unit => {
            Type::Unit
        }
        Type::Var(name) => {
            match vars.get(&name).cloned() {
                Some(name) => {
                    Type::Var(name)
                }
                None => {
                    if name == "number" {
                        Type::Var(name)
                    } else {
                        let new_name = env.next_name();
                        vars.insert(name, new_name.clone());
                        Type::Var(new_name)
                    }
                }
            }
        }
        Type::Tag(name, sub) => {
            Type::Tag(name, sub.into_iter().map(|i| rename_variables(env, vars, i)).collect())
        }
        Type::Fun(i, o) => {
            Type::Fun(Box::from(rename_variables(env, vars, *i)), Box::from(rename_variables(env, vars, *o)))
        }
        Type::Tuple(sub) => {
            Type::Tuple(sub.into_iter().map(|i| rename_variables(env, vars, i)).collect())
        }
        Type::Record(sub) => {
            Type::Record(sub.into_iter().map(|(s, i)| (s, rename_variables(env, vars, i))).collect())
        }
        Type::RecExt(name, sub) => {
            Type::RecExt(name, sub.into_iter().map(|(s, i)| (s, rename_variables(env, vars, i))).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use parsers::from_code;

    use super::*;

    #[test]
    fn check_unit() {
        let expr = from_code(b"()");
        let mut env = StaticEnv::new();
        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Unit));
    }

    #[test]
    fn check_literal() {
        let expr = from_code(b"123");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Var("number".s())));
    }

    #[test]
    fn check_fun() {
        let expr = from_code(b"fun 123");
        let mut env = StaticEnv::new();

        env.add_definition("fun", Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Tag("Int".s(), vec![])),
        ));

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_if() {
        let expr = from_code(b"if True then 1 else 0");
        let mut env = StaticEnv::new();

        env.add_definition("True", Type::Tag("Bool".s(), vec![]));

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Var("number".s())));
    }

    #[test]
    fn check_lambda() {
        let expr = from_code(b"\\x -> 1");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Fun(
            Box::new(Type::Var("a".s())),
            Box::new(Type::Var("number".s())),
        )));
    }

    #[test]
    fn check_list() {
        let expr = from_code(b"[1, 2, 3]");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag(
            "List".s(), vec![Type::Var("number".s())],
        )));
    }

    #[test]
    fn check_bad_list() {
        let expr = from_code(b"[1, 2, 'a']");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Err(
            ListNotHomogeneous(
                "List of 'number', but found element 'Char' at index: 2".s()
            )
        ));
    }

    #[test]
    fn check_record() {
        let expr = from_code(b"{ a = 1, b = \"Hi\" }");
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
        let expr = from_code(b"1 + 2");
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
        let expr = from_code(b"(1, \"a\", ())");
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
        let expr = from_code(b"{ x | a = 0 }");
        let mut env = StaticEnv::new();

        let record_type = Type::Record(vec![
            ("a".s(), Type::Var("number".s()))
        ]);

        env.add_definition("x", record_type.clone());

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(record_type));
    }

    #[test]
    fn check_case() {
        let expr = from_code(b"case 0 of\n 0 -> \"a\"\n _ -> \"b\"");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag("String".s(), vec![])));
    }

    #[test]
    fn check_case2() {
        let expr = from_code(b"case 0 of\n 0 -> 1\n _ -> \"b\"");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Err(CaseBranchDontMatchReturnType("".s())));
    }
}
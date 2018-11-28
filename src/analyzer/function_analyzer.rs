use analyzer::expression_analyzer::analyze_expression;
use analyzer::static_env::StaticEnv;
use analyzer::TypeError;
use ast::*;
use util::build_fun_type;
use util::create_vec_inv;
use util::StringConversion;
use analyzer::pattern_analyzer::*;
use analyzer::type_helper::is_assignable;

pub fn analyze_let_destructuring(env: &mut StaticEnv, pattern: &Pattern, expr: &Expr) -> Result<Vec<(String, Type)>, TypeError> {
    let (pat_ty, vars) = analyze_pattern(env, pattern)
        .map_err(|e| TypeError::InvalidPattern(e))?;

    let ty = analyze_expression(env, Some(&pat_ty), expr)?;

    if is_assignable(&pat_ty, &ty) {
        Ok(vars)
    } else {
        Err(TypeError::DefinitionTypeAndReturnTypeMismatch)
    }
}

pub fn analyze_function(env: &mut StaticEnv, fun: &Definition) -> Result<Type, TypeError> {
    let Definition { header, name, patterns, expr } = &fun;

    println!("analyze_function: {}", name);

    let save = env.name_seq.save();
    let (argument_types, local_vars) = analyze_function_arguments(env, patterns, header)?;

    env.enter_block();
    for (arg_name, ty) in &local_vars {
        env.add_definition(arg_name, ty.clone());
    }

    if let Some(ty) = header {
        // Enable recursivity
        env.add_definition(name, ty.clone());

        let fn_types = unpack_types(ty);
        let return_type = fn_types.last().expect("Expected last value to exist");

        // Infer return type and update env replacing var types with concrete types
        let expr_result = analyze_expression(env, Some(return_type), expr);

        env.exit_block();
        env.name_seq.restore(save);

        match expr_result {
            Ok(_) => {
                Ok(ty.clone())
            }
            Err(e) => {
                println!("Error in function: '{}'", name);
                Err(e)
            }
        }
    } else {

        // infer return type based on the return expression
        let self_type = create_vec_inv(&argument_types, Type::Var("z".s()));
        env.add_definition(name, build_fun_type(&self_type));

        // Infer return type and update env replacing var types with concrete types
        let return_type = analyze_expression(env, None, expr);
        let mut final_arg_types: Vec<Type> = vec![];

        // Update argument variable with concrete types
        'outer: for arg in &argument_types {
            if let Type::Var(arg_var_name) = arg {

                // search in local variables for the type of this variable,
                // this is needed because the number of arguments and local variables can be different
                for (name, ty) in &local_vars {
                    if let Type::Var(local_var_name) = ty {
                        if local_var_name == arg_var_name {
                            final_arg_types.push(env.find_definition(name).unwrap());
                            continue 'outer;
                        }
                    }
                }

                panic!("Unable to find variable '{}' in {:?}, for function: {}", &arg, local_vars, name);
            } else {
                final_arg_types.push(arg.clone());
            }
        }

        env.exit_block();
        env.name_seq.restore(save);

        match return_type {
            Ok(return_type) => {
                final_arg_types.push(return_type);

                Ok(build_fun_type(&final_arg_types))
            }
            Err(e) => {
                println!("Error in function: '{}'", name);
                Err(e)
            }
        }
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

pub fn analyze_function_arguments(env: &mut StaticEnv, patterns: &Vec<Pattern>, func_ty: &Option<Type>) -> Result<(Vec<Type>, Vec<(String, Type)>), TypeError> {
    let mut arguments: Vec<Type> = vec![];
    let mut argument_vars: Vec<(String, Type)> = vec![];

    let iter: Vec<(Option<Type>, &Pattern)> = match func_ty {
        Some(ty) => {
            let list = unpack_types(ty);

            if patterns.len() > list.len() {
                println!("patterns: {:?}", patterns);
                println!("list: {:?}", list);
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
            return Err(TypeError::InvalidPattern(PatternMatchingError::PatternNotExhaustive(patt.clone())));
        }

        let (ty, vars) = match ty {
            Some(ty) => {
                analyze_pattern_with_type(env, patt, ty)
                    .map_err(|e| TypeError::InvalidPattern(e))?
            }
            None => {
                analyze_pattern(env, patt)
                    .map_err(|e| TypeError::InvalidPattern(e))?
            }
        };

        arguments.push(ty);
        for pair in vars {
            argument_vars.push(pair);
        }
    }

    Ok((arguments, argument_vars))
}


#[cfg(test)]
mod tests {
    use ast::Statement;
    use parsers::from_code_stm;

    use super::*;
    use constructors::*;

    fn from_code_def(code: &[u8]) -> Definition {
        let stm = from_code_stm(code);
        match stm {
            Statement::Def(def) => def,
            _ => panic!("Expected definition but found: {:?}", stm)
        }
    }

    fn format_type(env: &mut StaticEnv, def: &Definition) -> String {
        format!("{}", analyze_function(env, def).expect("Run into type error"))
    }

    #[test]
    fn check_constant() {
        let def = from_code_def(b"const = 1");
        let mut env = StaticEnv::new();

        assert_eq!(format_type(&mut env, &def), "number");
    }

    #[test]
    fn check_identity() {
        let def = from_code_def(b"id arg1 = arg1");
        let mut env = StaticEnv::new();

        assert_eq!(format_type(&mut env, &def), "a -> a");
    }

    #[test]
    fn check_var_to_number() {
        let def = from_code_def(b"sum arg1 arg2 = arg1 + arg2");
        let mut env = StaticEnv::new();

        env.add_definition("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut env, &def), "number -> number -> number");
    }

    #[test]
    fn check_number_to_float() {
        let def = from_code_def(b"sum arg1 = arg1 + 1.5");
        let mut env = StaticEnv::new();

        env.add_definition("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut env, &def), "Float -> Float");
    }

    #[test]
    fn check_from_number_to_float() {
        let def = from_code_def(b"sum = (+) 1.5");
        let mut env = StaticEnv::new();

        env.add_definition("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut env, &def), "Float -> Float");
    }

    #[test]
    fn check_list_coercion() {
        let def = from_code_def(b"my = [1, 1.5]");
        let mut env = StaticEnv::new();

        assert_eq!(format_type(&mut env, &def), "List Float");
    }

    #[test]
    fn check_list_coercion2() {
        let def = from_code_def(b"my b = [1, 1.5, b]");
        let mut env = StaticEnv::new();

        assert_eq!(format_type(&mut env, &def), "Float -> List Float");
    }

    #[test]
    fn check_variable_separation() {
        let def = from_code_def(b"my a b = [a, b]");
        let mut env = StaticEnv::new();

        assert_eq!(format_type(&mut env, &def), "a -> a -> List a");
    }

    #[test]
    fn check_variable_separation2() {
        let def = from_code_def(b"my = (func, func)");
        let mut env = StaticEnv::new();

        env.add_definition("func", Type::Fun(
            Box::from(Type::Var("a".s())),
            Box::from(Type::Var("a".s())),
        ));

        assert_eq!(format_type(&mut env, &def), "( a -> a, b -> b )");
    }

    #[test]
    fn analyze_patterns_1() {
        analyze_pattern_test(
            type_int(),
            pattern_var("a"),
            "Int",
            r#"[("a", Tag("Int", []))]"#,
        );
    }

    #[test]
    fn analyze_patterns_2() {
        // this should not pass, but there is not parameter count checking
        analyze_pattern_test(
            type_tag_args("Bool", vec![type_var("item")]),
            pattern_tag_args("True", vec![pattern_var("a")]),
            "Bool item",
            r#"[("a", Var("item"))]"#,
        );
    }

    #[test]
    fn analyze_patterns_3() {
        analyze_pattern_test(
            type_int(),
            pattern_wildcard(),
            "Int",
            r#"[]"#,
        );
    }

    #[test]
    fn analyze_patterns_4() {
        analyze_pattern_test(
            type_unit(),
            pattern_unit(),
            "()",
            r#"[]"#,
        );
    }

    #[test]
    fn analyze_patterns_5() {
        analyze_pattern_test(
            type_tuple(vec![type_int(), type_unit()]),
            pattern_tuple(vec![pattern_var("a"), pattern_unit()]),
            "( Int, () )",
            r#"[("a", Tag("Int", []))]"#,
        );
    }

    #[test]
    fn analyze_patterns_6() {
        analyze_pattern_test(
            type_list(type_int()),
            pattern_list(vec![pattern_var("a"), pattern_var("b")]),
            "List Int",
            r#"[("a", Tag("Int", [])), ("b", Tag("Int", []))]"#,
        );
    }

    #[test]
    fn analyze_patterns_7() {
        analyze_pattern_test(
            type_record(vec![("x", type_int())]),
            pattern_record(vec!["x"]),
            "{ x : Int }",
            r#"[("x", Tag("Int", []))]"#,
        );
    }

    #[test]
    fn analyze_patterns_8() {
        analyze_pattern_test(
            type_list(type_int()),
            pattern_cons(pattern_var("x"), pattern_var("xs")),
            "List Int",
            r#"[("x", Tag("Int", [])), ("xs", Tag("List", [Tag("Int", [])]))]"#,
        );
    }

    #[test]
    fn analyze_patterns_9() {
        analyze_pattern_test(
            type_int(),
            pattern_int(1),
            "Int",
            r#"[]"#,
        );
    }

    #[test]
    fn analyze_patterns_10() {
        analyze_pattern_test(
            type_int(),
            pattern_alias(pattern_int(1), "x"),
            "Int",
            r#"[("x", Tag("Int", []))]"#,
        );
    }

    fn analyze_pattern_test(ty: Type, pattern: Pattern, type_str: &str, vars_str: &str) {
        let mut env = StaticEnv::new();
        let (res_ty, vars) = analyze_pattern_with_type(&mut env, &pattern, ty)
            .expect("Error");

        assert_eq!(format!("{}", res_ty), type_str);
        assert_eq!(format!("{:?}", vars), vars_str);
    }
}
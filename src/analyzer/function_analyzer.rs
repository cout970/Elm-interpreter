use analyzer::expression_analyzer::analyze_expression;
use analyzer::function_analyzer::PatternMatchingError::*;
use analyzer::static_env::StaticEnv;
use analyzer::TypeError;
use ast::*;
use constructors::*;
use util::build_fun_type;
use util::create_vec;
use util::create_vec_inv;
use util::StringConversion;
use util::VecExt;

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
}

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
//                println!("patterns: {:?}", patterns);
//                println!("list: {:?}", list);
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
            return Err(TypeError::InvalidPattern(PatternNotExhaustive(patt.clone())));
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

fn is_exhaustive(pattern: &Pattern) -> bool {
    match pattern {
        Pattern::Var(_) => true,
        Pattern::Adt(_, _) => true,
        Pattern::Wildcard => true,
        Pattern::Unit => true,
        Pattern::Tuple(sub_patterns) => {
            sub_patterns.iter().all(|p| is_exhaustive(p))
        }
        Pattern::List(_) => false,
        Pattern::Alias(pat, _) => is_exhaustive(pat),
        Pattern::BinaryOp(_, _, _) => false,
        Pattern::Record(_) => true,
        Pattern::Literal(_) => false,
    }
}

pub fn analyze_pattern(env: &mut StaticEnv, pattern: &Pattern) -> Result<(Type, Vec<(String, Type)>), PatternMatchingError> {
    match pattern {
        Pattern::Var(name) => {
            let ty_name = env.name_seq.next();
            Ok((Type::Var(ty_name.clone()), vec![(name.to_owned(), Type::Var(ty_name))]))
        }
        Pattern::Adt(name, sub_patterns) => {
            let mut sub_input = Vec::new();
            let mut sub_vars = Vec::new();

            for pattern in sub_patterns {
                let (ty, vars) = analyze_pattern(env, pattern)?;
                sub_input.push(ty);
                for v in vars {
                    sub_vars.push(v);
                }
            }

            let adt = env.find_adt_variant(name)
                .ok_or_else(|| PatternMatchingError::UnknownAdtVariant(name.clone()))?;

            Ok((Type::Tag(adt.name.clone(), sub_input), sub_vars))
        }
        Pattern::Wildcard => {
            Ok((Type::Var(env.name_seq.next()), vec![]))
        }
        Pattern::Unit => {
            Ok((Type::Unit, vec![]))
        }
        Pattern::Tuple(sub_patterns) => {
            let mut sub_input = Vec::new();
            let mut sub_vars = Vec::new();

            for pattern in sub_patterns {
                let (ty, vars) = analyze_pattern(env, pattern)?;
                sub_input.push(ty);
                for v in vars {
                    sub_vars.push(v);
                }
            }

            Ok((Type::Tuple(sub_input), sub_vars))
        }
        Pattern::List(sub_patterns) => {
            let mut sub_input = Vec::new();
            let mut sub_vars = Vec::new();

            for pattern in sub_patterns {
                let (ty, vars) = analyze_pattern(env, pattern)?;
                sub_input.push(ty);
                for v in vars {
                    sub_vars.push(v);
                }
            }

            let ty = calculate_common_type(&sub_input)
                .map_err(|(expected, found)| ListPatternsAreNotHomogeneous(expected.clone(), found.clone()))?;

            Ok((type_list(ty.clone()), sub_vars))
        }
        Pattern::BinaryOp(operand, left, right) => {
            if operand != "::" {
                return Err(UnknownOperatorPattern(operand.clone()));
            }

            let (_, left_vars) = analyze_pattern(env, left)?;
            let (right_ty, right_vars) = analyze_pattern(env, right)?;

            get_list_param_type(&right_ty)?;

            Ok((right_ty, left_vars.join_vec(&right_vars)))
        }
        Pattern::Record(entry_names) => {
            let mut entries = Vec::new();

            for name in entry_names {
                entries.push((name.to_owned(), Type::Var(env.name_seq.next())));
            }

            Ok((Type::Record(entries.clone()), entries))
        }
        Pattern::Literal(literal) => {
            match literal {
                Literal::Int(_) => Ok((type_int(), vec![])),
                Literal::Float(_) => Ok((type_float(), vec![])),
                Literal::String(_) => Ok((type_string(), vec![])),
                Literal::Char(_) => Ok((type_char(), vec![])),
            }
        }
        Pattern::Alias(pat, alias) => {
            let (ret_ty, vars) = analyze_pattern(env, pat)?;
            Ok((ret_ty.clone(), create_vec((alias.to_owned(), ret_ty), vars)))
        }
    }
}

pub fn analyze_pattern_with_type(env: &mut StaticEnv, pattern: &Pattern, ty: Type) -> Result<(Type, Vec<(String, Type)>), PatternMatchingError> {
    match pattern {
        Pattern::Var(name) => {
            Ok((ty.clone(), vec![(name.to_owned(), ty)]))
        }
        Pattern::Adt(name, sub_patterns) => {
            let mut sub_input = Vec::new();
            let mut sub_vars = Vec::new();

            let adt = env.find_adt_variant(name)
                .ok_or_else(|| PatternMatchingError::UnknownAdtVariant(name.clone()))?;

            let variant = adt.variants.iter().find(|v| &v.name == name).unwrap();

            let params = if let Type::Tag(ty_name, _) = ty.clone() {
                if ty_name == adt.name {
                    assert_eq!(variant.types.len(), sub_patterns.len());
                    variant.types.clone()
                } else {
                    return Err(PatternMatchingError::ExpectedAdt(adt.name.clone(), ty));
                }
            } else {
                return Err(PatternMatchingError::ExpectedAdt(adt.name.clone(), ty.clone()));
            };

            for (pattern, param_ty) in sub_patterns.iter().zip(params) {
                let (ty, vars) = analyze_pattern_with_type(env, pattern, param_ty)?;
                sub_input.push(ty);
                for v in vars {
                    sub_vars.push(v);
                }
            }

            Ok((Type::Tag(adt.name.clone(), sub_input), sub_vars))
        }
        Pattern::Wildcard => {
            Ok((ty, vec![]))
        }
        Pattern::Unit => {
            if ty != Type::Unit {
                return Err(PatternMatchingError::ExpectedUnit(ty));
            }
            Ok((Type::Unit, vec![]))
        }
        Pattern::Tuple(sub_patterns) => {
            let mut sub_input = Vec::new();
            let mut sub_vars = Vec::new();

            match ty {
                Type::Tuple(sub_types) => {
                    assert_eq!(sub_types.len(), sub_patterns.len());

                    for (pattern, ty) in sub_patterns.iter().zip(sub_types) {
                        let (ty, vars) = analyze_pattern_with_type(env, pattern, ty)?;
                        sub_input.push(ty);
                        for v in vars {
                            sub_vars.push(v);
                        }
                    }

                    Ok((Type::Tuple(sub_input), sub_vars))
                }
                _ => {
                    return Err(PatternMatchingError::ExpectedTuple(pattern.clone(), ty));
                }
            }
        }
        Pattern::List(sub_patterns) => {
            let mut sub_vars = Vec::new();
            let list_param = get_list_param_type(&ty)?;

            for pattern in sub_patterns {
                let (_, vars) = analyze_pattern_with_type(env, pattern, list_param.clone())?;
                for v in vars {
                    sub_vars.push(v);
                }
            }

            Ok((type_list(list_param.clone()), sub_vars))
        }
        Pattern::Record(pattern_entries) => {
            let mut entries = Vec::new();
            let pairs = get_record_entries(&ty)?;

            for pattern_name in pattern_entries {
                let (name, ty) = pairs.iter()
                    .find(|(name, _)| name == pattern_name)
                    .ok_or_else(|| PatternMatchingError::InvalidRecordEntryName(pattern_name.clone()))?;

                entries.push((name.to_owned(), ty.clone()));
            }

            Ok((ty.clone(), entries))
        }
        Pattern::BinaryOp(operand, left, right) => {
            if operand != "::" {
                return Err(UnknownOperatorPattern(operand.clone()));
            }

            let list_param = get_list_param_type(&ty)?;
            let (_, left_vars) = analyze_pattern_with_type(env, left, list_param.clone())?;
            let (_, right_vars) = analyze_pattern_with_type(env, right, ty.clone())?;

            Ok((type_list(list_param.clone()), left_vars.join_vec(&right_vars)))
        }
        Pattern::Literal(literal) => {
            match literal {
                Literal::Int(_) => {
                    check_type_literal(&ty, "Int")?;
                    Ok((ty, vec![]))
                }
                Literal::Float(_) => {
                    check_type_literal(&ty, "Float")?;
                    Ok((ty, vec![]))
                }
                Literal::String(_) => {
                    check_type_literal(&ty, "String")?;
                    Ok((ty, vec![]))
                }
                Literal::Char(_) => {
                    check_type_literal(&ty, "Char")?;
                    Ok((ty, vec![]))
                }
            }
        }
        Pattern::Alias(pat, alias) => {
            let (ret_ty, vars) = analyze_pattern_with_type(env, pat, ty)?;
            Ok((ret_ty.clone(), create_vec((alias.to_owned(), ret_ty), vars)))
        }
    }
}

fn check_type_literal(ty: &Type, literal_name: &str) -> Result<(), PatternMatchingError> {
    match ty {
        Type::Tag(name, params) => {
            if name != literal_name || !params.is_empty() {
                Err(PatternMatchingError::ExpectedLiteral(literal_name.to_owned(), ty.clone()))
            } else {
                Ok(())
            }
        }
        _ => Err(PatternMatchingError::ExpectedLiteral(literal_name.to_owned(), ty.clone()))
    }
}

fn get_list_param_type(ty: &Type) -> Result<&Type, PatternMatchingError> {
    match ty {
        Type::Tag(type_name, params) => {
            if type_name != "List" || params.len() != 1 {
                return Err(PatternMatchingError::ExpectedListType(ty.clone()));
            }

            Ok(&params[0])
        }
        _ => {
            Err(PatternMatchingError::ExpectedListType(ty.clone()))
        }
    }
}

fn get_record_entries(ty: &Type) -> Result<&Vec<(String, Type)>, PatternMatchingError> {
    match ty {
        Type::Record(entries) => Ok(entries),
        _ => Err(PatternMatchingError::ExpectedRecord(ty.clone())),
    }
}


pub fn calculate_common_type(types: &[Type]) -> Result<&Type, (&Type, &Type)> {
    let first = types.first().unwrap();

    for i in 1..types.len() {
        if !is_assignable(first, &types[i]) {
            return Err((first, &types[i]));
        }
    }
    Ok(first)
}

pub fn is_assignable(expected: &Type, found: &Type) -> bool {
    if expected == found { return true; }

    if let Type::Var(_) = found {
        match expected {
            Type::Var(_) => (),
            _ => {
                return true;
            }
        }
    }

    match expected {
        Type::Var(name) => {
            match name.as_str() {
                "number" => {
                    match found {
                        Type::Var(_) => true,
                        Type::Tag(ty_name, _) => ty_name == "Int" || ty_name == "Float",
                        _ => false
                    }
                }
                _ => true
            }
        }
        Type::Tag(name, sub) => {
            match found {
                Type::Tag(ty_name, ty_sub) => {
                    ty_name == name && sub.iter().zip(ty_sub).all(|(a, b)| is_assignable(a, b))
                }
                _ => false
            }
        }
        Type::Fun(input, output) => {
            match found {
                Type::Fun(a, b) => is_assignable(input, a) && is_assignable(output, b),
                _ => false
            }
        }
        Type::Tuple(sub) => {
            match found {
                Type::Tuple(ty_sub) => {
                    sub.iter().zip(ty_sub).all(|(a, b)| is_assignable(a, b))
                }
                _ => false
            }
        }
        Type::Record(entries) => {
            match found {
                Type::Record(entries_ty) => {
                    entries.iter().all(|(name, ty)| entries_ty.iter().any(|(n, t)|
                        n == name && is_assignable(ty, t)
                    ))
                }
                _ => false
            }
        }
        Type::RecExt(name, entries) => {
            match found {
                Type::RecExt(name_ty, entries_ty) => {
                    name == name_ty && entries.iter()
                        .all(|(name, ty)| entries_ty.iter().any(|(n, t)|
                            n == name && is_assignable(ty, t)
                        ))
                }
                _ => false
            }
        }
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use ast::Statement;
    use parsers::from_code_stm;

    use super::*;

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
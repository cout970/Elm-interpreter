use analyzer::expression_analyzer::analyze_expression;
use analyzer::function_analyzer::PatternMatchingError::*;
use analyzer::static_env::StaticEnv;
use analyzer::TypeError;
use analyzer::TypeError::InternalError;
use analyzer::TypeError::UnableToCalculateFunctionType;
use std::collections::HashMap;
use ast::Definition;
use ast::Expr;
use ast::Literal;
use ast::Pattern;
use ast::Type;
use types::Value;
use util::build_fun_type;
use util::create_vec_inv;
use util::name_sequence::NameSequence;
use util::StringConversion;
use util::VecExt;

#[derive(Clone, Debug, PartialEq)]
pub enum PatternMatchingError {
    ListPatternsAreNotHomogeneous(Type, Type),
    UnknownOperatorPattern(String),
    ExpectedListType(Type),
    PatternNotExhaustive(Pattern),
}

pub fn analyze_function(env: &mut StaticEnv, fun: &Definition) -> Result<Type, TypeError> {
    let Definition { name, patterns, expr, .. } = &fun;

    let save = env.name_seq.save();
    let (argument_types, local_vars) = analyze_function_arguments(&mut env.name_seq, patterns)?;

    env.enter_block();
    for (arg_name, value) in &local_vars {
        env.add(arg_name, value.clone());
    }

    // Enable recursivity
    let self_type = create_vec_inv(&argument_types, Type::Var("z".s()));
    env.add(name, build_fun_type(&self_type));

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
                        final_arg_types.push(env.find(name).unwrap());
                        continue 'outer;
                    }
                }
            }

            panic!("Unable to find variable '{}' in {:?}", &arg, local_vars);
        } else {
            final_arg_types.push(arg.clone());
        }
    }

    env.exit_block();
    env.name_seq.restore(save);

    // delayed ? to avoid inconsistent environment state
    final_arg_types.push(return_type?);

    Ok(build_fun_type(&final_arg_types))
}

pub fn analyze_function_arguments(gen: &mut NameSequence, patterns: &Vec<Pattern>) -> Result<(Vec<Type>, Vec<(String, Type)>), TypeError> {
    let mut arguments: Vec<Type> = vec![];
    let mut argument_vars: Vec<(String, Type)> = vec![];

    for patt in patterns {
        if !is_exhaustive(patt) {
            return Err(TypeError::InvalidPattern(PatternNotExhaustive(patt.clone())));
        }

        let (ty, vars) = analyze_pattern(gen, patt)
            .map_err(|e| TypeError::InvalidPattern(e))?;

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
        Pattern::BinaryOp(_, _, _) => false,
        Pattern::Record(_) => true,
        Pattern::Literal(_) => false,
    }
}

fn analyze_pattern(gen: &mut NameSequence, pattern: &Pattern) -> Result<(Type, Vec<(String, Type)>), PatternMatchingError> {
    match pattern {
        Pattern::Var(name) => {
            let ty_name = gen.next();
            Ok((Type::Var(ty_name.clone()), vec![(name.to_owned(), Type::Var(ty_name))]))
        }
        Pattern::Adt(name, sub_patterns) => {
            let mut sub_input = Vec::new();
            let mut sub_vars = Vec::new();

            for pattern in sub_patterns {
                let (ty, vars) = analyze_pattern(gen, pattern)?;
                sub_input.push(ty);
                for v in vars {
                    sub_vars.push(v);
                }
            }

            Ok((Type::Tag(name.to_owned(), sub_input), sub_vars))
        }
        Pattern::Wildcard => {
            Ok((Type::Var(gen.next()), vec![]))
        }
        Pattern::Unit => {
            Ok((Type::Unit, vec![]))
        }
        Pattern::Tuple(sub_patterns) => {
            let mut sub_input = Vec::new();
            let mut sub_vars = Vec::new();

            for pattern in sub_patterns {
                let (ty, vars) = analyze_pattern(gen, pattern)?;
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
                let (ty, vars) = analyze_pattern(gen, pattern)?;
                sub_input.push(ty);
                for v in vars {
                    sub_vars.push(v);
                }
            }

            let ty = calculate_common_type(&sub_input)
                .map_err(|(expected, found)| ListPatternsAreNotHomogeneous(expected.clone(), found.clone()))?;

            Ok((Type::Tag("List".s(), vec![ty.clone()]), sub_vars))
        }
        Pattern::BinaryOp(operand, left, right) => {
            if operand != "::" {
                return Err(UnknownOperatorPattern(operand.clone()));
            }

            let (left_ty, left_vars) = analyze_pattern(gen, left)?;
            let (right_ty, right_vars) = analyze_pattern(gen, right)?;

            match right_ty {
                Type::Tag(ref name, _) => {
                    if name != "List" {
                        return Err(ExpectedListType(right_ty.clone()));
                    }
                }
                _ => return Err(ExpectedListType(right_ty.clone())),
            };

            Ok((Type::Tag("List".s(), vec![left_ty]), left_vars.join_vec(&right_vars)))
        }
        Pattern::Record(entry_names) => {
            let mut entries = Vec::new();

            for name in entry_names {
                entries.push((name.to_owned(), Type::Var(gen.next())));
            }

            Ok((Type::Record(entries.clone()), entries))
        }
        Pattern::Literal(literal) => {
            match literal {
                Literal::Int(_) => Ok((Type::Tag("Int".s(), vec![]), vec![])),
                Literal::Float(_) => Ok((Type::Tag("Float".s(), vec![]), vec![])),
                Literal::String(_) => Ok((Type::Tag("String".s(), vec![]), vec![])),
                Literal::Char(_) => Ok((Type::Tag("Char".s(), vec![]), vec![])),
            }
        }
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
    use nom::*;
    use nom::verbose_errors::*;
    use parsers::from_code_stm;
    use super::*;
    use ast::Statement;

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

        env.add("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut env, &def), "number -> number -> number");
    }

    #[test]
    fn check_number_to_float() {
        let def = from_code_def(b"sum arg1 = arg1 + 1.5");
        let mut env = StaticEnv::new();

        env.add("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut env, &def), "Float -> Float");
    }

    #[test]
    fn check_from_number_to_float() {
        let def = from_code_def(b"sum = (+) 1.5");
        let mut env = StaticEnv::new();

        env.add("+", build_fun_type(&vec![
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

        env.add("func", Type::Fun(
            Box::from(Type::Var("a".s())),
            Box::from(Type::Var("a".s())),
        ));

        assert_eq!(format_type(&mut env, &def), "( a -> a, b -> b )");
    }
}
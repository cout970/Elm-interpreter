use analyzer::expression_analyzer::analyze_expression;
use analyzer::function_analyzer::PatternMatchingError::*;
use analyzer::TypeError;
use analyzer::TypeError::InternalError;
use analyzer::TypeError::UnableToCalculateFunctionType;
use std::collections::HashMap;
use types::Definition;
use types::Expr;
use types::Literal;
use types::Pattern;
use types::Type;
use types::Value;
use types::ValueDefinition;
use util::build_fun_type;
use util::name_sequence::NameSequence;
use util::StringConversion;
use util::VecExt;
use analyzer::static_env::StaticEnv;
use util::create_vec_inv;

#[derive(Clone, Debug, PartialEq)]
pub enum PatternMatchingError {
    ListPatternsAreNotHomogeneous(Type, Type),
    UnknownOperatorPattern(String),
    ExpectedListType(Type),
    PatternNotExhaustive(Pattern),
}

pub fn analyze_function(env: &mut StaticEnv, fun: &Definition) -> Result<Type, TypeError> {
    let ValueDefinition { name, patterns, expr } = &fun.1;

    let (mut arguments, argument_vars) = analyze_function_arguments(patterns)?;

    env.enter_block();
    for (arg_name, value) in &argument_vars {
        env.add(arg_name, value.clone());
    }

    let self_type = create_vec_inv(&arguments, Type::Var("a".s()));
    env.add(name, build_fun_type(&self_type));

    let expr_type = analyze_expression(env, None, expr);
    env.enter_block();

    arguments.push(expr_type?);

    Ok(build_fun_type(&arguments))
}

pub fn analyze_function_arguments(patterns: &Vec<Pattern>) -> Result<(Vec<Type>, Vec<(String, Type)>), TypeError> {
    let mut arguments: Vec<Type> = vec![];
    let mut argument_vars: Vec<(String, Type)> = vec![];
    let mut gen = NameSequence::new();

    for patt in patterns {
        if !is_exhaustive(patt) {
            return Err(TypeError::InvalidPattern(PatternNotExhaustive(patt.clone())));
        }

        let (ty, vars) = analyze_pattern(&mut gen, patt)
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
                        Type::Tag(ty_name, _) => ty_name == "Int" || ty_name == "Float",
                        _ => false
                    }
                }
                _ => false
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
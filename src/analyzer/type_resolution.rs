use analyzer::environment::Environment;
use types::CurriedFunc;
use types::Fun;
use types::Type;
use types::Value;
use util::StringConversion;

fn type_tag_of(name: &str, args: &[Type]) -> Type {
    Type::Tag(name.to_owned(), args.to_vec())
}

fn type_simple_tag_of(name: &str) -> Type {
    Type::Tag(name.to_owned(), Vec::new())
}


pub fn get_value_type(value: &Value) -> Type {
    match value {
        Value::Unit => {
            Type::Unit
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
                Type::Tag("List".s(), vec![get_value_type(items.first().unwrap())])
            }
        }
        Value::Tuple(items) => {
            Type::Tuple(items.iter().map(|i| get_value_type(i)).collect())
        }
        Value::Record(items) => {
            Type::Record(items.iter().map(|(s, i)| (s.to_owned(), get_value_type(i))).collect())
        }
        Value::Adt(name, items) => {
            Type::Tag(name.to_owned(), items.iter().map(|i| get_value_type(i)).collect())
        }
        Value::Fun(CurriedFunc { fun, args, .. }) => {
            let fun_ty = match fun {
                Fun::Builtin(_, ty) => ty,
                Fun::Expr(_, _, ty) => ty,
            };

            strip_fun_args(args.len(), &fun_ty).clone()
        }
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

pub fn particularize_type(def: &Type, _expr: &Type) -> Type {
    // TODO this is ony used in type definitions
    def.clone()
}

/*
pub fn find_super_type(env: &Environment, found: &Type, expected: &Type) -> Option<Type> {
    if expected == found { return Some(expected.clone()); }
    //TODO
    let opt: Option<Type> = match expected {
        Type::Var(name) => {
            // https://guide.elm-lang.org/types/reading_types.html
//            check_constrained_type_variables(name, found);
            None
        }
//        Type::Tag(e_name, e_children) => {
//            if let Type::Tag(f_name, f_children) = found {
//                e_name == f_name && e_children.iter().zip(f_children)
//                    .all(|(e, f)| type_assignable_from(env, f, e))
//            } else {
//                None
//            }
//        }
        Type::Fun(a, b) => {
            match found {
                Type::Fun(a2, b2) => {
                    let a3 = find_super_type(env, a2, a)?;
                    let b3 = find_super_type(env, b2, b)?;

                    Some(Type::Fun(Box::new(a3), Box::new(b3)))
                }
                _ => None
            }
        }
//        Type::Tuple(items) => {
//            match found {
//                Type::Tuple(items2) => {
//                    if items.len() == items2.len() {
//                        items2.iter().zip(items).all(|(a, b)| type_assignable_from(env, b, a))
//                    } else { None }
//                }
//                _ => None
//            }
//        }
        Type::Unit => None,
        Type::Record(_) => None,
        Type::RecExt(_, _) => None,
        _ => None,
    };

    None
}
*/

pub fn type_assignable_from(env: &Environment, found: &Type, expected: &Type) -> bool {
    if expected == found { return true; }

    match expected {
        Type::Var(name) => {
            // https://guide.elm-lang.org/types/reading_types.html
            check_constrained_type_variables(name, found)
        }
        Type::Tag(e_name, e_children) => {
            if let Type::Tag(f_name, f_children) = found {
                e_name == f_name && e_children.iter().zip(f_children)
                    .all(|(e, f)| type_assignable_from(env, f, e))
            } else {
                false
            }
        }
        Type::Fun(a, b) => {
            match found {
                Type::Fun(a2, b2) => {
                    type_assignable_from(env, a2, a) && type_assignable_from(env, b2, b)
                }
                _ => false
            }
        }
        Type::Unit => false,
        Type::Tuple(items) => {
            match found {
                Type::Tuple(items2) => {
                    if items.len() == items2.len() {
                        items2.iter().zip(items).all(|(a, b)| type_assignable_from(env, b, a))
                    } else { false }
                }
                _ => false
            }
        }
        Type::Record(_) => false,
        Type::RecExt(_, _) => false,
    }
}

fn check_constrained_type_variables(name: &str, ty: &Type) -> bool {
    //TODO
    match name {
        "number" => {
            ty == &type_simple_tag_of("Int") || ty == &type_simple_tag_of("Float")
        }
//        "appendable" => ty == Type::Tag("String".s(), vec![]) || ty == Type::Tag("List".s(), vec![]),
        //TODO
        _ => true
    }
}

pub fn calculate_common_type<'a>(env: &Environment, types: &[&'a Type]) -> Result<&'a Type, (&'a Type, &'a Type)> {
    let first = *types.first().unwrap();

    for i in 1..types.len() {
        if !type_assignable_from(env, &types[i], first) {
            return Err((first, types[i]));
        }
    }
    Ok(first)
}


#[cfg(test)]
mod tests {
    use super::*;
    use run_line;

    #[test]
    fn check_identity() {
        let mut env = Environment::new();
        run_line(&mut env, b"func arg1 = arg1").unwrap();
        let ty = run_line(&mut env, b"func").unwrap();
        assert_eq!(&ty, "<function> : a -> a")
    }
}
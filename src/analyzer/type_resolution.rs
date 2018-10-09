use analyzer::environment::Environment;
use types::Type;
use util::StringConversion;
use types::Value;
use types::CurriedFunc;
use types::Fun;

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
        Value::Fun(CurriedFunc { fun, ..}) => {
            match fun {
                Fun::Builtin(_, ty) => ty.clone(),
                Fun::Expr(_, _, ty) => ty.clone(),
            }
        }
    }
}

pub fn particularize_type(def: &Type, _expr: &Type) -> Type {
    // TODO
    def.clone()
}

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

fn check_constrained_type_variables(_name: &str, _ty: &Type) -> bool {
    //TODO
//    match name {
//        "number" => ty == Type::Tag("Int".s(), vec![]) || ty == Type::Tag("Float".s(), vec![]),
//        "appendable" => ty == Type::Tag("String".s(), vec![]) || ty == Type::Tag("List".s(), vec![]),
//        //TODO
//        _ => true
//    }
    true
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

//fn create_all_type_variables() -> Vec<ConstrainedTypeVariable> {
//    vec![
//        ConstrainedTypeVariable {
//            name: "number".s(),
//            variants: vec![Type::Tag("Int".s(), vec![]), Type::Tag("Float".s(), vec![])],
//        },
//        ConstrainedTypeVariable {
//            name: "appendable".s(),
//            variants: vec![Type::Tag("String".s(), vec![]), Type::Tag("List".s(), vec![Type::Var("a".s())])],
//        },
//        ConstrainedTypeVariable {
//            name: "compappend".s(),
//            variants: vec![Type::Tag("String".s(), vec![]), Type::Tag("List".s(), vec![Type::Var("comparable".s())])],
//        },
//        ConstrainedTypeVariable {
//            name: "comparable".s(),
//            variants: vec![
//                Type::Tag("String".s(), vec![]),
//                Type::Tag("Int".s(), vec![]),
//                Type::Tag("Float".s(), vec![]),
//                Type::Tag("Char".s(), vec![]),
//                Type::Tag("List".s(), vec![Type::Var("comparable".s())]),
//                Type::Tuple(vec![Type::Var("comparable".s())]),
//                Type::Tuple(vec![Type::Var("comparable".s()), Type::Var("comparable".s())]),
//                Type::Tuple(vec![Type::Var("comparable".s()), Type::Var("comparable".s()), Type::Var("comparable".s())]),
//            ],
//        },
//    ]
//}


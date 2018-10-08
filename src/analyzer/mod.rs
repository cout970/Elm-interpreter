use types::Type;
use types::Value;
use util::StringConversion;
use types::CurriedFunc;
use types::Fun;

pub mod environment;
pub mod type_analyzer;
pub mod expression_fold;
pub mod pattern_helper;

fn get_value_type(value: &Value) -> Type {
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
        Value::Fun(CurriedFunc { args, fun, arg_count }) => {
            match fun {
                Fun::Builtin(_, ty) => ty.clone(),
                Fun::Expr(_, _, ty) => ty.clone(),
            }
        }
    }
}
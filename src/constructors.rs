use std::vec::IntoIter;

use ast::Type;

pub fn type_unit() -> Type {
    Type::Unit
}

pub fn type_int() -> Type {
    Type::Tag(String::from("Int"), vec![])
}

pub fn type_float() -> Type {
    Type::Tag(String::from("Float"), vec![])
}

pub fn type_number() -> Type {
    Type::Var(String::from("number"))
}

pub fn type_bool() -> Type {
    Type::Tag(String::from("Bool"), vec![])
}

pub fn type_string() -> Type {
    Type::Tag(String::from("String"), vec![])
}

pub fn type_char() -> Type {
    Type::Tag(String::from("Char"), vec![])
}

pub fn type_list(var: Type) -> Type {
    Type::Tag(String::from("List"), vec![var])
}

pub fn type_array(var: Type) -> Type {
    Type::Tag(String::from("Array"), vec![var])
}

pub fn type_maybe(var: Type) -> Type {
    Type::Tag(String::from("Maybe"), vec![var])
}

pub fn type_var(var: &str) -> Type {
    Type::Var(String::from(var))
}

pub fn type_tag(var: &str) -> Type {
    Type::Tag(String::from(var), vec![])
}

pub fn type_tuple<T>(values: T) -> Type
    where T: IntoIterator<Item=Type, IntoIter=IntoIter<Type>>
{
    Type::Tuple(values.into_iter().collect())
}

pub fn type_fun<T>(types: T) -> Type
    where T: IntoIterator<Item=Type, IntoIter=IntoIter<Type>>
{
    let mut iter = types.into_iter();

    if iter.len() == 1 {
        return iter.next().unwrap();
    }

    if iter.len() == 2 {
        Type::Fun(
            Box::from(iter.next().unwrap()),
            Box::from(iter.next().unwrap()),
        )
    } else {
        Type::Fun(
            Box::from(iter.next().unwrap()),
            Box::from(type_fun(iter)),
        )
    }
}

pub fn type_record(entries: Vec<(&str, Type)>) -> Type {
    Type::Record(
        entries.into_iter()
            .map(|(s, t)| (String::from(s), t))
            .collect()
    )
}
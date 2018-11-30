use std::vec::IntoIter;

use ast::Int;
use ast::Pattern;
use ast::Type;
use parsers::parse_type;


// Type

pub fn type_of(code: &str) -> Type {
    parse_type(code).unwrap()
}

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

pub fn type_tag_args(var: &str, args: Vec<Type>) -> Type {
    Type::Tag(String::from(var), args)
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

// Pattern

pub fn pattern_var(name: &str) -> Pattern {
    Pattern::Var(String::from(name))
}

pub fn pattern_tag(var: &str) -> Pattern {
    Pattern::Adt(String::from(var), vec![])
}

pub fn pattern_tag_args(var: &str, args: Vec<Pattern>) -> Pattern {
    Pattern::Adt(String::from(var), args)
}

pub fn pattern_wildcard() -> Pattern {
    Pattern::Wildcard
}

pub fn pattern_unit() -> Pattern {
    Pattern::Unit
}

pub fn pattern_int(value: Int) -> Pattern {
    Pattern::LitInt(value)
}

pub fn pattern_cons(left: Pattern, right: Pattern) -> Pattern {
    Pattern::BinaryOp("::".to_owned(), Box::from(left), Box::from(right))
}

pub fn pattern_tuple<T>(values: T) -> Pattern
    where T: IntoIterator<Item=Pattern, IntoIter=IntoIter<Pattern>>
{
    Pattern::Tuple(values.into_iter().collect())
}

pub fn pattern_list<T>(values: T) -> Pattern
    where T: IntoIterator<Item=Pattern, IntoIter=IntoIter<Pattern>>
{
    Pattern::List(values.into_iter().collect())
}

pub fn pattern_record(entries: Vec<&str>) -> Pattern {
    Pattern::Record(
        entries.into_iter()
            .map(|s| String::from(s))
            .collect()
    )
}

pub fn pattern_alias(value: Pattern, alias: &str) -> Pattern {
    Pattern::Alias(Box::from(value), String::from(alias))
}

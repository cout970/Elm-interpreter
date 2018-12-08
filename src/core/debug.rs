use ast::Type;
use constructors::*;

pub fn get_debug_types() -> Vec<(&'static str, Type)> {
    get_debug_type_aux().into_iter()
        .map(|(a, b)| (a, type_of(b)))
        .collect()
}

fn get_debug_type_aux() -> Vec<(&'static str, &'static str)> {
    //@formatter:off
    vec![
        ("toString",    "a -> String"),
        ("log",         "String -> a -> a"),
        ("todo",        "String -> a"),
    ]
    //@formatter:on
}
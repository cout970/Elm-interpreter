use ast::Type;
use constructors::*;

pub fn get_utils_types() -> Vec<(&'static str, Type)> {
    get_utils_type_aux().into_iter()
        .map(|(a, b)| (a, type_of(b)))
        .collect()
}

fn get_utils_type_aux() -> Vec<(&'static str, &'static str)> {
    //@formatter:off
    vec![
        ("equal",       "a -> a -> Bool"),
        ("==",          "a -> a -> Bool"),
        ("notEqual",    "a -> a -> Bool"),
        ("/=",          "a -> a -> Bool"),
        ("cmp",         "a -> a -> Int"),
        ("compare",     "a -> a -> Int"),
        ("lt",          "a -> a -> Bool"),
        ("<",           "a -> a -> Bool"),
        ("le",          "a -> a -> Bool"),
        ("<=",          "a -> a -> Bool"),
        ("gt",          "a -> a -> Bool"),
        (">",           "a -> a -> Bool"),
        ("ge",          "a -> a -> Bool"),
        (">=",          "a -> a -> Bool"),
        ("append",      "String -> String -> String"),
        ("<|",          "(a -> b) -> a -> b"),
        ("|>",          "a -> (a -> b) -> b"),
        ("<<",          "(b -> c) -> (a -> b) -> (a -> c)"),
        (">>",          "(a -> b) -> (b -> c) -> (a -> c)"),
    ]
    //@formatter:on
}
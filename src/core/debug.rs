use ast::Type;
use constructors::*;

pub fn get_debug_types() -> Vec<(&'static str, Type)> {
    //@formatter:off
    vec![
        ("toString", type_fun(vec![type_var("a"), type_string()])),
        ("log",      type_fun(vec![type_string(),     type_var("a"), type_var("a")])),
        ("todo",     type_fun(vec![type_string(),     type_var("a")])),
    ]
    //@formatter:on
}
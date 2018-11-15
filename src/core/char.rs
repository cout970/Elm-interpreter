use ast::Type;
use constructors::*;

pub fn get_char_types() -> Vec<(&'static str, Type)> {
    //@formatter:off
    vec![
        ("toCode",          type_fun(vec![type_char(),  type_int()])),
        ("fromCode",        type_fun(vec![type_int(),   type_char()])),
        ("toUpper",         type_fun(vec![type_char(),  type_char()])),
        ("toLower",         type_fun(vec![type_char(),  type_char()])),
        ("toLocaleUpper",   type_fun(vec![type_char(),  type_char()])),
        ("toLocaleLower",   type_fun(vec![type_char(),  type_char()])),
    ]
    //@formatter:on
}
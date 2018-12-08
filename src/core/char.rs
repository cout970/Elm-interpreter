use ast::Type;
use constructors::*;

pub fn get_char_types() -> Vec<(&'static str, Type)> {
    get_char_type_aux().into_iter()
        .map(|(a, b)| (a, type_of(b)))
        .collect()
}

fn get_char_type_aux() -> Vec<(&'static str, &'static str)> {
    //@formatter:off
    vec![
        ("toCode",          "Char -> Int"),
        ("fromCode",        "Int -> Char"),
        ("toUpper",         "Char -> Char"),
        ("toLower",         "Char -> Char"),
        ("toLocaleUpper",   "Char -> Char"),
        ("toLocaleLower",   "Char -> Char"),
    ]
    //@formatter:on
}
use ast::Type;
use constructors::*;

pub fn get_bitwise_types() -> Vec<(&'static str, Type)> {
    get_bitwise_type_aux().into_iter()
        .map(|(a, b)| (a, type_of(b)))
        .collect()
}

fn get_bitwise_type_aux() -> Vec<(&'static str, &'static str)> {
    //@formatter:off
    vec![
        ("and",             "Int -> Int -> Int"),
        ("or",              "Int -> Int -> Int"),
        ("xor",             "Int -> Int -> Int"),
        ("complement",      "Int -> Int"),
        ("shiftLeftBy",     "Int -> Int -> Int"),
        ("shiftRightBy",    "Int -> Int -> Int"),
        ("shiftRightZfBy",  "Int -> Int -> Int"),
    ]
    //@formatter:on
}
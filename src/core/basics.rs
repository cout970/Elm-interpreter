use ast::Type;
use constructors::*;

pub fn get_basics_types() -> Vec<(&'static str, Type)> {
    get_basics_type_aux().into_iter()
        .map(|(a, b)| (a, type_of(b)))
        .collect()
}

fn get_basics_type_aux() -> Vec<(&'static str, &'static str)> {
    //@formatter:off
    vec![
        ("+",                   "number -> number -> number"),
        ("add",                 "number -> number -> number"),
        ("-",                   "number -> number -> number"),
        ("__internal__minus",   "number -> number"),
        ("sub",                 "number -> number -> number"),
        ("*",                   "number -> number -> number"),
        ("mul",                 "number -> number -> number"),
        ("/",                   "Float -> Float -> Float"),
        ("fdiv",                "Float -> Float -> Float"),
        ("//",                  "Int -> Int -> Int"),
        ("idiv",                "Int -> Int -> Int"),
        ("^",                   "number -> number -> number"),
        ("pow",                 "number -> number -> number"),
        ("remainderBy",         "Int -> Int -> Int"),
        ("modBy",               "Int -> Int -> Int"),
        ("++",                  "String -> String -> String"),
        ("pi",                  "Float"),
        ("e",                   "Float"),
        ("cos",                 "Float -> Float"),
        ("sin",                 "Float -> Float"),
        ("tan",                 "Float -> Float"),
        ("log",                 "Float -> Float"),
        ("acos",                "Float -> Float"),
        ("asin",                "Float -> Float"),
        ("atan",                "Float -> Float"),
        ("atan2",               "Float -> Float -> Float"),
        ("toFloat",             "Int -> Float"),
        ("truncate",            "Float -> Int"),
        ("isInfinite",          "Float -> Bool"),
        ("ceiling",             "Float -> Int"),
        ("floor",               "Float -> Int"),
        ("round",               "Float -> Int"),
        ("sqrt",                "Float -> Float"),
        ("isNaN",               "Float -> Bool"),
        ("&&",                  "Bool -> Bool -> Bool"),
        ("and",                 "Bool -> Bool -> Bool"),
        ("||",                  "Bool -> Bool -> Bool"),
        ("or",                  "Bool -> Bool -> Bool"),
        ("xor",                 "Bool -> Bool -> Bool"),
        ("not",                 "Bool -> Bool"),
    ]
    //@formatter:on
}
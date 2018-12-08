use ast::Type;
use constructors::*;

pub fn get_string_types() -> Vec<(&'static str, Type)> {
    get_string_type_aux().into_iter()
        .map(|(a, b)| (a, type_of(b)))
        .collect()
}

fn get_string_type_aux() -> Vec<(&'static str, &'static str)> {
    //@formatter:off
    vec![
        ("cons",        "Char -> String -> String"),
        ("uncons",      "String -> (Char, String)"),
        ("append",      "String -> String -> String"),
        ("length",      "String -> Int"),
        ("map",         "(Char -> Char) -> String -> String"),
        ("filter",      "(Char -> Bool) -> String -> String"),
        ("reverse",     "String -> String"),
        ("foldl",       "(Char -> b -> b) -> b -> String -> b"),
        ("foldr",       "(Char -> b -> b) -> b -> String -> b"),
        ("split",       "String -> String -> Array String"),
        ("join",        "String -> Array String -> String"),
        ("slice",       "Int -> Int -> String -> String"),
        ("trim",        "String -> String"),
        ("trimLeft",    "String -> String"),
        ("trimRight",   "String -> String"),
        ("words",       "String -> List String"),
        ("lines",       "String -> List String"),
        ("toUpper",     "String -> String"),
        ("toLower",     "String -> String"),
        ("any",         "(Char -> Bool) -> String -> Bool"),
        ("all",         "(Char -> Bool) -> String -> Bool"),
        ("contains",    "String -> String -> Bool"),
        ("startsWith",  "String -> String -> Bool"),
        ("endsWith",    "String -> String -> Bool"),
        ("indexes",     "String -> String -> List Int"),
        ("fromNumber",  "number -> String"),
        ("toInt",       "String -> Maybe Int"),
        ("toFloat",     "String -> Maybe Float"),
        ("fromList",    "List Char -> String"),
    ]
    //@formatter:on
}

use ast::Type;
use constructors::type_of;

pub fn get_operators_types() -> Vec<(&'static str, Type)> {
    get_operators_type_aux().into_iter()
        .map(|(a, b)| (a, type_of(b)))
        .collect()
}

fn get_operators_type_aux() -> Vec<(&'static str, &'static str)> {
    //@formatter:off
    vec![
        ("::",                  "a -> List a -> List a"),
        ("+",                   "number -> number -> number"),
        ("-",                   "number -> number -> number"),
        ("__internal__minus",   "number -> number"),
        ("*",                   "number -> number -> number"),
        ("/",                   "Float -> Float -> Float"),
        ("//",                  "Int -> Int -> Int"),
        ("^",                   "number -> number -> number"),
        ("++",                  "String -> String -> String"),
        ("&&",                  "Bool -> Bool -> Bool"),
        ("||",                  "Bool -> Bool -> Bool"),
    ]
    //@formatter:on
}
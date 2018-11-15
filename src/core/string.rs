use ast::Type;
use constructors::*;

pub fn get_string_types() -> Vec<(&'static str, Type)> {
    //@formatter:off
    vec![
        ("::",          type_fun(vec![type_char(),   type_string()])),
        ("cons",          type_fun(vec![type_char(),   type_string()])),
        ("uncons",      type_fun(vec![type_string(), type_tuple(vec![type_char(), type_string()])])),
        ("append",      type_fun(vec![type_string(), type_string(), type_string()])),
        ("length",      type_fun(vec![type_string(), type_int()])),
        ("map",         type_fun(vec![type_fun(vec![type_char(), type_char()]), type_string(), type_string()])),
        ("filter",      type_fun(vec![type_fun(vec![type_char(), type_bool()]), type_string(), type_string()])),
        ("reverse",     type_fun(vec![type_string(), type_string()])),
        ("foldl",       type_fun(vec![type_fun(vec![type_char(), type_var("b"), type_var("b")]), type_var("b"), type_string(), type_var("b")])),
        ("foldr",       type_fun(vec![type_fun(vec![type_char(), type_var("b"), type_var("b")]), type_var("b"), type_string(), type_var("b")])),
        ("split",       type_fun(vec![type_string(), type_string(), type_list(type_string())])),
        ("join",        type_fun(vec![type_string(), type_list(type_string()), type_string()])),
        ("slice",       type_fun(vec![type_int(), type_int(), type_string(), type_string()])),
        ("trim",        type_fun(vec![type_string(), type_string()])),
        ("trimLeft",    type_fun(vec![type_string(), type_string()])),
        ("trimRight",   type_fun(vec![type_string(), type_string()])),
        ("words",       type_fun(vec![type_string(), type_list(type_string())])),
        ("lines",       type_fun(vec![type_string(), type_list(type_string())])),
        ("toUpper",     type_fun(vec![type_string(), type_string()])),
        ("toLower",     type_fun(vec![type_string(), type_string()])),
        ("any",         type_fun(vec![type_fun(vec![type_char(), type_bool()]), type_string(), type_bool()])),
        ("all",         type_fun(vec![type_fun(vec![type_char(), type_bool()]), type_string(), type_bool()])),
        ("contains",    type_fun(vec![type_string(), type_string(), type_bool()])),
        ("startsWith",  type_fun(vec![type_string(), type_string(), type_bool()])),
        ("endsWith",    type_fun(vec![type_string(), type_string(), type_bool()])),
        ("indexes",     type_fun(vec![type_string(), type_string(), type_list(type_int())])),
        ("fromNumber",  type_fun(vec![type_number(), type_string()])),
        ("toInt",       type_fun(vec![type_string(), type_maybe(type_int())])),
        ("toFloat",     type_fun(vec![type_string(), type_maybe(type_float())])),
        ("fromList",    type_fun(vec![type_list(type_char()), type_string()])),
    ]
    //@formatter:on
}
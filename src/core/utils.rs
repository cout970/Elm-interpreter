use ast::Type;
use constructors::*;

pub fn get_utils_types() -> Vec<(&'static str, Type)> {
    //@formatter:off
    vec![
        ("equal",    type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("==",       type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("notEqual", type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("/=",       type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("cmp",      type_fun(vec![type_var("a"), type_var("a"), type_int()])),
        ("compare",  type_fun(vec![type_var("a"), type_var("a"), type_int()])),
        ("lt",       type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("<",        type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("le",       type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("<=",       type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("gt",       type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        (">",        type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("ge",       type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        (">=",       type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("append",   type_fun(vec![type_string(), type_string(), type_string()])),
        ("<|",       type_fun(vec![type_fun(vec![type_var("a"), type_var("b")]), type_var("a"), type_var("b")])),
        ("|>",       type_fun(vec![type_var("a"), type_fun(vec![type_var("a"), type_var("b")]), type_var("b")])),
        ("<<",       type_fun(vec![type_fun(vec![type_var("b"), type_var("c")]), type_fun(vec![type_var("a"), type_var("b")]), type_fun(vec![type_var("a"), type_var("c")])])),
        (">>",       type_fun(vec![type_fun(vec![type_var("a"), type_var("b")]), type_fun(vec![type_var("b"), type_var("c")]), type_fun(vec![type_var("a"), type_var("c")])])),
    ]
    //@formatter:on
}
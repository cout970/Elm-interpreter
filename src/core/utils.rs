use ast::Type;
use constructors::*;

pub fn get_utils_types() -> Vec<(&'static str, Type)> {
    //@formatter:off
    vec![
        ("equal",    type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("notEqual", type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("cmp",      type_fun(vec![type_var("a"), type_var("a"), type_int()])),
        ("lt",       type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("le",       type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("gt",       type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
        ("ge",       type_fun(vec![type_var("a"), type_var("a"), type_bool()])),
    ]
    //@formatter:on
}

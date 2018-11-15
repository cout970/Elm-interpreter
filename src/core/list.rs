use ast::Type;
use constructors::*;

pub fn get_list_types() -> Vec<(&'static str, Type)> {
    //@formatter:off
    vec![
        ("fromArray", type_fun(vec![type_array(type_var("a")), type_list(type_var("a"))])),
        ("toArray",   type_fun(vec![type_list(type_var("a")), type_array(type_var("a"))])),
        ("sortBy",    type_fun(vec![type_fun(vec![type_var("a"), type_var("comparable")]), type_list(type_var("a")), type_list(type_var("a"))])),
        ("sortWith",  type_fun(vec![type_fun(vec![type_var("a"), type_var("a"), type_tag("Order")]), type_list(type_var("a")), type_list(type_var("a"))])),
    ]
    //@formatter:on
}
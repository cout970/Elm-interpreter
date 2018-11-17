use ast::Type;
use constructors::*;

pub fn get_basics_types() -> Vec<(&'static str, Type)> {
    //@formatter:off
    vec![
        ("+",           type_fun(vec![type_number(), type_number(), type_number()])),
        ("add",         type_fun(vec![type_number(), type_number(), type_number()])),
        ("-",           type_fun(vec![type_number(), type_number(), type_number()])),
        ("sub",         type_fun(vec![type_number(), type_number(), type_number()])),
        ("*",           type_fun(vec![type_number(), type_number(), type_number()])),
        ("mul",         type_fun(vec![type_number(), type_number(), type_number()])),
        ("/",           type_fun(vec![type_float(),  type_float(),  type_float()])),
        ("fdiv",        type_fun(vec![type_float(),  type_float(),  type_float()])),
        ("//",          type_fun(vec![type_int(),    type_int(),    type_int()])),
        ("idiv",        type_fun(vec![type_int(),    type_int(),    type_int()])),
        ("^",           type_fun(vec![type_number(), type_number(), type_number()])),
        ("pow",         type_fun(vec![type_number(), type_number(), type_number()])),
        ("remainderBy", type_fun(vec![type_int(),    type_int(),    type_int()])),
        ("modBy",       type_fun(vec![type_int(),    type_int(),    type_int()])),
        ("pi",          type_float()),
        ("e",           type_float()),
        ("cos",         type_fun(vec![type_float(),  type_float()])),
        ("sin",         type_fun(vec![type_float(),  type_float()])),
        ("tan",         type_fun(vec![type_float(),  type_float()])),
        ("log",         type_fun(vec![type_float(),  type_float()])),
        ("acos",        type_fun(vec![type_float(),  type_float()])),
        ("asin",        type_fun(vec![type_float(),  type_float()])),
        ("atan",        type_fun(vec![type_float(),  type_float()])),
        ("atan2",       type_fun(vec![type_float(),  type_float(),   type_float()])),
        ("toFloat",     type_fun(vec![type_int(),    type_float()])),
        ("truncate",    type_fun(vec![type_float(),  type_int()])),
        ("isInfinite",  type_fun(vec![type_float(),  type_bool()])),
        ("ceiling",     type_fun(vec![type_float(),  type_int()])),
        ("floor",       type_fun(vec![type_float(),  type_int()])),
        ("round",       type_fun(vec![type_float(),  type_int()])),
        ("sqrt",        type_fun(vec![type_float(),  type_float()])),
        ("isNaN",       type_fun(vec![type_float(),  type_bool()])),
        ("&&",          type_fun(vec![type_bool(),   type_bool(),    type_bool()])),
        ("and",          type_fun(vec![type_bool(),   type_bool(),    type_bool()])),
        ("||",          type_fun(vec![type_bool(),   type_bool(),    type_bool()])),
        ("or",          type_fun(vec![type_bool(),   type_bool(),    type_bool()])),
        ("xor",         type_fun(vec![type_bool(),   type_bool(),    type_bool()])),
        ("not",         type_fun(vec![type_bool(),   type_bool()])),
    ]
    //@formatter:on
}
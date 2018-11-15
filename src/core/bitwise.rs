use ast::Type;
use constructors::*;

pub fn get_bitwise_types() -> Vec<(&'static str, Type)> {
    //@formatter:off
    vec![
        ("and",             type_fun(vec![type_int(), type_int(), type_int()])),
        ("or",              type_fun(vec![type_int(), type_int(), type_int()])),
        ("xor",             type_fun(vec![type_int(), type_int(), type_int()])),
        ("complement",      type_fun(vec![type_int(), type_int()])),
        ("shiftLeftBy",     type_fun(vec![type_int(), type_int(), type_int()])),
        ("shiftRightBy",    type_fun(vec![type_int(), type_int(), type_int()])),
        ("shiftRightZfBy",  type_fun(vec![type_int(), type_int(), type_int()])),
    ]
    //@formatter:on
}
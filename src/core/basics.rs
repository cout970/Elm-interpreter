use analyzer::static_env::StaticEnv;
use constructors::*;

pub fn register_basics(env: &mut StaticEnv) {
    env.add_definition("+", type_fun(vec![type_number(), type_number(), type_number()]));
    env.add_definition("-", type_fun(vec![type_number(), type_number(), type_number()]));
    env.add_definition("*", type_fun(vec![type_number(), type_number(), type_number()]));
    env.add_definition("/", type_fun(vec![type_float(), type_float(), type_float()]));
    env.add_definition("//", type_fun(vec![type_int(), type_int(), type_int()]));
    env.add_definition("^", type_fun(vec![type_number(), type_number(), type_number()]));
    env.add_definition("remainderBy", type_fun(vec![type_int(), type_int(), type_int()]));
    env.add_definition("modBy", type_fun(vec![type_int(), type_int(), type_int()]));
    env.add_definition("pi", type_float());
    env.add_definition("e", type_float());
    env.add_definition("cos", type_fun(vec![type_float(), type_float()]));
    env.add_definition("sin", type_fun(vec![type_float(), type_float()]));
    env.add_definition("tan", type_fun(vec![type_float(), type_float()]));
    env.add_definition("acos", type_fun(vec![type_float(), type_float()]));
    env.add_definition("asin", type_fun(vec![type_float(), type_float()]));
    env.add_definition("atan", type_fun(vec![type_float(), type_float()]));
    env.add_definition("atan2", type_fun(vec![type_float(), type_float(), type_float()]));
    env.add_definition("toFloat", type_fun(vec![type_int(), type_float()]));
    env.add_definition("truncate", type_fun(vec![type_float(), type_int()]));
    env.add_definition("isInfinite", type_fun(vec![type_float(), type_bool()]));
    env.add_definition("ceiling", type_fun(vec![type_float(), type_int()]));
    env.add_definition("floor", type_fun(vec![type_float(), type_int()]));
    env.add_definition("round", type_fun(vec![type_float(), type_int()]));
    env.add_definition("sqrt", type_fun(vec![type_float(), type_float()]));
    env.add_definition("isNaN", type_fun(vec![type_float(), type_bool()]));
    env.add_definition("&&", type_fun(vec![type_bool(), type_bool(), type_bool()]));
    env.add_definition("||", type_fun(vec![type_bool(), type_bool(), type_bool()]));
    env.add_definition("xor", type_fun(vec![type_bool(), type_bool(), type_bool()]));
    env.add_definition("not", type_fun(vec![type_bool(), type_bool()]));
}
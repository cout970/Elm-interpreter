use std::cmp::Ordering;

use ast::Type;
use constructors::*;
use core::func_of;
use errors::ElmError;
use interpreter::Interpreter;
use rust_interop::conversions::string_of;
use types::Value;

pub fn get_utils_funs() -> Vec<(&'static str, Type, Value)> {
    vec![
        func_of("equal", "a -> a -> Bool", equal),
        func_of("==", "a -> a -> Bool", equal),
        func_of("notEqual", "a -> a -> Bool", not_equal),
        func_of("/=", "a -> a -> Bool", not_equal),
        func_of("compare", "a -> a -> Bool", compare),
        func_of("lt", "a -> a -> Bool", lt),
        func_of("<", "a -> a -> Bool", lt),
        func_of("le", "a -> a -> Bool", le),
        func_of("<=", "a -> a -> Bool", le),
        func_of("gt", "a -> a -> Bool", gt),
        func_of(">", "a -> a -> Bool", gt),
        func_of("ge", "a -> a -> Bool", ge),
        func_of(">=", "a -> a -> Bool", ge),
        func_of("append", "String -> String -> String", append),
        func_of("<|", "(a -> b) -> a -> b", pipe_back),
        func_of("|>", "a -> (a -> b) -> b", pipe_front),
        func_of("<<", "(b -> c) -> (a -> b) -> (a -> c)", compose_back),
        func_of(">>", "(a -> b) -> (b -> c) -> (a -> c)", compose_front),
    ]
}

fn get_utils_type_aux() -> Vec<(&'static str, &'static str)> {
    //@formatter:off
    vec![
        ("<|",          "(a -> b) -> a -> b"),
        ("|>",          "a -> (a -> b) -> b"),
        ("<<",          "(b -> c) -> (a -> b) -> (a -> c)"),
        (">>",          "(a -> b) -> (b -> c) -> (a -> c)"),
    ]
    //@formatter:on
}

fn equal(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    if args[0] == args[1] {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn not_equal(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    if args[0] != args[1] {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn compare(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    match compare_values(&args[1], &args[1]) {
        Ordering::Less => Ok(Value::Int(-1)),
        Ordering::Equal => Ok(Value::Int(0)),
        Ordering::Greater => Ok(Value::Int(1)),
    }
}

fn lt(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    if compare_values(&args[1], &args[1]) == Ordering::Less {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn le(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    if compare_values(&args[1], &args[1]) != Ordering::Greater {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn gt(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    if compare_values(&args[1], &args[1]) == Ordering::Less {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn ge(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    if compare_values(&args[1], &args[1]) != Ordering::Less {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn append(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = string_of(&args[0])?;
    let b = string_of(&args[1])?;

    Ok(Value::String(format!("{}{}", a, b)))
}

fn pipe_back(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    unimplemented!()
}

fn pipe_front(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    unimplemented!()
}

fn compose_back(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    unimplemented!()
}

fn compose_front(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    unimplemented!()
}


fn compare_values(a: &Value, b: &Value) -> Ordering {
    if b == a { return Ordering::Equal }
    match a {
        Value::Number(na) => {
            if let Value::Number(nb) = b {
                na.cmp(nb)
            } else {
                Ordering::Less
            }
        },
        Value::Int(na) => {
            if let Value::Int(nb) = b {
                na.cmp(nb)
            } else {
                Ordering::Less
            }
        },
        Value::Float(na) => {
            if let Value::Float(nb) = b {
                na.partial_cmp(nb).unwrap_or(Ordering::Less)
            } else {
                Ordering::Less
            }
        },
        Value::String(na) => {
            if let Value::String(nb) = b {
                na.cmp(nb)
            } else {
                Ordering::Less
            }
        },
        Value::Char(na) => {
            if let Value::Char(nb) = b {
                na.cmp(nb)
            } else {
                Ordering::Less
            }
        },
        Value::Unit => Ordering::Less,
        Value::List(_) => Ordering::Less,
        Value::Tuple(_) => Ordering::Less,
        Value::Record(_) => Ordering::Less,
        Value::Adt(_, _, _) => Ordering::Less,
        Value::Fun { .. } => Ordering::Less,
    }
}
use std::f32::consts::E;
use std::f32::consts::PI;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

use ast::Type;
use builtin::func_of;
use constructors::*;
use errors::ElmError;
use errors::InterpreterError;
use errors::Wrappable;
use interpreter::Interpreter;
use rust_interop::conversions::bool_of;
use rust_interop::conversions::float_of;
use rust_interop::conversions::int_of;
use rust_interop::conversions::number_op;
use rust_interop::conversions::string_of;
use types::Value;

pub fn get_basics_funs() -> Vec<(&'static str, Type, Value)> {
    vec![
        func_of("+", "number -> number -> number", add),
        func_of("add", "number -> number -> number", add),
        func_of("-", "number -> number -> number", sub),
        func_of("sub", "number -> number -> number", sub),
        func_of("__internal__minus", "number -> number", unary_minus),
        func_of("*", "number -> number -> number", mul),
        func_of("mul", "number -> number -> number", mul),
        func_of("/", "Float -> Float -> Float", fdiv),
        func_of("fdiv", "Float -> Float -> Float", fdiv),
        func_of("//", "Int -> Int -> Int", idiv),
        func_of("idiv", "Int -> Int -> Int", idiv),
        func_of("^", "number -> number -> number", pow),
        func_of("pow", "number -> number -> number", pow),
        func_of("remainderBy", "Int -> Int -> Int", remainder_by),
        func_of("modBy", "Int -> Int -> Int", mod_by),
        func_of("++", "String -> String -> String", append),
        ("pi", type_float(), Value::Float(PI)),
        ("e", type_float(), Value::Float(E)),
        func_of("cos", "Float -> Float", cos),
        func_of("sin", "Float -> Float", sin),
        func_of("tan", "Float -> Float", tan),
        func_of("log", "Float -> Float", log),
        func_of("acos", "Float -> Float", acos),
        func_of("asin", "Float -> Float", asin),
        func_of("atan", "Float -> Float", atan),
        func_of("atan2", "Float -> Float -> Float", atan2),
        func_of("toFloat", "Int -> Float", to_float),
        func_of("truncate", "Float -> Int", truncate),
        func_of("ceiling", "Float -> Int", ceiling),
        func_of("floor", "Float -> Int", floor),
        func_of("round", "Float -> Int", round),
        func_of("isInfinite", "Float -> Bool", is_infinite),
        func_of("sqrt", "Float -> Float", sqrt),
        func_of("isNaN", "Float -> Bool", is_nan),
        func_of("and", "Bool -> Bool -> Bool", and),
        func_of("&&", "Bool -> Bool -> Bool", and),
        func_of("or", "Bool -> Bool -> Bool", or),
        func_of("||", "Bool -> Bool -> Bool", or),
        func_of("xor", "Bool -> Bool -> Bool", xor),
        func_of("not", "Bool -> Bool", not),
    ]
}

fn add(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    number_op(&args[0], &args[1], Add::add)
}

fn sub(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    number_op(&args[0], &args[1], Sub::sub)
}

fn unary_minus(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    match &args[0] {
        Value::Number(a) => Ok(Value::Number(-*a)),
        Value::Int(a) => Ok(Value::Int(-*a)),
        Value::Float(a) => Ok(Value::Float(-*a)),
        _ => Err(InterpreterError::ExpectedNumber(args[0].clone()).wrap())
    }
}

fn mul(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    number_op(&args[0], &args[1], Mul::mul)
}

fn fdiv(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    Ok(Value::Float(float_of(&args[0])? / float_of(&args[1])?))
}

fn idiv(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = int_of(&args[0])?;
    let b = int_of(&args[1])?;

    if b == 0 {
        Ok(Value::Int(0))
    } else {
        Ok(Value::Int(a / b))
    }
}

fn pow(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    number_op(&args[0], &args[1], |a, b| a.powf(b))
}

fn remainder_by(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = int_of(&args[0])?;
    let b = int_of(&args[1])?;

    let mut res = b % a;
    if res < 0 {
        res += a;
    }
    Ok(Value::Int(res))
}

fn mod_by(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = int_of(&args[0])?;
    let b = int_of(&args[1])?;

    Ok(Value::Int(b % a))
}

fn append(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = string_of(&args[0])?;
    let b = string_of(&args[1])?;

    Ok(Value::String(format!("{}{}", a, b)))
}


fn cos(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    Ok(Value::Float(a.cos()))
}

fn sin(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    Ok(Value::Float(a.sin()))
}

fn tan(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    Ok(Value::Float(a.tan()))
}

fn log(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    Ok(Value::Float(a.log(E)))
}

fn acos(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    Ok(Value::Float(a.acos()))
}

fn asin(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    Ok(Value::Float(a.asin()))
}

fn atan(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    Ok(Value::Float(a.atan()))
}

fn atan2(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;
    let b = float_of(&args[1])?;

    Ok(Value::Float(a.atan2(b)))
}

fn to_float(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = int_of(&args[0])?;

    Ok(Value::Float(a as f32))
}

fn truncate(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    Ok(Value::Int(a as i32))
}

fn ceiling(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    Ok(Value::Int(a.ceil() as i32))
}

fn floor(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    Ok(Value::Int(a.floor() as i32))
}

fn round(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    Ok(Value::Int(a.round() as i32))
}

fn sqrt(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    Ok(Value::Float(a.sqrt()))
}

fn is_infinite(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;

    if a.is_infinite() {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn is_nan(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = float_of(&args[0])?;
    if a.is_nan() {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn or(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = bool_of(&args[0])?;
    let b = bool_of(&args[1])?;

    if a || b {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn and(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = bool_of(&args[0])?;
    let b = bool_of(&args[1])?;

    if a && b {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn xor(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = bool_of(&args[0])?;
    let b = bool_of(&args[1])?;

    if a ^ b {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn not(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = bool_of(&args[0])?;

    if !a {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}


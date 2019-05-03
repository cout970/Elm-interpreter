use ast::Type;
use builtin::func_of;
use errors::ElmError;
use interpreter::Interpreter;
use rust_interop::conversions::int_of;
use types::Value;

pub fn get_bitwise_funs() -> Vec<(&'static str, Type, Value)> {
    vec![
        func_of("and", "Int -> Int -> Int", and),
        func_of("or", "Int -> Int -> Int", or),
        func_of("xor", "Int -> Int -> Int", xor),
        func_of("complement", "Int -> Int", complement),
        func_of("shiftLeftBy", "Int -> Int -> Int", shift_left_by),
        func_of("shiftRightBy", "Int -> Int -> Int", shift_right_by),
        func_of("shiftRightZfBy", "Int -> Int -> Int", shift_right_zf_by),
    ]
}

fn and(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = int_of(&args[0])?;
    let b = int_of(&args[1])?;

    Ok(Value::Int(a & b))
}

fn or(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = int_of(&args[0])?;
    let b = int_of(&args[1])?;

    Ok(Value::Int(a | b))
}

fn xor(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = int_of(&args[0])?;
    let b = int_of(&args[1])?;

    Ok(Value::Int(a ^ b))
}

fn complement(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = int_of(&args[0])?;

    Ok(Value::Int(!a))
}

fn shift_left_by(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = int_of(&args[0])?;
    let b = int_of(&args[1])?;

    Ok(Value::Int(b << a))
}

fn shift_right_by(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = int_of(&args[0])?;
    let b = int_of(&args[1])?;

    Ok(Value::Int(b >> a))
}

fn shift_right_zf_by(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = int_of(&args[0])?;
    let b = int_of(&args[1])?;

    Ok(Value::Int(((b as u32) >> (a as u32)) as i32))
}
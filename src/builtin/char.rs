use ast::Type;
use builtin::func_of;
use errors::ElmError;
use interpreter::Interpreter;
use rust_interop::conversions::char_of;
use rust_interop::conversions::int_of;
use types::Value;

pub fn get_char_funs() -> Vec<(&'static str, Type, Value)> {
    vec![
        func_of("toCode", "Char -> Int", to_code),
        func_of("fromCode", "Int -> Char", from_code),
        func_of("toUpper", "Char -> Char", to_upper),
        func_of("toLower", "Char -> Char", to_lower),
        func_of("toLocaleUpper", "Char -> Char", to_locale_upper),
        func_of("toLocaleLower", "Char -> Char", to_locale_lower),
    ]
}

fn to_code(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = char_of(&args[0])?;

    Ok(Value::Int(a as i32))
}

fn from_code(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = int_of(&args[0])?;

    Ok(Value::Char((a as u8) as char))
}

fn to_upper(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = char_of(&args[0])?;

    Ok(Value::Char(a.to_ascii_uppercase()))
}

fn to_lower(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = char_of(&args[0])?;

    Ok(Value::Char(a.to_ascii_lowercase()))
}

fn to_locale_upper(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = char_of(&args[0])?;

    Ok(Value::Char(a.to_uppercase().to_string().chars().next().unwrap()))
}

fn to_locale_lower(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = char_of(&args[0])?;

    Ok(Value::Char(a.to_ascii_lowercase().to_string().chars().next().unwrap()))
}
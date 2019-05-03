use ast::Float;
use ast::Int;
use ast::Type;
use builtin::func_of;
use errors::ElmError;
use errors::InterpreterError;
use errors::Wrappable;
use interpreter::Interpreter;
use rust_interop::conversions::bool_of;
use rust_interop::conversions::char_of;
use rust_interop::conversions::int_of;
use rust_interop::conversions::list_of;
use rust_interop::conversions::str_of;
use types::Value;

pub fn get_string_funs() -> Vec<(&'static str, Type, Value)> {
    vec![
        func_of("cons", "Char -> String -> String", cons),
        func_of("uncons", "String -> Maybe (Char, String)", uncons),
        func_of("append", "String -> String -> String", append),
        func_of("length", "String -> Int", length),
        func_of("map", "(Char -> Char) -> String -> String", map),
        func_of("filter", "(Char -> Bool) -> String -> String", filter),
        func_of("reverse", "String -> String", reverse),
        func_of("foldl", "(Char -> b -> b) -> b -> String -> b", foldl),
        func_of("foldr", "(Char -> b -> b) -> b -> String -> b", foldr),
        func_of("split", "String -> String -> Array String", split),
        func_of("join", "String -> Array String -> String", join),
        func_of("slice", "Int -> Int -> String -> String", slice),
        func_of("trim", "String -> String", trim),
        func_of("trimLeft", "String -> String", trim_left),
        func_of("trimRight", "String -> String", trim_right),
        func_of("words", "String -> List String", words),
        func_of("lines", "String -> List String", lines),
        func_of("toUpper", "String -> String", to_upper),
        func_of("toLower", "String -> String", to_lower),
        func_of("any", "(Char -> Bool) -> String -> Bool", any),
        func_of("all", "(Char -> Bool) -> String -> Bool", all),
        func_of("contains", "String -> String -> Bool", contains),
        func_of("startsWith", "String -> String -> Bool", starts_with),
        func_of("endsWith", "String -> String -> Bool", ends_with),
        func_of("indexes", "String -> String -> List Int", indexes),
        func_of("fromNumber", "number -> String", from_number),
        func_of("toInt", "String -> Maybe Int", to_int),
        func_of("toFloat", "String -> Maybe Float", to_float),
        func_of("fromList", "List Char -> String", from_list),
    ]
}

fn cons(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let ch = char_of(&args[0])?;
    let string = str_of(&args[1])?;

    let mut result = String::new();
    result.push(ch);
    result.push_str(string);

    Ok(Value::String(result))
}

fn uncons(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = str_of(&args[0])?;

    if string.len() == 0 {
        return Ok(i.adt_value("Nothing", &[])?);
    }

    let first = string.chars().next().unwrap();
    let rest = string[1..].to_string();

    let tuple = Value::Tuple(vec![Value::Char(first), Value::String(rest)]);

    Ok(i.adt_value("Just", &[tuple])?)
}

fn append(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let base = str_of(&args[0])?;
    let suffix = str_of(&args[1])?;

    let mut result = String::new();
    result.push_str(base);
    result.push_str(suffix);

    Ok(Value::String(result))
}

fn length(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let base = str_of(&args[0])?;

    Ok(Value::Int(base.len() as Int))
}

fn map(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let mut result = String::new();
    let func = &args[0];
    let string = str_of(&args[1])?;

    for ch in string.chars() {
        let res = i.apply_function(func.clone(), &[Value::Char(ch)])?;

        result.push(char_of(&res)?);
    }

    Ok(Value::String(result))
}

fn filter(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let mut result = String::new();
    let func = &args[0];
    let string = str_of(&args[1])?;

    for ch in string.chars() {
        let res = i.apply_function(func.clone(), &[Value::Char(ch)])?;

        if bool_of(&res)? {
            result.push(ch);
        }
    }

    Ok(Value::String(result))
}

fn reverse(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = str_of(&args[0])?;
    let result = string.chars().rev().collect::<String>();

    Ok(Value::String(result))
}

fn foldl(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let func = &args[0];
    let string = str_of(&args[2])?;

    let mut result = args[1].clone();

    for ch in string.chars() {
        result = i.apply_function(func.clone(), &[Value::Char(ch), result])?;
    }

    Ok(result)
}

fn foldr(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let func = &args[0];
    let string = str_of(&args[2])?;

    let mut result = args[1].clone();

    for ch in string.chars().rev() {
        result = i.apply_function(func.clone(), &[Value::Char(ch), result])?;
    }

    Ok(result)
}

fn split(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let pattern = str_of(&args[0])?;
    let string = str_of(&args[1])?;

    let mut result = vec![];

    for sub in string.split(pattern) {
        result.push(Value::String(sub.to_string()));
    }

    // TODO array not list
    Ok(Value::List(result))
}

fn join(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let glue = str_of(&args[0])?;
    // TODO array not list
    let array = list_of(&args[1])?;

    let mut result = String::new();

    for (i, item) in array.iter().enumerate() {
        result.push_str(str_of(item)?);

        if i == array.len() - 1 {
            result.push_str(glue);
        }
    }

    Ok(Value::String(result))
}

fn slice(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let start = int_of(&args[0])?;
    let end = int_of(&args[1])?;
    let string = str_of(&args[2])?;

    let start = string.char_indices().nth(start as usize).unwrap().0;
    let end = string.char_indices().nth(end as usize).unwrap().0;
    let result = string[start..end].to_string();

    Ok(Value::String(result))
}

fn trim(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = str_of(&args[0])?;

    Ok(Value::String(string.trim().to_string()))
}

fn trim_left(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = str_of(&args[0])?;

    Ok(Value::String(string.trim_start().to_string()))
}

fn trim_right(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = str_of(&args[0])?;

    Ok(Value::String(string.trim_end().to_string()))
}

fn words(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = str_of(&args[0])?;
    let mut result = vec![];

    for line in string.split_whitespace() {
        result.push(Value::String(line.to_string()))
    }

    Ok(Value::List(result))
}

fn lines(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = str_of(&args[0])?;
    let mut result = vec![];

    for line in string.split_terminator('\n') {
        result.push(Value::String(line.to_string()))
    }

    Ok(Value::List(result))
}

fn to_upper(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = str_of(&args[0])?;

    Ok(Value::String(string.to_uppercase()))
}

fn to_lower(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = str_of(&args[0])?;

    Ok(Value::String(string.to_lowercase()))
}

fn any(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let func = &args[0];
    let string = str_of(&args[1])?;

    for ch in string.chars() {
        let res = i.apply_function(func.clone(), &[Value::Char(ch)])?;

        if bool_of(&res)? {
            return Ok(i.true_value());
        }
    }

    Ok(i.false_value())
}

fn all(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let func = &args[0];
    let string = str_of(&args[1])?;

    for ch in string.chars() {
        let res = i.apply_function(func.clone(), &[Value::Char(ch)])?;

        if !bool_of(&res)? {
            return Ok(i.false_value());
        }
    }

    Ok(i.true_value())
}

fn contains(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let pattern = str_of(&args[0])?;
    let string = str_of(&args[1])?;

    if string.contains(pattern) {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn starts_with(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let pattern = str_of(&args[0])?;
    let string = str_of(&args[1])?;

    if string.starts_with(pattern) {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn ends_with(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let pattern = str_of(&args[0])?;
    let string = str_of(&args[1])?;

    if string.ends_with(pattern) {
        Ok(i.true_value())
    } else {
        Ok(i.false_value())
    }
}

fn indexes(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let sub = str_of(&args[0])?;
    let string = str_of(&args[1])?;

    let sub_len = sub.len();

    if sub_len < 1 {
        return Ok(Value::List(vec![]));
    }

    let mut i = 0;
    let mut result = vec![];

    while let Some(new_i) = string[i..].find(sub) {
        result.push(Value::Int(new_i as Int));
        i = new_i + sub_len;
    }

    Ok(Value::List(result))
}

fn from_number(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    match &args[0] {
        Value::Number(i) | Value::Int(i) => {
            Ok(Value::String(format!("{}", i)))
        }
        Value::Float(i) => {
            Ok(Value::String(format!("{}", i)))
        }
        _ => Err(InterpreterError::ExpectedNumber(args[0].clone()).wrap())
    }
}

fn to_int(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = str_of(&args[0])?;

    match string.parse::<Int>() {
        Ok(v) => Ok(i.adt_value("Just", &[Value::Int(v)])?),
        _ => Ok(i.adt_value("Nothing", &[])?)
    }
}

fn to_float(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = str_of(&args[0])?;

    match string.parse::<Float>() {
        Ok(v) => Ok(i.adt_value("Just", &[Value::Float(v)])?),
        _ => Ok(i.adt_value("Nothing", &[])?)
    }
}

fn from_list(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let list = list_of(&args[0])?;
    let mut result = String::new();

    for ch in list {
        result.push(char_of(ch)?);
    }

    Ok(Value::String(result))
}
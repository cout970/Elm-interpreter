use interpreter::RuntimeError;
use interpreter::RuntimeError::*;
use ast::Type;
use types::Value;
use util::StringConversion;


pub fn builtin_function(id: u32, args: &[Value]) -> Result<Value, RuntimeError> {
    let ret = match id {
        0 => Value::Unit,
        1 => number_op(&args[0], &args[1], |a, b| a + b)?,
        2 => number_op(&args[0], &args[1], |a, b| a - b)?,
        3 => number_op(&args[0], &args[1], |a, b| a * b)?,
        4 => Value::Float(float_of(&args[0])? / float_of(&args[1])?),
        5 => Value::Int(int_of(&args[0])? / int_of(&args[1])?),
        6 => { // Record access function (.x)
            match &args[0] {
                Value::Record(entries) => {
                    if let Value::String(field) = &args[1] {
                        let opt = entries.iter()
                            .find(|(name, _)| name == field)
                            .map(|(_, val)| val);

                        match opt {
                            Some(val) => val.clone(),
                            None => {
                                return Err(RecordFieldNotFound(field.clone(), args[0].clone()));
                            }
                        }
                    } else {
                        return Err(InternalErrorRecordAccess(args[1].clone()));
                    }
                }
                _ => { return Err(ExpectedRecord(args[0].clone())); }
            }
        }
        7 => { // Adt constructor
            if let Value::Adt(var, _, adt) = &args[0] {
                let mut vals: Vec<Value> = vec![];
                for i in 1..args.len() {
                    vals.push(args[i].clone());
                }

                Value::Adt(var.to_owned(), vals, adt.clone())
            } else {
                return Err(InternalErrorAdtCreation(args[0].clone()));
            }
        }
        _ => { return Err(UnknownBuiltinFunction(id)); }
    };

    Ok(ret)
}

fn float_of(value: &Value) -> Result<f32, RuntimeError> {
    match value {
        Value::Number(a) => Ok(*a as f32),
        Value::Float(a) => Ok(*a),
        _ => {
            Err(ExpectedFloat(value.clone()))
        }
    }
}

fn int_of(value: &Value) -> Result<i32, RuntimeError> {
    match value {
        Value::Number(a) => Ok(*a),
        Value::Int(a) => Ok(*a),
        _ => {
            Err(ExpectedInt(value.clone()))
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum NumberState {
    Number,
    Int,
    Float,
}

fn number_op<F: FnOnce(f32, f32) -> f32>(val_a: &Value, val_b: &Value, op: F) -> Result<Value, RuntimeError> {
    let mut strong_type: NumberState;

    let a = match val_a {
        Value::Number(a) => {
            strong_type = NumberState::Number;
            *a as f32
        }
        Value::Int(a) => {
            strong_type = NumberState::Int;
            *a as f32
        }
        Value::Float(a) => {
            strong_type = NumberState::Float;
            *a
        }
        _ => {
            return Err(ExpectedNumber(val_a.clone()));
        }
    };

    let b = match val_b {
        Value::Number(a) => {
            strong_type = merge(strong_type, NumberState::Number, val_b)?;
            *a as f32
        }
        Value::Int(a) => {
            strong_type = merge(strong_type, NumberState::Int, val_b)?;
            *a as f32
        }
        Value::Float(a) => {
            strong_type = merge(strong_type, NumberState::Float, val_b)?;
            *a
        }
        _ => {
            return Err(ExpectedNumber(val_a.clone()));
        }
    };

    let result = op(a, b);

    Ok(match strong_type {
        NumberState::Number => Value::Number(result as i32),
        NumberState::Int => Value::Int(result as i32),
        NumberState::Float => Value::Float(result),
    })
}

/*
Truth table of number for:
(+) : number, number -> number

Float, Float -> Float
number, Float -> Float
Float, number -> Float

Int, Int -> Int
number, Int -> Int
Int, number -> Int

Int, Float -> error
Float, Int -> error
*/
fn merge(a: NumberState, b: NumberState, value: &Value) -> Result<NumberState, RuntimeError> {
    match a {
        NumberState::Number => Ok(b),
        NumberState::Int => {
            if b == NumberState::Int || b == NumberState::Number {
                Ok(a)
            } else {
                Err(ExpectedInt(value.clone()))
            }
        }
        NumberState::Float => {
            if b == NumberState::Float || b == NumberState::Number {
                Ok(a)
            } else {
                Err(ExpectedFloat(value.clone()))
            }
        }
    }
}
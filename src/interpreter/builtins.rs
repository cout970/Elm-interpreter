use errors::*;
use errors::RuntimeError::*;
use Runtime;
use types::ExternalFunc;
use types::Value;

// Language builtins

pub fn builtin_unit_fun() -> ExternalFunc {
    ExternalFunc {
        name: "unit".to_string(),
        fun: |_, _| Ok(Value::Unit),
    }
}

pub fn builtin_record_access() -> ExternalFunc {
    ExternalFunc {
        name: "record access".to_string(),
        fun: |_, args| {
            match &args[0] {
                Value::Record(entries) => {
                    if let Value::String(field) = &args[1] {
                        let opt = entries.iter()
                            .find(|(name, _)| name == field)
                            .map(|(_, val)| val);

                        match opt {
                            Some(val) => Ok(val.clone()),
                            None => {
                                Err(RecordFieldNotFound(field.clone(), args[0].clone()))
                            }
                        }
                    } else {
                        Err(InternalErrorRecordAccess(args[1].clone()))
                    }
                }
                _ => Err(ExpectedRecord(args[0].clone()))
            }
        },
    }
}

pub fn builtin_adt_constructor() -> ExternalFunc {
    ExternalFunc {
        name: "ADT constructor".to_string(),
        fun: |_, args| {
            if let Value::Adt(var, _, adt) = &args[0] {
                let mut vals: Vec<Value> = vec![];
                for i in 1..args.len() {
                    vals.push(args[i].clone());
                }

                Ok(Value::Adt(var.to_owned(), vals, adt.clone()))
            } else {
                Err(InternalErrorAdtCreation(args[0].clone()))
            }
        },
    }
}

// Aridmetic operators

pub fn builtin_add(_: &mut Runtime, args: &Vec<Value>) -> Result<Value, RuntimeError> {
    number_op(&args[0], &args[1], |a, b| a + b)
}

pub fn builtin_sub(_: &mut Runtime, args: &Vec<Value>) -> Result<Value, RuntimeError> {
    number_op(&args[0], &args[1], |a, b| a - b)
}

pub fn builtin_times(_: &mut Runtime, args: &Vec<Value>) -> Result<Value, RuntimeError> {
    number_op(&args[0], &args[1], |a, b| a * b)
}

pub fn builtin_float_div(_: &mut Runtime, args: &Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(Value::Float(float_of(&args[0])? / float_of(&args[1])?))
}

pub fn builtin_int_div(_: &mut Runtime, args: &Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(Value::Int(int_of(&args[0])? / int_of(&args[1])?))
}

pub fn builtin_string_append(_: &mut Runtime, args: &Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(Value::String(format!("{}{}", string_of(&args[0])?, string_of(&args[1])?)))
}

// (<<) << (<<)

// Utility functions

fn apply(_func: Value, _arg: Value) -> Result<Value, RuntimeError> {
    //TODO use a Value::Fun
    unimplemented!()
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

fn string_of(value: &Value) -> Result<String, RuntimeError> {
    match value {
        Value::String(string) => Ok(string.clone()),
        _ => {
            Err(ExpectedString(value.clone()))
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
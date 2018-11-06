use errors::ErrorWrapper;
use interpreter::RuntimeError;
use interpreter::RuntimeError::*;
use types::BuiltinFunction;
use types::Value;
use types::BuiltinFunctionRef;
use std::cell::RefCell;

impl<T> BuiltinFunction for T
    where T: Fn(&Vec<Value>) -> Result<Value, RuntimeError>
{
    fn call_function(&mut self, args: &Vec<Value>) -> Result<Value, ErrorWrapper> {
        self(args).map_err(|e| ErrorWrapper::Runtime(e))
    }
}

pub fn of_closure<C: Fn(&Vec<Value>) -> Result<Value, RuntimeError> + 'static>(closure: C) -> BuiltinFunctionRef {
    RefCell::new(Box::new(closure))
}

// Language builtins

pub fn builtin_unit_fun() -> BuiltinFunctionRef {
    of_closure(|_: &Vec<Value>| Ok(Value::Unit))
}

pub fn builtin_record_access() -> BuiltinFunctionRef {
    of_closure(|args: &Vec<Value>| {
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
    })
}

pub fn builtin_adt_constructor() -> BuiltinFunctionRef {
    of_closure(|args: &Vec<Value>| {
        if let Value::Adt(var, _, adt) = &args[0] {
            let mut vals: Vec<Value> = vec![];
            for i in 1..args.len() {
                vals.push(args[i].clone());
            }

            Ok(Value::Adt(var.to_owned(), vals, adt.clone()))
        } else {
            Err(InternalErrorAdtCreation(args[0].clone()))
        }
    })
}

// Aridmetic operators

pub fn builtin_add(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    number_op(&args[0], &args[1], |a, b| a + b)
}

pub fn builtin_sub(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    number_op(&args[0], &args[1], |a, b| a - b)
}

pub fn builtin_times(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    number_op(&args[0], &args[1], |a, b| a * b)
}

pub fn builtin_float_div(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(Value::Float(float_of(&args[0])? / float_of(&args[1])?))
}

pub fn builtin_int_div(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(Value::Int(int_of(&args[0])? / int_of(&args[1])?))
}

// Combinators

pub fn builtin_id(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(args[0].clone())
}

pub fn builtin_mockingbird(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    apply(args[0].clone(), args[0].clone())
}

pub fn builtin_kestrel(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(args[0].clone())
}

pub fn builtin_kite(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(args[1].clone())
}

pub fn builtin_cardinal(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    apply(apply(args[0].clone(), args[2].clone())?, args[1].clone())
}

pub fn builtin_bluebird(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    apply(args[0].clone(), apply(args[1].clone(), args[2].clone())?)
}

pub fn builtin_thrush(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    apply(args[1].clone(), args[0].clone())
}

pub fn builtin_vireo(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    apply(apply(args[2].clone(), args[0].clone())?, args[1].clone())
}

pub fn builtin_blackbird(args: &Vec<Value>) -> Result<Value, RuntimeError> {
    apply(args[0].clone(), apply(apply(args[1].clone(), args[2].clone())?, args[3].clone())?)
}

// (<<) << (<<)

// Utility functions

fn apply(func: Value, arg: Value) -> Result<Value, RuntimeError> {
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
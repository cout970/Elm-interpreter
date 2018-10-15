use interpreter::RuntimeError;
use interpreter::RuntimeError::InternalError;
use interpreter::RuntimeError::TODO;
use types::Type;
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
        6 => {
            match &args[0] {
                Value::Record(entries) => {
                    if let Value::String(field) = &args[1] {
                        let opt = entries.iter()
                            .find(|(name, _)| name == field)
                            .map(|(_, val)| val);

                        match opt {
                            Some(val) => val.clone(),
                            None => {
                                return Err(TODO(format!("Field '{}' not found in record: {}", field, &args[0])));
                            }
                        }
                    } else {
                        return Err(TODO(format!("Internal error, expecting String found: {}", &args[1])));
                    }
                }
                _ => { return Err(TODO(format!("Expecting record but found: {}", id))); }
            }
        }
        7 => {
            if let Value::Adt(var, _, adt) = &args[0] {
                let mut vals: Vec<Value> = vec![];
                for i in 1..args.len() {
                    vals.push(args[i].clone());
                }

                Value::Adt(var.to_owned(), vals, adt.clone())
            } else {
                return Err(InternalError);
            }
        }
        _ => { return Err(TODO(format!("Invalid builtin function: {}", id))); }
    };

    Ok(ret)
}

fn float_of(value: &Value) -> Result<f32, RuntimeError> {
    match value {
        Value::Number(a) => Ok(*a as f32),
        Value::Float(a) => Ok(*a),
        _ => {
            Err(TODO(format!("Expected Float but found: {}", value)))
        }
    }
}

fn int_of(value: &Value) -> Result<i32, RuntimeError> {
    match value {
        Value::Number(a) => Ok(*a),
        Value::Int(a) => Ok(*a),
        _ => {
            Err(TODO(format!("Expected Int but found: {}", value)))
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
            return Err(TODO(format!("Expected number but found: {}", val_a)));
        }
    };

    let b = match val_b {
        Value::Number(a) => {
            strong_type = merge(strong_type, NumberState::Number)?;
            *a as f32
        }
        Value::Int(a) => {
            strong_type = merge(strong_type, NumberState::Int)?;
            *a as f32
        }
        Value::Float(a) => {
            strong_type = merge(strong_type, NumberState::Float)?;
            *a
        }
        _ => {
            return Err(TODO(format!("Expected number but found: {}", val_b)));
        }
    };

    let result = op(a, b);

    Ok(match strong_type {
        NumberState::Number => Value::Number(result as i32),
        NumberState::Int => Value::Int(result as i32),
        NumberState::Float => Value::Float(result),
    })
}

fn merge(a: NumberState, b: NumberState) -> Result<NumberState, RuntimeError> {
    match a {
        NumberState::Number => Ok(b),
        NumberState::Int => {
            if b == NumberState::Int || b == NumberState::Number {
                Ok(a)
            } else {
                Err(TODO(format!("Expected Int but found: {:?}", b)))
            }
        }
        NumberState::Float => {
            if b == NumberState::Float || b == NumberState::Number {
                Ok(a)
            } else {
                Err(TODO(format!("Expected Float but found: {:?}", b)))
            }
        }
    }
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
use types::Value;
use interpreter::RuntimeError;
use interpreter::RuntimeError::TODO;


pub fn builtin_function(id: u32, args: &[Value]) -> Result<Value, RuntimeError> {
    let ret = match id {
        0 => Value::Unit,
        1 => Value::Float(float_of(&args[0])? + float_of(&args[1])?),
        2 => Value::Float(float_of(&args[0])? - float_of(&args[1])?),
        3 => Value::Float(float_of(&args[0])? * float_of(&args[1])?),
        4 => Value::Float(float_of(&args[0])? / float_of(&args[1])?),
        5 => Value::Int(float_of(&args[0])? as i32 / float_of(&args[1])? as i32),
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
        _ => { return Err(TODO(format!("Invalid builtin function: {}", id))); }
    };

    Ok(ret)
}

fn float_of(value: &Value) -> Result<f32, RuntimeError> {
    match value {
        Value::Int(a) => Ok(*a as f32),
        Value::Float(a) => Ok(*a),
        _ => {
            Err(TODO(format!("Expected number but found: {}", value)))
        }
    }
}
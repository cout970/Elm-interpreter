use std::any::Any;
use std::collections::HashMap;

use ast::Float;
use ast::Int;
use types::Value;

pub fn convert_to_rust(value: &Value) -> Option<Box<Any>> {
    match value {
        Value::Unit => {
            return Some(Box::new(()));
        }
        Value::Number(val) => {
            return Some(Box::new(val.clone()));
        }
        Value::Int(val) => {
            return Some(Box::new(val.clone()));
        }
        Value::Float(val) => {
            return Some(Box::new(val.clone()));
        }
        Value::String(val) => {
            return Some(Box::new(val.clone()));
        }
        Value::Char(val) => {
            return Some(Box::new(val.clone()));
        }
        Value::List(items) => {
            return Some(Box::new(items.clone()));
        }
        Value::Tuple(items) => {
            return Some(Box::new(items.clone()));
        }
        Value::Record(entries) => {
            return Some(Box::new(entries.clone()));
        }
        Value::Adt(_, _, _) => {
            return None;
        }
        Value::Fun { .. } => {
            return None;
        }
    }
}

pub fn convert_from_rust(val: &Any) -> Option<Value> {
    if let Some(()) = val.downcast_ref::<()>() {
        return Some(Value::Unit);
    }
    if let Some(unwrapped) = val.downcast_ref::<Int>() {
        return Some(Value::Int(*unwrapped));
    }
    if let Some(unwrapped) = val.downcast_ref::<Float>() {
        return Some(Value::Float(*unwrapped));
    }
    if let Some(unwrapped) = val.downcast_ref::<String>() {
        return Some(Value::String(unwrapped.clone()));
    }
    if let Some(unwrapped) = val.downcast_ref::<char>() {
        return Some(Value::Char(*unwrapped));
    }

    if let Some(unwrapped) = val.downcast_ref::<Vec<Box<Any>>>() {
        let values = unwrapped.iter()
            .map(|t| convert_from_rust(t))
            .collect::<Option<Vec<Value>>>()?;

        return Some(Value::List(values));
    }

    if let Some(unwrapped) = val.downcast_ref::<HashMap<String, Box<Any>>>() {
        let mut values: Vec<(String, Value)> = vec![];

        for (key, value) in unwrapped {
            values.push((key.clone(), convert_from_rust(value)?));
        }

        return Some(Value::Record(values));
    }

    if let Some(unwrapped) = val.downcast_ref::<(Box<Any>, Box<Any>)>() {
        let values = vec![
            convert_from_rust(&*unwrapped.0)?,
            convert_from_rust(&*unwrapped.1)?,
        ];
        return Some(Value::Tuple(values));
    }
    if let Some(unwrapped) = val.downcast_ref::<(Box<Any>, Box<Any>, Box<Any>)>() {
        let values = vec![
            convert_from_rust(&*unwrapped.0)?,
            convert_from_rust(&*unwrapped.1)?,
            convert_from_rust(&*unwrapped.2)?,
        ];
        return Some(Value::Tuple(values));
    }
    if let Some(unwrapped) = val.downcast_ref::<(Box<Any>, Box<Any>, Box<Any>, Box<Any>)>() {
        let values = vec![
            convert_from_rust(&*unwrapped.0)?,
            convert_from_rust(&*unwrapped.1)?,
            convert_from_rust(&*unwrapped.2)?,
            convert_from_rust(&*unwrapped.3)?,
        ];
        return Some(Value::Tuple(values));
    }
    if let Some(unwrapped) = val.downcast_ref::<(Box<Any>, Box<Any>, Box<Any>, Box<Any>, Box<Any>)>() {
        let values = vec![
            convert_from_rust(&*unwrapped.0)?,
            convert_from_rust(&*unwrapped.1)?,
            convert_from_rust(&*unwrapped.2)?,
            convert_from_rust(&*unwrapped.3)?,
            convert_from_rust(&*unwrapped.4)?,
        ];
        return Some(Value::Tuple(values));
    }

    if let Some(unwrapped) = val.downcast_ref::<(Box<Any>, Box<Any>, Box<Any>, Box<Any>, Box<Any>, Box<Any>)>() {
        let values = vec![
            convert_from_rust(&*unwrapped.0)?,
            convert_from_rust(&*unwrapped.1)?,
            convert_from_rust(&*unwrapped.2)?,
            convert_from_rust(&*unwrapped.3)?,
            convert_from_rust(&*unwrapped.4)?,
            convert_from_rust(&*unwrapped.5)?,
        ];
        return Some(Value::Tuple(values));
    }

    if let Some(unwrapped) = val.downcast_ref::<(Box<Any>, Box<Any>, Box<Any>, Box<Any>, Box<Any>, Box<Any>, Box<Any>)>() {
        let values = vec![
            convert_from_rust(&*unwrapped.0)?,
            convert_from_rust(&*unwrapped.1)?,
            convert_from_rust(&*unwrapped.2)?,
            convert_from_rust(&*unwrapped.3)?,
            convert_from_rust(&*unwrapped.4)?,
            convert_from_rust(&*unwrapped.5)?,
            convert_from_rust(&*unwrapped.6)?,
        ];
        return Some(Value::Tuple(values));
    }

    if let Some(unwrapped) = val.downcast_ref::<(Box<Any>, Box<Any>, Box<Any>, Box<Any>, Box<Any>, Box<Any>, Box<Any>, Box<Any>)>() {
        let values = vec![
            convert_from_rust(&*unwrapped.0)?,
            convert_from_rust(&*unwrapped.1)?,
            convert_from_rust(&*unwrapped.2)?,
            convert_from_rust(&*unwrapped.3)?,
            convert_from_rust(&*unwrapped.4)?,
            convert_from_rust(&*unwrapped.5)?,
            convert_from_rust(&*unwrapped.6)?,
            convert_from_rust(&*unwrapped.7)?,
        ];
        return Some(Value::Tuple(values));
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_int() {
        let result = convert_to_rust(&convert_from_rust(&1).unwrap()).unwrap();
        assert_eq!(*result.downcast::<Int>().unwrap(), 1);
    }

    #[test]
    fn check_float() {
        let result = convert_to_rust(&convert_from_rust(&(1.5 as Float)).unwrap()).unwrap();
        assert_eq!(*result.downcast::<Float>().unwrap(), 1.5);
    }

    #[test]
    fn check_string() {
        let result = convert_to_rust(&convert_from_rust(&String::from("Hello world")).unwrap()).unwrap();
        assert_eq!(&*result.downcast::<String>().unwrap(), "Hello world");
    }

}
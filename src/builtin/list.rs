use std::cmp::Ordering;

use ast::Type;
use builtin::func_of;
use builtin::utils::compare_values;
use errors::*;
use interpreter::Interpreter;
use rust_interop::conversions::list_of;
use types::Value;

pub fn get_list_funs() -> Vec<(&'static str, Type, Value)> {
    vec![
        func_of("cons", "a -> List a -> List a", cons),
        func_of("::", "a -> List a -> List a", cons),
        func_of("map2", "(a -> b -> result) -> List a -> List b -> List result", map2),
        func_of("map3", "(a -> b -> c -> result) -> List a -> List b -> List c -> List result", map3),
        func_of("map4", "(a -> b -> c -> d -> result) -> List a -> List b -> List c -> List d -> List result", map4),
        func_of("map5", "(a -> b -> c -> d -> e -> result) -> List a -> List b -> List c -> List d -> List e -> List result", map5),
        func_of("fromArray", "Array a -> List a", temp_do_nothing),
        func_of("toArray", "List a -> Array a", temp_do_nothing),
        func_of("sortBy", "(a -> comparable) ->  List a -> List a", sort_by),
        func_of("sortWith", "(a -> a -> Order) ->  List a -> List a", sort_with),
    ]
}

fn temp_do_nothing(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    Ok(args[0].clone())
}

fn cons(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let mut result = vec![args[0].clone()];
    let list = list_of(&args[1])?;

    for val in list {
        result.push(val.clone());
    }

    Ok(Value::List(result))
}

fn map2(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let mut result = vec![];
    let func = &args[0];
    let list_a = list_of(&args[1])?;
    let list_b = list_of(&args[2])?;
    let len = list_a.len().min(list_b.len());

    for index in 0..len {
        let res = i.apply_function(func.clone(), &[
            list_a[index].clone(),
            list_b[index].clone(),
        ])?;

        result.push(res);
    }

    Ok(Value::List(result))
}

fn map3(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let mut result = vec![];
    let func = &args[0];
    let list_a = list_of(&args[1])?;
    let list_b = list_of(&args[2])?;
    let list_c = list_of(&args[3])?;
    let len = list_a.len().min(list_b.len()).min(list_c.len());

    for index in 0..len {
        let res = i.apply_function(func.clone(), &[
            list_a[index].clone(),
            list_b[index].clone(),
            list_c[index].clone(),
        ])?;

        result.push(res);
    }

    Ok(Value::List(result))
}

fn map4(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let mut result = vec![];
    let func = &args[0];
    let list_a = list_of(&args[1])?;
    let list_b = list_of(&args[2])?;
    let list_c = list_of(&args[3])?;
    let list_d = list_of(&args[4])?;
    let len = list_a.len().min(list_b.len()).min(list_c.len()).min(list_d.len());

    for index in 0..len {
        let res = i.apply_function(func.clone(), &[
            list_a[index].clone(),
            list_b[index].clone(),
            list_c[index].clone(),
            list_d[index].clone(),
        ])?;

        result.push(res);
    }

    Ok(Value::List(result))
}

fn map5(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let mut result = vec![];
    let func = &args[0];
    let list_a = list_of(&args[1])?;
    let list_b = list_of(&args[2])?;
    let list_c = list_of(&args[3])?;
    let list_d = list_of(&args[4])?;
    let list_e = list_of(&args[5])?;
    let len = list_a.len().min(list_b.len()).min(list_c.len()).min(list_d.len()).min(list_e.len());

    for index in 0..len {
        let res = i.apply_function(func.clone(), &[
            list_a[index].clone(),
            list_b[index].clone(),
            list_c[index].clone(),
            list_d[index].clone(),
            list_e[index].clone(),
        ])?;

        result.push(res);
    }

    Ok(Value::List(result))
}


//fn from_array(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
//    let mut result = vec![];
//    let func = &args[0];
//
//    Ok(Value::List(result))
//}

fn sort_by(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let func = &args[0];
    let mut list = list_of(&args[0])?.to_vec();
    let mut errors = vec![];

    list.sort_by(|a, b| {
        let cmp_a = i.apply_function(func.clone(), &[a.clone()]);
        let cmp_b = i.apply_function(func.clone(), &[b.clone()]);

        match (cmp_a, cmp_b) {
            (Ok(a), Ok(b)) => compare_values(&a, &b),
            (Err(e), _) | (_, Err(e)) => {
                errors.push(e);
                Ordering::Equal
            }
        }
    });

    if !errors.is_empty() {
        Err(ElmError::List(errors))
    } else {
        Ok(Value::List(list))
    }
}

fn sort_with(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let func = &args[0];
    let mut list = list_of(&args[0])?.to_vec();
    let mut errors = vec![];
    // (a -> a -> Order) ->  List a -> List a

    list.sort_by(|a, b| {
        let order = i.apply_function(func.clone(), &[a.clone(), b.clone()]);

        match order {
            Ok(Value::Adt(name, args, adt)) => {
                match name.as_str() {
                    "GT" => Ordering::Greater,
                    "LT" => Ordering::Less,
                    "EQ" => Ordering::Equal,
                    _ => {
                        errors.push(InterpreterError::ExpectedAdt(Value::Adt(name, args, adt)).wrap());
                        Ordering::Equal
                    }
                }
            }
            Ok(value) => {
                errors.push(InterpreterError::ExpectedAdt(value).wrap());
                Ordering::Equal
            }
            Err(e) => {
                errors.push(e);
                Ordering::Equal
            }
        }
    });

    if !errors.is_empty() {
        Err(ElmError::List(errors))
    } else {
        Ok(Value::List(list))
    }
}
use ast::Type;
use builtin::func_of;
use errors::ElmError;
use errors::InterpreterError;
use errors::Wrappable;
use interpreter::Interpreter;
use rust_interop::conversions::string_of;
use types::Value;

pub fn get_debug_funs() -> Vec<(&'static str, Type, Value)> {
    vec![
        func_of("toString", "a -> String", to_string),
        func_of("log", "String -> a -> a", log),
        func_of("todo", "String -> a", todo),
    ]
}

fn to_string(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    Ok(Value::String(format!("{}", &args[0])))
}

fn log(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = string_of(&args[0])?;
    println!("[log] {}", string);
    Ok(args[1].clone())
}

fn todo(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let string = string_of(&args[0])?;

    Err(InterpreterError::FunctionTODO(string).wrap())
}

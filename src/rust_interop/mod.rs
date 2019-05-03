use std::any::Any;
use std::any::TypeId;
use std::sync::Arc;

use ast::Float;
use ast::Int;
use ast::Type;
use errors::*;
use errors::ElmError;
use interpreter::Interpreter;
use Runtime;
use rust_interop::conversions::convert_from_rust;
use rust_interop::conversions::convert_to_rust;
use rust_interop::function_register::FunctionRegister;
use types::Function;
use types::next_fun_id;
use types::Value;
use types::WrapperFunc;
use util::build_fun_type;

pub mod conversions;
pub mod function_register;
pub mod function_call;


pub type FnAny = Fn(&mut Interpreter, Vec<&mut Any>) -> Result<Box<Any>, ElmError> + Sync + Send;

struct FnWrapper {
    fun: Box<FnAny>
}

pub fn call_function(this: &WrapperFunc, i: &mut Interpreter, args: &Vec<Value>) -> Result<Value, ElmError> {
    let mut rust_values = vec![];

    for arg in args {
        let value = convert_to_rust(arg)
            .ok_or_else(|| InterpreterError::ImpossibleConversion.wrap())?;

        rust_values.push(value);
    }

    let mut arguments: Vec<&mut Any> = vec![];

    for val in rust_values.iter_mut() {
        arguments.push(val);
    }

    let result: Result<Box<Any>, ElmError> = (this.fun)(i, arguments);
    match result {
        Ok(boxed) => {
            convert_from_rust(&*boxed)
                .ok_or_else(|| InterpreterError::ImpossibleConversion.wrap())
        }
        Err(e) => Err(e)
    }
}

impl FunctionRegister for Runtime {
    fn register_fn_raw(&mut self, name: String, args: Vec<TypeId>, ret: TypeId, boxed: Box<FnAny>) -> Result<(), ElmError> {
        let len = args.len() as u32;
        let ty = type_from_ids(args, ret)
            .map_err(|e| e.wrap())?;

        let function = Arc::new(Function::Wrapper(
            next_fun_id(),
            WrapperFunc { name: name.to_string(), fun: boxed },
            ty.clone(),
        ));

        let value = Value::Fun {
            arg_count: len,
            args: vec![],
            fun: function,
        };

        self.analyzer.add_port(&name, ty);
        self.interpreter.stack.add(&name, value);
        Ok(())
    }
}

fn type_from_ids(args: Vec<TypeId>, ret: TypeId) -> Result<Type, InteropError> {
    let mut types = vec![];

    for (arg_index, arg) in args.into_iter().enumerate() {
        match type_from_id(arg) {
            Some(ty) => {
                types.push(ty);
            }
            None => {
                return Err(InteropError::FunRegistrationUnknownTypeArg(arg_index));
            }
        }
    }

    match type_from_id(ret) {
        Some(ty) => {
            types.push(ty);
        }
        None => {
            return Err(InteropError::FunRegistrationUnknownTypeRet);
        }
    }

    Ok(build_fun_type(&types))
}

fn type_from_id(id: TypeId) -> Option<Type> {
    if id == TypeId::of::<()>() {
        return Some(Type::Unit);
    }

    if id == TypeId::of::<String>() {
        return Some(Type::Tag("String".to_owned(), vec![]));
    }

    if id == TypeId::of::<Int>() {
        return Some(Type::Tag("Int".to_owned(), vec![]));
    }

    if id == TypeId::of::<Float>() {
        return Some(Type::Tag("Float".to_owned(), vec![]));
    }

    if id == TypeId::of::<char>() {
        return Some(Type::Tag("Char".to_owned(), vec![]));
    }

    if id == TypeId::of::<bool>() {
        return Some(Type::Tag("Bool".to_owned(), vec![]));
    }

    None
}

#[cfg(test)]
mod tests {
    use rust_interop::function_register::RegisterFn;

    use super::*;

    #[test]
    fn test_register_function() {
        let mut i = Runtime::new();
        i.register_fn("test_function", test_function).unwrap();
    }

    #[test]
    fn test_register_invalid_function() {
        let mut i = Runtime::new();
        let result = i.register_fn("test_function2", test_function2);
        assert_eq!(result, Err(InteropError::FunRegistrationUnknownTypeArg(0).wrap()));
    }

    fn test_function(a: i32) -> i32 { a }

    fn test_function2(a: Type) -> Type { a }
}
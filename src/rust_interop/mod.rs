use ast::Float;
use ast::Int;
use ast::Type;
use errors::ErrorWrapper;
use Interpreter;
use interpreter::RuntimeError;
use rust_interop::conversions::convert_from_rust;
use rust_interop::conversions::convert_to_rust;
use rust_interop::function_register::FunctionRegister;
use std::any::Any;
use std::any::TypeId;
use std::sync::Arc;
use types::BuiltinFunction;
use types::Function;
use types::Value;
use util::build_fun_type;
use std::cell::RefCell;
use types::BuiltinFunctionRef;

pub mod conversions;
pub mod function_register;
pub mod function_call;

#[derive(Clone, Debug, PartialEq)]
pub enum InteropError {
    FunctionArgMismatch,
    MismatchOutputType,
    FunctionNotFound(String),
    FunRegistrationUnknownTypeArg(usize),
    FunRegistrationUnknownTypeRet,
}

type FnAny = FnMut(Vec<&mut Any>) -> Result<Box<Any>, InteropError>;

struct FnWrapper{
    fun: Box<FnAny>
}

impl BuiltinFunction for FnWrapper {
    fn call_function(&mut self, args: &Vec<Value>) -> Result<Value, ErrorWrapper> {
        let mut rust_values = vec![];

        for arg in args {
            let value = convert_to_rust(arg)
                .ok_or_else(|| ErrorWrapper::Runtime(RuntimeError::ImpossibleConversion))?;

            rust_values.push(value);
        }

        let mut arguments: Vec<&mut Any> = vec![];

        for val in rust_values.iter_mut() {
            arguments.push(val);
        }

        let result: Result<Box<Any>, InteropError> = (self.fun)(arguments);
        match result {
            Ok(boxed) => {
                convert_from_rust(&*boxed)
                    .ok_or_else(|| ErrorWrapper::Runtime(RuntimeError::ImpossibleConversion))
            }
            Err(e) => {
                Err(ErrorWrapper::Interop(e))
            }
        }
    }
}

impl FunctionRegister for Interpreter {
    fn register_fn_raw(&mut self, name: String, args: Vec<TypeId>, ret: TypeId, boxed: Box<FnAny>) -> Result<(), InteropError> {
        let len = args.len() as u32;
        let ty = type_from_ids(args, ret)?;
        let func_ref: BuiltinFunctionRef = RefCell::new(Box::new(FnWrapper { fun: boxed} ));

        let function = Arc::new(Function::Builtin(
            self.env.next_fun_id(),
            func_ref,
            ty.clone(),
        ));

        let value = Value::Fun {
            arg_count: len,
            args: vec![],
            fun: function,
        };

        self.env.add(&name, value, ty);
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
    use super::*;
    use rust_interop::function_register::RegisterFn;

    #[test]
    fn test_register_function() {
        let mut i = Interpreter::new();
        i.register_fn("test_function", test_function).unwrap();
    }

    #[test]
    fn test_register_invalid_function() {
        let mut i = Interpreter::new();
        let result = i.register_fn("test_function2", test_function2);
        assert_eq!(result, Err(InteropError::FunRegistrationUnknownTypeArg(0)));
    }

    fn test_function(a: i32) -> i32 { a }

    fn test_function2(a: Type) -> Type { a }
}
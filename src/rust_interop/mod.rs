use std::any::Any;

pub mod conversions;
pub mod function_register;
pub mod function_call;

#[derive(Clone, Debug)]
pub enum InteropError {
    FunctionArgMismatch,
    MismatchOutputType,
    FunctionNotFound(String),
}

type FnAny = FnMut(Vec<&mut Any>) -> Result<Box<Any>, InteropError>;

/*
 * Copied (with modifications) from https://github.com/jonathandturner/rhai/blob/master/src/fn_register.rs
 * which has MIT/Apache-2.0 license
 */

use rust_interop::FnAny;
use rust_interop::InteropError;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;

//#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
//pub struct FnSpec {
//    ident: String,
//    args: Option<Vec<TypeId>>,
//}
//
//pub struct FunctionHandler {
//    pub functions: HashMap<FnSpec, Box<FnAny>>,
//}
//
//impl FunctionHandler {
//    pub fn new() -> Self {
//        Self {
//            functions: HashMap::new(),
//        }
//    }
//
//    pub fn register_fn_raw(&mut self, name: String, args: Option<Vec<TypeId>>, boxed: Box<FnAny>) {
//        self.functions.insert(FnSpec { ident: name, args }, boxed);
//    }
//
//    pub fn call_fn<'a, I, A, T>(&mut self, ident: I, args: A) -> Result<T, InteropError>
//        where
//            I: Into<String>,
//            A: FunArgs<'a>,
//            T: Any + Clone,
//    {
//        let types = args.get_types();
//        let arg_vec = args.into_vec();
//
//        self.call_fn_raw(ident.into(), arg_vec, types)
//            .and_then(|b| {
//                b.downcast()
//                    .map(|b| *b)
//                    .map_err(|_| InteropError::MismatchOutputType)
//            })
//    }
//
//    pub fn call_fn_raw(&mut self, ident: String, args: Vec<&mut Any>, arg_types: Vec<TypeId>) -> Result<Box<Any>, InteropError> {
//        let spec = FnSpec { ident: ident.clone(), args: Some(arg_types) };
//
//        self.functions.get_mut(&spec)
//            .ok_or_else(|| InteropError::FunctionNotFound(ident.clone()))
//            .and_then(move |f| {
//                f(args)
//            })
//    }
//}
//
//
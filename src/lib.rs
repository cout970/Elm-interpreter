// Development only {
// cargo watch -s 'clear && cargo test --color always 2>&1'
#![allow(dead_code)]
// }

#[macro_use]
extern crate nom;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod ast;
pub mod types;
#[macro_use]
mod util;
pub mod tokenizer;
pub mod parsers;
pub mod analyzer;
pub mod interpreter;
pub mod errors;
mod rust_interop;

// Tests
//pub trait ElmInterpreter {
//    fn eval_expr(&mut self, expr: &str) -> Result<Value, ErrorWrapper>;
//    fn eval_statement(&mut self, stm: &str) -> Result<(), ErrorWrapper>;
//    fn eval_module(&mut self, file: &str) -> Result<(), ErrorWrapper>;
//
//    fn register_file_resolution_callback<F>(&mut self, callback: &F)
//        where F: Fn(&str) -> String;
//
//    fn reset(&mut self);
//}
//
//pub struct TreeWalkerInterpreter {}
//
//impl ElmInterpreter for TreeWalkerInterpreter {
//    fn eval_expr(&mut self, expr: &str) -> Result<Value, ErrorWrapper> {
//        unimplemented!()
//    }
//
//    fn eval_statement(&mut self, stm: &str) -> Result<(), ErrorWrapper> {
//        unimplemented!()
//    }
//
//    fn eval_module(&mut self, file: &str) -> Result<(), ErrorWrapper> {
//        unimplemented!()
//    }
//
//    fn register_file_resolution_callback<F>(&mut self, callback: &F) where F: Fn(&str) -> String {
//        unimplemented!()
//    }
//
//    fn reset(&mut self) {
//        unimplemented!()
//    }
//}
//
//
//pub fn tree_walker_interpreter() -> impl ElmInterpreter {
//    TreeWalkerInterpreter {}
//}
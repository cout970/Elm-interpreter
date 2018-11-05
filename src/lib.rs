// Development only {
// cargo watch -s 'clear && cargo test --color always 2>&1'
#![allow(dead_code)]
// }

#[macro_use]
extern crate nom;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use errors::ErrorWrapper;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::eval_expression;
use interpreter::eval_statement;
use types::Value;

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

pub struct Interpreter {
    env: DynamicEnv,
}

impl Interpreter {

    /// Creates a new Interpreter
    pub fn new() -> Interpreter {
        Interpreter {
            env: DynamicEnv::default_lang_env(),
        }
    }

    /// Evaluates an expression like `1 + 2`
    pub fn eval_expr(&mut self, expr: &str) -> Result<Value, ErrorWrapper> {
//        let tokens = tokenize(expr.as_bytes())
//            .map_err(|e| ErrorWrapper::Lexical(e))?;
//
//        let expr = parse_expr(TokenStream::new(&tokens))
//            .map_err(|e| ErrorWrapper::Syntactic(e))?;
//
//        eval_expr(&mut self.env, &expr)
//            .map_err(|e| ErrorWrapper::Runtime(e))?;

        eval_expression(&mut self.env, expr)
    }

    /// Evaluates an statement, for example:
    /// `x = 1`,
    /// `sum a b = a + b`,
    /// `type alias Boolean = Bool`,
    /// `type List a = Cons a (List a) | Nil`
    pub fn eval_statement(&mut self, stm: &str) -> Result<Option<Value>, ErrorWrapper> {
        eval_statement(&mut self.env, stm)
    }

    /// Evaluates a module, for example:
    /// ```
    /// module Util exposing(..)
    ///
    /// import Tuple
    ///
    /// toRecord : (a, b) -> { x: a, y: b }
    /// toRecord (a, b) = { x = a, y = b }
    /// ```
    pub fn eval_module(&mut self, _content: &str) -> Result<(), ErrorWrapper> {
        unimplemented!()
    }

    /// Evaluates a module and it's dependencies in a project
    /// folder is the path to the project containing all the source files
    /// main_file is the name of the first file to load without the .elm extension
    pub fn eval_files(&mut self, _folder: &str, _main_file: &str) -> Result<(), ErrorWrapper> {
        unimplemented!()
    }

    /// Clear the state of the interpreter, erasing all the types, modules and definitions
    fn reset(&mut self) {
        self.env = DynamicEnv::default_lang_env();
    }
}
// Development only {
// cargo watch -s 'clear && cargo test --color always 2>&1'
// cargo watch -c -q -s 'cargo rustc -lib -- -Awarnings -Zno-codegen && cargo test'
#![allow(dead_code)]
// }

extern crate nom;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use std::sync::Arc;

use ast::Type;
use errors::ErrorWrapper;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::eval_expression;
use interpreter::eval_statement;
use types::BuiltinFunctionRef;
use types::Function;
use types::Value;
use util::build_fun_type;
use util::create_vec_inv;

pub mod ast;
pub mod types;
#[macro_use]
mod util;
mod tokenizer;
mod parsers;
pub mod analyzer;
pub mod constructors;
mod core;
mod interpreter;
pub mod errors;
pub mod rust_interop;
pub mod loader;
pub mod source;

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
    /// ```elm
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
    pub fn eval_files(&mut self, folder: &str) -> Result<(), ErrorWrapper> {
        unimplemented!()
    }

    /// Registers a function that can be called in elm,
    /// the return value is not checked so make sure it matches the return type
    pub fn register_callback(&mut self, name: &str, args: &[Type], ret: Type, func_ref: BuiltinFunctionRef) -> Result<(), ErrorWrapper> {
        let arg_count = args.len() as u32;
        let function_type = build_fun_type(&create_vec_inv(args, ret));

        let function = Arc::new(Function::Builtin(
            self.env.next_fun_id(),
            func_ref,
            function_type.clone(),
        ));

        let function_value = Value::Fun {
            arg_count,
            args: vec![],
            fun: function,
        };

        self.env.add(name, function_value, function_type);
        Ok(())
    }

    /// Clear the state of the interpreter, erasing all the types, modules and definitions
    fn reset(&mut self) {
        self.env = DynamicEnv::default_lang_env();
    }
}

#[cfg(test)]
mod test {
    use ast::Int;

    use super::*;

    #[test]
    fn test_eval_expr() {
        let mut i = Interpreter::new();
        i.eval_expr("1 + 2 / 3").expect("Expect expression to execute correctly");
    }

    #[test]
    fn test_eval_stm() {
        let mut i = Interpreter::new();
        i.eval_statement("x = 2").expect("Expect x to be defined as 2");
        i.eval_expr("1 + x / 3").expect("Expect expression to execute correctly");
    }

    #[test]
    #[ignore]
    fn test_eval_module() {
        let mut i = Interpreter::new();
        let module = r#"
        sum x y = x + y
        div x y = x / y
        result = sum 1 (div 2 3)
        "#;

        i.eval_module(module).expect("Expect x to be defined as 2");
        i.eval_expr("result").expect("Expect expression to execute correctly");
    }

    #[test]
    #[ignore]
    fn test_register_fn() {
        use rust_interop::function_register::RegisterFn;

        let mut i = Interpreter::new();

        fn sum(x: Int, y: Int) -> Int { x + y }

        i.register_fn("sum", sum).expect("Expect sum to be defined");

        i.eval_expr("sum 1 3").expect("Expect expression to execute correctly");
    }
}
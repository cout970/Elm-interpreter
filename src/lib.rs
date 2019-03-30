// Development only {
// cargo watch -s 'clear && cargo test --color always 2>&1'
// cargo watch -c -q -s 'cargo rustc --lib -- -Awarnings -Zno-codegen && cargo test'
#![allow(dead_code)]
// }

extern crate nom;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use std::collections::HashMap;
use std::sync::Arc;

use analyzer::Analyser;
use analyzer::module_analyser::analyze_statement;
use ast::Type;
use errors::ElmError;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::eval_expression;
use interpreter::eval_statement;
use parsers::Parser;
use source::SourceCode;
use tokenizer::Tokenizer;
use types::ExternalFunc;
use types::Function;
use types::next_fun_id;
use types::Value;
use util::build_fun_type;
use util::create_vec_inv;

pub mod ast;
pub mod typed_ast;
pub mod types;
#[macro_use]
pub mod util;
pub mod tokenizer;
pub mod parsers;
pub mod analyzer;
pub mod constructors;
pub mod core;
pub mod interpreter;
pub mod errors;
pub mod rust_interop;
pub mod loader;
pub mod source;
#[cfg(test)]
pub mod test_utils;

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

    fn wrap(env: &DynamicEnv) -> Interpreter {
        Interpreter {
            env: env.clone(),
        }
    }

    /// Evaluates an expression like `1 + 2`
    pub fn eval_expr(&mut self, expr: &str) -> Result<Value, ElmError> {
        let code = SourceCode::from_str(expr);
        let tokenizer = Tokenizer::new(&code);
        let mut parser = Parser::new(tokenizer);
        eval_expression(&mut self.env, &parser.parse_expression()?)
    }

    /// Evaluates an statement, for example:
    /// `x = 1`,
    /// `sum a b = a + b`,
    /// `type alias Boolean = Bool`,
    /// `type List a = Cons a (List a) | Nil`
    pub fn eval_statement(&mut self, stm: &str) -> Result<Option<Value>, ElmError> {
        let code = SourceCode::from_str(stm);
        let tokenizer = Tokenizer::new(&code);
        let mut parser = Parser::new(tokenizer);
        let stm = parser.parse_statement()?;
        let mut analyser = Analyser::new();
        analyze_statement(&mut self.env.types, &stm);
        eval_statement(&mut self.env, &stm)
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
    pub fn eval_module(&mut self, _module: &str) -> Result<(), ElmError> {
//        let code = SourceCode::from_str(_module);
//        let tokenizer = Tokenizer::new(&code);
//        let mut parser = Parser::new(tokenizer);
//        eval_module(&mut self.env, &mut loader, parser.parse_module()?)
        unimplemented!()
    }

    /// Evaluates a module and it's dependencies in a project
    /// folder is the path to the project containing all the source files
    /// main_file is the name of the first file to load without the .elm extension
    pub fn eval_files(&mut self, _folder: &str) -> Result<(), ElmError> {
        unimplemented!()
    }

    /// Registers a function that can be called in elm,
    /// the return value is not checked so make sure it matches the return type
    pub fn register_callback(&mut self, name: &str, args: &[Type], ret: Type, func_ref: ExternalFunc) -> Result<(), ElmError> {
        let arg_count = args.len() as u32;
        let function_type = build_fun_type(&create_vec_inv(args, ret));

        let function = Arc::new(Function::External(
            next_fun_id(),
            func_ref,
            function_type.clone(),
        ));

        let function_value = Value::Fun {
            arg_count,
            args: vec![],
            captures: HashMap::new(),
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

    #[test]
    fn test_closure() {
        let mut i = Interpreter::new();
        i.eval_statement("genClosure x = \\y -> x + y").expect("1\n");
        i.eval_statement("addFive = genClosure 5").expect("2\n");
        i.eval_statement("result = addFive 3").expect("3\n");
        let result = i.eval_expr("result").expect("Expect expression to execute correctly");
        assert_eq!(Value::Number(8), result);
    }
}
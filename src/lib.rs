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

use analyzer::Analyzer;
use ast::Type;
use builtin::get_core_kernel_modules;
use errors::ElmError;
use errors::LoaderError;
use errors::Wrappable;
use interpreter::Interpreter;
use loader::AnalyzedModule;
use loader::LoadedModule;
use loader::ModuleLoader;
use loader::RuntimeModule;
use loader::SourceFile;
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
pub mod builtin;
pub mod interpreter;
pub mod errors;
pub mod rust_interop;
pub mod loader;
pub mod source;
#[cfg(test)]
pub mod test_utils;

#[derive(Debug)]
pub struct Runtime {
    interpreter: Interpreter,
    analyzer: Analyzer,
    loaded_modules: HashMap<String, LoadedModule>,
    analyzed_modules: HashMap<String, AnalyzedModule>,
    runtime_modules: HashMap<String, RuntimeModule>,
}

impl Runtime {
    /// Creates a new Interpreter
    pub fn new() -> Runtime {
        let mut run = Runtime {
            interpreter: Interpreter::new(),
            analyzer: Analyzer::new(SourceCode::from_str("")),
            loaded_modules: HashMap::new(),
            analyzed_modules: HashMap::new(),
            runtime_modules: HashMap::new(),
        };

        for (name, analyzed, runtime) in get_core_kernel_modules() {
            run.analyzed_modules.insert(name.to_string(), analyzed);
            run.runtime_modules.insert(name.to_string(), runtime);
        }

        run.include_file("/Data/Dev/Elm/core-master/src/Basics.elm").unwrap();
        run.import_module_as("Basics", "").unwrap();
        run
    }

    /// Evaluates an expression like `1 + 2`
    pub fn eval_expr(&mut self, expr: &str) -> Result<Value, ElmError> {
        let code = SourceCode::from_str(expr);
        let tokenizer = Tokenizer::new(&code);
        let mut parser = Parser::new(tokenizer);
        let expr = parser.parse_expression()?;
        let typed_expr = self.analyzer.with(code).analyze_expression(&expr)?;
        let value = self.interpreter.eval_expr(&typed_expr)?;
        Ok(value)
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
        let mut analyser = Analyzer::new(code.clone());
        let declarations = analyser.analyze_statement(&stm).expect("Analysis error");
        unimplemented!()
//        eval_statement(&mut self.env, &stm)
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
    pub fn eval_module(&mut self, module: &str, name: &str) -> Result<(), ElmError> {
        let code = SourceCode::from_str(module);
        let tokenizer = Tokenizer::new(&code);
        let mut parser = Parser::new(tokenizer);
        let module = parser.parse_module()?;

        ModuleLoader::include_module(
            self,
            SourceFile {
                name: name.to_string(),
                path: name.to_string(),
                source: code,
            },
            module,
        )?;

        self.import_module(name)
    }

    pub fn include_files(&mut self, folder_path: &str) -> Result<(), ElmError> {
        ModuleLoader::include_folder(self, folder_path)
    }

    pub fn include_file(&mut self, file_path: &str) -> Result<(), ElmError> {
        ModuleLoader::include_file(self, "", file_path)
    }

    pub fn import_module(&mut self, module_name: &str) -> Result<(), ElmError> {
        self.import_module_as(module_name, module_name)
    }

    pub fn import_module_as(&mut self, module_name: &str, alias: &str) -> Result<(), ElmError> {
        if !self.runtime_modules.contains_key(module_name) {
            self.load_analyzed_module(module_name)?;
        }

        self.load_runtime_module(module_name)?;
        self.import_module_definitions(module_name, alias)?;
        Ok(())
    }

    fn import_module_definitions(&mut self, name: &str, alias: &str) -> Result<(), ElmError> {
        let module = self.runtime_modules.get(name).expect("Expected module to be already loaded");

        for (def_name, val) in &module.definitions {
            eprintln!("Running: {} = {} : {}", def_name, val, val.get_type());
            let name = if alias.is_empty() {
                def_name.clone()
            } else {
                format!("{}.{}", alias, def_name)
            };

            self.analyzer.add_definition(&name, val.get_type());
            self.interpreter.stack.add(&name, val.clone());
        }

        Ok(())
    }

    fn load_analyzed_module(&mut self, module_name: &str) -> Result<(), ElmError> {
        let dependencies = self.loaded_modules.get(module_name)
            .ok_or_else(|| LoaderError::MissingModule { module: module_name.to_string() }.wrap())?
            .dependencies.clone();

        // Load dependencies
        for dep in &dependencies {
            if !self.analyzed_modules.contains_key(dep) {
                self.load_analyzed_module(dep)?;
            }
        }

        eprintln!("Analyzing {}", module_name);
        let module = self.loaded_modules.get(module_name)
            .ok_or_else(|| LoaderError::MissingModule { module: module_name.to_string() }.wrap())?;


        // Analyze module
        let mut analyzer = Analyzer::new(module.src.source.clone());
        let analyzed_module = analyzer.analyze_module(&self.analyzed_modules, module)?;

        self.analyzed_modules.insert(module_name.to_string(), analyzed_module);
        Ok(())
    }

    fn load_runtime_module(&mut self, module_name: &str) -> Result<(), ElmError> {
        let dependencies = self.analyzed_modules.get(module_name)
            .ok_or_else(|| LoaderError::MissingModule { module: module_name.to_string() }.wrap())?
            .dependencies.clone();

        // Load dependencies
        for dep in &dependencies {
            if !self.runtime_modules.contains_key(dep) {
                self.load_runtime_module(dep)?;
            }
        }

        eprintln!("Evaluating {}", module_name);
        let mut interpreter = Interpreter::new();
        let runtime_module = {
            let module = self.analyzed_modules.get(module_name)
                .ok_or_else(|| LoaderError::MissingModule { module: module_name.to_string() }.wrap())?;

            interpreter.eval_module(&self.runtime_modules, module)?
        };
        let runtime_module = interpreter.eval_constants(self, runtime_module)?;

        self.runtime_modules.insert(module_name.to_string(), runtime_module);
        Ok(())
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
            fun: function,
        };

        // TODO
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use ast::Int;
    use util::test_resource;

    use super::*;

    #[test]
    fn run_hello_world_project() {
        let mut runtime = Runtime::new();
        runtime.include_files(&test_resource("sample_project")).unwrap();
        runtime.import_module("Main").unwrap();

        let value = runtime.eval_expr("Main.sayHello")
            .expect("Expected correct execution, but failed");

        assert_eq!(Value::String("hello world".to_string()), value);
    }

    #[test]
    fn test_eval_expr() {
        let mut i = Runtime::new();
        i.eval_expr("1 + 2 / 3").expect("Expect expression to execute correctly");
    }

    #[test]
    fn test_eval_stm() {
        let mut i = Runtime::new();
        i.eval_statement("x = 2").expect("Expect x to be defined as 2");
        i.eval_expr("1 + x / 3").expect("Expect expression to execute correctly");
    }

    #[test]
    #[ignore]
    fn test_eval_module() {
        let mut i = Runtime::new();
        let module = r#"
        sum x y = x + y
        div x y = x / y
        result = sum 1 (div 2 3)
        "#;

        i.eval_module(module, "Main").expect("Expect x to be defined as 2");
        i.eval_expr("result").expect("Expect expression to execute correctly");
    }

    #[test]
    #[ignore]
    fn test_register_fn() {
        use rust_interop::function_register::RegisterFn;

        let mut i = Runtime::new();

        fn sum(x: Int, y: Int) -> Int { x + y }

        i.register_fn("sum", sum).expect("Expect sum to be defined");

        i.eval_expr("sum 1 3").expect("Expect expression to execute correctly");
    }

    #[test]
    fn test_closure() {
        let mut i = Runtime::new();
        i.eval_statement("genClosure x = \\y -> x + y").expect("1\n");
        i.eval_statement("addFive = genClosure 5").expect("2\n");
        i.eval_statement("result = addFive 3").expect("3\n");
        let result = i.eval_expr("result").expect("Expect expression to execute correctly");
        assert_eq!(Value::Number(8), result);
    }
}
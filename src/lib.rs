// Development only {
// cargo watch -s 'clear && cargo test --color always 2>&1'
// cargo watch -c -q -s 'cargo rustc --lib -- -Awarnings -Zno-codegen && cargo test'
#![allow(dead_code)]
// }
// TODO port to Rust 2018

extern crate hashbrown;
extern crate nom;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::sync::Arc;

use analyzer::Analyzer;
use ast::Type;
use builtin::ELM_CORE_MODULES;
use builtin::get_core_kernel_modules;
use errors::ElmError;
use errors::LoaderError;
use errors::Wrappable;
use interpreter::Interpreter;
use loader::AnalyzedModule;
use loader::declaration_name;
use loader::declaration_type;
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
use util::{build_fun_type, create_vec_inv, resource_path};

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
    /// Creates a new Runtime instance
    pub fn new() -> Runtime {
        let mut run = Self::empty_runtime();

        // Add kernel modules, in the standard compiler those modules are written in JS,
        // but here they are in Rust
        for (name, analyzed, runtime) in get_core_kernel_modules() {
            run.analyzed_modules.insert(name.to_string(), analyzed);
            run.runtime_modules.insert(name.to_string(), runtime);
        }

        // Load Elm-core modules, except Array, Process, Task and Platform as they doesn't make sense
        // outside JS and a browser sandbox

        // Load from packed modules
        for name in ELM_CORE_MODULES.iter() {
            let path = format!("{}/{}.json", resource_path("packed_modules/core"), name);
            run.include_packed_module(&path).unwrap();
        }

        /* DEBUG ONLY
        // Load from elm-core source files.
        // (To get nice error messages when debugging, download the git repo and update the path)
        for name in ELM_CORE_MODULES.iter() {
            let path = format!("/Data/Dev/Elm/core/src/{}.elm", name);
            run.include_file(&path).unwrap();
        }
        */

        // Analyze and evaluate all core modules
        for name in ELM_CORE_MODULES.iter() {
            run.load_analyzed_module(name).expect("Unable to load analyzed module");
            run.load_runtime_module(name).expect("Unable to load runtime module");
        }

        // Import the default modules into the current environment
        let imports = run.analyzer.get_default_imports(&run.analyzed_modules)
            .expect("Default imports cannot be included");

        for import in &imports {
            let module = run.runtime_modules.get(&import.source)
                .expect("Module not found");

            let value = module.definitions.get(&import.source_name)
                .expect("Definition not found");

            run.interpreter.stack.add(&import.destine_name, value.clone());
        };

        run
    }

    pub fn empty_runtime() -> Runtime {
        Runtime {
            interpreter: Interpreter::new(),
            analyzer: Analyzer::new(SourceCode::from_str("")),
            loaded_modules: HashMap::new(),
            analyzed_modules: HashMap::new(),
            runtime_modules: HashMap::new(),
        }
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

    /// Evaluates a statement, for example:
    /// `x = 1`,
    /// `sum a b = a + b`,
    /// `type alias Boolean = Bool`,
    /// `type List a = Cons a (List a) | Nil`
    pub fn eval_statement(&mut self, stm: &str) -> Result<Option<Value>, ElmError> {
        let code = SourceCode::from_str(stm);
        let tokenizer = Tokenizer::new(&code);
        let mut parser = Parser::new(tokenizer);
        let stm = parser.parse_statement()?;
        let declarations = self.analyzer.with(code.clone()).analyze_statement(&stm)?;

        let mut opt_value = None;

        for decl in &declarations {
            opt_value = self.interpreter.eval_declaration(decl)?;

            if let Some(ty) = declaration_type(decl) {
                self.analyzer.add_port(declaration_name(decl), ty.clone());
            }
        }

        Ok(opt_value)
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

    /// Loads all source files in the folder and checks for missing dependencies
    /// Note: it doesn't import the modules, you have to import them with [import_module]
    pub fn include_files(&mut self, folder_path: &str) -> Result<(), ElmError> {
        ModuleLoader::include_folder(self, folder_path)
    }

    /// Loads a source file and checks its dependencies
    /// Note: it doesn't import the module, you have to import it with [import_module]
    pub fn include_file(&mut self, file_path: &str) -> Result<(), ElmError> {
        ModuleLoader::include_file(self, "", file_path)
    }

    /// Loads a module packed as a binary blob and checks its dependencies
    /// Note: it doesn't import the module, you have to import it with [import_module]
    pub fn include_packed_module(&mut self, file_path: &str) -> Result<(), ElmError> {
        ModuleLoader::include_packed_module(self, "", file_path)
    }

    /// Import a module, previously loaded with include_file/include_files, into the
    /// current environment
    pub fn import_module(&mut self, module_name: &str) -> Result<(), ElmError> {
        self.import_module_as(module_name, module_name)
    }

    /// Import a module, previously loaded with include_file/include_files, into the
    /// current environment with an alias
    pub fn import_module_as(&mut self, module_name: &str, alias: &str) -> Result<(), ElmError> {
        if !self.runtime_modules.contains_key(module_name) {
            self.load_analyzed_module(module_name)?;
        }

        self.load_runtime_module(module_name)?;
        self.import_module_definitions(module_name, alias)?;
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

        self.analyzer.add_port(name, function_type);
        self.interpreter.stack.add(name, function_value);
        Ok(())
    }

    /// Print all the values in the stack
    pub fn debug(&self) -> String {
        self.interpreter.debug()
    }

    fn import_module_definitions(&mut self, name: &str, alias: &str) -> Result<(), ElmError> {
        let module = self.runtime_modules.get(name).expect("Expected module to be already loaded");

        for (def_name, val) in &module.definitions {
            let name = if alias.is_empty() {
                def_name.clone()
            } else {
                format!("{}.{}", alias, def_name)
            };

            self.analyzer.add_port(&name, val.get_type());
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

//        eprintln!("Analyzing {}", module_name);
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

//        eprintln!("Evaluating {}", module_name);
        let mut interpreter = Interpreter::new();
        let runtime_module = {
            let module = self.analyzed_modules.get(module_name)
                .ok_or_else(|| LoaderError::MissingModule { module: module_name.to_string() }.wrap())?;

            interpreter.eval_module(&self.runtime_modules, module)?
        };
        let runtime_module = interpreter.eval_constants(runtime_module)?;

        self.runtime_modules.insert(module_name.to_string(), runtime_module);
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
        eprintln!("genClosure : {}", i.eval_expr("genClosure").unwrap().get_type());
        i.eval_statement("addFive = genClosure 5").expect("2\n");
        eprintln!("addFive : {}", i.eval_expr("addFive").unwrap().get_type());
        i.eval_statement("result = addFive 3").expect("3\n");
        let result = i.eval_expr("result").expect("Expect expression to execute correctly");
        assert_eq!(Value::Number(8), result);
    }
}
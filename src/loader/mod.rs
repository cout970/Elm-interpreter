use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::sync::Arc;

use analyzer::module_analyser::analyze_module_declarations;
use analyzer::module_analyser::analyze_module_imports;
use analyzer::static_env::StaticEnv;
use ast::Module;
use ast::Type;
use core::register_core;
use errors::ElmError;
use errors::LoaderError;
use parsers::parse_mod;
use source::SourceCode;
use tokenizer::tokenize;
use types::Adt;
use util::sort::sort_dependencies;

#[derive(Clone, Debug)]
pub struct ModuleLoader {
    loaded_modules: HashMap<String, LoadedModule>
}

#[derive(Clone, Debug)]
pub struct SourceFile {
    name: String,
    path: String,
    source: SourceCode,
}

#[derive(Clone, Debug)]
pub struct LoadedModule {
    pub src: SourceFile,
    pub ast: Module,
    pub env: StaticEnv,
    pub declarations: Vec<Declaration>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Def(String, Type),
    Alias(String, Type),
    Adt(String, Arc<Adt>),
}

impl ModuleLoader {
    pub fn new() -> Self {
        ModuleLoader { loaded_modules: HashMap::new() }
    }

    pub fn get_module(&self, name: &str) -> Option<&LoadedModule> {
        self.loaded_modules.get(name)
    }

    pub fn include_folder(&mut self, path: &str) -> Result<(), ElmError> {
        let mut sources = vec![];
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut data: HashMap<String, (SourceFile, Module)> = HashMap::new();

        get_all_source_files(&mut sources, "", path).map_err(io_error)?;

        for src in sources {
            let ast = load_source_file(&src)?;
            graph.insert(src.name.to_string(), get_module_dependencies(&ast));
            data.insert(src.name.to_string(), (src, ast));
        }

        let sorted = sort_dependencies(graph)
            .map_err(|e| ElmError::Loader { info: LoaderError::CyclicDependency { cycle: e } })?;

        for name in sorted {
            let (source, ast) = data.remove(&name).unwrap();
            self.include_module(source, ast)?;
        }
        Ok(())
    }

    pub fn include_file(&mut self, inner_path: &str, path: &str) -> Result<(), ElmError> {
        let source = get_source_file(inner_path, path).map_err(io_error)?;
        let ast = load_source_file(&source)?;
        self.include_module(source, ast)
    }

    fn include_module(&mut self, src: SourceFile, ast: Module) -> Result<(), ElmError> {
        let name = src.name.clone();
        let deps = get_module_dependencies(&ast);

        let missing_deps = deps.iter()
            .filter(|dep| !self.loaded_modules.contains_key(*dep))
            .cloned()
            .collect::<Vec<String>>();

        if !missing_deps.is_empty() {
            return Err(ElmError::Loader { info: LoaderError::MissingDependencies { dependencies: missing_deps } });
        }

        let mut env = StaticEnv::new();
        register_core(&mut env);

        analyze_module_imports(&self.loaded_modules, &mut env, &ast.imports)?;
        let declarations = analyze_module_declarations(&mut env, &ast.statements)
            .map_err(|e| ElmError::Analyser { code: src.source.clone(), info: e })?;

        let module = LoadedModule {
            src,
            ast,
            env,
            declarations,
            dependencies: deps,
        };

        self.loaded_modules.insert(name, module);
        Ok(())
    }
}

pub fn declaration_name(decl: &Declaration) -> &str {
    match decl {
        Declaration::Def(name, _) => name,
        Declaration::Alias(name, _) => name,
        Declaration::Adt(name, _) => name,
    }
}

// Private helpers

fn get_module_dependencies(module: &Module) -> Vec<String> {
    module.imports.iter().map(|import| import.path.join(".")).collect::<Vec<String>>()
}

fn load_source_file(file: &SourceFile) -> Result<Module, ElmError> {
    let tokens = tokenize(file.source.as_bytes())
        .map_err(|e| ElmError::Tokenizer { code: file.source.clone(), info: e })?;

    let ast = parse_mod(&file.source, tokens)
        .map_err(|e| ElmError::Parser { code: file.source.clone(), info: e })?;

    Ok(ast)
}

fn io_error(err: Error) -> ElmError {
    ElmError::Loader { info: LoaderError::IO { error: Arc::new(err) } }
}

fn get_all_source_files(dst: &mut Vec<SourceFile>, inner_path: &str, path: &str) -> Result<(), Error> {
    let directory = fs::read_dir(path)?;

    for entry_result in directory {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name().to_str().unwrap().to_string();
        let file_path = format!("{}/{}", path, file_name);

        if file_type.is_file() && file_name.ends_with(".elm") {
            dst.push(get_source_file(inner_path, &file_path)?);
        } else if file_type.is_dir() {
            let inner: String = if inner_path.is_empty() {
                file_name
            } else {
                format!("{}.{}", inner_path, file_name)
            };

            get_all_source_files(dst, &inner, &file_path)?
        }
    }
    Ok(())
}

fn get_source_file(inner_path: &str, abs_path: &str) -> Result<SourceFile, Error> {
    let path = Path::new(abs_path);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    let module_name = if inner_path.is_empty() {
        file_name.to_string()
    } else {
        format!("{}.{}", inner_path, file_name)
    };

    let file_contents = fs::read(abs_path)?;

    let loaded_module = SourceFile {
        name: module_name.trim_end_matches(".elm").to_string(),
        path: abs_path.to_string(),
        source: SourceCode::from_bytes(file_contents),
    };

    Ok(loaded_module)
}

#[cfg(test)]
mod test {
    use interpreter::dynamic_env::DynamicEnv;
    use interpreter::eval_expression;
    use interpreter::eval_module;
    use util::test_resource;

    use super::*;

    #[test]
    fn test_include_folder() {
        let mut loader = ModuleLoader::new();
        loader.include_folder(&test_resource("sample_project")).unwrap();

        let mut env = DynamicEnv::default_lang_env();
        eval_module(&mut env, &loader, "SubModule1").unwrap();
        eval_module(&mut env, &loader, "Mod.SubModule2").unwrap();
        eval_module(&mut env, &loader, "Main").unwrap();

        let result = eval_expression(&mut env, "sayHello").unwrap();
        println!("{}", result);
    }
}
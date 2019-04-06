use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::sync::Arc;

use analyzer::Analyzer;
use analyzer::static_env::StaticEnv;
use ast::Module;
use ast::Type;
use core::register_core;
use errors::ElmError;
use errors::err_list;
use errors::LoaderError;
use interpreter::dynamic_env::DynamicEnv;
use parsers::Parser;
use Runtime;
use source::SourceCode;
use tokenizer::Tokenizer;
use typed_ast::TypedDefinition;
use types::Adt;
use types::Value;
use util::sort::sort_dependencies;

#[derive(Clone, Debug)]
pub struct ModuleLoader {}

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub name: String,
    pub path: String,
    pub source: SourceCode,
}

#[derive(Clone, Debug)]
pub struct LoadedModule {
    pub src: SourceFile,
    pub ast: Module,
    pub dependencies: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct AnalyzedModule {
    pub name: String,
    pub dependencies: Vec<String>,
    pub all_declarations: Vec<Declaration>,

    pub definitions: Vec<TypedDefinition>,
    pub imports: Vec<ModuleImport>,
}

#[derive(Clone, Debug)]
pub struct RuntimeModule {
    pub name: String,
    pub definitions: HashMap<String, Value>,
    pub imports: Vec<ModuleImport>,
}

#[derive(Clone, Debug)]
pub struct ModuleImport {
    pub source: String,
    pub source_name: String,
    pub destine_name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Port(String, Type),
    Definition(String, TypedDefinition),
    Alias(String, Type),
    Adt(String, Arc<Adt>),
}

impl ModuleLoader {
    pub fn include_folder(run: &mut Runtime, path: &str) -> Result<(), ElmError> {
        let mut sources = vec![];
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut data: HashMap<String, (SourceFile, Module)> = HashMap::new();

        get_all_source_files(&mut sources, "", path).map_err(io_error)?;

        for src in sources {
            let ast = load_source_file(&src)?;
            graph.insert(src.name.to_string(), get_module_dependencies(&ast));
            data.insert(src.name.to_string(), (src, ast));
        }

        let keys = graph.keys().cloned().collect::<HashSet<_>>();

        for (_, value) in graph.iter_mut() {
            *value = value.iter().filter(|dep| keys.contains(*dep)).cloned().collect();
        }

        let sorted = sort_dependencies(graph)
            .map_err(|e| ElmError::Loader { info: LoaderError::CyclicDependency { cycle: e } })?;

        for name in sorted {
            let (source, ast) = data.remove(&name).unwrap();
            Self::include_module(run, source, ast)?;
        }
        Ok(())
    }

    pub fn include_file(run: &mut Runtime, inner_path: &str, path: &str) -> Result<(), ElmError> {
        let source = get_source_file(inner_path, path).map_err(io_error)?;
        let ast = load_source_file(&source)?;
        Self::include_module(run, source, ast)
    }

    fn include_module(run: &mut Runtime, src: SourceFile, ast: Module) -> Result<(), ElmError> {
        let name = src.name.clone();
        let deps = get_module_dependencies(&ast);

        let missing_deps = deps.iter()
            .filter(|dep| !run.loaded_modules.contains_key(*dep) && !run.analyzed_modules.contains_key(*dep))
            .cloned()
            .collect::<Vec<String>>();

        if !missing_deps.is_empty() {
            return Err(ElmError::Loader { info: LoaderError::MissingDependencies { dependencies: missing_deps } });
        }

        let module = LoadedModule { src, ast, dependencies: deps };

        run.loaded_modules.insert(name, module);
        Ok(())
    }
}

pub fn declaration_name(decl: &Declaration) -> &str {
    match decl {
        Declaration::Port(name, _) => name,
        Declaration::Definition(name, _) => name,
        Declaration::Alias(name, _) => name,
        Declaration::Adt(name, _) => name,
    }
}

// Private helpers

fn get_module_dependencies(module: &Module) -> Vec<String> {
    module.imports.iter().map(|import| import.path.join(".")).collect::<Vec<String>>()
}

fn load_source_file(file: &SourceFile) -> Result<Module, ElmError> {
    Parser::new(Tokenizer::new(&file.source)).parse_module()
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
    use ast::Expr;
    use interpreter::dynamic_env::DynamicEnv;
    use util::test_resource;

    use super::*;

    #[test]
    fn test_include_folder() {
//        let mut loader = ModuleLoader::new();
//        loader.include_folder(&test_resource("sample_project")).unwrap();

//        let mut env = DynamicEnv::default_lang_env();
//        eval_module(&mut env, &loader, "SubModule1").unwrap();
//        eval_module(&mut env, &loader, "Mod.SubModule2").unwrap();
//        eval_module(&mut env, &loader, "Main").unwrap();

//        let result = eval_expression(&mut env, &Expr::Ref((0, 0), "sayHello".to_string())).unwrap();
//        println!("{}", result);
    }
}
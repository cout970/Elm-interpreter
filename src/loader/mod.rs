use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use ast::{Module, TypeAlias};
use ast::Type;
use errors::ElmError;
use errors::LoaderError;
use errors::Wrappable;
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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PackedModule {
    pub name: String,
    pub ast: Module,
}

#[derive(Clone, Debug)]
pub struct AnalyzedModule {
    pub name: String,
    pub dependencies: Vec<String>,
    pub imports: Vec<ModuleImport>,
    pub all_declarations: Vec<Declaration>,
}

#[derive(Clone, Debug)]
pub struct RuntimeModule {
    pub name: String,
    pub definitions: HashMap<String, Value>,
    pub imports: Vec<ModuleImport>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModuleImport {
    pub source: String,
    pub source_name: String,
    pub destine_name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Port(String, Type),
    Definition(String, TypedDefinition),
    Alias(TypeAlias),
    Adt(String, Arc<Adt>),
    Infix(String, String, Type),
}

impl ModuleLoader {
    pub fn include_folder(run: &mut Runtime, path: &str) -> Result<(), ElmError> {
        let mut sources = vec![];
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut data: HashMap<String, (SourceFile, Module)> = HashMap::new();

        get_all_source_files(&mut sources, "", path)?;

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
            .map_err(|e| LoaderError::CyclicDependency { cycle: e }.wrap())?;

        for name in sorted {
            let (source, ast) = data.remove(&name).unwrap();
            Self::include_module(run, source, ast)?;
        }
        Ok(())
    }

    pub fn include_file(run: &mut Runtime, inner_path: &str, path: &str) -> Result<(), ElmError> {
        let source = get_source_file(inner_path, path)?;
        let ast = load_source_file(&source)?;
        Self::include_module(run, source, ast)
    }

    pub fn include_packed_module(run: &mut Runtime, inner_path: &str, path: &str) -> Result<(), ElmError> {
        let module = get_packed_module(inner_path, path)?;
        let source = SourceFile {
            name: module.name.to_string(),
            path: path.to_string(),
            source: SourceCode::from_str(""),
        };

        Self::include_module(run, source, module.ast)
    }

    pub fn include_module(run: &mut Runtime, src: SourceFile, ast: Module) -> Result<(), ElmError> {
        let name = src.name.clone();
        let deps = get_module_dependencies(&ast);

        let missing_deps = deps.iter()
            .filter(|dep| !run.loaded_modules.contains_key(*dep) && !run.analyzed_modules.contains_key(*dep))
            .cloned()
            .collect::<Vec<String>>();

        if !missing_deps.is_empty() {
            return Err(LoaderError::MissingDependencies { dependencies: missing_deps, src: src.clone() }.wrap());
        }

//        // Save modules to pck files
//        let path = format!("{}/{}.pck", resource_path("packed_modules/core"), name);
//        save_as_packed_module(&path, &name, &ast);

        let module = LoadedModule { src, ast, dependencies: deps };

        run.loaded_modules.insert(name, module);
        Ok(())
    }
}

pub fn declaration_name(decl: &Declaration) -> &str {
    match decl {
        Declaration::Port(name, ..) => name,
        Declaration::Definition(name, ..) => name,
        Declaration::Alias(alias, ..) => &alias.name,
        Declaration::Adt(name, ..) => name,
        Declaration::Infix(name, ..) => name,
    }
}

pub fn declaration_type(decl: &Declaration) -> Option<&Type> {
    match decl {
        Declaration::Port(_, ty) => Some(ty),
        Declaration::Definition(_, ty) => Some(&ty.header),
        Declaration::Alias(_) => None,
        Declaration::Adt(_, _) => None,
        Declaration::Infix(_, _, ty) => Some(ty),
    }
}

// Private helpers

fn get_module_dependencies(module: &Module) -> Vec<String> {
    module.imports.iter().map(|import| import.path.join(".")).collect::<Vec<String>>()
}

fn load_source_file(file: &SourceFile) -> Result<Module, ElmError> {
    Parser::new(Tokenizer::new(&file.source)).parse_module()
}

fn get_all_source_files(dst: &mut Vec<SourceFile>, inner_path: &str, path: &str) -> Result<(), ElmError> {
    let directory = fs::read_dir(path)
        .map_err(|err| LoaderError::IO { error: Arc::new(err), msg: format!("read folder '{}'", path) }.wrap())?;

    for entry_result in directory {
        let entry = entry_result
            .map_err(|err| LoaderError::IO { error: Arc::new(err), msg: format!("read folder entry") }.wrap())?;

        let file_type = entry.file_type()
            .map_err(|err| LoaderError::IO { error: Arc::new(err), msg: format!("read file type") }.wrap())?;

        let file_name = entry.file_name().to_str().unwrap().to_string();
        let file_path = format!("{}/{}", path, file_name);

        if file_type.is_file() && file_name.ends_with(".elm") {
            let file = get_source_file(inner_path, &file_path)?;
            dst.push(file);
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

fn get_source_file(inner_path: &str, abs_path: &str) -> Result<SourceFile, ElmError> {
    let path = Path::new(abs_path);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    let module_name = if inner_path.is_empty() {
        file_name.to_string()
    } else {
        format!("{}.{}", inner_path, file_name)
    };

    let file_contents = fs::read(abs_path)
        .map_err(|err| LoaderError::IO { error: Arc::new(err), msg: format!("read file '{}'", abs_path) }.wrap())?;

    let loaded_module = SourceFile {
        name: module_name.trim_end_matches(".elm").to_string(),
        path: abs_path.to_string(),
        source: SourceCode::from_bytes(file_contents, abs_path),
    };

    Ok(loaded_module)
}

fn get_packed_module(inner_path: &str, abs_path: &str) -> Result<PackedModule, ElmError> {
    let path = Path::new(abs_path);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    let _module_name = if inner_path.is_empty() {
        file_name.to_string()
    } else {
        format!("{}.{}", inner_path, file_name)
    };

    let file_contents = fs::read(abs_path)
        .map_err(|err| LoaderError::IO { error: Arc::new(err), msg: format!("read file '{}'", abs_path) }.wrap())?;

    let module_ast: Module = serde_json::from_slice(&file_contents)
        .map_err(|err| LoaderError::ModulePacking {
            msg: format!("{}", err),
            path: abs_path.to_string(),
        }.wrap())?;

    let packed_module = PackedModule {
        name: module_ast.header.as_ref().unwrap().name.to_string(),
        ast: module_ast,
    };

    Ok(packed_module)
}

pub fn save_as_packed_module(abs_path: &str, _name: &str, ast: &Module) -> Result<(), ElmError> {
    let file_contents = serde_json::to_string_pretty(ast)
        .expect("Module to packed failed");

    fs::write(abs_path, file_contents)
        .expect("File write failed");

    Ok(())
}
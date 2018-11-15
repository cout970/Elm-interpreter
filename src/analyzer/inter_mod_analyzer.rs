use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::sync::Arc;

use analyzer::module_analyser::analyze_module;
use analyzer::module_analyser::CheckedModule;
use ast::*;
use core::get_core_module_by_path;
use errors::ErrorWrapper;
use interpreter::RuntimeError;
use parsers::parse_module;
use types::Adt;

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Def(String, Type),
    Alias(String, Type),
    Adt(String, Arc<Adt>),
}

pub type Declarations = Vec<Declaration>;

pub type ModulePath = Vec<String>;

pub type InterModuleInfo = HashMap<ModulePath, CheckedModule>;

pub fn analyze_all_modules(modules: Vec<(ModulePath, Module)>) -> Result<InterModuleInfo, ErrorWrapper> {
    let mut loaded: HashMap<ModulePath, CheckedModule> = HashMap::new();

    for (path, module) in modules {
        let view = analyze_module(&loaded, &path, module)
            .map_err(|e| ErrorWrapper::Type(e))?;
        loaded.insert(path, view);
    }

    Ok(loaded)
}

pub fn load_all_modules<F>(path: &ModulePath, getter: F) -> Result<Vec<(ModulePath, Module)>, ErrorWrapper>
    where F: Fn(&ModulePath) -> Result<String, ErrorWrapper> {
    let mut visited: HashSet<ModulePath> = HashSet::new();
    let mut inv_load_order: Vec<(ModulePath, Module)> = vec![];
    let mut to_visit: Vec<ModulePath> = vec![path.clone()];

    while let Some(path) = to_visit.pop() {
        let module = match get_core_module_by_path(&path) {
            Some(module) => module,
            None => {
                let module_code = getter(&path)?;
                parse_module(&module_code)?
            }
        };

        let deps = get_module_dependencies(&module);

        for dep in deps {
            if !visited.contains(&dep) && !to_visit.contains(&dep) {
                to_visit.push(dep);
            }
        }

        inv_load_order.push((path.clone(), module));
        visited.insert(path);
    }

    Ok(inv_load_order.into_iter().rev().collect())
}


fn get_module_dependencies(module: &Module) -> Vec<ModulePath> {
    let mut dependencies = vec![];

    for import in &module.imports {
        dependencies.push(import.path.clone());
    }

    dependencies
}

pub fn create_path_string(base: &str, path: &ModulePath) -> String {
    let mut file_path = String::new();
    file_path.push_str(base);
    for p in path {
        file_path.push('/');
        file_path.push_str(p);
    }

    file_path.push_str(".elm");
    file_path
}


pub fn find_module_func(base_paths: &'static [&str]) -> impl Fn(&ModulePath) -> Result<String, ErrorWrapper> {
    move |path| {
        let mut file = None;

        println!("Loading module: {:?} ", path);

        // check all base paths for the file
        for base_path in base_paths {
            let path_str = create_path_string(*base_path, path);

            match File::open(&path_str) {
                Ok(f) => {
                    file = Some(f);
                    break;
                }
                Err(_) => (),
            }
        }

        match file {
            Some(file) => {
                let mut code = String::new();
                BufReader::new(file).read_to_string(&mut code).unwrap();

                Ok(code)
            }
            None => Err(ErrorWrapper::Runtime(RuntimeError::MissingSourceFile))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_project() {
        let mods = load_all_modules(
            &vec!["type_check".to_owned()],
            find_module_func(&[
                "benches/data",
                "/Data/Dev/Elm/core-master/src"
            ]),
        ).unwrap();

        let checked = analyze_all_modules(mods).unwrap();
        for (path, module) in checked {
            println!("{:?}: {:?}", path, module);
        }
    }
}
use analyzer::module_analyser::analyze_module;
use analyzer::module_analyser::CheckedModule;
use analyzer::TypeError;
use ast::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use types::Adt;
use errors::ErrorWrapper;
use parsers::parse_module;

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
        let module_code = getter(&path)?;
        let module = parse_module(&module_code)?;
        let deps = get_module_dependencies(&module);

        for dep in deps {
            if !visited.contains(&dep) && !to_visit.contains(&dep) {
                to_visit.push(dep);
            }
        }

        visited.insert(path.clone());
        inv_load_order.push((path, module));
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

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;
    use super::*;
    use parsers::from_code_mod;
    use interpreter::RuntimeError;


    #[test]
    fn test_project() {
        let mods = load_all_modules(&vec!["Element".to_owned()], |path| {
            println!("Reading file at: '{:?}'", path);

            let file = File::open(&create_path_string("/Data/Dev/Elm/AI/src", path))
                .or_else(|_| File::open(&create_path_string("/Data/Dev/Elm/core-master/src", path)));

            match file {
                Ok(file) => {
                    let mut buf_reader = BufReader::new(file);
                    let mut buff = String::new();
                    buf_reader.read_to_string(&mut buff).unwrap();

                    Ok(buff)
                }
                Err(_) => {
                    Err(ErrorWrapper::Runtime(RuntimeError::MissingSourceFile))
                }
            }

        }).unwrap();

        for (path, module) in mods {
            println!("{:?}: {}", path, module.statements.len());
        }
    }
}
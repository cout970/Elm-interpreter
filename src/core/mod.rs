use analyzer::inter_mod_analyzer::ModulePath;
use analyzer::static_env::StaticEnv;
use ast::Module;
use ast::ModuleExposing;
use ast::ModuleHeader;
use ast::Statement;
use ast::Type;
use core::basics::get_basics_types;
use core::debug::get_debug_types;
use core::char::get_char_types;
use core::string::get_string_types;
use core::list::get_list_types;
use core::bitwise::get_bitwise_types;
use core::utils::get_utils_types;

mod basics;
mod debug;
mod char;
mod string;
mod list;
mod bitwise;
mod utils;

pub fn register_core(env: &mut StaticEnv) {
    for (name, ty) in get_basics_types() {
        env.add_definition(name, ty);
    }
    for (name, ty) in get_utils_types() {
        env.add_definition(name, ty);
    }
}

pub fn get_core_module_by_path(path: &ModulePath) -> Option<Module> {
    let slices: Vec<_> = path.iter().map(|x| x.as_str()).collect();

    match slices[..] {
        ["Elm", "Kernel", "Basics"] => {
            Some(create_module("Elm.Kernel.Basics", get_basics_types()))
        }
        ["Elm", "Kernel", "Platform"] => {
            Some(create_module("Elm.Kernel.Platform", vec![]))
        }
        ["Elm", "Kernel", "Scheduler"] => {
            Some(create_module("Elm.Kernel.Scheduler", vec![]))
        }
        ["Elm", "Kernel", "Debug"] => {
            Some(create_module("Elm.Kernel.Debug", get_debug_types()))
        }
        ["Elm", "Kernel", "Char"] => {
            Some(create_module("Elm.Kernel.Char", get_char_types()))
        }
        ["Elm", "Kernel", "String"] => {
            Some(create_module("Elm.Kernel.String", get_string_types()))
        }
        ["Elm", "Kernel", "List"] => {
            Some(create_module("Elm.Kernel.List", get_list_types()))
        }
        ["Elm", "Kernel", "Bitwise"] => {
            Some(create_module("Elm.Kernel.Bitwise", get_bitwise_types()))
        }
        ["Elm", "Kernel", "Utils"] => {
            Some(create_module("Elm.Kernel.Utils", get_utils_types()))
        }
        _ => None
    }
}

fn create_module(name: &str, types: Vec<(&str, Type)>) -> Module {
    let header = ModuleHeader {
        name: String::from(name),
        exposing: ModuleExposing::All,
    };

    let mut statements = vec![];

    for (def, ty) in types {
        statements.push(Statement::Port(String::from(def), ty));
    }

    Module { header: Some(header), imports: vec![], statements }
}
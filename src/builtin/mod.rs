use std::collections::HashMap;
use std::sync::Arc;

use ast::Type;
use builtin::basics::get_basics_funs;
use builtin::bitwise::get_bitwise_funs;
use builtin::char::get_char_funs;
use builtin::debug::get_debug_funs;
use builtin::list::get_list_funs;
use builtin::string::get_string_funs;
use builtin::utils::get_utils_funs;
use constructors::type_fun;
use constructors::type_of;
use constructors::type_tag_args;
use errors::ElmError;
use errors::InterpreterError;
use errors::Wrappable;
use interpreter::Interpreter;
use loader::AnalyzedModule;
use loader::Declaration;
use loader::RuntimeModule;
use types::Adt;
use types::AdtVariant;
use types::ElmFn;
use types::ExternalFunc;
use types::Function;
use types::next_fun_id;
use types::Value;
use util::arg_count;

mod basics;
mod debug;
mod char;
mod string;
mod list;
mod bitwise;
mod utils;

pub const ELM_CORE_MODULES: [&str; 11] = [
    "Basics", "Bitwise", "Char", "Maybe", "Result", "List", "String", "Debug", "Dict", "Set", "Tuple"
];

/// Returns a list of the Elm Core kernel modules, adding the basic building blocks of the language
/// The elm core needs to be loaded to expose and expand the definitions to all the other elm source files
pub fn get_core_kernel_modules() -> Vec<(&'static str, AnalyzedModule, RuntimeModule)> {
    vec![
        core_kernel_module("Elm.Kernel.Basics", get_basics_funs),
        core_kernel_module("Elm.Kernel.Utils", get_utils_funs),
        core_kernel_module("Elm.Kernel.Bitwise", get_bitwise_funs),
        core_kernel_module("Elm.Kernel.Debug", get_debug_funs),
        core_kernel_module("Elm.Kernel.Char", get_char_funs),
        core_kernel_module("Elm.Kernel.List", get_list_funs),
        core_kernel_module("Elm.Kernel.String", get_string_funs),
    ]
}

fn core_kernel_module(name: &'static str, func: fn() -> Vec<(&'static str, Type, Value)>) -> (&'static str, AnalyzedModule, RuntimeModule) {
    let mut all_declarations = vec![];
    let mut definitions = HashMap::new();

    for (name, ty, val) in func() {
        all_declarations.push(Declaration::Port(name.to_string(), ty));
        definitions.insert(name.to_string(), val);
    }

    (
        name,
        AnalyzedModule {
            name: name.to_string(),
            dependencies: vec![],
            all_declarations,
            imports: vec![],
        },
        RuntimeModule {
            name: name.to_string(),
            definitions,
            imports: vec![],
        }
    )
}

pub fn record_access(ty: &Type, field: &str) -> Value {
    Value::Fun {
        arg_count: 1,
        args: vec![Value::String(field.to_owned())],
        fun: Arc::new(Function::External(next_fun_id(), builtin_record_access(), ty.clone())),
    }
}

// { a = 0 } .a
pub fn builtin_record_access() -> ExternalFunc {
    let fun: ElmFn = |_, args| {
        match &args[0] {
            Value::Record(entries) => {
                if let Value::String(field) = &args[1] {
                    let opt = entries.iter()
                        .find(|(name, _)| name == field)
                        .map(|(_, val)| val);

                    match opt {
                        Some(val) => Ok(val.clone()),
                        None => {
                            Err(InterpreterError::RecordFieldNotFound(field.clone(), args[0].clone()).wrap())
                        }
                    }
                } else {
                    Err(InterpreterError::InternalErrorRecordAccess(args[1].clone()).wrap())
                }
            }
            _ => Err(InterpreterError::ExpectedRecord(args[0].clone()).wrap())
        }
    };

    ExternalFunc { name: "record access".to_string(), fun }
}

pub fn adt_constructor(adt: Arc<Adt>, variant: &AdtVariant) -> Value {
    let mut func_types = vec![type_tag_args(&variant.name, vec![])];
    func_types.extend(variant.types.clone().into_iter());
    func_types.push(type_tag_args(&variant.name, variant.types.clone()));

    Value::Fun {
        arg_count: func_types.len() as u32,
        args: vec![Value::Adt(variant.name.to_string(), vec![], adt)],
        fun: Arc::new(Function::External(next_fun_id(), builtin_adt_constructor(), type_fun(func_types))),
    }
}

pub fn builtin_adt_constructor() -> ExternalFunc {
    ExternalFunc {
        name: "ADT constructor".to_string(),
        fun: |_, args| {
            if let Value::Adt(var, _, adt) = &args[0] {
                let mut vals: Vec<Value> = vec![];

                for i in 1..args.len() {
                    vals.push(args[i].clone());
                }

                Ok(Value::Adt(var.to_owned(), vals, adt.clone()))
            } else {
                Err(InterpreterError::InternalErrorAdtCreation(args[0].clone()).wrap())
            }
        },
    }
}


/// Create a function value from a function name, the function type in string format and a rust function reference
fn func_of(name: &'static str, ty: &'static str, fun: ElmFn) -> (&'static str, Type, Value) {
    let func_type = type_of(ty);
    let external = ExternalFunc { name: name.to_string(), fun };
    let func = Value::Fun {
        arg_count: arg_count(&func_type),
        args: vec![],
        fun: Arc::new(Function::External(next_fun_id(), external, func_type.clone())),
    };

    (name, func_type, func)
}

fn ignore(_: &mut Interpreter, _args: &[Value]) -> Result<Value, ElmError> {
    unreachable!()
}

// Combinators

// Identity
// a -> a
pub fn builtin_id(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    Ok(args[0].clone())
}

// self application
// f = f f
// Has recursive type
pub fn builtin_mockingbird(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    i.apply_function(args[0].clone(), &[args[0].clone()])
}

// True, first, const
// a -> b -> a
pub fn builtin_kestrel(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    Ok(args[0].clone())
}

// False, second
// a -> b -> b
pub fn builtin_kite(_: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    Ok(args[1].clone())
}

// Invert order, flip
// (a -> b -> c) -> b -> a -> c
pub fn builtin_cardinal(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let abc = &args[0];
    let b = &args[1];
    let a = &args[2];

    i.apply_function(abc.clone(), &[a.clone(), b.clone()])
}

// Composition
// (b -> c) -> (a -> b) -> a -> c
pub fn builtin_bluebird(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let bc = &args[0];
    let ab = &args[1];
    let a = &args[2];
    let b = i.apply_function(ab.clone(), &[a.clone()])?;

    i.apply_function(bc.clone(), &[b])
}

// hold an argument
// a -> (a -> b) -> b
pub fn builtin_thrush(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = &args[0];
    let ab = &args[1];

    i.apply_function(ab.clone(), &[a.clone()])
}

// Hold 2 arguments
// a -> b -> (a -> b -> c) -> c
pub fn builtin_vireo(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let a = &args[0];
    let b = &args[1];
    let abc = &args[2];

    i.apply_function(abc.clone(), &[a.clone(), b.clone()])
}

// Double composition
// (c -> d) -> (a -> b -> c) -> a -> b -> d
pub fn builtin_blackbird(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let cd = &args[0];
    let abc = &args[1];
    let a = &args[2];
    let b = &args[2];
    let c = i.apply_function(abc.clone(), &[a.clone(), b.clone()])?;

    i.apply_function(cd.clone(), &[c])
}

// (a -> b -> c) -> (a -> b) -> a -> c
pub fn builtin_starling(i: &mut Interpreter, args: &[Value]) -> Result<Value, ElmError> {
    let abc = &args[0];
    let ab = &args[1];
    let a = &args[2];
    let b = i.apply_function(ab.clone(), &[a.clone()])?;

    i.apply_function(abc.clone(), &[a.clone(), b])
}

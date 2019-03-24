use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem::transmute;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use ast::Expr;
use ast::Float;
use ast::Int;
use ast::Pattern;
use ast::Type;
use errors::*;
use errors::ElmError;
use Interpreter;
use rust_interop::FnAny;

// Represents the final value after the evaluation of an expression tree
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    /// The unit value, similar to void in other languages
    Unit,
    /// Value that can be automatically converted to Float or Int based on the context
    Number(Int),
    /// A integer value
    Int(Int),
    /// A float value
    Float(Float),
    /// UTF-8 string
    String(String),
    /// Unicode character
    Char(char),
    /// Collection of values of the same type
    List(Vec<Value>),
    /// Collection of values of different types
    Tuple(Vec<Value>),
    /// A map between keys and values, where keys are identifiers
    Record(Vec<(String, Value)>),
    /// A custom type a.k.a enum a.k.a algebraic data type
    Adt(String, Vec<Value>, Arc<Adt>),
    /// A function value, contains values from partial application
    Fun {
        arg_count: u32,
        args: Vec<Value>,
        captures: HashMap<String, Value>,
        fun: Arc<Function>,
    },
}

/// Represents a function call,
/// it has a Function and it's Argument
/// it's used as key in caches for function memoization
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FunCall {
    pub function: Value,
    pub argument: Value,
}

/// Unique id for fast comparison between functions
pub type FunId = usize;

/// Global atomic incremented next free function id
static FUN_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Retrieves and increments the next free function id
pub fn next_fun_id() -> FunId {
    FUN_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

pub struct ExternalFunc {
    pub name: String,
    pub fun: fn(&mut Interpreter, &Vec<Value>) -> Result<Value, RuntimeError>,
}

pub struct WrapperFunc {
    pub name: String,
    pub fun: Box<FnAny>,
}

/// Represents a function that can be a definition or builtin
#[derive(Debug)]
pub enum Function {
    External(FunId, ExternalFunc, Type),
    Wrapper(FunId, WrapperFunc, Type),
    Expr(FunId, Vec<Pattern>, Expr, Type),
}

/// Represents an Adt type with all the information about the variants
#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Adt {
    pub name: String,
    pub types: Vec<String>,
    pub variants: Vec<AdtVariant>,
}

/// Is a variant in an Adt
#[derive(Debug, PartialEq, Clone, Hash)]
pub struct AdtVariant {
    pub name: String,
    pub types: Vec<Type>,
}

// Fun are compared using only the FunId
impl Eq for Function {}

impl PartialEq for Function {
    fn eq(&self, other: &Function) -> bool {
        let self_id = match self {
            Function::External(id, _, _) => { *id }
            Function::Wrapper(id, _, _) => { *id }
            Function::Expr(id, _, _, _) => { *id }
        };

        let other_id = match other {
            Function::External(id, _, _) => { *id }
            Function::Wrapper(id, _, _) => { *id }
            Function::Expr(id, _, _, _) => { *id }
        };

        self_id == other_id
    }
}

// Values are used in FunCall, so they must be valid map keys
impl Eq for Value {}

// Values are used in FunCall, so they must be valid map keys
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Unit => { state.write_i32(0) }
            Value::Number(i) => { state.write_i32(*i) }
            Value::Int(i) => { state.write_i32(*i) }
            Value::Float(i) => { state.write_i32(unsafe { transmute::<f32, i32>(*i) }) }
            Value::String(i) => { i.hash(state) }
            Value::Char(i) => { state.write_u32(*i as u32) }
            Value::List(i) => { i.hash(state) }
            Value::Tuple(i) => { i.hash(state) }
            Value::Record(i) => { i.hash(state) }
            Value::Adt(a, b, c) => {
                a.hash(state);
                b.hash(state);
                c.hash(state);
            }
            Value::Fun { arg_count, args, captures, fun } => {
                state.write_u32(*arg_count);
                args.hash(state);

                match fun.deref() {
                    Function::External(id, _, _) => { state.write_usize(*id) }
                    Function::Wrapper(id, _, _) => { state.write_usize(*id) }
                    Function::Expr(id, _, _, _) => { state.write_usize(*id) }
                }
            }
        }
    }
}
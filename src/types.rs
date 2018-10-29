use std::hash::Hash;
use std::hash::Hasher;
use std::mem::transmute;
use std::ops::Deref;
use std::sync::Arc;
use ast::Type;
use ast::Pattern;
use ast::Expr;
use ast::Int;
use ast::Float;

// Represents the final value after the evaluation of an expression tree
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Unit,
    Number(Int),
    Int(Int),
    Float(Float),
    String(String),
    Char(char),
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Record(Vec<(String, Value)>),
    Adt(String, Vec<Value>, Arc<Adt>),
    Fun {
        arg_count: u32,
        args: Vec<Value>,
        fun: Arc<Fun>,
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
pub type FunId = u32;

/// Represents a function that can be a definition or builtin
#[derive(Debug, Clone)]
pub enum Fun {
    Builtin(FunId, u32, Type),
    Expr(FunId, Vec<Pattern>, Expr, Type),
}

/// Represents an Adt type with all the information about the variants
#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Adt {
    pub name: String,
    pub types: Vec<Type>,
    pub variants: Vec<AdtVariant>,
}

/// Is a variant in an Adt
#[derive(Debug, PartialEq, Clone, Hash)]
pub struct AdtVariant {
    pub name: String,
    pub types: Vec<Type>,
}

// Fun are compared using only the FunId
impl Eq for Fun {}

impl PartialEq for Fun {
    fn eq(&self, other: &Fun) -> bool {
        let self_id = match self {
            Fun::Builtin(id, _, _) => { *id }
            Fun::Expr(id, _, _, _) => { *id }
        };

        let other_id = match other {
            Fun::Builtin(id, _, _) => { *id }
            Fun::Expr(id, _, _, _) => { *id }
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
            Value::Fun { arg_count, args, fun } => {
                state.write_u32(*arg_count);
                args.hash(state);

                match fun.deref() {
                    Fun::Builtin(id, _, _) => { state.write_u32(*id) }
                    Fun::Expr(id, _, _, _) => { state.write_u32(*id) }
                }
            }
        }
    }
}
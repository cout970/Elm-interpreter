use std::hash::Hash;
use std::hash::Hasher;
use std::mem::transmute;
use std::sync::Arc;
use std::rc::Rc;
use std::ops::Deref;

pub type Int = i32;
pub type Float = f32;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(Int),
    Float(Float),
    String(String),
    Char(char),
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum Type {
    Var(String),
    Tag(String, Vec<Type>),
    Fun(Box<Type>, Box<Type>),
    Unit,
    Tuple(Vec<Type>),
    Record(Vec<(String, Type)>),
    RecExt(String, Vec<(String, Type)>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Var(String),
    Adt(String, Vec<Pattern>),
    Wildcard,
    Unit,
    Tuple(Vec<Pattern>),
    List(Vec<Pattern>),
    BinaryOp(String, Box<Pattern>, Box<Pattern>),
    Record(Vec<String>),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Alias(String, Vec<String>, Type),
    Adt(String, Vec<String>, Vec<(String, Vec<Type>)>),
    Port(String, Type),
    Def(Definition),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Unit,
    Tuple(Vec<Expr>),
    List(Vec<Expr>),
    Record(Vec<(String, Expr)>),
    Adt(String),
    RecordUpdate(String, Vec<(String, Expr)>),
    QualifiedRef(Vec<String>, String),
    RecordField(Box<Expr>, String),
    RecordAccess(String),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Case(Box<Expr>, Vec<(Pattern, Expr)>),
    Lambda(Vec<Pattern>, Box<Expr>),
    Application(Box<Expr>, Box<Expr>),
    Let(Vec<Definition>, Box<Expr>),
    OpChain(Vec<Expr>, Vec<String>),
    Literal(Literal),
    Ref(String),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FunCall {
    pub function: Value,
    pub argument: Value,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Unit,
    Number(i32),
    Int(i32),
    Float(f32),
    String(String),
    Char(char),
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Record(Vec<(String, Value)>),
    Adt(String, Vec<Value>, Rc<Adt>),
    Fun {
        arg_count: u32,
        args: Vec<Value>,
        fun: Rc<Fun>,
    },
}

pub type FunId = u32;

#[derive(Debug, Clone)]
pub enum Fun {
    Builtin(FunId, u32, Type),
    Expr(FunId, Vec<Pattern>, Expr, Type),
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Adt {
    pub name: String,
    pub types: Vec<Type>,
    pub variants: Vec<AdtVariant>,
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct AdtVariant {
    pub name: String,
    pub types: Vec<Type>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ValueDefinition {
    pub name: String,
    pub patterns: Vec<Pattern>,
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Definition(
    pub Option<Type>,
    pub ValueDefinition,
);

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Module {
    pub header: Option<ModuleHeader>,
    pub imports: Vec<Import>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModuleHeader {
    pub name: Vec<String>,
    pub exports: Vec<Export>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Export {
    Adt(String, Vec<String>),
    AdtNone(String),
    AdtAll(String),
    AdtRef(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub path: Vec<String>,
    pub alias: Option<String>,
    pub exposing: Vec<Export>,
}

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

impl Eq for Value {}

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


impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Literal::Int(i) => { state.write_i32(*i) }
            Literal::Float(i) => { state.write_i32(unsafe { transmute::<f32, i32>(*i) }) }
            Literal::String(i) => { i.hash(state) }
            Literal::Char(i) => { state.write_u32(*i as u32) }
        }
    }
}

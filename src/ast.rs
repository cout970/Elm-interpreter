use std::hash::Hash;
use std::hash::Hasher;

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use util::transmute_float_to_int;

// TODO add a crate feature to use 32 or 64 bits

/// Default type for integer values
pub type Int = i32;

/// Default type for floating point values
pub type Float = f32;

/// A module represents an AST tree of a source file
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Module {
    /* Optional module header `module Main exposing (..)` */
    pub header: Option<ModuleHeader>,
    /* Imports of this module `import Util` or `import Util exposing (func)`*/
    pub imports: Vec<Import>,
    /* All the statements of this module */
    pub statements: Vec<Statement>,
}

// Module Header
/// Module header with the module name and the list of exposed definitions/types
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ModuleHeader {
    pub name: String,
    pub exposing: ModuleExposing,
}

/// List exposed definitions/types
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ModuleExposing {
    Just(Vec<Exposing>),
    All,
}

/// Exposed definitions, types or Adt types
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Exposing {
    Adt(String, AdtExposing),
    Type(String),
    Definition(String),
    BinaryOperator(String),
}

/// Variants of an Adt Exposed
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AdtExposing {
    Variants(Vec<String>),
    All,
}

// Import

/// A module import
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Import {
    pub path: Vec<String>,
    pub alias: Option<String>,
    pub exposing: Option<ModuleExposing>,
}

// Statement

/// A statement in a module, can be
/// a type alias,
/// a algebraic data type,
/// a port,
/// or a definition
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Statement {
    Alias(String, Vec<String>, Type),
    Adt(String, Vec<String>, Vec<(Span, String, Vec<Type>)>),
    Port(Span, String, Type),
    Def(Definition),
    Infix(String, Int, String, String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TypeAlias {
    pub name: String,
    pub variables: Vec<String>,
    pub replacement: Type,
}

/// The representation of a type
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Hash)]
pub enum Type {
    Unit,
    Var(String),
    Tag(String, Vec<Type>),
    Fun(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Record(Vec<(String, Type)>),
    /// Record with a set of required fields, that can have more fields
    RecExt(String, Vec<(String, Type)>),
}

/// A function definition
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Definition {
    pub header: Option<Type>,
    pub name: String,
    pub patterns: Vec<Pattern>,
    pub expr: Expr,
}

// A pattern that represents 1 or more function arguments
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Pattern {
    Var(Span, String),
    Adt(Span, String, Vec<Pattern>),
    Wildcard(Span),
    Unit(Span),
    Tuple(Span, Vec<Pattern>),
    List(Span, Vec<Pattern>),
    BinaryOp(Span, String, Box<Pattern>, Box<Pattern>),
    Record(Span, Vec<String>),
    LitInt(Span, Int),
    LitString(Span, String),
    LitChar(Span, char),
    Alias(Span, Box<Pattern>, String),
}

pub type Span = (u32, u32);

// An unevaluated expression tree
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Expr {
    /* The unit value `()` */
    Unit(Span),
    /* A tuple of 2 or more elements */
    Tuple(Span, Vec<Expr>),
    /* A list of homogeneous values */
    List(Span, Vec<Expr>),
    /* A record, also know as map of key-value pairs, the keys must be valid identifiers */
    Record(Span, Vec<(String, Expr)>),
    /* An update operation over a record value, this operation changes the value of a record field */
    RecordUpdate(Span, String, Vec<(String, Expr)>),
    /* A reference to a definition specifying a Module path `List.map` */
    QualifiedRef(Span, Vec<String>, String),
    /* Access to a field in a record */
    RecordField(Span, Box<Expr>, String),
    /* Creation of a definition that can extract a value from a record field */
    RecordAccess(Span, String),
    /* If expression */
    If(Span, Box<Expr>, Box<Expr>, Box<Expr>),
    /* Case expression */
    Case(Span, Box<Expr>, Vec<(Pattern, Expr)>),
    /* Creation of an anonymous function */
    Lambda(Span, Vec<Pattern>, Box<Expr>),
    /* Function call, the first expression if the function and the second it's argument */
    Application(Span, Box<Expr>, Box<Expr>),
    /* A let definition, allows to create local functions to use in the final expression */
    Let(Span, Vec<LetDeclaration>, Box<Expr>),
    /* Stores a chain of binary operations,
     * the order and associativity of the operations can be changed later
     * to create a binary tree (with Expr::Application, see expression_fold.rs)
     */
    OpChain(Span, Vec<Expr>, Vec<String>),
    /* A value literal, `1`, '"Hello World"', '3.14', etc */
    Literal(Span, Literal),
    /* A reference to a definition `map`, `sum`, etc */
    Ref(Span, String),
}

/// A let declaration, yeah, really
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum LetDeclaration {
    Def(Definition),
    Pattern(Pattern, Expr),
}

/// A value literal
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Literal {
    Int(Int),
    Float(Float),
    String(String),
    Char(char),
}

impl Expr {
    pub fn get_span(&self) -> Span {
        *match self {
            Expr::Unit(span) => span,
            Expr::Tuple(span, _) => span,
            Expr::List(span, _) => span,
            Expr::Record(span, _) => span,
            Expr::RecordUpdate(span, _, _) => span,
            Expr::QualifiedRef(span, _, _) => span,
            Expr::RecordField(span, _, _) => span,
            Expr::RecordAccess(span, _) => span,
            Expr::If(span, _, _, _) => span,
            Expr::Case(span, _, _) => span,
            Expr::Lambda(span, _, _) => span,
            Expr::Application(span, _, _) => span,
            Expr::Let(span, _, _) => span,
            Expr::OpChain(span, _, _) => span,
            Expr::Literal(span, _) => span,
            Expr::Ref(span, _) => span,
        }
    }
}

impl Pattern {
    pub fn get_span(&self) -> Span {
        *match self {
            Pattern::Var(span, _) => span,
            Pattern::Adt(span, _, _) => span,
            Pattern::Wildcard(span) => span,
            Pattern::Unit(span) => span,
            Pattern::Tuple(span, _) => span,
            Pattern::List(span, _) => span,
            Pattern::BinaryOp(span, _, _, _) => span,
            Pattern::Record(span, _) => span,
            Pattern::LitInt(span, _) => span,
            Pattern::LitString(span, _) => span,
            Pattern::LitChar(span, _) => span,
            Pattern::Alias(span, _, _) => span,
        }
    }
}

pub fn span(a: &Expr) -> Span {
    *match a {
        Expr::Unit(span) => span,
        Expr::Tuple(span, _) => span,
        Expr::List(span, _) => span,
        Expr::Record(span, _) => span,
        Expr::RecordUpdate(span, _, _) => span,
        Expr::QualifiedRef(span, _, _) => span,
        Expr::RecordField(span, _, _) => span,
        Expr::RecordAccess(span, _) => span,
        Expr::If(span, _, _, _) => span,
        Expr::Case(span, _, _) => span,
        Expr::Lambda(span, _, _) => span,
        Expr::Application(span, _, _) => span,
        Expr::Let(span, _, _) => span,
        Expr::OpChain(span, _, _) => span,
        Expr::Literal(span, _) => span,
        Expr::Ref(span, _) => span,
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Expr) -> bool {
        match self {
            Expr::Unit(_) => {
                if let Expr::Unit(_) = other { true } else { false }
            }
            Expr::Tuple(_, a) => {
                if let Expr::Tuple(_, a2) = other { a == a2 } else { false }
            }
            Expr::List(_, a) => {
                if let Expr::List(_, a2) = other { a == a2 } else { false }
            }
            Expr::Record(_, a) => {
                if let Expr::Record(_, a2) = other { a == a2 } else { false }
            }
            Expr::RecordUpdate(_, a, b) => {
                if let Expr::RecordUpdate(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            Expr::QualifiedRef(_, a, b) => {
                if let Expr::QualifiedRef(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            Expr::RecordField(_, a, b) => {
                if let Expr::RecordField(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            Expr::RecordAccess(_, a) => {
                if let Expr::RecordAccess(_, a2) = other { a == a2 } else { false }
            }
            Expr::If(_, a, b, c) => {
                if let Expr::If(_, a2, b2, c2) = other { a == a2 && b == b2 && c == c2 } else { false }
            }
            Expr::Case(_, a, b) => {
                if let Expr::Case(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            Expr::Lambda(_, a, b) => {
                if let Expr::Lambda(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            Expr::Application(_, a, b) => {
                if let Expr::Application(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            Expr::Let(_, a, b) => {
                if let Expr::Let(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            Expr::OpChain(_, a, b) => {
                if let Expr::OpChain(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            Expr::Literal(_, a) => {
                if let Expr::Literal(_, a2) = other { a == a2 } else { false }
            }
            Expr::Ref(_, a) => {
                if let Expr::Ref(_, a2) = other { a == a2 } else { false }
            }
        }
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Pattern) -> bool {
        match self {
            Pattern::Var(_, a) => {
                if let Pattern::Var(_, a2) = other { a == a2 } else { false }
            }
            Pattern::Adt(_, a, b) => {
                if let Pattern::Adt(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            Pattern::Wildcard(_) => {
                if let Pattern::Wildcard(_) = other { true } else { false }
            }
            Pattern::Unit(_) => {
                if let Pattern::Unit(_) = other { true } else { false }
            }
            Pattern::Tuple(_, a) => {
                if let Pattern::Tuple(_, a2) = other { a == a2 } else { false }
            }
            Pattern::List(_, a) => {
                if let Pattern::List(_, a2) = other { a == a2 } else { false }
            }
            Pattern::BinaryOp(_, a, b, c) => {
                if let Pattern::BinaryOp(_, a2, b2, c2) = other { a == a2 && b == b2 && c == c2 } else { false }
            }
            Pattern::Record(_, a) => {
                if let Pattern::Record(_, a2) = other { a == a2 } else { false }
            }
            Pattern::LitInt(_, a) => {
                if let Pattern::LitInt(_, a2) = other { a == a2 } else { false }
            }
            Pattern::LitString(_, a) => {
                if let Pattern::LitString(_, a2) = other { a == a2 } else { false }
            }
            Pattern::LitChar(_, a) => {
                if let Pattern::LitChar(_, a2) = other { a == a2 } else { false }
            }
            Pattern::Alias(_, a, b) => {
                if let Pattern::Alias(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
        }
    }
}

//impl PartialEq for Type {
//    fn eq(&self, other: &Type) -> bool {
//        fuzzy_eq_type(&mut HashMap::new(), self, other)
//    }
//}

fn fuzzy_eq_type(vars: &mut HashMap<String, String>, this: &Type, other: &Type) -> bool {
    match this {
        Type::Unit => if let Type::Unit = other { true } else { false },
        Type::Var(a) => {
            if let Type::Var(a2) = other {
                // TODO
//                match vars.get(a).cloned() {
//                    None => {
//                        vars.insert(a.clone(), a2.clone());
//                        true
//                    },
//                    Some(alt_a2) => {
//                        a2 == &alt_a2
//                    },
//                }

                a == a2 || (a.starts_with("number") && a2.starts_with("number"))
            } else {
                false
            }
        }
        Type::Tag(a, b) => {
            if let Type::Tag(a2, b2) = other {
                a == a2 && b.iter().zip(b2).all(|(t0, t1)| fuzzy_eq_type(vars, t0, t1))
            } else { false }
        }
        Type::Fun(a, b) => {
            if let Type::Fun(a2, b2) = other {
                fuzzy_eq_type(vars, a.as_ref(), a2.as_ref()) && fuzzy_eq_type(vars, b.as_ref(), b2.as_ref())
            } else { false }
        }
        Type::Tuple(a) => {
            if let Type::Tuple(a2) = other {
                a.iter().zip(a2).all(|(t0, t1)| fuzzy_eq_type(vars, t0, t1))
            } else { false }
        }
        Type::Record(a) => {
            if let Type::Record(a2) = other {
                a.iter().zip(a2).all(|((n0, t0), (n1, t1))| {
                    n0 == n1 && fuzzy_eq_type(vars, t0, t1)
                })
            } else { false }
        }
        Type::RecExt(a, b) => {
            if let Type::RecExt(a2, b2) = other {
                a == a2 && b.iter().zip(b2).all(|((n0, t0), (n1, t1))| {
                    n0 == n1 && fuzzy_eq_type(vars, t0, t1)
                })
            } else { false }
        }
    }
}

/// Literals implement hash for function memoization,
/// a custom implementation is needed to handle floating values
impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Literal::Int(i) => { state.write_i32(*i) }
            Literal::Float(i) => {
                // Floats have edge cases for hash computation, I ignore those cases for simplicity
                state.write_i32(transmute_float_to_int(*i))
            }
            Literal::String(i) => { i.hash(state) }
            Literal::Char(i) => { state.write_u32(*i as u32) }
        }
    }
}

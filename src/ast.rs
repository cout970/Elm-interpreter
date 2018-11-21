use std::hash::Hash;
use std::hash::Hasher;
use std::mem::transmute;

// TODO add a crate feature to use 32 or 64 bits

/// Default type for integer values
pub type Int = i32;

/// Default type for floating point values
pub type Float = f32;

/// A module represents an AST tree of a source file
#[derive(Debug, PartialEq, Clone, Default)]
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
#[derive(Debug, PartialEq, Clone)]
pub struct ModuleHeader {
    pub name: String,
    pub exposing: ModuleExposing,
}

/// List exposed definitions/types
#[derive(Debug, PartialEq, Clone)]
pub enum ModuleExposing {
    Just(Vec<Exposing>),
    All,
}

/// Exposed definitions, types or Adt types
#[derive(Debug, PartialEq, Clone)]
pub enum Exposing {
    Adt(String, AdtExposing),
    Type(String),
    Definition(String),
    BinaryOperator(String),
}

/// Variants of an Adt Exposed
#[derive(Debug, PartialEq, Clone)]
pub enum AdtExposing {
    Variants(Vec<String>),
    All,
}

// Import

/// A module import
#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub path: Vec<String>,
    pub alias : Option<String>,
    pub exposing: Option<ModuleExposing>,
}

// Statement

/// A statement in a module, can be
/// a type alias,
/// a algebraic data type,
/// a port,
/// or a definition
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Alias(String, Vec<String>, Type),
    Adt(String, Vec<String>, Vec<(String, Vec<Type>)>),
    Port(String, Type),
    Def(Definition),
    Infix(String, Int, String, String),
}

/// The representation of a type
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Type {
    Unit,
    Var(String),
    Tag(String, Vec<Type>),
    Fun(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Record(Vec<(String, Type)>),
    RecExt(String, Vec<(String, Type)>),
}

/// A function definition
#[derive(Debug, PartialEq, Clone)]
pub struct Definition {
    pub header: Option<Type>,
    pub name: String,
    pub patterns: Vec<Pattern>,
    pub expr: Expr,
}

// A pattern that represents 1 or more function arguments
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
    LitInt(Int),
    LitString(String),
    LitChar(char),
    Alias(Box<Pattern>, String),
}

// An unevaluated expression tree
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /* The unit value `()` */
    Unit,
    /* A tuple of 2 or more elements */
    Tuple(Vec<Expr>),
    /* A list of homogeneous values */
    List(Vec<Expr>),
    /* A record, also know as map of key-value pairs, the keys must be valid identifiers */
    Record(Vec<(String, Expr)>),
    /* An update operation over a record value, this operation changes the value of a record field */
    RecordUpdate(String, Vec<(String, Expr)>),
    /* A reference to a definition specifying a Module path `List.map` */
    QualifiedRef(Vec<String>, String),
    /* Access to a field in a record */
    RecordField(Box<Expr>, String),
    /* Creation of a definition that can extract a value from a record field */
    RecordAccess(String),
    /* If expression */
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    /* Case expression */
    Case(Box<Expr>, Vec<(Pattern, Expr)>),
    /* Creation of an anonymous function */
    Lambda(Vec<Pattern>, Box<Expr>),
    /* Function call, the first expression if the function and the second it's argument */
    Application(Box<Expr>, Box<Expr>),
    /* A let definition, allows to create local functions to use in the final expression */
    Let(Vec<LetDeclaration>, Box<Expr>),
    /* Stores a chain of binary operations,
     * the order and associativity of the operations can be changed later
     * to create a binary tree (with Expr::Application, see expression_fold.rs)
     */
    OpChain(Vec<Expr>, Vec<String>),
    /* A value literal, `1`, '"Hello World"', '3.14', etc */
    Literal(Literal),
    /* A reference to a definition `map`, `sum`, etc */
    Ref(String),
}

/// A let declaration, yeah, really
#[derive(Debug, Clone, PartialEq)]
pub enum LetDeclaration {
    Def(Definition),
    Pattern(Pattern, Expr),
}

/// A value literal
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(Int),
    Float(Float),
    String(String),
    Char(char),
}

/// Literals implement hash for function memoization,
/// a custom implementation is needed to handle floating values
impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Literal::Int(i) => { state.write_i32(*i) }
            Literal::Float(i) => {
                // Floats have edge cases for hash computation, I ignore those cases for simplicity
                state.write_i32(unsafe { transmute::<f32, i32>(*i) })
            }
            Literal::String(i) => { i.hash(state) }
            Literal::Char(i) => { state.write_u32(*i as u32) }
        }
    }
}

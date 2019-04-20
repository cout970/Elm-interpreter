use serde::{Deserialize, Serialize};

use ast::Pattern;
use ast::Type;
use types::Value;

// An unevaluated expression tree
#[derive(Debug, Clone)]
pub enum TypedExpr {
    /* A value like unit, 1, 'a', "A", etc */
    Const(Type, Value),
    /* A tuple of 2 or more elements */
    Tuple(Type, Vec<TypedExpr>),
    /* A list of homogeneous values */
    List(Type, Vec<TypedExpr>),
    /* A record, also know as map of key-value pairs, the keys must be valid identifiers */
    Record(Type, Vec<(String, TypedExpr)>),
    /* An update operation over a record value, this operation changes the value of a record field */
    RecordUpdate(Type, Box<TypedExpr>, Vec<(String, TypedExpr)>),
    /* A reference to a definition specifying a Module path `List.map` */
    Ref(Type, String),
    /* Access to a field in a record */
    RecordField(Type, Box<TypedExpr>, String),
    /* Creation of a definition that can extract a value from a record field */
    RecordAccess(Type, String),
    /* If expression */
    If(Type, Box<TypedExpr>, Box<TypedExpr>, Box<TypedExpr>),
    /* Case expression */
    Case(Type, Box<TypedExpr>, Vec<(Pattern, TypedExpr)>),
    /* Creation of an anonymous function */
    Lambda(Type, Vec<Pattern>, Box<TypedExpr>),
    /* Function call, the first expression if the function and the second it's argument */
    Application(Type, Box<TypedExpr>, Box<TypedExpr>),
    /* A let definition, allows to create local functions to use in the final expression */
    Let(Type, Vec<LetEntry>, Box<TypedExpr>),
}

/// A function definition
#[derive(Debug, PartialEq, Clone)]
pub struct TypedDefinition {
    pub header: Type,
    pub name: String,
    pub patterns: Vec<Pattern>,
    pub expr: TypedExpr,
}

/// A let declaration declaration
#[derive(Debug, Clone, PartialEq)]
pub enum LetEntry {
    Definition(TypedDefinition),
    Pattern(Pattern, TypedExpr),
}

pub fn expr_type(expr: &TypedExpr) -> Type {
    match expr {
        TypedExpr::Const(ty, value) => ty.clone(),
        TypedExpr::Tuple(ty, _) => ty.clone(),
        TypedExpr::List(ty, _) => ty.clone(),
        TypedExpr::Record(ty, _) => ty.clone(),
        TypedExpr::RecordUpdate(ty, _, _) => ty.clone(),
        TypedExpr::Ref(ty, _) => ty.clone(),
        TypedExpr::RecordField(ty, _, _) => ty.clone(),
        TypedExpr::RecordAccess(ty, _) => ty.clone(),
        TypedExpr::If(ty, _, _, _) => ty.clone(),
        TypedExpr::Case(ty, _, _) => ty.clone(),
        TypedExpr::Lambda(ty, _, _) => ty.clone(),
        TypedExpr::Application(ty, _, _) => ty.clone(),
        TypedExpr::Let(ty, _, _) => ty.clone(),
    }
}

impl PartialEq for TypedExpr {
    fn eq(&self, other: &TypedExpr) -> bool {
        match self {
            TypedExpr::Const(_, a) => {
                if let TypedExpr::Const(_, a2) = other { a == a2 } else { false }
            }
            TypedExpr::Tuple(_, a) => {
                if let TypedExpr::Tuple(_, a2) = other { a == a2 } else { false }
            }
            TypedExpr::List(_, a) => {
                if let TypedExpr::List(_, a2) = other { a == a2 } else { false }
            }
            TypedExpr::Record(_, a) => {
                if let TypedExpr::Record(_, a2) = other { a == a2 } else { false }
            }
            TypedExpr::RecordUpdate(_, a, b) => {
                if let TypedExpr::RecordUpdate(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            TypedExpr::RecordField(_, a, b) => {
                if let TypedExpr::RecordField(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            TypedExpr::RecordAccess(_, a) => {
                if let TypedExpr::RecordAccess(_, a2) = other { a == a2 } else { false }
            }
            TypedExpr::If(_, a, b, c) => {
                if let TypedExpr::If(_, a2, b2, c2) = other { a == a2 && b == b2 && c == c2 } else { false }
            }
            TypedExpr::Case(_, a, b) => {
                if let TypedExpr::Case(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            TypedExpr::Lambda(_, a, b) => {
                if let TypedExpr::Lambda(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            TypedExpr::Application(_, a, b) => {
                if let TypedExpr::Application(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            TypedExpr::Let(_, a, b) => {
                if let TypedExpr::Let(_, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            TypedExpr::Ref(_, a) => {
                if let TypedExpr::Ref(_, a2) = other { a == a2 } else { false }
            }
        }
    }
}
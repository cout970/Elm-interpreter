use serde::{Deserialize, Serialize};

use ast::{Int, Span};
use ast::Type;
use constructors::{type_char, type_int, type_string};
use types::Value;

// An unevaluated expression tree
#[derive(Debug, Clone)]
pub enum TypedExpr {
    /* A value like unit, 1, 'a', "A", etc */
    Const(Span, Type, Value),
    /* A tuple of 2 or more elements */
    Tuple(Span, Type, Vec<TypedExpr>),
    /* A list of homogeneous values */
    List(Span, Type, Vec<TypedExpr>),
    /* A record, also know as map of key-value pairs, the keys must be valid identifiers */
    Record(Span, Type, Vec<(String, TypedExpr)>),
    /* An update operation over a record value, this operation changes the value of a record field */
    RecordUpdate(Span, Type, Box<TypedExpr>, Vec<(String, TypedExpr)>),
    /* A reference to a definition specifying a Module path `List.map` */
    Ref(Span, Type, String),
    /* Access to a field in a record */
    RecordField(Span, Type, Box<TypedExpr>, String),
    /* Creation of a definition that can extract a value from a record field */
    RecordAccess(Span, Type, String),
    /* If expression */
    If(Span, Type, Box<TypedExpr>, Box<TypedExpr>, Box<TypedExpr>),
    /* Case expression */
    Case(Span, Type, Box<TypedExpr>, Vec<(TypedPattern, TypedExpr)>),
    /* Creation of an anonymous function */
    Lambda(Span, Type, Vec<TypedPattern>, Box<TypedExpr>),
    /* Function call, the first expression if the function and the second it's argument */
    Application(Span, Type, Box<TypedExpr>, Box<TypedExpr>),
    /* A let definition, allows to create local functions to use in the final expression */
    Let(Span, Type, Vec<LetEntry>, Box<TypedExpr>),
}

/// A function definition
#[derive(Debug, PartialEq, Clone)]
pub struct TypedDefinition {
    pub header: Type,
    pub name: String,
    pub patterns: Vec<TypedPattern>,
    pub expr: TypedExpr,
}

/// A let declaration declaration
#[derive(Debug, Clone, PartialEq)]
pub enum LetEntry {
    Definition(TypedDefinition),
    Pattern(TypedPattern, TypedExpr),
}

// A pattern that represents 1 or more function arguments
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TypedPattern {
    Var(Span, Type, String),
    Adt(Span, Type, Type, Vec<TypedPattern>),
    Wildcard(Span),
    Unit(Span),
    Tuple(Span, Type, Vec<TypedPattern>),
    List(Span, Type, Vec<TypedPattern>),
    BinaryOp(Span, Type, String, Box<TypedPattern>, Box<TypedPattern>),
    Record(Span, Type, Vec<String>),
    LitInt(Span, Int),
    LitString(Span, String),
    LitChar(Span, char),
    Alias(Span, Type, Box<TypedPattern>, String),
}

impl TypedPattern {
    pub fn get_span(&self) -> Span {
        *match self {
            TypedPattern::Var(span, _, _) => span,
            TypedPattern::Adt(span, _, _, _) => span,
            TypedPattern::Wildcard(span) => span,
            TypedPattern::Unit(span) => span,
            TypedPattern::Tuple(span, _, _) => span,
            TypedPattern::List(span, _, _) => span,
            TypedPattern::BinaryOp(span, _, _, _, _) => span,
            TypedPattern::Record(span, _, _) => span,
            TypedPattern::LitInt(span, _) => span,
            TypedPattern::LitString(span, _) => span,
            TypedPattern::LitChar(span, _) => span,
            TypedPattern::Alias(span, _, _, _) => span,
        }
    }
    pub fn get_type(&self) -> Type {
        match self {
            TypedPattern::Var(_, ty, _) => ty.clone(),
            TypedPattern::Adt(_, ty, _, _) => ty.clone(),
            TypedPattern::Wildcard(_) => Type::Var("_".to_string()),
            TypedPattern::Unit(_) => Type::Unit,
            TypedPattern::Tuple(_, ty, _) => ty.clone(),
            TypedPattern::List(_, ty, _) => ty.clone(),
            TypedPattern::BinaryOp(_, ty, _, _, _) => ty.clone(),
            TypedPattern::Record(_, ty, _) => ty.clone(),
            TypedPattern::LitInt(_, _) => type_int(),
            TypedPattern::LitString(_, _) => type_string(),
            TypedPattern::LitChar(_, _) => type_char(),
            TypedPattern::Alias(_, ty, _, _) => ty.clone(),
        }
    }
}

impl TypedExpr {
    pub fn get_span(&self) -> Span {
        *match self {
            TypedExpr::Const(span, _, _) => span,
            TypedExpr::Tuple(span, _, _) => span,
            TypedExpr::List(span, _, _) => span,
            TypedExpr::Record(span, _, _) => span,
            TypedExpr::RecordUpdate(span, _, _, _) => span,
            TypedExpr::Ref(span, _, _) => span,
            TypedExpr::RecordField(span, _, _, _) => span,
            TypedExpr::RecordAccess(span, _, _) => span,
            TypedExpr::If(span, _, _, _, _) => span,
            TypedExpr::Case(span, _, _, _) => span,
            TypedExpr::Lambda(span, _, _, _) => span,
            TypedExpr::Application(span, _, _, _) => span,
            TypedExpr::Let(span, _, _, _) => span,
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            TypedExpr::Const(_, ty, _) => ty.clone(),
            TypedExpr::Tuple(_, ty, _) => ty.clone(),
            TypedExpr::List(_, ty, _) => ty.clone(),
            TypedExpr::Record(_, ty, _) => ty.clone(),
            TypedExpr::RecordUpdate(_, ty, _, _) => ty.clone(),
            TypedExpr::Ref(_, ty, _) => ty.clone(),
            TypedExpr::RecordField(_, ty, _, _) => ty.clone(),
            TypedExpr::RecordAccess(_, ty, _) => ty.clone(),
            TypedExpr::If(_, ty, _, _, _) => ty.clone(),
            TypedExpr::Case(_, ty, _, _) => ty.clone(),
            TypedExpr::Lambda(_, ty, _, _) => ty.clone(),
            TypedExpr::Application(_, ty, _, _) => ty.clone(),
            TypedExpr::Let(_, ty, _, _) => ty.clone(),
        }
    }
}

// TODO refactor
pub fn expr_type(expr: &TypedExpr) -> Type {
    expr.get_type()
}

impl PartialEq for TypedExpr {
    fn eq(&self, other: &TypedExpr) -> bool {
        match self {
            TypedExpr::Const(_, _, a) => {
                if let TypedExpr::Const(_, _, a2) = other { a == a2 } else { false }
            }
            TypedExpr::Tuple(_, _, a) => {
                if let TypedExpr::Tuple(_, _, a2) = other { a == a2 } else { false }
            }
            TypedExpr::List(_, _, a) => {
                if let TypedExpr::List(_, _, a2) = other { a == a2 } else { false }
            }
            TypedExpr::Record(_, _, a) => {
                if let TypedExpr::Record(_, _, a2) = other { a == a2 } else { false }
            }
            TypedExpr::RecordUpdate(_, _, a, b) => {
                if let TypedExpr::RecordUpdate(_, _, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            TypedExpr::RecordField(_, _, a, b) => {
                if let TypedExpr::RecordField(_, _, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            TypedExpr::RecordAccess(_, _, a) => {
                if let TypedExpr::RecordAccess(_, _, a2) = other { a == a2 } else { false }
            }
            TypedExpr::If(_, _, a, b, c) => {
                if let TypedExpr::If(_, _, a2, b2, c2) = other { a == a2 && b == b2 && c == c2 } else { false }
            }
            TypedExpr::Case(_, _, a, b) => {
                if let TypedExpr::Case(_, _, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            TypedExpr::Lambda(_, _, a, b) => {
                if let TypedExpr::Lambda(_, _, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            TypedExpr::Application(_, _, a, b) => {
                if let TypedExpr::Application(_, _, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            TypedExpr::Let(_, _, a, b) => {
                if let TypedExpr::Let(_, _, a2, b2) = other { a == a2 && b == b2 } else { false }
            }
            TypedExpr::Ref(_, _, a) => {
                if let TypedExpr::Ref(_, _, a2) = other { a == a2 } else { false }
            }
        }
    }
}
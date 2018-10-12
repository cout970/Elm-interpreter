use analyzer::environment::Environment;
use analyzer::expression_analyzer::analyze_expression;
use analyzer::function_analyzer::PatternMatchingError;
use std::collections::HashMap;
use types::CurriedFunc;
use types::Expr;
use types::Fun;
use types::Type;
use types::Value;
use util::StringConversion;

pub mod environment;
pub mod pattern_helper;
pub mod type_resolution;
mod function_analyzer;
mod expression_analyzer;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeError {
    MissingAdt(String),
    MissingDefinition(String),
    ListNotHomogeneous(String),
    IfWithNonBoolCondition(String),
    IfBranchesDoesntMatch(String),
    ArgumentsDoNotMatch(String),
    NotAFunction(String),
    InvalidOperandChain(String),
    RecordUpdateOnNonRecord(String),
    RecordUpdateUnknownField(String),
    CaseBranchDontMatchReturnType(String),
    DefinitionTypeAndReturnTypeMismatch,
    InvalidPattern(PatternMatchingError),
    ConstantEvaluationError(String),
    VariableAlreadyDeclared(String),
    UnableToCalculateFunctionType(String),
    VariableNameShadowed(String),
    InternalError,
}

pub fn type_check_expression(env: &mut Environment, expr: &Expr) -> Result<Type, TypeError> {
    let mut vars = HashMap::new();
    analyze_expression(env, &mut vars, expr, None)
}
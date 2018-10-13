use analyzer::expression_analyzer::analyze_expression;
use analyzer::function_analyzer::PatternMatchingError;
use std::collections::HashMap;
use types::Expr;
use types::Fun;
use types::Type;
use types::Value;
use util::StringConversion;
use types::Definition;
use analyzer::function_analyzer::analyze_function;
use analyzer::static_env::StaticEnv;

mod function_analyzer;
mod expression_analyzer;
pub mod static_env;

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

pub fn type_check_expression(env: &mut StaticEnv, expr: &Expr) -> Result<Type, TypeError> {
    analyze_expression(env, None, expr)
}

pub fn type_check_function(env: &mut StaticEnv, fun: &Definition) -> Result<Type, TypeError> {
    analyze_function(env, fun)
}

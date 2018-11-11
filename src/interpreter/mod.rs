use analyzer::TypeError;
use ast::Pattern;
use errors::ErrorWrapper;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::expression_eval::eval_expr;
use interpreter::statement_eval::eval_stm;
use parsers::parse_expression;
use parsers::parse_statement;
use types::Value;
use util::expression_fold::ExprTreeError;

pub mod dynamic_env;
mod builtins;
mod expression_eval;
mod statement_eval;
mod module_eval;

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeError {
    MissingDefinition(String, DynamicEnv),
    IncorrectDefType(TypeError),
    RecordUpdateOnNonRecord(String, Value),
    InvalidIfCondition(Value),
    InvalidExpressionChain(ExprTreeError),
    RecordFieldNotFound(String, Value),
    CaseExpressionNonExhaustive(Value, Vec<Pattern>),
    FunArgumentSizeMismatch(u32, u32),
    ExpectedRecord(Value),
    ExpectedFunction(Value),
    ExpectedAdt(Value),
    ExpectedTuple(Value),
    ExpectedList(Value),
    ExpectedFloat(Value),
    ExpectedInt(Value),
    ExpectedNumber(Value),
    ExpectedNonEmptyList(Value),
    UnknownOperatorPattern(String),
    InternalErrorRecordAccess(Value),
    InternalErrorAdtCreation(Value),
    UnknownBuiltinFunction(u32),
    BuiltinFunctionError,
    ImpossibleConversion
}

pub fn eval_statement(env: &mut DynamicEnv, code: &str) -> Result<Option<Value>, ErrorWrapper> {
    let stm = parse_statement(code)?;

    eval_stm(env, &stm)
        .map_err(|e| ErrorWrapper::Runtime(e))
}

pub fn eval_expression(env: &mut DynamicEnv, code: &str) -> Result<Value, ErrorWrapper> {
    let expr = parse_expression(code)?;

    eval_expr(env, &expr)
        .map_err(|e| ErrorWrapper::Runtime(e))
}
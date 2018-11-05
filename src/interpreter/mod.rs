use analyzer::TypeError;
use errors::ErrorWrapper;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::expression_eval::eval_expr;
use interpreter::statement_eval::eval_stm;
use parsers::parse_expr;
use parsers::parse_statement;
use tokenizer::tokenize;
use ast::Pattern;
use types::Value;
use util::expression_fold::ExprTreeError;
use tokenizer::TokenStream;

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
    BuiltinFunctionError
}

pub fn eval_statement(env: &mut DynamicEnv, code: &str) -> Result<Option<Value>, ErrorWrapper> {
    let tokens = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let stm = parse_statement(TokenStream::new(&tokens))
        .map_err(|e| ErrorWrapper::Syntactic(e))?;

    eval_stm(env, &stm)
        .map_err(|e| ErrorWrapper::Runtime(e))
}

pub fn eval_expression(env: &mut DynamicEnv, code: &str) -> Result<Value, ErrorWrapper> {
    let  tokens = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let expr = parse_expr(TokenStream::new(&tokens))
        .map_err(|e| ErrorWrapper::Syntactic(e))?;

    eval_expr(env, &expr)
        .map_err(|e| ErrorWrapper::Runtime(e))
}
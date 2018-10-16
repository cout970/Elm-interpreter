use analyzer::TypeError;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::expression_eval::eval_expr;
use interpreter::RuntimeError::ParseError;
use parsers::parse_expr;
use tokenizer::tokenize;
use types::Value;
use parsers::parse_statement;
use interpreter::statement_eval::eval_stm;
use types::Type;

pub mod dynamic_env;
mod builtins;
mod expression_eval;
mod statement_eval;

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeError {
    MissingDef(String, DynamicEnv),
    TODO(String),
    ParseError,
    TypeError(TypeError),
    InternalError,
}

pub fn eval_statement(env: &mut DynamicEnv, code: &str) -> Result<Option<Value>, RuntimeError> {
    let tokens = tokenize(code.as_bytes()).unwrap();
    let stm = parse_statement(&tokens).map_err(|_| ParseError)?;
    eval_stm(env, &stm)
}

pub fn eval_expression(env: &mut DynamicEnv, code: &str) -> Result<Value, RuntimeError> {
    let tokens = tokenize(code.as_bytes()).unwrap();
    let expr = parse_expr(&tokens).map_err(|_| ParseError)?;
    eval_expr(env, &expr)
}
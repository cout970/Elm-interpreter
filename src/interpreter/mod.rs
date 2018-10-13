
pub mod dynamic_env;
mod builtins;
mod expression_eval;
mod statement_eval;

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeError {
    TODO(String),
    InternalError
}
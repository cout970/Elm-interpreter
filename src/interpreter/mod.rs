use errors::*;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::expression_eval::eval_expr;
use interpreter::module_eval::eval_mod;
use interpreter::statement_eval::eval_stm;
use loader::ModuleLoader;
use parsers::parse_expression;
use parsers::parse_statement;
use types::Value;

pub mod dynamic_env;
mod builtins;
mod expression_eval;
mod statement_eval;
mod module_eval;


pub fn eval_statement(env: &mut DynamicEnv, code: &str) -> Result<Option<Value>, ElmError> {
    let stm = parse_statement(code)?;

    eval_stm(env, &stm)
        .map_err(|e| ElmError::Interpreter { info: e })
}

pub fn eval_expression(env: &mut DynamicEnv, code: &str) -> Result<Value, ElmError> {
    let expr = parse_expression(code)?;

    eval_expr(env, &expr)
        .map_err(|e| ElmError::Interpreter { info: e })
}

pub fn eval_module(env: &mut DynamicEnv, loader: &ModuleLoader, name: &str) -> Result<(), ElmError> {
    let module = loader.get_module(name)
        .ok_or_else(|| ElmError::Interpreter { info: RuntimeError::MissingModule(vec![name.to_string()]) })?;

    eval_mod(env, module)
        .map_err(|e| ElmError::Interpreter { info: e })
}
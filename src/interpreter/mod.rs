use ast::*;
use errors::*;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::expression_eval::eval_expr;
use interpreter::module_eval::eval_mod;
use interpreter::statement_eval::eval_stm;
use loader::ModuleLoader;
use types::Value;

pub mod dynamic_env;
mod builtins;
mod expression_eval;
mod statement_eval;
mod module_eval;


pub fn eval_statement(env: &mut DynamicEnv, stm: &Statement) -> Result<Option<Value>, ElmError> {
    eval_stm(env, &stm).map_err(|e| ElmError::Interpreter { info: e })
}

pub fn eval_expression(env: &mut DynamicEnv, expr: &Expr) -> Result<Value, ElmError> {
    eval_expr(env, expr).map_err(|e| ElmError::Interpreter { info: e })
}

pub fn eval_module(env: &mut DynamicEnv, loader: &ModuleLoader, name: &str) -> Result<(), ElmError> {
    let module = loader.get_module(name)
        .ok_or_else(|| ElmError::Interpreter { info: RuntimeError::MissingModule(vec![name.to_string()]) })?;

    eval_mod(env, module)
        .map_err(|e| ElmError::Interpreter { info: e })
}
use errors::*;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::eval_statement;
use interpreter::statement_eval::eval_stm;
use loader::LoadedModule;

pub fn eval_mod(env: &mut DynamicEnv, module: &LoadedModule) -> Result<(), RuntimeError> {

    // TODO evaluate dependencies
    for stm in &module.ast.statements {
        eval_stm(env, stm)?;
    }
    Ok(())
}
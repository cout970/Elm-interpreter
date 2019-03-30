use analyzer::Analyser;
use analyzer::static_env::StaticEnv;
use analyzer::TypeError;
use ast::*;

pub fn analyze_function(env: &mut StaticEnv, fun: &Definition) -> Result<Type, TypeError> {
    let expr = Analyser::from(env.clone()).analyze_definition(fun)?;
    Ok(expr.header)
}
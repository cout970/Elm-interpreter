use analyzer::Analyser;
use analyzer::expression_analyzer::analyze_expression;
use analyzer::pattern_analyzer::*;
use analyzer::PatternMatchingError;
use analyzer::static_env::StaticEnv;
use analyzer::type_helper::is_assignable;
use analyzer::TypeError;
use ast::*;
use typed_ast::expr_type;
use util::build_fun_type;
use util::create_vec_inv;
use util::StringConversion;

pub fn analyze_function(env: &mut StaticEnv, fun: &Definition) -> Result<Type, TypeError> {
    let expr = Analyser::from(env.clone()).analyze_definition(fun)?;
    Ok(expr.header)
}
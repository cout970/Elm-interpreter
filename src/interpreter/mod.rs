use analyzer::inter_mod_analyzer::ModulePath;
use analyzer::TypeError;
use ast::Pattern;
use errors::ErrorWrapper;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::expression_eval::eval_expr;
use interpreter::module_eval::eval_mod;
use interpreter::statement_eval::eval_stm;
use loader::Declaration;
use loader::ModuleLoader;
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
    MissingModule(ModulePath),
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
    ExpectedString(Value),
    ExpectedNumber(Value),
    ExpectedNonEmptyList(Value),
    UnknownOperatorPattern(String),
    InternalErrorRecordAccess(Value),
    InternalErrorAdtCreation(Value),
    UnknownBuiltinFunction(u32),
    BuiltinFunctionError,
    ImpossibleConversion,
    MissingSourceFile,
    CyclicModuleDependency(Vec<ModulePath>),
    MissingExposing(String, Vec<Declaration>),
    InternalError,
}

pub fn eval_statement(env: &mut DynamicEnv, code: &str) -> Result<Option<Value>, ErrorWrapper> {
    let stm = parse_statement(code)?;

    eval_stm(env, &stm)
        .map_err(|e| ErrorWrapper::RuntimeError(e))
}

pub fn eval_expression(env: &mut DynamicEnv, code: &str) -> Result<Value, ErrorWrapper> {
    let expr = parse_expression(code)?;

    eval_expr(env, &expr)
        .map_err(|e| ErrorWrapper::RuntimeError(e))
}

pub fn eval_module(env: &mut DynamicEnv, loader: &ModuleLoader, name: &str) -> Result<(), ErrorWrapper> {
    let module = loader.get_module(name)
        .ok_or_else(|| ErrorWrapper::RuntimeError(RuntimeError::MissingModule(vec![name.to_string()])))?;

    eval_mod(env, module)
        .map_err(|e| ErrorWrapper::RuntimeError(e))
}
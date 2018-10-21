use analyzer::expression_analyzer::analyze_expression;
use analyzer::function_analyzer::PatternMatchingError;
use std::collections::HashMap;
use types::Expr;
use types::Fun;
use types::Type;
use types::Value;
use util::StringConversion;
use types::Definition;
use analyzer::function_analyzer::analyze_function;
use analyzer::static_env::StaticEnv;
use std::sync::Arc;
use types::Adt;
use analyzer::expression_analyzer::get_adt_type;
use std::ops::Deref;

mod function_analyzer;
mod expression_analyzer;
pub mod static_env;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeError {
    MissingAdt(String),
    MissingDefinition(String),
    ListNotHomogeneous(String),
    IfWithNonBoolCondition(String),
    IfBranchesDoesntMatch(String),
    ArgumentsDoNotMatch(String),
    NotAFunction(String),
    InvalidOperandChain(String),
    RecordUpdateOnNonRecord(String),
    RecordUpdateUnknownField(String),
    CaseBranchDontMatchReturnType(String),
    DefinitionTypeAndReturnTypeMismatch,
    InvalidPattern(PatternMatchingError),
    ConstantEvaluationError(String),
    VariableAlreadyDeclared(String),
    UnableToCalculateFunctionType(String),
    VariableNameShadowed(String),
    InternalError,
}

pub fn type_check_expression(env: &mut StaticEnv, expr: &Expr) -> Result<Type, TypeError> {
    analyze_expression(env, None, expr)
}

pub fn type_check_function(env: &mut StaticEnv, fun: &Definition) -> Result<Type, TypeError> {
    analyze_function(env, fun)
}

pub fn type_of_value(value: &Value) -> Type {
    match value {
        Value::Unit => {
            Type::Unit
        }
        Value::Number(_) => {
            Type::Var("number".s())
        }
        Value::Int(_) => {
            Type::Tag("Int".s(), vec![])
        }
        Value::Float(_) => {
            Type::Tag("Float".s(), vec![])
        }
        Value::String(_) => {
            Type::Tag("String".s(), vec![])
        }
        Value::Char(_) => {
            Type::Tag("Char".s(), vec![])
        }
        Value::List(items) => {
            if items.is_empty() {
                Type::Tag("List".s(), vec![Type::Var("a".s())])
            } else {
                Type::Tag("List".s(), vec![type_of_value(items.first().unwrap())])
            }
        }
        Value::Tuple(items) => {
            Type::Tuple(items.iter().map(|i| type_of_value(i)).collect())
        }
        Value::Record(items) => {
            Type::Record(items.iter().map(|(s, i)| (s.to_owned(), type_of_value(i))).collect())
        }
        Value::Adt(var_name, items, adt) => {
            get_adt_type(var_name, items, adt.clone())
        }
        Value::Fun { fun, args, .. } => {
            let fun_ty = match fun.deref() {
                Fun::Builtin(_, _, ty) => ty,
                Fun::Expr(_, _, _, ty) => ty,
            };

            strip_fun_args(args.len(), &fun_ty).clone()
        }
    }
}

fn strip_fun_args(args: usize, ty: &Type) -> &Type {
    if args == 0 {
        return ty;
    }

    if let Type::Fun(_, ref output) = ty {
        strip_fun_args(args - 1, output)
    } else {
        ty
    }
}

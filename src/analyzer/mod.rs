use analyzer::expression_analyzer::analyze_expression;
use ast::Expr;
use types::Function;
use ast::Type;
use types::Value;
use util::StringConversion;
use ast::Definition;
use analyzer::function_analyzer::analyze_function;
use analyzer::static_env::StaticEnv;
use analyzer::expression_analyzer::get_adt_type;
use std::ops::Deref;
use analyzer::pattern_analyzer::PatternMatchingError;

pub mod static_env;
pub mod inter_mod_analyzer;
mod function_analyzer;
mod expression_analyzer;
mod module_analyser;
mod dependency_sorter;
mod pattern_analyzer;
mod type_helper;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeError {
    List(Vec<TypeError>),
    MissingModule(Vec<String>),
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
    UndeclaredTypeVariables(Vec<String>),
    UnusedTypeVariables(Vec<String>),
    InvalidPatternAmount(usize, usize),
    InternalError,
    CyclicStatementDependency(Vec<String>),
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
                Function::Builtin(_, _, ty) => ty,
                Function::Expr(_, _, _, ty) => ty,
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


#[cfg(test)]
mod tests {
    use analyzer::module_analyser::analyze_module;
    use parsers::from_code_mod;
    use analyzer::inter_mod_analyzer::InterModuleInfo;

    #[test]
    #[ignore]
    fn type_check1(){
        let module = from_code_mod(include_bytes!("../../benches/data/type_check.elm"));
        let info = InterModuleInfo::new();
        let path = vec![];


        let checked = analyze_module(&info, &path, module).expect("Type error");
        println!("{:?}", checked);
        panic!();
    }
}
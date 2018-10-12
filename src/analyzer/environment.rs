use analyzer::pattern_helper::add_pattern_variables;
use analyzer::type_resolution::particularize_type;
use analyzer::TypeError;
use analyzer::TypeError::ConstantEvaluationError;
use analyzer::TypeError::UnableToCalculateFunctionType;
use analyzer::TypeError::VariableAlreadyDeclared;
use interpreter::eval;
use std::collections::HashMap;
use types::Adt;
use types::CurriedFunc;
use types::Definition;
use types::Fun;
use types::Type;
use types::Value;
use util::arg_count;
use util::build_fun_type;
use util::builtin_fun_of;
use util::StringConversion;
use analyzer::expression_analyzer::analyze_expression;

#[derive(Clone)]
pub struct Environment(Vec<Block>);

#[derive(Clone)]
struct Block {
    defs: HashMap<String, Value>,
    adts: HashMap<String, Adt>,
    alias: HashMap<String, Type>,
    variables: HashMap<String, Type>,
}

pub fn default_lang_env() -> Environment {
    let mut env = Environment::new();

    env.add("+", builtin_fun_of(1, build_fun_type(&vec![
        Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
    ])));
    env.add("-", builtin_fun_of(2, build_fun_type(&vec![
        Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
    ])));
    env.add("*", builtin_fun_of(3, build_fun_type(&vec![
        Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
    ])));
    env.add("/", builtin_fun_of(4, build_fun_type(&vec![
        Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
    ])));
    env.add("//", builtin_fun_of(5, build_fun_type(&vec![
        Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
    ])));

    env
}

impl Environment {
    pub fn new() -> Self {
        Environment(vec![
            Block {
                defs: HashMap::new(),
                adts: HashMap::new(),
                alias: HashMap::new(),
                variables: HashMap::new(),
            }
        ])
    }

    pub fn add(&mut self, name: &str, def: Value) {
        self.0.last_mut().unwrap().defs.insert(name.to_owned(), def);
    }

    pub fn find(&self, name: &str) -> Option<Value> {
        self.find_map(|block| block.defs.get(name))
    }

    pub fn find_variable(&self, name: &str) -> Option<Type> {
        self.find_map(|block| block.variables.get(name))
    }

    pub fn add_variable(&mut self, name: &str, var: Type) {
        self.0.last_mut().unwrap().variables.insert(name.to_owned(), var);
    }

    fn find_map<T, F>(&self, func: F) -> Option<T>
        where
            T: Clone,
            F: Fn(&Block) -> Option<&T>
    {
        for b in self.0.iter().rev() {
            if let Some(t) = func(b) {
                return Some(t.clone());
            }
        }

        None
    }

    pub fn enter_block(&mut self) {
        self.0.push(Block {
            defs: HashMap::new(),
            adts: HashMap::new(),
            alias: HashMap::new(),
            variables: HashMap::new(),
        });
    }

    pub fn exit_block(&mut self) {
        self.0.pop().expect("Tried to exit the global block");
    }
}

pub fn expand_env(env: &mut Environment, defs: Vec<&Definition>) -> Result<(), TypeError> {
//    for Definition(opt_ty, value) in defs {
//
//        let expr_ty = analyze_expression(env, vars, &value.expr, None);
//
//        let val: Value = if value.patterns.is_empty() {
//            eval(env, &value.expr).map_err(|e| ConstantEvaluationError(e))?
//        } else {
//            let mut args_ty = (&value.patterns).iter()
//                .map(|p| pattern_to_type(p))
//                .collect::<Result<Vec<Type>, _>>()
//                .map_err(|e| UnableToCalculateFunctionType(e))?;
//
//            args_ty.push(expr_ty?);
//            let fun_ty = build_fun_type(&args_ty);
//
//            let ty = opt_ty.clone()
//                .map(|t| particularize_type(&t, &fun_ty))
//                .unwrap_or(fun_ty);
//
//            Value::Fun(CurriedFunc {
//                args: vec![],
//                arg_count: arg_count(&ty),
//                fun: Fun::Expr(value.patterns.clone(), value.expr.clone(), ty),
//            })
//        };
//
//        env.add(&value.name, val);
//    }

    Ok(())
}
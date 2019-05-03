use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::Arc;

use builtin::adt_constructor;
use builtin::record_access;
use constructors::type_bool;
use errors::*;
use interpreter::runtime_stack::RuntimeStack;
use loader::AnalyzedModule;
use loader::Declaration;
use loader::RuntimeModule;
use rust_interop::call_function;
use typed_ast::{TypedDefinition, TypedPattern};
use typed_ast::TypedExpr;
use types::Adt;
use types::AdtVariant;
use types::Function;
use types::Value;
use util::VecExt;

pub mod runtime_stack;
//mod builtins;
mod closure_helper;

#[derive(Clone, Debug)]
pub struct Interpreter {
    pub stack: RuntimeStack,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: RuntimeStack::new()
        }
    }

    pub fn adt_value(&mut self, adt_name: &str, arguments: &[Value]) -> Result<Value, ElmError> {
        let ctor = self.find_value(adt_name)
            .ok_or_else(|| InterpreterError::MissingDefinition(adt_name.to_string()).wrap())?;

        self.apply_function(ctor, arguments)
    }

    pub fn find_value(&mut self, name: &str) -> Option<Value> {
        self.stack.find(name)
    }

    pub fn true_value(&mut self) -> Value {
        self.eval_expr(&TypedExpr::Ref((0, 0), type_bool(), "True".to_string())).unwrap()
    }

    pub fn false_value(&mut self) -> Value {
        self.eval_expr(&TypedExpr::Ref((0, 0), type_bool(), "False".to_string())).unwrap()
    }

    pub fn eval_constants(&mut self, module: RuntimeModule) -> Result<RuntimeModule, ElmError> {
        let RuntimeModule { name, definitions: old_definitions, imports } = module;
        let mut definitions = HashMap::new();

        for (name, value) in old_definitions.into_iter() {
            let new_value = self.eval_const(value)?;

            definitions.insert(name, new_value);
        }

        Ok(RuntimeModule { name, definitions, imports })
    }

    fn eval_const(&mut self, value: Value) -> Result<Value, ElmError> {
        let opt = if let Value::Fun { arg_count, fun, .. } = &value {
            if *arg_count == 0 {
                Some(self.exec_fun(fun.borrow(), vec![])?)
            } else {
                None
            }
        } else {
            None
        };

        Ok(opt.unwrap_or(value))
    }

    pub fn eval_module(&mut self, modules: &HashMap<String, RuntimeModule>, module: &AnalyzedModule) -> Result<RuntimeModule, ElmError> {
        let mut definitions = HashMap::new();

        for import in &module.imports {
            let module = modules.get(&import.source)
                .ok_or_else(|| InterpreterError::MissingModule(vec![import.source.to_string()]).wrap())?;

            let value = module.definitions.get(&import.source_name)
                .ok_or_else(|| {
                    eprintln!("Failed to find {} in {} {:#?}", import.source_name, import.source, module.definitions.keys().collect::<Vec<_>>());
                    InterpreterError::MissingDefinition(import.source_name.to_string()).wrap()
                })?;

            self.stack.add(&import.destine_name, value.clone());
        };

        for decl in &module.all_declarations {
            match decl {
                Declaration::Port(_, _) => {}
                Declaration::Definition(_, def) => {
                    let (name, value) = self.eval_definition(def);
                    definitions.insert(name, value);
                }
                Declaration::Alias(_) => {}
                Declaration::Adt(_, adt) => {
                    for variant in &adt.variants {
                        let (name, value) = self.eval_adt_variant(adt.clone(), variant);
                        definitions.insert(name, value);
                    }
                }
                Declaration::Infix(_, _, _) => {}
            }
        }

        Ok(RuntimeModule {
            name: module.name.to_string(),
            definitions,
            imports: module.imports.clone(),
        })
    }

    pub fn eval_declaration(&mut self, decl: &Declaration) -> Result<Option<Value>, ElmError> {
        if let Declaration::Definition(_, def) = decl {
            let (name, value) = self.eval_definition(def);
            let value = self.eval_const(value)?;

            self.stack.add(&name, value.clone());
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn eval_definition(&mut self, def: &TypedDefinition) -> (String, Value) {
        let name = def.name.clone();
        let value = Self::create_function_closure(&mut self.stack, def);

        self.stack.add(&name, value.clone());

        (name, value)
    }

    fn eval_adt_variant(&mut self, adt: Arc<Adt>, variant: &AdtVariant) -> (String, Value) {
        let name = variant.name.clone();
        let value = adt_constructor(adt.clone(), &variant);

        self.stack.add(&name, value.clone());

        (name, value)
    }

    pub fn eval_expr(&mut self, expr: &TypedExpr) -> Result<Value, ElmError> {
        match expr {
            TypedExpr::Ref(_, _, name) => {
                let opt = self.stack.find(name);
                match opt {
                    Some(val) => Ok(val),
                    None => {
                        Err(InterpreterError::MissingDefinition(name.clone()).wrap())
                    }
                }
            }
            TypedExpr::Const(_, _, value) => Ok(value.clone()),
            TypedExpr::Tuple(_, _, items) => {
                let values = items.iter()
                    .map(|e| self.eval_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Value::Tuple(values))
            }
            TypedExpr::List(_, _, items) => {
                let values = items.iter()
                    .map(|e| self.eval_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Value::List(values))
            }
            TypedExpr::Record(_, _, items) => {
                let values = items.iter()
                    .map(|(s, e)| {
                        self.eval_expr(e).map(|e| (s.clone(), e))
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Value::Record(values))
            }
            TypedExpr::RecordUpdate(_, _, name, items) => {
                let val = self.eval_expr(name.as_ref())?;

                if let Value::Record(values) = &val {
                    let entries = values.iter().map(|(name, value)| {
                        items.iter()
                            .find(|(_name, _)| name == _name)
                            .and_then(|(nam, expr)| {
                                self.eval_expr(expr).map(|val| (nam.clone(), val)).ok()
                            })
                            .unwrap_or((name.clone(), value.clone()))
                    }).collect();

                    Ok(Value::Record(entries))
                } else {
                    Err(InterpreterError::RecordUpdateOnNonRecord(name.as_ref().clone(), val.clone()).wrap())
                }
            }
            TypedExpr::If(_, _, cond, a, b) => {
                let cond = self.eval_expr(cond)?;

                match &cond {
                    Value::Adt(ref name, ref vals, _) => {
                        if name == "True" && vals.is_empty() {
                            self.eval_expr(a)
                        } else if name == "False" && vals.is_empty() {
                            self.eval_expr(b)
                        } else {
                            Err(InterpreterError::InvalidIfCondition(cond.clone()).wrap())
                        }
                    }
                    _ => {
                        Err(InterpreterError::InvalidIfCondition(cond.clone()).wrap())
                    }
                }
            }
            TypedExpr::Lambda(_, ty, patt, expr) => {
                Ok(Self::create_lambda_closure(&mut self.stack, ty, patt, expr))
            }
            TypedExpr::RecordField(_, _, record, field) => {
                let rec = self.eval_expr(record)?;

                if let Value::Record(entries) = &rec {
                    let (_, value) = entries.iter()
                        .find(|(name, _)| name == field)
                        .ok_or(InterpreterError::RecordFieldNotFound(field.to_owned(), rec.clone()).wrap())?;

                    Ok(value.clone())
                } else {
                    Err(InterpreterError::ExpectedRecord(rec.clone()).wrap())
                }
            }
            TypedExpr::RecordAccess(_, ty, field) => {
                Ok(record_access(ty, field))
            }
            TypedExpr::Case(_, _, cond, branches) => {
                let cond_val = self.eval_expr(cond)?;
                for (patt, expr) in branches {
                    if matches_pattern(patt, &cond_val) {
                        return self.eval_expr(expr);
                    }
                }

                return Err(InterpreterError::CaseExpressionNonExhaustive(cond_val, branches.map(|(p, _)| p.clone())).wrap());
            }
            TypedExpr::Let(..) => Ok(Value::Unit), // TODO
            TypedExpr::Application(_, _, fun, input) => {
                let function = self.eval_expr(fun)?;
                let input = self.eval_expr(input)?;
                self.application(function, input)
            }
        }
    }

    pub fn apply_function(&mut self, function: Value, arguments: &[Value]) -> Result<Value, ElmError> {
        let mut value = function;

        for arg in arguments {
            value = self.application(value, arg.clone())?;
        }

        Ok(value)
    }

    fn application(&mut self, fun_value: Value, input: Value) -> Result<Value, ElmError> {
        // Get from cache
//      let fun_call = FunCall { function: fun_value.clone(), argument: input.clone() };
//
//      if let Some(val) = self.get_from_cache(&fun_call) {
//          return Ok(val.clone());
//      }

        if let Value::Fun { arg_count, args, fun } = &fun_value {
            let argc = args.len() as u32 + 1;

            if *arg_count < argc {
                return Err(InterpreterError::FunArgumentSizeMismatch(*arg_count, argc, fun.clone()).wrap());
            }

            let mut arg_vec = args.clone();
            arg_vec.push(input);

            let value = if *arg_count == argc {
                self.exec_fun(fun, arg_vec)?
            } else {
                Value::Fun { args: arg_vec, arg_count: *arg_count, fun: fun.clone() }
            };

            // Update cache
//            self.add_to_cache(fun_call, value.clone());
            Ok(value)
        } else {
            Err(InterpreterError::ExpectedFunction(fun_value.clone()).wrap())
        }
    }

    fn exec_fun(&mut self, fun: &Function, args: Vec<Value>) -> Result<Value, ElmError> {
        self.stack.enter_block();
        let res = match fun {
            Function::External(_, func, _) => {
                (func.fun)(self, &args)
                    .map_err(|_| InterpreterError::BuiltinFunctionError.wrap())
            }
            Function::Wrapper(_, func, _) => {
                call_function(func, self, &args)
                    .map_err(|_| InterpreterError::BuiltinFunctionError.wrap())
            }
            Function::Definition { patterns, expression, captures, .. } => {
                assert_eq!(patterns.len(), args.len());

                for (name, val) in captures {
                    self.stack.add(name, val.clone())
                }

                for (patt, val) in patterns.iter().zip(args) {
                    add_pattern_values(self, patt, val).unwrap();
                }

                self.eval_expr(expression)
            }
        };
        self.stack.exit_block();
        Ok(res?)
    }

    pub fn debug(&self) -> String {
        self.stack.debug()
    }
}

fn matches_pattern(pattern: &TypedPattern, value: &Value) -> bool {
    match pattern {
        TypedPattern::Var(_, _, _) => true,
        TypedPattern::Wildcard(_) => true,
        TypedPattern::Alias(_, _, pat, _) => matches_pattern(pat, value),
        TypedPattern::Adt(_, _, p_ty, p_sub) => {
            let val_ty = value.get_type();
            if let Value::Adt(_, v_sub, _) = value {
                p_ty == &val_ty && p_sub.iter().zip(v_sub).all(|(a, b)| matches_pattern(a, b))
            } else {
                false
            }
        }
        TypedPattern::Unit(_) => value == &Value::Unit,
        TypedPattern::Tuple(_, _, p_sub) => {
            if let Value::Tuple(v_sub) = value {
                p_sub.iter().zip(v_sub).all(|(a, b)| matches_pattern(a, b))
            } else {
                false
            }
        }
        TypedPattern::List(_, _, p_sub) => {
            if let Value::List(v_sub) = value {
                p_sub.iter().zip(v_sub).all(|(a, b)| matches_pattern(a, b))
            } else {
                false
            }
        }
        TypedPattern::BinaryOp(_, _, op, first, rest) => {
            assert_eq!(op.as_str(), "::");

            if let Value::List(v_sub) = value {
                if !v_sub.is_empty() {
                    matches_pattern(first, &v_sub[0]) &&
                        matches_pattern(rest, &Value::List(v_sub[1..].to_vec()))
                } else {
                    false
                }
            } else {
                false
            }
        }
        TypedPattern::Record(_, _, fields) => {
            if let Value::Record(entries) = value {
                fields.iter().all(|field_name| {
                    entries.iter().find(|(name, _)| name == field_name).is_some()
                })
            } else {
                false
            }
        }
        TypedPattern::LitInt(_, p) => {
            match value {
                Value::Int(v) => {
                    (*p) == (*v)
                }
                Value::Number(v) => {
                    (*p) == (*v)
                }
                _ => {
                    false
                }
            }
        }
        TypedPattern::LitString(_, p) => {
            if let Value::String(v) = value { p == v } else { false }
        }
        TypedPattern::LitChar(_, p) => {
            if let Value::Char(v) = value { *p == *v } else { false }
        }
    }
}

pub fn add_pattern_values(env: &mut Interpreter, pattern: &TypedPattern, value: Value) -> Result<(), InterpreterError> {
    match pattern {
        TypedPattern::Var(_, _, n) => {
            env.stack.add(&n, value);
        }
        TypedPattern::Alias(_, _, pat, name) => {
            env.stack.add(name, value.clone());
            add_pattern_values(env, pat, value)?;
        }
        TypedPattern::Record(_, _, items) => {
            if let Value::Record(vars) = &value {
                for patt in items {
                    let (name, val) = vars.iter()
                        .find(|(name, _)| name == patt)
                        .ok_or(InterpreterError::RecordFieldNotFound(patt.clone(), value.clone()))?;

                    env.stack.add(name, val.clone());
                }
            } else {
                return Err(InterpreterError::ExpectedRecord(value.clone()));
            }
        }
        TypedPattern::Adt(_, _, _, items) => {
            if let Value::Adt(_, vars, _) = &value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val.clone())?;
                }
            } else {
                return Err(InterpreterError::ExpectedAdt(value.clone()));
            }
        }
        TypedPattern::Tuple(_, _, items) => {
            if let Value::Tuple(vars) = &value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val.clone())?;
                }
            } else {
                return Err(InterpreterError::ExpectedTuple(value.clone()));
            }
        }
        TypedPattern::List(_, _, items) => {
            if let Value::List(vars) = &value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val.clone())?;
                }
            } else {
                return Err(InterpreterError::ExpectedList(value.clone()));
            }
        }
        TypedPattern::LitInt(_, _) => {}
        TypedPattern::LitString(_, _) => {}
        TypedPattern::LitChar(_, _) => {}
        TypedPattern::Wildcard(_) => {}
        TypedPattern::Unit(_) => {}
        TypedPattern::BinaryOp(_, _, op, a, b) => {
            if op == "::" {
                if let Value::List(vars) = &value {
                    if vars.len() == 0 {
                        return Err(InterpreterError::ExpectedNonEmptyList(value.clone()));
                    }

                    let first = vars[0].clone();
                    let mut rest: Vec<Value> = Vec::new();
                    for i in 1..vars.len() {
                        rest.push(vars[i].clone());
                    }

                    add_pattern_values(env, a, first)?;
                    add_pattern_values(env, b, Value::List(rest))?;
                } else {
                    return Err(InterpreterError::ExpectedList(value.clone()));
                }
            } else {
                return Err(InterpreterError::UnknownOperatorPattern(op.clone()));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use interpreter::Interpreter;
    use test_utils::Test;
    use types::Value;

    #[test]
    fn check_unit() {
        let expr = Test::typed_expr("()");
        let mut env = Interpreter::new();

        assert_eq!(env.eval_expr(&expr), Ok(Value::Unit));
    }

    #[test]
    fn check_list() {
        let expr = Test::typed_expr("[1, 2, 3]");
        let mut env = Interpreter::new();

        assert_eq!(env.eval_expr(&expr), Ok(Value::List(vec![
            Value::Number(1),
            Value::Number(2),
            Value::Number(3),
        ])));
    }

//    #[test]
//    fn check_lambda() {
//        let expr = Test::typed_expr("\\x -> 1");
//        let mut env = Interpreter::new();
//
//        let value = env.eval_expr(&expr).unwrap();
//        match value {
//            Value::Fun { args, fun, .. } => {
//                assert_eq!(args, vec![]);
//                // TODO
//            }
//            _ => panic!("Not a function: {}", value)
//        }
//    }

    #[test]
    fn check_record() {
        let expr = Test::typed_expr("{ a = 0 }.a");
        let mut env = Interpreter::new();

        assert_eq!(env.eval_expr(&expr), Ok(Value::Number(0)));
    }

//    #[test]
//    fn check_number() {
//        let expr = Test::typed_expr("1 / 3");
//        let mut env = Interpreter::new();
//
//        assert_eq!(env.eval_expr(&expr), Ok(Value::Float(0.3333333333333333)));
//    }
//
//    #[test]
//    fn check_number2() {
//        let expr = Test::typed_expr("4 // 3");
//        let mut env = Interpreter::new();
//
//        assert_eq!(env.eval_expr(&expr), Ok(Value::Int(1)));
//    }
//
//    #[test]
//    fn check_number3() {
//        let expr = Test::typed_expr("4 + 3");
//        let mut env = Interpreter::new();
//
//        assert_eq!(env.eval_expr(&expr), Ok(Value::Number(7)));
//    }
}

// TODO
//#[cfg(test)]
//mod tests {
//    use test_utils::Test;
//    use util::StringConversion;
//
//    use super::*;
//
//    fn formatted(env: &mut DynamicEnv, stm: &Statement) -> String {
//        let result = eval_stm(env, stm);
//        let option = result.unwrap();
//        let value = option.unwrap();
//        let ty = value.get_type();
//
//        format!("{} : {}", value, ty)
//    }
//
//    fn formatted_expr(env: &mut DynamicEnv, expr: &Expr) -> String {
//        unimplemented!()
////        let result = eval_expr(env, expr);
////        let value = result.unwrap();
////        let ty = value.get_type();
////
////        format!("{} : {}", value, ty)
//    }
//
//    #[test]
//    fn check_constant() {
//        let stm = Test::statement("x = 1");
//        let mut env = DynamicEnv::new();
//
//        assert_eq!(formatted(&mut env, &stm), "1 : number".s());
//    }
//
//    #[test]
//    fn check_identity() {
//        let stm = Test::statement("id value = value");
//        let mut env = DynamicEnv::new();
//
//        assert_eq!(formatted(&mut env, &stm), "<function> : a -> a".s());
//    }
//
//    #[test]
//    fn check_recursivity() {
//        let stm = Test::statement("fib num = case num of \n 0 -> 0\n 1 -> 1\n _ -> fib (num - 1) + fib (num - 2)");
//        let mut env = DynamicEnv::default_lang_env();
//
//        assert_eq!(formatted(&mut env, &stm), "<function> : Int -> number".s());
//    }
//
//    #[test]
//    fn check_adt() {
//        let decl = Test::statement("type Adt = A | B");
//        let mut env = DynamicEnv::default_lang_env();
//
//        eval_stm(&mut env, &decl).unwrap();
//
//        assert_eq!(formatted_expr(&mut env, &Test::expr("A")), "A : Adt".s());
//        assert_eq!(formatted_expr(&mut env, &Test::expr("B")), "B : Adt".s());
//    }
//
//    #[test]
//    fn check_adt2() {
//        let decl = Test::statement("type Adt a = A a | B Int");
//        let mut env = DynamicEnv::default_lang_env();
//
//        eval_stm(&mut env, &decl).unwrap();
//
//        assert_eq!(formatted_expr(&mut env, &Test::expr("A")), "<function> : a -> Adt a".s());
//        assert_eq!(formatted_expr(&mut env, &Test::expr("B")), "<function> : Int -> Adt a".s());
//        assert_eq!(formatted_expr(&mut env, &Test::expr("A 1")), "A 1 : Adt number".s());
//        assert_eq!(formatted_expr(&mut env, &Test::expr("B 1")), "B 1 : Adt a".s());
//    }
//
//    #[test]
//    fn check_fib() {
//        let decl = Test::statement("fib num = case num of \n0 -> 0 \n1 -> 1 \n_ -> fib (num - 1) + fib (num - 2)");
//        let mut env = DynamicEnv::default_lang_env();
//
//        eval_stm(&mut env, &decl).unwrap();
//
//        assert_eq!(formatted_expr(&mut env, &Test::expr("fib")), "<function> : Int -> number".s());
//        assert_eq!(formatted_expr(&mut env, &Test::expr("fib 0")), "0 : number".s());
//        assert_eq!(formatted_expr(&mut env, &Test::expr("fib 1")), "1 : number".s());
//        assert_eq!(formatted_expr(&mut env, &Test::expr("fib 2")), "1 : number".s());
//        assert_eq!(formatted_expr(&mut env, &Test::expr("fib 3")), "2 : number".s());
//        assert_eq!(formatted_expr(&mut env, &Test::expr("fib 4")), "3 : number".s());
//        assert_eq!(formatted_expr(&mut env, &Test::expr("fib 5")), "5 : number".s());
//    }
//}
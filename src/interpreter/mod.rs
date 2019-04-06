use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::Arc;

use ast::*;
use errors::*;
use interpreter::builtins::builtin_record_access;
use interpreter::dynamic_env::RuntimeStack;
use interpreter::expression_eval::eval_expr;
use interpreter::statement_eval::eval_stm;
use loader::AnalyzedModule;
use loader::Declaration;
use loader::LoadedModule;
use loader::ModuleLoader;
use loader::RuntimeModule;
use Runtime;
use rust_interop::call_function;
use typed_ast::TypedDefinition;
use typed_ast::TypedExpr;
use types::FunCall;
use types::Function;
use types::next_fun_id;
use types::Value;
use util::VecExt;

pub mod dynamic_env;
mod builtins;
mod expression_eval;
mod statement_eval;
mod closure_helper;

#[derive(Clone, Debug)]
pub struct Interpreter {
    stack: RuntimeStack,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: RuntimeStack::new()
        }
    }

    pub fn eval_constants(&mut self, run: &mut Runtime, module: RuntimeModule) -> Result<RuntimeModule, ElmError> {
        let RuntimeModule { name, definitions: old_definitions, imports } = module;
        let mut definitions = HashMap::new();

        for (name, value) in old_definitions.into_iter() {
            let opt = if let Value::Fun { arg_count, fun, .. } = &value {
                if *arg_count == 0 {
                    Some(self.exec_fun(fun.borrow(), vec![])?)
                } else {
                    None
                }
            } else {
                None
            };

            let new_value = opt.unwrap_or(value);

            definitions.insert(name, new_value);
        }

        Ok(RuntimeModule { name, definitions, imports })
    }

    pub fn eval_module(&mut self, modules: &HashMap<String, RuntimeModule>, module: &AnalyzedModule) -> Result<RuntimeModule, ElmError> {
        let mut definitions = HashMap::new();

        for import in &module.imports {
            let module = modules.get(&import.source)
                .ok_or_else(|| ElmError::Interpreter { info: RuntimeError::InternalError })?;

            let value = module.definitions.get(&import.source_name)
                .ok_or_else(|| ElmError::Interpreter { info: RuntimeError::InternalError })?;

            self.stack.add(&import.destine_name, value.clone());
        };

        for def in &module.definitions {
            let name = def.name.clone();
            let value = Self::create_function_closure(&mut self.stack, def);

            self.stack.add(&name, value.clone());
            definitions.insert(name, value);
        }

        Ok(RuntimeModule {
            name: module.name.to_string(),
            definitions,
            imports: module.imports.clone(),
        })
    }

    pub fn eval_expr(&mut self, expr: &TypedExpr) -> Result<Value, ElmError> {
        match expr {
            TypedExpr::Ref(_, name) => {
                let opt = self.stack.find(name);
                match opt {
                    Some(val) => Ok(val),
                    None => {
                        Err(ElmError::Interpreter {
                            info: RuntimeError::MissingDefinition(name.clone())
                        })
                    }
                }
            }
            TypedExpr::Const(value) => Ok(value.clone()),
            TypedExpr::Tuple(_, items) => {
                let values = items.iter()
                    .map(|e| self.eval_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Value::Tuple(values))
            }
            TypedExpr::List(_, items) => {
                let values = items.iter()
                    .map(|e| self.eval_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Value::List(values))
            }
            TypedExpr::Record(_, items) => {
                let values = items.iter()
                    .map(|(s, e)| {
                        self.eval_expr(e).map(|e| (s.clone(), e))
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Value::Record(values))
            }
            TypedExpr::RecordUpdate(_, name, items) => {
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
                    Err(ElmError::Interpreter {
                        info: RuntimeError::RecordUpdateOnNonRecord(name.as_ref().clone(), val.clone())
                    })
                }
            }
            TypedExpr::If(_, cond, a, b) => {
                let cond = self.eval_expr(cond)?;

                match &cond {
                    Value::Adt(ref name, ref vals, _) => {
                        if name == "True" && vals.is_empty() {
                            self.eval_expr(a)
                        } else if name == "False" && vals.is_empty() {
                            self.eval_expr(b)
                        } else {
                            Err(ElmError::Interpreter {
                                info: RuntimeError::InvalidIfCondition(cond.clone())
                            })
                        }
                    }
                    _ => {
                        Err(ElmError::Interpreter {
                            info: RuntimeError::InvalidIfCondition(cond.clone())
                        })
                    }
                }
            }
            TypedExpr::Lambda(ty, patt, expr) => {
                Ok(Self::create_lambda_closure(&mut self.stack, ty, patt, expr))
            }
            TypedExpr::RecordField(_, record, field) => {
                let rec = self.eval_expr(record)?;

                if let Value::Record(entries) = &rec {
                    let (_, value) = entries.iter()
                        .find(|(name, _)| name == field)
                        .ok_or(ElmError::Interpreter { info: RuntimeError::RecordFieldNotFound(field.to_owned(), rec.clone()) })?;

                    Ok(value.clone())
                } else {
                    Err(ElmError::Interpreter {
                        info: RuntimeError::ExpectedRecord(rec.clone())
                    })
                }
            }
            TypedExpr::RecordAccess(ty, field) => {
                Ok(Value::Fun {
                    arg_count: 1,
                    args: vec![Value::String(field.to_owned())],
                    fun: Arc::new(Function::External(next_fun_id(), builtin_record_access(), ty.clone())),
                })
            }
            TypedExpr::Case(_, cond, branches) => {
                let cond_val = self.eval_expr(cond)?;
                for (patt, expr) in branches {
                    if matches_pattern(patt, &cond_val) {
                        return self.eval_expr(expr);
                    }
                }

                return Err(ElmError::Interpreter {
                    info: RuntimeError::CaseExpressionNonExhaustive(cond_val, branches.map(|(p, _)| p.clone()))
                });
            }
            TypedExpr::Let(..) => Ok(Value::Unit), // TODO
            TypedExpr::Application(_, fun, input) => {
                let function = self.eval_expr(fun)?;
                let input = self.eval_expr(input)?;
                self.application(function, input)
            }
        }
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
                return Err(ElmError::Interpreter {
                    info: RuntimeError::FunArgumentSizeMismatch(*arg_count, argc)
                });
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
            Err(ElmError::Interpreter {
                info: RuntimeError::ExpectedFunction(fun_value.clone())
            })
        }
    }

    fn exec_fun(&mut self, fun: &Function, args: Vec<Value>) -> Result<Value, ElmError> {
        self.stack.enter_block();
        let res = match fun {
            Function::External(_, func, _) => {
                (func.fun)(self, &args)
                    .map_err(|_| ElmError::Interpreter { info: RuntimeError::BuiltinFunctionError })
            }
            Function::Wrapper(_, func, _) => {
                call_function(func, self, &args)
                    .map_err(|_| ElmError::Interpreter { info: RuntimeError::BuiltinFunctionError })
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
}

fn matches_pattern(pattern: &Pattern, value: &Value) -> bool {
    match pattern {
        Pattern::Var(_) => true,
        Pattern::Wildcard => true,
        Pattern::Alias(pat, _) => matches_pattern(pat, value),
        Pattern::Adt(p_name, p_sub) => {
            if let Value::Adt(v_name, v_sub, _) = value {
                p_name == v_name && p_sub.iter().zip(v_sub).all(|(a, b)| matches_pattern(a, b))
            } else {
                false
            }
        }
        Pattern::Unit => value == &Value::Unit,
        Pattern::Tuple(p_sub) => {
            if let Value::Tuple(v_sub) = value {
                p_sub.iter().zip(v_sub).all(|(a, b)| matches_pattern(a, b))
            } else {
                false
            }
        }
        Pattern::List(p_sub) => {
            if let Value::List(v_sub) = value {
                p_sub.iter().zip(v_sub).all(|(a, b)| matches_pattern(a, b))
            } else {
                false
            }
        }
        Pattern::BinaryOp(op, first, rest) => {
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
        Pattern::Record(fields) => {
            if let Value::Record(entries) = value {
                fields.iter().all(|field_name| {
                    entries.iter().find(|(name, _)| name == field_name).is_some()
                })
            } else {
                false
            }
        }
        Pattern::LitInt(p) => {
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
        Pattern::LitString(p) => {
            if let Value::String(v) = value { p == v } else { false }
        }
        Pattern::LitChar(p) => {
            if let Value::Char(v) = value { *p == *v } else { false }
        }
    }
}

pub fn add_pattern_values(env: &mut Interpreter, pattern: &Pattern, value: Value) -> Result<(), RuntimeError> {
    match pattern {
        Pattern::Var(n) => {
            env.stack.add(&n, value);
        }
        Pattern::Alias(pat, name) => {
            env.stack.add(name, value.clone());
            add_pattern_values(env, pat, value)?;
        }
        Pattern::Record(items) => {
            if let Value::Record(vars) = &value {
                for patt in items {
                    let (name, val) = vars.iter()
                        .find(|(name, _)| name == patt)
                        .ok_or(RuntimeError::RecordFieldNotFound(patt.clone(), value.clone()))?;

                    env.stack.add(name, val.clone());
                }
            } else {
                return Err(RuntimeError::ExpectedRecord(value.clone()));
            }
        }
        Pattern::Adt(_, ref items) => {
            if let Value::Adt(_, vars, _) = &value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val.clone())?;
                }
            } else {
                return Err(RuntimeError::ExpectedAdt(value.clone()));
            }
        }
        Pattern::Tuple(ref items) => {
            if let Value::Tuple(ref vars) = &value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val.clone())?;
                }
            } else {
                return Err(RuntimeError::ExpectedTuple(value.clone()));
            }
        }
        Pattern::List(ref items) => {
            if let Value::List(ref vars) = &value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val.clone())?;
                }
            } else {
                return Err(RuntimeError::ExpectedList(value.clone()));
            }
        }
        Pattern::LitInt(_) => {}
        Pattern::LitString(_) => {}
        Pattern::LitChar(_) => {}
        Pattern::Wildcard => {}
        Pattern::Unit => {}
        Pattern::BinaryOp(ref op, ref a, ref b) => {
            if op == "::" {
                if let Value::List(ref vars) = &value {
                    if vars.len() == 0 {
                        return Err(RuntimeError::ExpectedNonEmptyList(value.clone()));
                    }

                    let first = vars[0].clone();
                    let mut rest: Vec<Value> = Vec::new();
                    for i in 1..vars.len() {
                        rest.push(vars[i].clone());
                    }

                    add_pattern_values(env, a, first)?;
                    add_pattern_values(env, b, Value::List(rest))?;
                } else {
                    return Err(RuntimeError::ExpectedList(value.clone()));
                }
            } else {
                return Err(RuntimeError::UnknownOperatorPattern(op.clone()));
            }
        }
    }

    Ok(())
}
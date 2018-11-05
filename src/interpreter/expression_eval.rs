use analyzer::type_check_expression;
use analyzer::type_of_value;
use interpreter::builtins::builtin_function;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::RuntimeError;
use interpreter::RuntimeError::*;
use std::sync::Arc;
use ast::*;
use types::Function;
use types::FunCall;
use types::Value;
use util::expression_fold::create_expr_tree;
use util::expression_fold::ExprTree;
use util::StringConversion;
use util::VecExt;
use util::qualified_name;
use interpreter::builtins::builtin_record_access;

pub fn eval_expr(env: &mut DynamicEnv, expr: &Expr) -> Result<Value, RuntimeError> {
    let res: Value = match expr {
        Expr::Unit => Value::Unit,
        Expr::Tuple(items) => {
            Value::Tuple(items.iter()
                .map(|e| eval_expr(env, e))
                .collect::<Result<Vec<_>, _>>()?)
        }
        Expr::List(items) => {
            Value::List(items.iter()
                .map(|e| eval_expr(env, e))
                .collect::<Result<Vec<_>, _>>()?)
        }
        Expr::Record(items) => {
            Value::Record(items.iter()
                .map(|(s, e)| eval_expr(env, e).map(|e| (s.clone(), e)))
                .collect::<Result<Vec<_>, _>>()?)
        }
        Expr::RecordUpdate(name, items) => {
            let val = eval_expr(env, &Expr::Ref(name.clone()))?;

            if let Value::Record(values) = &val {
                Value::Record(values.iter().map(|(name, value)| {
                    items.iter()
                        .find(|(_name, _)| name == _name)
                        .and_then(|(nam, expr)| {
                            eval_expr(env, expr).map(|val| (nam.clone(), val)).ok()
                        })
                        .unwrap_or((name.clone(), value.clone()))
                }).collect())
            } else {
                return Err(RuntimeError::RecordUpdateOnNonRecord(name.to_owned(), val.clone()));
            }
        }
        Expr::If(cond, a, b) => {
            let cond = eval_expr(env, cond)?;

            match &cond {
                Value::Adt(ref name, ref vals, _) => {
                    if name == "True" && vals.is_empty() {
                        eval_expr(env, a)?
                    } else if name == "False" && vals.is_empty() {
                        eval_expr(env, b)?
                    } else {
                        return Err(InvalidIfCondition(cond.clone()));
                    }
                }
                _ => return Err(InvalidIfCondition(cond.clone()))
            }
        }
        Expr::Lambda(patt, _expr) => {
            let ty = type_check_expression(&mut env.types, expr)
                .map_err(|e| IncorrectDefType(e))?;

            Value::Fun {
                args: vec![],
                arg_count: patt.len() as u32,
                fun: Arc::new(
                    Function::Expr(env.next_fun_id(), patt.clone(), (&**_expr).clone(), ty)
                ),
            }
        }

        Expr::OpChain(exprs, ops) => {
            let tree = create_expr_tree(exprs, ops)
                .map_err(|e| InvalidExpressionChain(e))?;

            let expr = tree_as_expr(env, &tree);

            eval_expr(env, &expr)?
        }
        Expr::Literal(lit) => {
            match lit {
                Literal::Int(i) => Value::Number(*i),
                Literal::Float(i) => Value::Float(*i),
                Literal::String(i) => Value::String(i.clone()),
                Literal::Char(i) => Value::Char(*i),
            }
        }
        Expr::Ref(name) | Expr::Adt(name) => {
            env.find(name)
                .map(|(val, _)| val)
                .ok_or(MissingDefinition(name.clone(), env.clone()))?
        }
        Expr::QualifiedRef(path, name) => {
            let full_name = qualified_name(path, name);

            let is_adt = name.chars().next().unwrap().is_uppercase();

            let expr = if is_adt {
                Expr::Adt(full_name)
            } else {
                Expr::Ref(full_name)
            };

            eval_expr(env, &expr)?
        }
        Expr::RecordField(record, field) => {
            let rec = eval_expr(env, record)?;
            if let Value::Record(ref entries) = &rec {
                let (_, value) = entries.iter()
                    .find(|(name, _)| name == field)
                    .ok_or(RecordFieldNotFound(field.to_owned(), rec.clone()))?;

                value.clone()
            } else {
                return Err(ExpectedRecord(rec.clone()));
            }
        }
        Expr::RecordAccess(field) => {
            let ty = Type::Fun(
                Box::new(Type::RecExt("b".s(), vec![(field.to_owned(), Type::Var("a".s()))])),
                Box::new(Type::Var("a".s())),
            );

            Value::Fun {
                args: vec![Value::String(field.to_owned())],
                fun: Arc::new(Function::Builtin(env.next_fun_id(), builtin_record_access(), ty)),
                arg_count: 1,
            }
        }
        Expr::Case(cond, branches) => {
            let cond_val = eval_expr(env, cond)?;
            for (patt, expr) in branches {
                if matches_pattern(patt, &cond_val) {
                    return eval_expr(env, expr);
                }
            }

            return Err(CaseExpressionNonExhaustive(cond_val, branches.map(|(p, _)| p.clone())));
        }
        Expr::Let(_, _) => Value::Unit, // TODO

        Expr::Application(fun, input) => {
            let mut fun_value = eval_expr(env, fun)?;
            let input = eval_expr(env, input)?;
            let fun_call = FunCall { function: fun_value.clone(), argument: input.clone() };

            if let Some(val) = env.get_from_cache(&fun_call) {
                return Ok(val.clone());
            }

            match fun_value {
                Value::Fun { ref arg_count, ref args, ref fun } => {
                    let argc = args.len() as u32 + 1;

                    if *arg_count < argc {
                        return Err(FunArgumentSizeMismatch(*arg_count, argc));
                    }

                    let mut arg_vec = args.clone();
                    arg_vec.push(input);

                    let value = if *arg_count == argc {
                        exec_fun(env, fun, &arg_vec)?
                    } else {
                        Value::Fun { args: arg_vec, fun: fun.clone(), arg_count: *arg_count }
                    };

                    env.add_to_cache(fun_call, value.clone());
                    value
                },
                _ => {
                    return Err(ExpectedFunction(fun_value.clone()));
                }
            }
        }
    };

    Ok(res)
}

fn exec_fun(env: &mut DynamicEnv, fun: &Function, args: &Vec<Value>) -> Result<Value, RuntimeError> {
    match fun {
        Function::Builtin(_, func, _) => {
            func.call_function(args).map_err(|_| RuntimeError::BuiltinFunctionError)
        }
        Function::Expr(_, ref patterns, ref expr, _) => {
            env.enter_block();
            assert_eq!(patterns.len(), args.len());

            for (patt, val) in patterns.iter().zip(args) {
                add_pattern_values(env, patt, val).unwrap();
            }

            let res = eval_expr(env, expr);
            env.exit_block();
            Ok(res?)
        }
    }
}

fn matches_pattern(pattern: &Pattern, value: &Value) -> bool {
    match pattern {
        Pattern::Var(_) => true,
        Pattern::Wildcard => true,
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
        Pattern::Literal(lit) => {
            match lit {
                Literal::Int(p) => {
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
                Literal::Float(p) => {
                    if let Value::Float(v) = value { *p == *v } else { false }
                }
                Literal::String(p) => {
                    if let Value::String(v) = value { p == v } else { false }
                }
                Literal::Char(p) => {
                    if let Value::Char(v) = value { *p == *v } else { false }
                }
            }
        }
    }
}

pub fn add_pattern_values(env: &mut DynamicEnv, pattern: &Pattern, value: &Value) -> Result<(), RuntimeError> {
    match pattern {
        Pattern::Var(n) => {
            env.add(&n, value.clone(), type_of_value(value));
        }
        Pattern::Record(ref items) => {
            if let Value::Record(ref vars) = &value {
                for patt in items {
                    let (name, val) = vars.iter()
                        .find(|(name, _)| name == patt)
                        .ok_or(RecordFieldNotFound(patt.clone(), value.clone()))?;

                    env.add(name, val.clone(), type_of_value(val));
                }
            } else {
                return Err(ExpectedRecord(value.clone()));
            }
        }
        Pattern::Adt(_, ref items) => {
            if let Value::Adt(_, ref vars, _) = &value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val)?;
                }
            } else {
                return Err(ExpectedAdt(value.clone()));
            }
        }
        Pattern::Tuple(ref items) => {
            if let Value::Tuple(ref vars) = &value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val)?;
                }
            } else {
                return Err(ExpectedTuple(value.clone()));
            }
        }
        Pattern::List(ref items) => {
            if let Value::List(ref vars) = &value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val)?;
                }
            } else {
                return Err(ExpectedList(value.clone()));
            }
        }
        Pattern::Literal(_) => {}
        Pattern::Wildcard => {}
        Pattern::Unit => {}
        Pattern::BinaryOp(ref op, ref a, ref b) => {
            if op == "::" {
                if let Value::List(ref vars) = &value {
                    if vars.len() == 0 {
                        return Err(ExpectedNonEmptyList(value.clone()));
                    }

                    let first = vars[0].clone();
                    let mut rest: Vec<Value> = Vec::new();
                    for i in 1..vars.len() {
                        rest.push(vars[i].clone());
                    }

                    add_pattern_values(env, a, &first)?;
                    add_pattern_values(env, b, &Value::List(rest))?;
                } else {
                    return Err(ExpectedList(value.clone()));
                }
            } else {
                return Err(UnknownOperatorPattern(op.clone()));
            }
        }
    }

    Ok(())
}

fn tree_as_expr(env: &mut DynamicEnv, expr: &ExprTree) -> Expr {
    match expr {
        ExprTree::Leaf(e) => e.clone(),
        ExprTree::Branch(op, a, b) => {
            let op_fun = Expr::Ref(op.clone());
            let a_branch = tree_as_expr(env, &**a);
            let b_branch = tree_as_expr(env, &**b);

            Expr::Application(
                Box::new(
                    Expr::Application(
                        Box::new(op_fun),
                        Box::new(a_branch),
                    )
                ),
                Box::new(b_branch),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use parsers::from_code;
    use super::*;
    use ast::Pattern;
    use ast::Type;
    use util::builtin_fun_of;
    use interpreter::builtins::builtin_unit_fun;

    #[test]
    fn check_unit() {
        let expr = from_code(b"()");
        let mut env = DynamicEnv::new();

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Unit));
    }

    #[test]
    fn check_list() {
        let expr = from_code(b"[1, 2, 3]");
        let mut env = DynamicEnv::new();

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::List(vec![
            Value::Number(1),
            Value::Number(2),
            Value::Number(3),
        ])));
    }

    #[test]
    fn check_lambda() {
        let expr = from_code(b"\\x -> 1");
        let mut env = DynamicEnv::new();

        assert_eq!(eval_expr(&mut env, &expr), Ok(
            Value::Fun {
                arg_count: 1,
                args: vec![],
                fun: Arc::new(Function::Expr(
                    0,
                    vec![Pattern::Var("x".s())],
                    Expr::Literal(Literal::Int(1)),
                    Type::Fun(
                        Box::new(Type::Var("a".s())),
                        Box::new(Type::Var("number".s())),
                    ),
                )),
            }
        ));
    }

    #[test]
    fn check_record() {
        let expr = from_code(b"{ a = 0 }.a");
        let mut env = DynamicEnv::new();

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Number(0)));
    }

    #[test]
    fn check_application() {
        let expr = from_code(b"fun 0");
        let mut env = DynamicEnv::new();

        let ty = Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Unit),
        );

        let fun = builtin_fun_of(env.next_fun_id(), builtin_unit_fun(), ty.clone());
        env.add("fun", fun, ty);

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Unit));
    }

    #[test]
    fn check_number() {
        let expr = from_code(b"1 / 3");
        let mut env = DynamicEnv::default_lang_env();

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Float(0.3333333333333333)));
    }

    #[test]
    fn check_number2() {
        let expr = from_code(b"4 // 3");
        let mut env = DynamicEnv::default_lang_env();

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Int(1)));
    }

    #[test]
    fn check_number3() {
        let expr = from_code(b"4 + 3");
        let mut env = DynamicEnv::default_lang_env();


        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Number(7)));
    }
}
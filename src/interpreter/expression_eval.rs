use std::collections::HashMap;
use std::sync::Arc;

use analyzer::type_check_expression;
use analyzer::type_of_value;
use ast::*;
use errors::*;
use errors::RuntimeError::*;
use Interpreter;
use interpreter::builtins::builtin_record_access;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::statement_eval::extract_captures;
use rust_interop::call_function;
use types::FunCall;
use types::Function;
use types::next_fun_id;
use types::Value;
use util::expression_fold::create_expr_tree;
use util::expression_fold::ExprTree;
use util::qualified_name;
use util::StringConversion;
use util::VecExt;

pub fn eval_expr(env: &mut DynamicEnv, expr: &Expr) -> Result<Value, RuntimeError> {
    let res: Value = match expr {
        Expr::Unit(..) => Value::Unit,
        Expr::Tuple(_, items) => {
            Value::Tuple(items.iter()
                .map(|e| eval_expr(env, e))
                .collect::<Result<Vec<_>, _>>()?)
        }
        Expr::List(_, items) => {
            Value::List(items.iter()
                .map(|e| eval_expr(env, e))
                .collect::<Result<Vec<_>, _>>()?)
        }
        Expr::Record(_, items) => {
            Value::Record(items.iter()
                .map(|(s, e)| eval_expr(env, e).map(|e| (s.clone(), e)))
                .collect::<Result<Vec<_>, _>>()?)
        }
        Expr::RecordUpdate(_, name, items) => {
            let val = eval_expr(env, &Expr::Ref((0, 0), name.clone()))?;

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
        Expr::If(_, cond, a, b) => {
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
        Expr::Lambda(_, patt, _expr) => {
            let ty = type_check_expression(&mut env.types, expr)
                .map_err(|e| IncorrectDefType(e))?;

            Value::Fun {
                args: vec![],
                arg_count: patt.len() as u32,
                captures: extract_captures(env, &**_expr),
                fun: Arc::new(
                    Function::Expr(next_fun_id(), patt.clone(), (&**_expr).clone(), ty)
                ),
            }
        }
        Expr::OpChain(_, exprs, ops) => {
            let tree = create_expr_tree(exprs, ops)
                .map_err(|e| InvalidExpressionChain(e))?;

            let expr = tree_as_expr(env, &tree);

            eval_expr(env, &expr)?
        }
        Expr::Literal(_, lit) => {
            match lit {
                Literal::Int(i) => Value::Number(*i),
                Literal::Float(i) => Value::Float(*i),
                Literal::String(i) => Value::String(i.clone()),
                Literal::Char(i) => Value::Char(*i),
            }
        }
        Expr::Ref(_, name) => {
            env.find(name)
                .map(|(val, _)| val)
                .ok_or(MissingDefinition(name.clone(), env.clone()))?
        }
        Expr::QualifiedRef(_, path, name) => {
            let full_name = qualified_name(path, name);
            env.find(&full_name)
                .map(|(val, _)| val)
                .ok_or(MissingDefinition(name.clone(), env.clone()))?
        }
        Expr::RecordField(_, record, field) => {
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
        Expr::RecordAccess(_, field) => {
            let ty = Type::Fun(
                Box::new(Type::RecExt("b".s(), vec![(field.to_owned(), Type::Var("a".s()))])),
                Box::new(Type::Var("a".s())),
            );

            Value::Fun {
                arg_count: 1,
                args: vec![Value::String(field.to_owned())],
                captures: HashMap::new(),
                fun: Arc::new(Function::External(next_fun_id(), builtin_record_access(), ty)),
            }
        }
        Expr::Case(_, cond, branches) => {
            let cond_val = eval_expr(env, cond)?;
            for (patt, expr) in branches {
                if matches_pattern(patt, &cond_val) {
                    return eval_expr(env, expr);
                }
            }

            return Err(CaseExpressionNonExhaustive(cond_val, branches.map(|(p, _)| p.clone())));
        }
        Expr::Let(..) => Value::Unit, // TODO
        Expr::Application(_, fun, input) => {
            let mut fun_value = eval_expr(env, fun)?;
            let input = eval_expr(env, input)?;
            let fun_call = FunCall { function: fun_value.clone(), argument: input.clone() };

            if let Some(val) = env.get_from_cache(&fun_call) {
                return Ok(val.clone());
            }

            match fun_value {
                Value::Fun { ref arg_count, ref args, ref captures, ref fun } => {
                    let argc = args.len() as u32 + 1;

                    if *arg_count < argc {
                        return Err(FunArgumentSizeMismatch(*arg_count, argc));
                    }

                    let mut arg_vec = args.clone();
                    arg_vec.push(input);

                    let value = if *arg_count == argc {
                        exec_fun(env, fun, captures, &arg_vec)?
                    } else {
                        Value::Fun { args: arg_vec, fun: fun.clone(), captures: captures.clone(), arg_count: *arg_count }
                    };

                    env.add_to_cache(fun_call, value.clone());
                    value
                }
                _ => {
                    return Err(ExpectedFunction(fun_value.clone()));
                }
            }
        }
    };

    Ok(res)
}

fn exec_fun(env: &mut DynamicEnv, fun: &Function, captures: &HashMap<String, Value>, args: &Vec<Value>) -> Result<Value, RuntimeError> {
    env.enter_block();
    for (name, val) in captures {
        env.add(name, val.clone(), type_of_value(val))
    }
    let res = match fun {
        Function::External(_, func, _) => {
            (func.fun)(&mut Interpreter::wrap(env), args)
                .map_err(|_| RuntimeError::BuiltinFunctionError)
        }
        Function::Wrapper(_, func, _) => {
            call_function(func, &mut Interpreter::wrap(env), args)
                .map_err(|_| RuntimeError::BuiltinFunctionError)
        }
        Function::Expr(_, ref patterns, ref expr, _) => {
            assert_eq!(patterns.len(), args.len());

            for (patt, val) in patterns.iter().zip(args) {
                add_pattern_values(env, patt, val).unwrap();
            }

            eval_expr(env, expr)
        }
    };
    env.exit_block();
    Ok(res?)
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

pub fn add_pattern_values(env: &mut DynamicEnv, pattern: &Pattern, value: &Value) -> Result<(), RuntimeError> {
    match pattern {
        Pattern::Var(n) => {
            env.add(&n, value.clone(), type_of_value(value));
        }
        Pattern::Alias(pat, name) => {
            env.add(name, value.clone(), type_of_value(value));
            add_pattern_values(env, pat, value)?;
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
        Pattern::LitInt(_) => {}
        Pattern::LitString(_) => {}
        Pattern::LitChar(_) => {}
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
            let a_branch = tree_as_expr(env, &**a);
            let b_branch = tree_as_expr(env, &**b);
            let span = (span(&a_branch).0, span(&b_branch).1);
            let op_fun = Expr::Ref(span, op.clone());

            Expr::Application(
                span,
                Box::new(
                    Expr::Application(
                        span,
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
    use ast::Pattern;
    use ast::Type;
    use interpreter::builtins::builtin_unit_fun;
    use parsers::from_code;
    use util::builtin_fun_of;

    use super::*;

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
                captures: HashMap::new(),
                fun: Arc::new(Function::Expr(
                    1,
                    vec![Pattern::Var("x".s())],
                    Expr::Literal((6, 7), Literal::Int(1)),
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

        let fun = builtin_fun_of(builtin_unit_fun(), ty.clone());
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
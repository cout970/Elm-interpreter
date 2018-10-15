use analyzer::type_check_expression;
use interpreter::builtins::builtin_function;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::RuntimeError;
use interpreter::RuntimeError::*;
use std::collections::HashMap;
use types::Expr;
use types::Fun;
use types::Literal;
use types::Pattern;
use types::Type;
use types::Value;
use util::expression_fold::create_expr_tree;
use util::expression_fold::ExprTree;
use util::StringConversion;
use types::FunCall;

pub fn eval_expr(env: &mut DynamicEnv, expr: &Expr) -> Result<Value, RuntimeError> {
    env.eval_calls += 1;
    println!("eval  : {}", expr);
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

            if let Value::Record(values) = val {
                Value::Record(values.iter().map(|(name, value)| {
                    items.iter()
                        .find(|(_name, _)| name == _name)
                        .and_then(|(nam, expr)| {
                            eval_expr(env, expr).map(|val| (nam.clone(), val)).ok()
                        })
                        .unwrap_or((name.clone(), value.clone()))
                }).collect())
            } else {
                return Err(TODO(format!("Not a record: {}", name)));
            }
        }
        Expr::If(cond, a, b) => {
            let cond = eval_expr(env, cond)?;

            match cond {
                Value::Adt(name, vals, _) => {
                    if name == "True" && vals.is_empty() {
                        eval_expr(env, a)?
                    } else if name == "False" && vals.is_empty() {
                        eval_expr(env, b)?
                    } else {
                        return Err(TODO(format!("Invalid If condition: {}", name)));
                    }
                }
                _ => return Err(TODO(format!("Invalid If condition")))
            }
        }
        Expr::Lambda(patt, _expr) => {
            let ty = type_check_expression(&mut env.types, expr).map_err(|it| TODO(format!("{:?}", it)))?;

            Value::Fun {
                args: vec![],
                arg_count: patt.len() as u32,
                fun: Fun::Expr(env.next_fun_id(), patt.clone(), (&**_expr).clone(), ty),
            }
        }

        Expr::OpChain(exprs, ops) => {
            let tree = create_expr_tree(exprs, ops).map_err(|e| TODO(format!("{:?}", e)))?;
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
                .ok_or(MissingDef(name.clone(), env.clone()))?
        }
        Expr::QualifiedRef(_, name) => {
            // TODO
            eval_expr(env, &Expr::Ref(name.clone()))?
        }
        Expr::RecordField(record, field) => {
            let rec = eval_expr(env, record)?;
            if let Value::Record(ref entries) = rec {
                entries.iter()
                    .find(|(name, _)| name == field)
                    .ok_or(TODO(format!("Missing field with name {} in {}", field, rec)))?
                    .1.clone()
            } else {
                return Err(TODO(format!("Expected record but found {}", rec)));
            }
        }
        Expr::RecordAccess(field) => {
            let ty = Type::Fun(
                Box::new(Type::RecExt("b".s(), vec![(field.to_owned(), Type::Var("a".s()))])),
                Box::new(Type::Var("a".s())),
            );

            Value::Fun {
                args: vec![Value::String(field.to_owned())],
                fun: Fun::Builtin(env.next_fun_id(), 6, ty),
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

            return Err(TODO(format!("case values does not match any branch: {}", cond_val)));
        }
        Expr::Let(_, _) => Value::Unit, // TODO

        Expr::Application(fun, input) => {
            let fun = eval_expr(env, fun)?;
            let input = eval_expr(env, input)?;
            let fun_call = FunCall { function: fun.clone(), argument: input.clone() };

            if let Some(val) = env.get_from_cache(&fun_call) {
                println!("eval (cached): {:?} = {}", fun_call, val);
                return Ok(val.clone());
            }

            if let Value::Fun { arg_count, args, fun } = &fun {
                let argc = args.len() as u32 + 1;

                if *arg_count < argc {
                    return Err(TODO(format!("To much arguments, expected: {}, found: {}", arg_count, argc)));
                }

                let mut arg_vec = args.clone();
                arg_vec.push(input);

                let value = if *arg_count == argc {
                    exec_fun(env, &fun, &arg_vec)?
                } else {
                    Value::Fun { args: arg_vec, fun: fun.clone(), arg_count: *arg_count }
                };

                env.add_to_cache(fun_call, value.clone());
                value
            } else {
                return Err(TODO(format!("Expected a function but found: {}", fun)));
            }
        }
    };

    Ok(res)
}

fn exec_fun(env: &mut DynamicEnv, fun: &Fun, args: &[Value]) -> Result<Value, RuntimeError> {
    match fun {
        Fun::Builtin(_, id, _) => {
            builtin_function(*id, args)
        }
        Fun::Expr(_, ref patterns, ref expr, _) => {
//            println!("exec: {:?}", fun);
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
            env.add(&n, value.clone(), get_value_type(value));
        }
        Pattern::Record(ref items) => {
            if let Value::Record(vars) = value {
                for patt in items {
                    let (name, val) = vars.iter()
                        .find(|(name, _)| name == patt)
                        .ok_or(TODO(format!("Unable to find field {} in {}", patt, value)))?;

                    env.add(name, val.clone(), get_value_type(val));
                }
            } else {
                return Err(TODO(format!("Expected Record but found: {}", value)));
            }
        }
        Pattern::Adt(_, ref items) => {
            if let Value::Adt(_, vars, _) = value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val)?;
                }
            } else {
                return Err(TODO(format!("Expected Adt but found: {}", value)));
            }
        }
        Pattern::Tuple(ref items) => {
            if let Value::Tuple(vars) = value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val)?;
                }
            } else {
                return Err(TODO(format!("Expected Tuple but found: {}", value)));
            }
        }
        Pattern::List(ref items) => {
            if let Value::List(vars) = value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val)?;
                }
            } else {
                return Err(TODO(format!("Expected List but found: {}", value)));
            }
        }
        Pattern::Literal(_) => {}
        Pattern::Wildcard => {}
        Pattern::Unit => {}
        Pattern::BinaryOp(op, ref a, ref b) => {
            if op == "::" {
                if let Value::List(vars) = value {
                    if vars.len() == 0 {
                        return Err(TODO(format!("Expected Non Empty List but it was empty")));
                    }

                    let first = vars[0].clone();
                    let mut rest: Vec<Value> = Vec::new();
                    for i in 1..vars.len() {
                        rest.push(vars[i].clone());
                    }

                    add_pattern_values(env, a, &first)?;
                    add_pattern_values(env, b, &Value::List(rest))?;
                } else {
                    return Err(TODO(format!("Expected List but found: {}", value)));
                }
            } else {
                return Err(TODO(format!("Unknown operator pattern '{}'", op)));
            }
        }
    }

    Ok(())
}

pub fn get_value_type(value: &Value) -> Type {
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
                Type::Tag("List".s(), vec![get_value_type(items.first().unwrap())])
            }
        }
        Value::Tuple(items) => {
            Type::Tuple(items.iter().map(|i| get_value_type(i)).collect())
        }
        Value::Record(items) => {
            Type::Record(items.iter().map(|(s, i)| (s.to_owned(), get_value_type(i))).collect())
        }
        Value::Adt(_, items, ty) => {
            Type::Tag(ty.to_owned(), items.iter().map(|i| get_value_type(i)).collect())
        }
        Value::Fun { fun, args, .. } => {
            let fun_ty = match fun {
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
    use nom::*;
    use nom::verbose_errors::*;
    use parsers::from_code;
    use super::*;
    use tokenizer::tokenize;
    use types::Pattern;
    use types::Type;
    use util::builtin_fun_of;

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
                fun: Fun::Expr(
                    0,
                    vec![Pattern::Var("x".s())],
                    Expr::Literal(Literal::Int(1)),
                    Type::Fun(
                        Box::new(Type::Var("a".s())),
                        Box::new(Type::Var("number".s())),
                    ),
                ),
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

        let fun = builtin_fun_of(env.next_fun_id(),0, ty.clone());
        env.add("fun", fun, ty);

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Unit));
    }

    #[test]
    fn check_number() {
        let expr = from_code(b" 1 / 3");
        let mut env = DynamicEnv::new();

        let ty = Type::Fun(
            Box::new(Type::Tag("Float".s(), vec![])),
            Box::new(Type::Fun(
                Box::new(Type::Tag("Float".s(), vec![])),
                Box::new(Type::Tag("Float".s(), vec![])),
            )),
        );

        let fun = builtin_fun_of(env.next_fun_id(),4, ty.clone());
        env.add("/", fun, ty);

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Float(0.3333333333333333)));
    }

    #[test]
    fn check_number2() {
        let expr = from_code(b" 4 // 3");
        let mut env = DynamicEnv::new();

        let ty = Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Fun(
                Box::new(Type::Tag("Int".s(), vec![])),
                Box::new(Type::Tag("Int".s(), vec![])),
            )),
        );

        let fun = builtin_fun_of(env.next_fun_id(),5, ty.clone());
        env.add("//", fun, ty);

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Int(1)));
    }

    #[test]
    fn check_number3() {
        let expr = from_code(b" 4 + 3");
        let mut env = DynamicEnv::new();

        let ty = Type::Fun(
            Box::new(Type::Var("number".s())),
            Box::new(Type::Fun(
                Box::new(Type::Var("number".s())),
                Box::new(Type::Var("number".s())),
            )),
        );

        let fun = builtin_fun_of(env.next_fun_id(),1, ty.clone());
        env.add("+", fun, ty);

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Number(7)));
    }
}
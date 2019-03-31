use std::collections::HashMap;
use std::sync::Arc;

use ast::*;
use constructors::type_unit;
use errors::*;
use errors::RuntimeError::*;
use interpreter::builtins::builtin_record_access;
use interpreter::dynamic_env::DynamicEnv;
use Runtime;
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
    unimplemented!()
//    let res: Value = match expr {
//        Expr::Unit(..) => Value::Unit,
//        Expr::Tuple(_, items) => {
//            Value::Tuple(items.iter()
//                .map(|e| eval_expr(env, e))
//                .collect::<Result<Vec<_>, _>>()?)
//        }
//        Expr::List(_, items) => {
//            Value::List(items.iter()
//                .map(|e| eval_expr(env, e))
//                .collect::<Result<Vec<_>, _>>()?)
//        }
//        Expr::Record(_, items) => {
//            Value::Record(items.iter()
//                .map(|(s, e)| eval_expr(env, e).map(|e| (s.clone(), e)))
//                .collect::<Result<Vec<_>, _>>()?)
//        }
//        Expr::RecordUpdate(_, name, items) => {
//            let val = eval_expr(env, &Expr::Ref((0, 0), name.clone()))?;
//
//            if let Value::Record(values) = &val {
//                Value::Record(values.iter().map(|(name, value)| {
//                    items.iter()
//                        .find(|(_name, _)| name == _name)
//                        .and_then(|(nam, expr)| {
//                            eval_expr(env, expr).map(|val| (nam.clone(), val)).ok()
//                        })
//                        .unwrap_or((name.clone(), value.clone()))
//                }).collect())
//            } else {
//                return Err(RuntimeError::RecordUpdateOnNonRecord(name.to_owned(), val.clone()));
//            }
//        }
//        Expr::If(_, cond, a, b) => {
//            let cond = eval_expr(env, cond)?;
//
//            match &cond {
//                Value::Adt(ref name, ref vals, _) => {
//                    if name == "True" && vals.is_empty() {
//                        eval_expr(env, a)?
//                    } else if name == "False" && vals.is_empty() {
//                        eval_expr(env, b)?
//                    } else {
//                        return Err(InvalidIfCondition(cond.clone()));
//                    }
//                }
//                _ => return Err(InvalidIfCondition(cond.clone()))
//            }
//        }
//        Expr::Lambda(_, patt, _expr) => {
////            let ty = type_check_expression(&mut env.types, expr)
////                .map_err(|e| IncorrectDefType(e))?;
//
//            // TODO replace hole interpreter
//            let ty = type_unit();
//
//            Value::Fun {
//                args: vec![],
//                arg_count: patt.len() as u32,
//                captures: extract_captures(env, &**_expr),
//                fun: Arc::new(
//                    Function::Definition(next_fun_id(), patt.clone(), (&**_expr).clone(), ty)
//                ),
//            }
//        }
//        Expr::OpChain(_, exprs, ops) => {
//            let tree = create_expr_tree(exprs, ops)
//                .map_err(|e| InvalidExpressionChain(e))?;
//
//            let expr = tree_as_expr(env, &tree);
//
//            eval_expr(env, &expr)?
//        }
//        Expr::Literal(_, lit) => {
//            match lit {
//                Literal::Int(i) => Value::Number(*i),
//                Literal::Float(i) => Value::Float(*i),
//                Literal::String(i) => Value::String(i.clone()),
//                Literal::Char(i) => Value::Char(*i),
//            }
//        }
//        Expr::Ref(_, name) => {
//            env.find(name)
//                .map(|(val, _)| val)
//                .ok_or(MissingDefinition(name.clone(), env.clone()))?
//        }
//        Expr::QualifiedRef(_, path, name) => {
//            let full_name = qualified_name(path, name);
//            env.find(&full_name)
//                .map(|(val, _)| val)
//                .ok_or(MissingDefinition(name.clone(), env.clone()))?
//        }
//        Expr::RecordField(_, record, field) => {
//            let rec = eval_expr(env, record)?;
//            if let Value::Record(ref entries) = &rec {
//                let (_, value) = entries.iter()
//                    .find(|(name, _)| name == field)
//                    .ok_or(RecordFieldNotFound(field.to_owned(), rec.clone()))?;
//
//                value.clone()
//            } else {
//                return Err(ExpectedRecord(rec.clone()));
//            }
//        }
//        Expr::RecordAccess(_, field) => {
//            let ty = Type::Fun(
//                Box::new(Type::RecExt("b".s(), vec![(field.to_owned(), Type::Var("a".s()))])),
//                Box::new(Type::Var("a".s())),
//            );
//
//            Value::Fun {
//                arg_count: 1,
//                args: vec![Value::String(field.to_owned())],
//                captures: HashMap::new(),
//                fun: Arc::new(Function::External(next_fun_id(), builtin_record_access(), ty)),
//            }
//        }
//        Expr::Case(_, cond, branches) => {
//            let cond_val = eval_expr(env, cond)?;
//            for (patt, expr) in branches {
//                if matches_pattern(patt, &cond_val) {
//                    return eval_expr(env, expr);
//                }
//            }
//
//            return Err(CaseExpressionNonExhaustive(cond_val, branches.map(|(p, _)| p.clone())));
//        }
//        Expr::Let(..) => Value::Unit, // TODO
//        Expr::Application(_, fun, input) => {
//            let mut fun_value = eval_expr(env, fun)?;
//            let input = eval_expr(env, input)?;
//            let fun_call = FunCall { function: fun_value.clone(), argument: input.clone() };
//
//            if let Some(val) = env.get_from_cache(&fun_call) {
//                return Ok(val.clone());
//            }
//
//            match fun_value {
//                Value::Fun { ref arg_count, ref args, ref captures, ref fun } => {
//                    let argc = args.len() as u32 + 1;
//
//                    if *arg_count < argc {
//                        return Err(FunArgumentSizeMismatch(*arg_count, argc));
//                    }
//
//                    let mut arg_vec = args.clone();
//                    arg_vec.push(input);
//
//                    let value = if *arg_count == argc {
//                        exec_fun(env, fun, captures, &arg_vec)?
//                    } else {
//                        Value::Fun { args: arg_vec, fun: fun.clone(), captures: captures.clone(), arg_count: *arg_count }
//                    };
//
//                    env.add_to_cache(fun_call, value.clone());
//                    value
//                }
//                _ => {
//                    return Err(ExpectedFunction(fun_value.clone()));
//                }
//            }
//        }
//    };
//
//    Ok(res)
}



#[cfg(test)]
mod tests {
    use ast::Pattern;
    use ast::Type;
    use interpreter::builtins::builtin_unit_fun;
    use test_utils::Test;
    use util::builtin_fun_of;

    use super::*;

    #[test]
    fn check_unit() {
        let expr = Test::expr("()");
        let mut env = DynamicEnv::new();

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Unit));
    }

    #[test]
    fn check_list() {
        let expr = Test::expr("[1, 2, 3]");
        let mut env = DynamicEnv::new();

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::List(vec![
            Value::Number(1),
            Value::Number(2),
            Value::Number(3),
        ])));
    }

    #[test]
    fn check_lambda() {
        let expr = Test::expr("\\x -> 1");
        let mut env = DynamicEnv::new();

        let value = eval_expr(&mut env, &expr).unwrap();
        match value {
            Value::Fun { args, fun, .. } => {
                assert_eq!(args, vec![]);
                // TODO
            }
            _ => panic!("Not a function: {}", value)
        }
    }

    #[test]
    fn check_record() {
        let expr = Test::expr("{ a = 0 }.a");
        let mut env = DynamicEnv::new();

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Number(0)));
    }

    #[test]
    fn check_application() {
        let expr = Test::expr("fun 0");
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
        let expr = Test::expr("1 / 3");
        let mut env = DynamicEnv::default_lang_env();

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Float(0.3333333333333333)));
    }

    #[test]
    fn check_number2() {
        let expr = Test::expr("4 // 3");
        let mut env = DynamicEnv::default_lang_env();

        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Int(1)));
    }

    #[test]
    fn check_number3() {
        let expr = Test::expr("4 + 3");
        let mut env = DynamicEnv::default_lang_env();


        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Number(7)));
    }
}
use std::collections::HashMap;
use std::sync::Arc;

use ast::*;
use constructors::type_unit;
use errors::*;
use errors::RuntimeError::*;
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
}


#[cfg(test)]
mod tests {
    use ast::Pattern;
    use ast::Type;
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

//    #[test]
//    fn check_application() {
//        let expr = Test::expr("fun 0");
//        let mut env = DynamicEnv::new();
//
//        let ty = Type::Fun(
//            Box::new(Type::Tag("Int".s(), vec![])),
//            Box::new(Type::Unit),
//        );
//
//        let fun = builtin_fun_of(builtin_unit_fun(), ty.clone());
//        env.add("fun", fun, ty);
//
//        assert_eq!(eval_expr(&mut env, &expr), Ok(Value::Unit));
//    }

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
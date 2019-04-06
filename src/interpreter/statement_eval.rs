use std::collections::HashMap;
use std::sync::Arc;

use ast::*;
use ast::Definition;
use constructors::type_unit;
use errors::*;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::expression_eval::eval_expr;
use types::Adt;
use types::AdtVariant;
use types::Function;
use types::next_fun_id;
use types::Value;
use util::build_fun_type;
use util::create_vec_inv;
use util::qualified_name;

pub fn eval_stm(env: &mut DynamicEnv, stm: &Statement) -> Result<Option<Value>, RuntimeError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use test_utils::Test;
    use util::StringConversion;

    use super::*;

    fn formatted(env: &mut DynamicEnv, stm: &Statement) -> String {
        let result = eval_stm(env, stm);
        let option = result.unwrap();
        let value = option.unwrap();
        let ty = value.get_type();

        format!("{} : {}", value, ty)
    }

    fn formatted_expr(env: &mut DynamicEnv, expr: &Expr) -> String {
        let result = eval_expr(env, expr);
        let value = result.unwrap();
        let ty = value.get_type();

        format!("{} : {}", value, ty)
    }

    #[test]
    fn check_constant() {
        let stm = Test::statement("x = 1");
        let mut env = DynamicEnv::new();

        assert_eq!(formatted(&mut env, &stm), "1 : number".s());
    }

    #[test]
    fn check_identity() {
        let stm = Test::statement("id value = value");
        let mut env = DynamicEnv::new();

        assert_eq!(formatted(&mut env, &stm), "<function> : a -> a".s());
    }

    #[test]
    fn check_recursivity() {
        let stm = Test::statement("fib num = case num of \n 0 -> 0\n 1 -> 1\n _ -> fib (num - 1) + fib (num - 2)");
        let mut env = DynamicEnv::default_lang_env();

        assert_eq!(formatted(&mut env, &stm), "<function> : Int -> number".s());
    }

    #[test]
    fn check_adt() {
        let decl = Test::statement("type Adt = A | B");
        let mut env = DynamicEnv::default_lang_env();

        eval_stm(&mut env, &decl).unwrap();

        assert_eq!(formatted_expr(&mut env, &Test::expr("A")), "A : Adt".s());
        assert_eq!(formatted_expr(&mut env, &Test::expr("B")), "B : Adt".s());
    }

    #[test]
    fn check_adt2() {
        let decl = Test::statement("type Adt a = A a | B Int");
        let mut env = DynamicEnv::default_lang_env();

        eval_stm(&mut env, &decl).unwrap();

        assert_eq!(formatted_expr(&mut env, &Test::expr("A")), "<function> : a -> Adt a".s());
        assert_eq!(formatted_expr(&mut env, &Test::expr("B")), "<function> : Int -> Adt a".s());
        assert_eq!(formatted_expr(&mut env, &Test::expr("A 1")), "A 1 : Adt number".s());
        assert_eq!(formatted_expr(&mut env, &Test::expr("B 1")), "B 1 : Adt a".s());
    }

    #[test]
    fn check_fib() {
        let decl = Test::statement("fib num = case num of \n0 -> 0 \n1 -> 1 \n_ -> fib (num - 1) + fib (num - 2)");
        let mut env = DynamicEnv::default_lang_env();

        eval_stm(&mut env, &decl).unwrap();

        assert_eq!(formatted_expr(&mut env, &Test::expr("fib")), "<function> : Int -> number".s());
        assert_eq!(formatted_expr(&mut env, &Test::expr("fib 0")), "0 : number".s());
        assert_eq!(formatted_expr(&mut env, &Test::expr("fib 1")), "1 : number".s());
        assert_eq!(formatted_expr(&mut env, &Test::expr("fib 2")), "1 : number".s());
        assert_eq!(formatted_expr(&mut env, &Test::expr("fib 3")), "2 : number".s());
        assert_eq!(formatted_expr(&mut env, &Test::expr("fib 4")), "3 : number".s());
        assert_eq!(formatted_expr(&mut env, &Test::expr("fib 5")), "5 : number".s());
    }
}
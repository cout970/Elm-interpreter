use std::collections::HashMap;
use types::Expr;
use types::Literal;
use types::Type;
use util::*;
use interpreter::TypeAnalyzeResult::Success;

enum TypeAnalyzeResult {
    Success(Type),
    Incomplete { requires: String },
    Error(TypeError),
}

enum TypeError {
    ListNotHomogeneous(String),
    IfWithNonBoolCondition(String),
}

fn get_type(env: &StaticEnv, expr: Expr) -> TypeAnalyzeResult {
    match expr {
        Expr::Unit => {
            Success(Type::Unit)
        }
        Expr::Literal(lit) => {
            let name = match lit {
                Literal::Int(_) => "Int".s(),
                Literal::Float(_) => "Float".s(),
                Literal::Char(_) => "Char".s(),
                Literal::String(_) => "String".s(),
            };
            Success(Type::Tag(name, vec![]))
        }
        Expr::Adt(name) => {
            env.adts.get(&name).map(|c| c.clone())
        }
        Expr::Application(i, _) => {
            if let Some(Type::Fun(_, out)) = get_type(env, *i) {
                Some(*out)
            } else {
                None
            }
        }
        Expr::Ref(name) => {
            env.defs.get(&name).map(|c| c.clone())
        }
        Expr::If(cond, a, b) => {
            let types = (get_type(env, *cond), get_type(env, *a), get_type(env, *b));

            if let (Some(Type::Tag(n, _)), Some(a), Some(b)) = types {
                if n == "Bool" && a == b { Success(a) } else { None }
            } else {
                None
            }
        }
        Expr::Lambda(_patterns, expr) => {
            let out = get_type(env, *expr)?;

            Success(Type::Fun(
                Box::new(Type::Var("x".s())), // TODO
                Box::new(out),
            ))
        }
        Expr::List(exprs) => {
            if exprs.is_empty() {
                Success(Type::Tag("List".s(), vec![Type::Var("a".s())]))
            } else {
                let types: Vec<Type> = exprs.iter()
                    .map(|e| get_type(env, e.clone()).unwrap())
                    .collect();

                let first = types.first().unwrap();

                if types.iter().all(|i| i == first) {
                    Success(Type::Tag("List".s(), vec![first.clone()]))
                } else {
                    None
                }
            }
        }
        _ => None
    }
}

struct StaticEnv {
    adts: HashMap<String, Type>,
    defs: HashMap<String, Type>,
}

impl StaticEnv {
    pub fn new() -> Self {
        Self {
            adts: HashMap::new(),
            defs: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use nom::*;
    use parsers::expression::read_expr;
    use super::*;
    use tokenizer::get_all_tokens;
    use util::Tk;

    fn from_code(code: &[u8]) -> Expr {
        let stream = get_all_tokens(code);
        read_expr(&stream)
            .map(|(_, e)| e)
            .ok()
            .unwrap()
    }

    #[test]
    fn check_unit() {
        let expr = from_code(b"()");
        let env = StaticEnv::new();
        assert_eq!(get_type(&env, expr), Success(Type::Unit));
    }

    #[test]
    fn check_literal() {
        let expr = from_code(b"123");
        let env = StaticEnv::new();
        assert_eq!(get_type(&env, expr), Success(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_fun() {
        let expr = from_code(b"fun 123");
        let mut env = StaticEnv::new();
        env.defs.insert("fun".s(), Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Tag("Int".s(), vec![])),
        ));

        assert_eq!(get_type(&env, expr), Success(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_if() {
        let expr = from_code(b"if True then 1 else 0");
        let mut env = StaticEnv::new();
        env.adts.insert("True".s(), Type::Tag("Bool".s(), vec![]));

        assert_eq!(get_type(&env, expr), Success(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_lambda() {
        let expr = from_code(b"\\x -> 1");
        let env = StaticEnv::new();

        assert_eq!(get_type(&env, expr), Success(Type::Fun(
            Box::new(Type::Var("x".s())),
            Box::new(Type::Tag("Int".s(), vec![])),
        )));
    }

    #[test]
    fn check_list() {
        let expr = from_code(b"[1, 2, 3]");
        let env = StaticEnv::new();

        assert_eq!(get_type(&env, expr), Success(Type::Tag(
            "List".s(), vec![Type::Tag("Int".s(), vec![])],
        )));
    }
}
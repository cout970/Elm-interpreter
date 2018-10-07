use analyzer::environment::Environment;
use analyzer::expression_fold::create_expr_tree;
use analyzer::expression_fold::ExprTree;
use analyzer::type_analyzer::get_type;
use types::CurriedFunc;
use types::Definition;
use types::Expr;
use types::Fun;
use types::Literal;
use types::Pattern;
use types::Type;
use types::Value;
use util::StringConversion;


pub fn eval(env: &mut Environment, expr: &Expr) -> Result<Value, String> {
    let res: Value = match expr {
        Expr::Unit => Value::Unit,
        Expr::Tuple(items) => {
            Value::Tuple(items.iter()
                .map(|e| eval(env, e))
                .collect::<Result<Vec<_>, _>>()?)
        }
        Expr::List(items) => {
            Value::List(items.iter()
                .map(|e| eval(env, e))
                .collect::<Result<Vec<_>, _>>()?)
        }
        Expr::Record(items) => {
            Value::Record(items.iter()
                .map(|(s, e)| eval(env, e).map(|e| (s.clone(), e)))
                .collect::<Result<Vec<_>, _>>()?)
        }
        Expr::RecordUpdate(name, items) => {
            let val = eval(env, &Expr::Ref(name.clone()))?;

            if let Value::Record(values) = val {
                Value::Record(values.iter().map(|(name, value)| {
                    items.iter()
                        .find(|(_name, _)| name == _name)
                        .and_then(|(nam, expr)| {
                            eval(env, expr).map(|val| (nam.clone(), val)).ok()
                        })
                        .unwrap_or((name.clone(), value.clone()))
                }).collect())
            } else {
                return Err(format!("Not a record: {}", name));
            }
        }
        Expr::If(cond, a, b) => {
            let cond = eval(env, cond)?;

            match cond {
                Value::Adt(name, vals) => {
                    if name == "True" && vals.is_empty() {
                        eval(env, a)?
                    } else if name == "True" && vals.is_empty() {
                        eval(env, b)?
                    } else {
                        return Err(format!("Invalid If condition: {}", name));
                    }
                }
                _ => return Err(format!("Invalid If condition"))
            }
        }
        Expr::Lambda(patt, _expr) => {
            let ty = get_type(env, expr).map_err(|it| format!("{:?}", it))?;

            Value::Fun(CurriedFunc {
                args: vec![],
                arg_count: 1,
                fun: Fun::Expr(patt.clone(), (&**_expr).clone(), ty),
            })
        }

        Expr::OpChain(exprs, ops) => {
            let tree = create_expr_tree(exprs, ops).map_err(|e| format!("{:?}", e))?;
            let expr = tree_as_expr(env, &tree);

            eval(env, &expr)?
        }
        Expr::Literal(lit) => {
            match lit {
                Literal::Int(i) => Value::Int(*i),
                Literal::Float(i) => Value::Float(*i),
                Literal::String(i) => Value::String(i.clone()),
                Literal::Char(i) => Value::Char(*i),
            }
        }
        Expr::Ref(name) | Expr::Adt(name) => {
            env.find(name).ok_or(format!("Missing definition for {}", name))?
        }
        Expr::QualifiedRef(_, name) => {
            // TODO
            eval(env, &Expr::Ref(name.clone()))?
        }
        Expr::RecordField(record, field) => {
            let rec = eval(env, record)?;
            if let Value::Record(ref entries) = rec {
                entries.iter()
                    .find(|(name, _)| name == field)
                    .ok_or(format!("Missing field with name {} in {}", field, rec))?
                    .1.clone()
            } else {
                return Err(format!("Expected record but found {}", rec));
            }
        }
        Expr::RecordAccess(field) => {
            let ty = Type::Fun(
                Box::new(Type::RecExt("b".s(), vec![(field.to_owned(), Type::Var("a".s()))])),
                Box::new(Type::Var("a".s())),
            );

            Value::Fun(CurriedFunc {
                args: vec![Value::String(field.to_owned())],
                fun: Fun::Builtin(0, ty),
                arg_count: 1,
            })
        }
        Expr::Case(_, _) => Value::Unit, // TODO
        Expr::Let(_, _) => Value::Unit, // TODO
        Expr::Application(fun, arg) => {
            let fun = eval(env, fun)?;
            let arg = eval(env, arg)?;

            if let Value::Fun(ref func) = fun {
                call_fun(env, func, arg)?
            } else {
                return Err(format!("Expected a function but found: {}", fun));
            }
        }
    };

    Ok(res)
}

fn tree_as_expr(env: &mut Environment, expr: &ExprTree) -> Expr {
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

fn call_fun(env: &mut Environment, func: &CurriedFunc, arg: Value) -> Result<Value, String> {
    let args = func.args.len() as u32 + 1;

    if func.arg_count < args {
        return Err(format!("To much arguments, expected: {}, found: {}", func.arg_count, args));
    }

    let mut arg_vec = func.args.clone();
    arg_vec.push(arg);

    if func.arg_count == args {
        exec_fun(env, &func.fun, &arg_vec)
    } else {
        Ok(Value::Fun(CurriedFunc { args: arg_vec, fun: func.fun.clone(), arg_count: func.arg_count }))
    }
}

fn exec_fun(env: &mut Environment, fun: &Fun, args: &[Value]) -> Result<Value, String> {
    match fun {
        Fun::Builtin(id, _) => {
            builtin_fun(*id, args)
        }
        Fun::Expr(ref patterns, ref expr, _) => {
            env.enter_block();
            init_local_vals(env, patterns, args);
            let res = eval(env, expr);
            env.exit_block();
            Ok(res?)
        }
    }
}

fn builtin_fun(id: u32, args: &[Value]) -> Result<Value, String> {
    let ret = match id {
        0 => Value::Int(0),
        1 => match [&args[0], &args[1]] { // +
            [Value::Int(a), Value::Int(b)] => Value::Int(a + b),
            [Value::Float(a), Value::Float(b)] => Value::Float(a + b),
            _ => Value::Unit
        },
        2 => match [&args[0], &args[1]] { // -
            [Value::Int(a), Value::Int(b)] => Value::Int(a - b),
            [Value::Float(a), Value::Float(b)] => Value::Float(a - b),
            _ => Value::Unit
        },
        3 => match [&args[0], &args[1]] { // *
            [Value::Int(a), Value::Int(b)] => Value::Int(a * b),
            [Value::Float(a), Value::Float(b)] => Value::Float(a * b),
            _ => Value::Unit
        },
        4 => match [&args[0], &args[1]] { // /
            [Value::Int(a), Value::Int(b)] => Value::Float((*a as f32) / (*b as f32)),
            [Value::Float(a), Value::Float(b)] => Value::Float(a / b),
            _ => Value::Unit
        },
        5 => match [&args[0], &args[1]] {// //
            [Value::Int(a), Value::Int(b)] => Value::Int(a / b),
            [Value::Float(a), Value::Float(b)] => Value::Int((*a as i32) / (*b as i32)),
            _ => Value::Unit
        },
        _ => Value::Unit,
    };

    Ok(ret)
}

fn init_local_vals(env: &mut Environment, patterns: &[Pattern], args: &[Value]) {
    assert_eq!(patterns.len(), args.len());
}

#[cfg(test)]
mod tests {
    use analyzer::environment::builtin_fun_of;
    use nom::*;
    use nom::verbose_errors::*;
    use parsers::expression::read_expr;
    use super::*;
    use tokenizer::get_all_tokens;
    use types::Pattern;
    use types::Type;
    use util::Tk;

    fn from_code(code: &[u8]) -> Expr {
        let stream = get_all_tokens(code);
        let expr: IResult<Tk, Expr> = read_expr(&stream);

        match expr {
            Ok((_, e)) => e,
            Err(e) => {
                match e {
                    Err::Incomplete(need) => panic!("Tokens needed: {:?}", need),
                    Err::Failure(ctx) => panic!("Parsing failure: {:#?}", ctx),
                    Err::Error(ctx) => panic!("Syntax error: {:#?}", ctx),
                };
            }
        }
    }

    #[test]
    fn check_unit() {
        let expr = from_code(b"()");
        let mut env = Environment::new();

        assert_eq!(eval(&mut env, &expr), Ok(Value::Unit));
    }

    #[test]
    fn check_list() {
        let expr = from_code(b"[1, 2, 3]");
        let mut env = Environment::new();

        assert_eq!(eval(&mut env, &expr), Ok(Value::List(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3),
        ])));
    }

    #[test]
    fn check_lambda() {
        let expr = from_code(b"\\x -> 1");
        let mut env = Environment::new();

        assert_eq!(eval(&mut env, &expr), Ok(
            Value::Fun(CurriedFunc {
                arg_count: 1,
                args: vec![],
                fun: Fun::Expr(
                    vec![Pattern::Var("x".s())],
                    Expr::Literal(Literal::Int(1)),
                    Type::Fun(
                        Box::new(Type::Var("x".s())),
                        Box::new(Type::Tag("Int".s(), vec![])),
                    ),
                ),
            })
        ));
    }

    #[test]
    fn check_record() {
        let expr = from_code(b"{ a = 0 }.a");
        let mut env = Environment::new();

        assert_eq!(eval(&mut env, &expr), Ok(Value::Int(0)));
    }

    #[test]
    fn check_application() {
        let expr = from_code(b"fun 0");
        let mut env = Environment::new();

        env.add("fun", builtin_fun_of(0, Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Tag("Int".s(), vec![])),
        )));

        assert_eq!(eval(&mut env, &expr), Ok(Value::Int(0)));
    }
}
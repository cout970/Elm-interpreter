use interpreter::expression_fold::create_expr_tree;
use interpreter::expression_fold::ExprTree;
use interpreter::TypeError::*;
use std::collections::HashMap;
use types::Definition;
use types::Expr;
use types::Literal;
use types::Type;
use util::*;

mod expression_fold;

#[derive(Debug, PartialEq)]
pub enum TypeError {
    MissingAdt(String),
    MissingDefinition(String),
    ListNotHomogeneous(String),
    IfWithNonBoolCondition(String),
    IfBranchesDoesntMatch(String),
    ArgumentsDoNotMatch(String),
    NotAFunction(String),
    InvalidOperandChain(String),
    InternalError,
}

#[derive(Clone)]
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

fn get_type(env: &StaticEnv, expr: &Expr) -> Result<Type, TypeError> {
    match expr {
        Expr::Unit => {
            Ok(Type::Unit)
        }
        Expr::Literal(lit) => {
            let name = match lit {
                Literal::Int(_) => "Int".s(),
                Literal::Float(_) => "Float".s(),
                Literal::Char(_) => "Char".s(),
                Literal::String(_) => "String".s(),
            };
            Ok(Type::Tag(name, vec![]))
        }
        Expr::Adt(name) => {
            match env.adts.get(name) {
                Some(t) => Ok(t.clone()),
                None => Err(MissingAdt(format!("Missing ADT {:?}", name)))
            }
        }
        Expr::Ref(name) => {
            match env.defs.get(name) {
                Some(t) => Ok(t.clone()),
                None => Err(MissingDefinition(format!("Missing def {:?}", name)))
            }
        }
        Expr::QualifiedRef(_path, name) => {
            if name.chars().next().unwrap().is_uppercase() {
                match env.adts.get(name) {
                    Some(t) => Ok(t.clone()),
                    None => Err(MissingDefinition(format!("Missing ADT {:?}", name)))
                }
            } else {
                match env.defs.get(name) {
                    Some(t) => Ok(t.clone()),
                    None => Err(MissingDefinition(format!("Missing def {:?}", name)))
                }
            }
        }
        Expr::Application(i, o) => {
            let function = match get_type(env, i) {
                Ok(t) => t.clone(),
                Err(e) => return Err(e)
            };

            let input = match get_type(env, o) {
                Ok(t) => t.clone(),
                Err(e) => return Err(e)
            };

            match function {
                Type::Fun(ref argument, ref result) => {
                    if **argument != input {
                        Err(ArgumentsDoNotMatch(format!("Expected argument: {:?}, found: {:?}", argument, input)))
                    } else {
                        Ok(*result.clone())
                    }
                }
                _ => {
                    Err(NotAFunction(format!("Expected function found: {:?}", function)))
                }
            }
        }
        Expr::If(cond, a, b) => {
            let cond = match get_type(env, cond) {
                Ok(t) => t.clone(),
                Err(e) => return Err(e)
            };

            let true_branch = match get_type(env, a) {
                Ok(t) => t.clone(),
                Err(e) => return Err(e)
            };

            let false_branch = match get_type(env, b) {
                Ok(t) => t.clone(),
                Err(e) => return Err(e)
            };

            match cond {
                Type::Tag(name, _) => {
                    if name != "Bool" {
                        return Err(IfWithNonBoolCondition(format!("Expected Bool expression but found {:?}", name)));
                    }
                }
                _ => {
                    return Err(IfWithNonBoolCondition("Expected Bool expression".s()));
                }
            }

            if true_branch == false_branch {
                Ok(true_branch)
            } else {
                Err(IfBranchesDoesntMatch(format!("True Branch: {:?}, False Branch: {:?}", true_branch, false_branch)))
            }
        }
        Expr::Lambda(_patterns, expr) => {
            let out = get_type(env, expr)?;
            // TODO patterns to variables
            let var = Type::Var("x".s());

            Ok(Type::Fun(
                Box::new(var),
                Box::new(out),
            ))
        }
        Expr::List(exprs) => {
            if exprs.is_empty() {
                Ok(Type::Tag("List".s(), vec![Type::Var("a".s())]))
            } else {
                let mut types: Vec<Type> = vec![];

                for e in exprs {
                    let e_type = match get_type(env, e) {
                        Ok(t) => t.clone(),
                        Err(e) => { return Err(e); }
                    };

                    types.push(e_type);
                }

                let first = types.first().unwrap();

                for i in 1..types.len() {
                    if &types[i] != first {
                        let msg = format!("List of {:?}, but found element {:?} at {}", first, types[i], i);
                        return Err(ListNotHomogeneous(msg));
                    }
                }

                Ok(Type::Tag("List".s(), vec![first.clone()]))
            }
        }
        Expr::Let(defs, expr) => {
            let new_env = expand_env(defs, env);
            // TODO
            get_type(&new_env, expr)
        }
        Expr::OpChain(exprs, ops) => {
            let tree = create_expr_tree(exprs, ops);

            match tree {
                Ok(tree) => get_tree_type(&env, tree),
                Err(_) => Err(InvalidOperandChain(format!("You cannot mix >> and << without parentheses"))),
            }
        }
        Expr::Record(entries) => {
            let mut types: Vec<(String, Type)> = vec![];

            for (name, expr) in entries {
                match get_type(&env, expr) {
                    Ok(ty) => types.push((name.clone(), ty)),
                    Err(e) => return Err(e)
                }
            }

            Ok(Type::Record(types))
        }
        Expr::RecordAccess(_) => {
            Ok(Type::Fun(
                Box::new(Type::Var("a".s())),
                Box::new(Type::Var("b".s())),
            ))
        }
        Expr::RecordField(expr, name) => {
            let record = match get_type(env, expr) {
                Ok(t) => t.clone(),
                Err(e) => { return Err(e); }
            };

            if let Type::Record(fields) = record {
                let field: Option<&Type> = fields
                    .iter()
                    .find(|(f_name, _)| f_name == name)
                    .map(|(_, f_type)| f_type);

                match field {
                    Some(t) => Ok(t.clone()),
                    None => Err(InternalError)
                }
            } else {
                Err(InternalError)
            }
        }
        _ => Err(InternalError)
    }
}

fn get_tree_type(env: &StaticEnv, tree: ExprTree) -> Result<Type, TypeError> {
    match tree {
        ExprTree::Leaf(e) => get_type(env, &e),
        ExprTree::Branch(op, left, right) => {
            let op_type = match env.defs.get(&op) {
                Some(t) => t.clone(),
                None => return Err(MissingDefinition(format!("Missing def {:?}", op)))
            };

            let left_value = match get_tree_type(env, *left) {
                Ok(t) => t.clone(),
                Err(e) => return Err(e)
            };

            let right_value = match get_tree_type(env, *right) {
                Ok(t) => t.clone(),
                Err(e) => return Err(e)
            };

            if let Type::Fun(ref argument, ref next_func) = op_type {
                if **argument != left_value {
                    return Err(ArgumentsDoNotMatch(
                        format!("Expected argument: {:?}, found: {:?}", argument, left_value)
                    ));
                }
                if let Type::Fun(ref argument, ref result) = **next_func {
                    if **argument != right_value {
                        return Err(ArgumentsDoNotMatch(
                            format!("Expected argument: {:?}, found: {:?}", argument, right_value)
                        ));
                    }

                    Ok(*result.clone())
                } else {
                    Err(NotAFunction(format!("Expected infix operator but found: {:?} after first evaluation", op_type)))
                }
            } else {
                Err(NotAFunction(format!("Expected infix operator but found: {:?}", op_type)))
            }
        }
    }
}

fn expand_env(_defs: &Vec<Definition>, old_env: &StaticEnv) -> StaticEnv {
    old_env.clone()
}

#[cfg(test)]
mod tests {
    use nom::*;
    use nom::verbose_errors::*;
    use parsers::expression::read_expr;
    use super::*;
    use tokenizer::get_all_tokens;
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
        let env = StaticEnv::new();
        assert_eq!(get_type(&env, &expr), Ok(Type::Unit));
    }

    #[test]
    fn check_literal() {
        let expr = from_code(b"123");
        let env = StaticEnv::new();
        assert_eq!(get_type(&env, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_fun() {
        let expr = from_code(b"fun 123");
        let mut env = StaticEnv::new();
        env.defs.insert("fun".s(), Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Tag("Int".s(), vec![])),
        ));

        assert_eq!(get_type(&env, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_if() {
        let expr = from_code(b"if True then 1 else 0");
        let mut env = StaticEnv::new();
        env.adts.insert("True".s(), Type::Tag("Bool".s(), vec![]));

        assert_eq!(get_type(&env, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_lambda() {
        let expr = from_code(b"\\x -> 1");
        let env = StaticEnv::new();

        assert_eq!(get_type(&env, &expr), Ok(Type::Fun(
            Box::new(Type::Var("x".s())),
            Box::new(Type::Tag("Int".s(), vec![])),
        )));
    }

    #[test]
    fn check_list() {
        let expr = from_code(b"[1, 2, 3]");
        let env = StaticEnv::new();

        assert_eq!(get_type(&env, &expr), Ok(Type::Tag(
            "List".s(), vec![Type::Tag("Int".s(), vec![])],
        )));
    }

    #[test]
    fn check_record() {
        let expr = from_code(b"{ a = 1, b = \"Hi\" }");
        let env = StaticEnv::new();

        assert_eq!(get_type(&env, &expr), Ok(
            Type::Record(vec![
                ("a".s(), Type::Tag("Int".s(), vec![])),
                ("b".s(), Type::Tag("String".s(), vec![])),
            ])
        ));
    }

    #[test]
    fn check_operator_chain() {
        let expr = from_code(b"1 + 2");
        let mut env = StaticEnv::new();

        env.defs.insert("+".s(), Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Fun(
                Box::new(Type::Tag("Int".s(), vec![])),
                Box::new(Type::Tag("Int".s(), vec![])),
            )),
        ));

        assert_eq!(get_type(&env, &expr), Ok(
            Type::Tag("Int".s(), vec![])
        ));
    }
}
use analyzer::environment::StaticEnv;
use types::Expr;
use types::Type;
use types::Literal;
use util::StringConversion;
use analyzer::type_analyzer::TypeError::*;
use analyzer::expression_fold::ExprTree;
use analyzer::expression_fold::create_expr_tree;

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
    RecordUpdateOnNonRecord(String),
    RecordUpdateUnknownField(String),
    CaseBranchDontMatchReturnType(String),
    InternalError,
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
            env.get_adt_type(name).ok_or(MissingAdt(format!("Missing ADT {:?}", name)))
        }
        Expr::Ref(name) => {
            env.get_def_type(name).ok_or(MissingDefinition(format!("Missing def {:?}", name)))
        }
        Expr::QualifiedRef(_path, name) => {
            // TODO resolve path
            if name.chars().next().unwrap().is_uppercase() {
                env.get_adt_type(name).ok_or(MissingAdt(format!("Missing ADT {:?}", name)))
            } else {
                env.get_def_type(name).ok_or(MissingDefinition(format!("Missing def {:?}", name)))
            }
        }
        Expr::Application(i, o) => {
            let function = get_type(env, i).map(|i| i.clone())?;
            let input = get_type(env, o).map(|i| i.clone())?;

            if let Type::Fun(ref argument, ref result) = function {
                if **argument != input {
                    Err(ArgumentsDoNotMatch(format!("Expected argument: {:?}, found: {:?}", argument, input)))
                } else {
                    Ok(*result.clone())
                }
            } else {
                Err(NotAFunction(format!("Expected function found: {:?}", function)))
            }
        }
        Expr::If(cond, a, b) => {
            let cond = get_type(env, cond).map(|i| i.clone())?;
            let true_branch = get_type(env, a).map(|i| i.clone())?;
            let false_branch = get_type(env, b).map(|i| i.clone())?;

            if let Type::Tag(name, _) = cond {
                if name != "Bool" {
                    return Err(IfWithNonBoolCondition(format!("Expected Bool expression but found {:?}", name)));
                }
            } else {
                return Err(IfWithNonBoolCondition("Expected Bool expression".s()));
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
                let types: Vec<Type> = exprs.iter()
                    .map(|e| get_type(env, e))
                    .collect::<Result<_, _>>()?;

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
            let new_env = env.expanded(defs);
            get_type(&new_env, expr)
        }
        Expr::OpChain(exprs, ops) => {
            match create_expr_tree(exprs, ops) {
                Ok(tree) => get_tree_type(&env, tree),
                Err(_) => Err(InvalidOperandChain(format!("You cannot mix >> and << without parentheses"))),
            }
        }
        Expr::Record(entries) => {
            let types: Vec<(String, Type)> = entries.iter()
                .map(|(name, expr)| get_type(&env, expr).map(|ty| (name.clone(), ty)))
                .collect::<Result<_, _>>()?;

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
        Expr::Tuple(items) => {
            let types: Vec<Type> = items.iter()
                .map(|e| get_type(env, e))
                .collect::<Result<_, _>>()?;

            Ok(Type::Tuple(types))
        }
        Expr::RecordUpdate(name, updates) => {
            let record_type = env.get_def_type(name)
                .ok_or(MissingDefinition(format!("Missing def {:?}", name)))?;


            if let Type::Record(fields) = &record_type {
                for (field_name, _) in updates {
                    let found = fields.iter().any(|(field, _)| field == field_name);
                    if !found {
                        return Err(RecordUpdateUnknownField(
                            format!("Field '{:?}' not found in record: {:?} of type: {:?}", field_name, name, record_type)
                        ));
                    }
                }

                Ok(record_type.clone())
            } else {
                Err(RecordUpdateOnNonRecord(
                    format!("Expecting record to update but found: {:?}", record_type)
                ))
            }
        }
        Expr::Case(expr, branches) => {

            // check that the case expression has a valid type
            get_type(env, expr)?;

            let mut iter = branches.iter();
            let (_, e) = iter.next().unwrap();
            let first_type = get_type(env, e)?;

            while let Some((_, e)) = iter.next() {
                let ret = get_type(env, e)?;
                if ret != first_type {
                    return Err(CaseBranchDontMatchReturnType("".s()));
                }
            }

            Ok(first_type)
        }
    }
}

fn get_tree_type(env: &StaticEnv, tree: ExprTree) -> Result<Type, TypeError> {
    match tree {
        ExprTree::Leaf(e) => get_type(env, &e),
        ExprTree::Branch(op, left, right) => {

            let op_type = env.get_def_type(&op)
                .ok_or(MissingDefinition(format!("Missing def {:?}", op)))?;

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
        env.add_def_type("fun", &Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Tag("Int".s(), vec![])),
        ));

        assert_eq!(get_type(&env, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_if() {
        let expr = from_code(b"if True then 1 else 0");
        let mut env = StaticEnv::new();
        env.add_adt_type("True", &Type::Tag("Bool".s(), vec![]));

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

        env.add_def_type("+", &Type::Fun(
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

    #[test]
    fn check_tuple() {
        let expr = from_code(b"(1, \"a\", ())");
        let env = StaticEnv::new();

        assert_eq!(get_type(&env, &expr), Ok(
            Type::Tuple(vec![
                Type::Tag("Int".s(), vec![]),
                Type::Tag("String".s(), vec![]),
                Type::Unit,
            ])
        ));
    }

    #[test]
    fn check_record_update() {
        let expr = from_code(b"{ x | a = 0}");
        let mut env = StaticEnv::new();
        let record_type = Type::Record(vec![
            ("a".s(), Type::Tag("Int".s(), vec![]))
        ]);

        env.add_def_type("x", &record_type);

        assert_eq!(get_type(&env, &expr), Ok(record_type));
    }

    #[test]
    fn check_case() {
        let expr = from_code(b"case 0 of\n 0 -> \"a\"\n _ -> \"b\"");
        let env = StaticEnv::new();

        assert_eq!(get_type(&env, &expr), Ok(Type::Tag("String".s(), vec![])));
    }

    #[test]
    fn check_case2() {
        let expr = from_code(b"case 0 of\n 0 -> 1\n _ -> \"b\"");
        let env = StaticEnv::new();

        assert_eq!(get_type(&env, &expr), Err(CaseBranchDontMatchReturnType("".s())));
    }
}
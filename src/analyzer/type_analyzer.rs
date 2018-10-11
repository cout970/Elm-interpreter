use analyzer::environment::arg_count;
use analyzer::environment::Environment;
use analyzer::expression_fold::create_expr_tree;
use analyzer::expression_fold::ExprTree;
use analyzer::pattern_helper::add_pattern_variables;
use analyzer::pattern_helper::pattern_to_type;
use analyzer::type_analyzer::TypeError::*;
use interpreter::eval;
use std::ops::Deref;
use types::CurriedFunc;
use types::Definition;
use types::Expr;
use types::Fun;
use types::Literal;
use types::Pattern;
use types::Type;
use types::Value;
use types::ValueDefinition;
use util::build_fun_type;
use util::name_sequence::NameSequence;
use util::StringConversion;
use analyzer::environment::expand_env;
use analyzer::type_resolution::type_assignable_from;
use analyzer::type_resolution::calculate_common_type;
use analyzer::type_resolution::get_value_type;

#[derive(Clone, Debug, PartialEq)]
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
    DefinitionTypeAndReturnTypeMismatch,
    InvalidLambdaPattern(String),
    ConstantEvaluationError(String),
    VariableAlreadyDeclared(String),
    UnableToCalculateFunctionType(String),
    InternalError,
}

pub fn get_type(env: &mut Environment, expr: &Expr) -> Result<Type, TypeError> {
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
            env.find(name)
                .map(|val| get_value_type(&val))
                .or_else(|| env.find_variable(name))
                .ok_or(MissingAdt(format!("Missing ADT {}", name)))
        }
        Expr::Ref(name) => {
            env.find(name)
                .map(|val| get_value_type(&val))
                .or_else(|| env.find_variable(name))
                .ok_or(MissingDefinition(format!("Missing def {}", name)))
        }
        Expr::QualifiedRef(_path, name) => {
            // TODO resolve path
            let is_adt = name.chars().next().unwrap().is_uppercase();

            if is_adt {
                get_type(env, &Expr::Adt(name.to_owned()))
            } else {
                get_type(env, &Expr::Ref(name.to_owned()))
            }
        }
        Expr::Application(i, o) => {
            let function = get_type(env, i).map(|i| i.clone())?;
            let input = get_type(env, o).map(|i| i.clone())?;

            if let Type::Fun(ref argument, ref result) = function {
                if !type_assignable_from(env, &input, &**argument) {
                    Err(ArgumentsDoNotMatch(format!("Expected argument: {}, found: {}", argument, input)))
                } else {

                    Ok(*result.clone())
                }
            } else {
                Err(NotAFunction(format!("Expected function found: {}, (in: {}, out: {})", function, i, o)))
            }
        }
        Expr::If(cond, a, b) => {
            let cond = get_type(env, cond).map(|i| i.clone())?;
            let true_branch = get_type(env, a).map(|i| i.clone())?;
            let false_branch = get_type(env, b).map(|i| i.clone())?;

            if !type_assignable_from(env, &Type::Tag("Bool".s(), vec![]), &cond) {
                return Err(IfWithNonBoolCondition(format!("Expected Bool expression but found {}", cond)));
            }

            let ret_ty = calculate_common_type(env, &[&true_branch, &false_branch]);
            match ret_ty {
                Ok(ty) => Ok(ty.clone()),
                Err((a, b)) => Err(
                    IfBranchesDoesntMatch(format!("True Branch: {}, False Branch: {}", a, b))
                )
            }
        }
        Expr::Lambda(patterns, expr) => {
            env.enter_block();
            for patt in patterns {
                add_pattern_variables(env, patt).map_err(|e| VariableAlreadyDeclared(e))?;
            }
            let out_ = get_type(env, expr);
            env.exit_block();
            let out = out_?;

            let mut var = patterns.iter()
                .map(|p| pattern_to_type(p))
                .collect::<Result<Vec<Type>, String>>()
                .map_err(|s| InvalidLambdaPattern(s))?;

            var.push(out);

            Ok(build_fun_type(&var))
        }
        Expr::List(exprs) => {
            if exprs.is_empty() {
                Ok(Type::Tag("List".s(), vec![Type::Var("a".s())]))
            } else {
                let types: Vec<Type> = exprs.iter()
                    .map(|e| get_type(env, e))
                    .collect::<Result<_, _>>()?;

                let ret_ty = calculate_common_type(env, &types.iter().collect::<Vec<&Type>>());
                match ret_ty {
                    Ok(ty) => {
                        Ok(Type::Tag("List".s(), vec![ty.clone()]))
                    }
                    Err((a, b)) => {
                        let index = types.iter()
                            .enumerate()
                            .find(|(_, ty)| ty == &b)
                            .unwrap()
                            .0;

                        Err(ListNotHomogeneous(
                            format!("List of '{}', but found element '{}' at index: {}", a, b, index)
                        ))
                    }
                }
            }
        }
        Expr::Let(defs, expr) => {
            env.enter_block();
            let res = expand_env(env, defs.iter().collect());
            let ty = get_type(env, expr);
            env.exit_block();
            res?;
            ty
        }
        Expr::OpChain(exprs, ops) => {
            match create_expr_tree(exprs, ops) {
                Ok(tree) => get_tree_type(env, tree),
                Err(_) => Err(InvalidOperandChain(format!("You cannot mix >> and << without parentheses"))),
            }
        }
        Expr::Record(entries) => {
            let types: Vec<(String, Type)> = entries.iter()
                .map(|(name, expr)| get_type(env, expr).map(|ty| (name.clone(), ty)))
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
            let record_type = get_type(env, &Expr::Ref(name.to_owned()))?;

            if let Type::Record(fields) = &record_type {
                for (field_name, _) in updates {
                    let found = fields.iter().any(|(field, _)| field == field_name);
                    if !found {
                        return Err(RecordUpdateUnknownField(
                            format!("Field '{}' not found in record: {} of type: {}", field_name, name, record_type)
                        ));
                    }
                }

                Ok(record_type.clone())
            } else {
                Err(RecordUpdateOnNonRecord(
                    format!("Expecting record to update but found: {}", record_type)
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
                if !type_assignable_from(env, &ret, &first_type) {
                    return Err(CaseBranchDontMatchReturnType("".s()));
                }
            }

            Ok(first_type)
        }
    }
}

fn get_tree_type(env: &mut Environment, tree: ExprTree) -> Result<Type, TypeError> {
    match tree {
        ExprTree::Leaf(e) => get_type(env, &e),
        ExprTree::Branch(op, left, right) => {
            let op_type = get_type(env, &Expr::Ref(op.to_owned()))?;

            let left_value = get_tree_type(env, *left).map(|t| t.clone())?;
            let right_value = get_tree_type(env, *right).map(|t| t.clone())?;

            if let Type::Fun(ref argument, ref next_func) = op_type {
                if !type_assignable_from(env, &left_value, &**argument) {
                    return Err(ArgumentsDoNotMatch(
                        format!("Expected argument: {}, found: {}", argument, left_value)
                    ));
                }
                if let Type::Fun(ref argument, ref result) = **next_func {
                    if !type_assignable_from(env, &right_value, &**argument) {
                        return Err(ArgumentsDoNotMatch(
                            format!("Expected argument: {}, found: {}", argument, right_value)
                        ));
                    }

                    Ok(*result.clone())
                } else {
                    Err(NotAFunction(format!("Expected infix operator but found: {} after first evaluation", op_type)))
                }
            } else {
                Err(NotAFunction(format!("Expected infix operator but found: {}", op_type)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use analyzer::environment::builtin_fun_of;
    use nom::*;
    use nom::verbose_errors::*;
    use parsers::expression::read_expr;
    use super::*;
    use tokenizer::get_all_tokens;
    use types::CurriedFunc;
    use types::Fun;
    use types::Value;
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
        assert_eq!(get_type(&mut env, &expr), Ok(Type::Unit));
    }

    #[test]
    fn check_literal() {
        let expr = from_code(b"123");
        let mut env = Environment::new();
        assert_eq!(get_type(&mut env, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_fun() {
        let expr = from_code(b"fun 123");
        let mut env = Environment::new();
        env.add("fun", builtin_fun_of(0, Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Tag("Int".s(), vec![])),
        )));

        assert_eq!(get_type(&mut env, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_if() {
        let expr = from_code(b"if True then 1 else 0");
        let mut env = Environment::new();

        env.add("True", builtin_fun_of(
            0, Type::Tag("Bool".s(), vec![]),
        ));

        assert_eq!(get_type(&mut env, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_lambda() {
        let expr = from_code(b"\\x -> 1");
        let mut env = Environment::new();

        assert_eq!(get_type(&mut env, &expr), Ok(Type::Fun(
            Box::new(Type::Var("x".s())),
            Box::new(Type::Tag("Int".s(), vec![])),
        )));
    }

    #[test]
    fn check_list() {
        let expr = from_code(b"[1, 2, 3]");
        let mut env = Environment::new();

        assert_eq!(get_type(&mut env, &expr), Ok(Type::Tag(
            "List".s(), vec![Type::Tag("Int".s(), vec![])],
        )));
    }

    #[test]
    fn check_bad_list() {
        let expr = from_code(b"[1, 2, 'a']");
        let mut env = Environment::new();

        assert_eq!(get_type(&mut env, &expr), Err(
            ListNotHomogeneous(
                "List of 'Int', but found element 'Char' at index: 2".s()
            )
        ));
    }

    #[test]
    fn check_record() {
        let expr = from_code(b"{ a = 1, b = \"Hi\" }");
        let mut env = Environment::new();

        assert_eq!(get_type(&mut env, &expr), Ok(
            Type::Record(vec![
                ("a".s(), Type::Tag("Int".s(), vec![])),
                ("b".s(), Type::Tag("String".s(), vec![])),
            ])
        ));
    }

    #[test]
    fn check_operator_chain() {
        let expr = from_code(b"1 + 2");
        let mut env = Environment::new();

        env.add("+", builtin_fun_of(0, Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Fun(
                Box::new(Type::Tag("Int".s(), vec![])),
                Box::new(Type::Tag("Int".s(), vec![])),
            )),
        )));

        assert_eq!(get_type(&mut env, &expr), Ok(
            Type::Tag("Int".s(), vec![])
        ));
    }

    #[test]
    fn check_tuple() {
        let expr = from_code(b"(1, \"a\", ())");
        let mut env = Environment::new();

        assert_eq!(get_type(&mut env, &expr), Ok(
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
        let mut env = Environment::new();
        let record_type = Type::Record(vec![
            ("a".s(), Type::Tag("Int".s(), vec![]))
        ]);

        env.add("x", builtin_fun_of(0, record_type.clone()));

        assert_eq!(get_type(&mut env, &expr), Ok(record_type));
    }

    #[test]
    fn check_case() {
        let expr = from_code(b"case 0 of\n 0 -> \"a\"\n _ -> \"b\"");
        let mut env = Environment::new();

        assert_eq!(get_type(&mut env, &expr), Ok(Type::Tag("String".s(), vec![])));
    }

    #[test]
    fn check_case2() {
        let expr = from_code(b"case 0 of\n 0 -> 1\n _ -> \"b\"");
        let mut env = Environment::new();

        assert_eq!(get_type(&mut env, &expr), Err(CaseBranchDontMatchReturnType("".s())));
    }
}
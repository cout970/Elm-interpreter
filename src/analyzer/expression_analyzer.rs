use analyzer::function_analyzer::analyze_function;
use analyzer::function_analyzer::analyze_function_arguments;
use analyzer::function_analyzer::PatternMatchingError;
use analyzer::static_env::StaticEnv;
use analyzer::TypeError::*;
use analyzer::TypeError;
use std::collections::HashMap;
use std::ops::Deref;
use types::Definition;
use types::Expr;
use types::Fun;
use types::Literal;
use types::Pattern;
use types::Type;
use types::ValueDefinition;
use util::build_fun_type;
use util::expression_fold::create_expr_tree;
use util::expression_fold::ExprTree;
use util::name_sequence::NameSequence;
use util::StringConversion;
use analyzer::function_analyzer::is_assignable;
use analyzer::function_analyzer::calculate_common_type;

pub fn analyze_expression(env: &mut StaticEnv, expected: Option<&Type>, expr: &Expr) -> Result<Type, TypeError> {
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
            env.find(name).ok_or(MissingAdt(format!("Missing ADT {}", name)))
        }
        Expr::Ref(name) => {
            env.find(name).ok_or(MissingDefinition(format!("Missing def {}", name)))
        }
        Expr::QualifiedRef(_path, name) => {
            // TODO resolve path
            let is_adt = name.chars().next().unwrap().is_uppercase();

            if is_adt {
                analyze_expression(env, expected, &Expr::Adt(name.to_owned()))
            } else {
                analyze_expression(env, expected, &Expr::Ref(name.to_owned()))
            }
        }
        Expr::Application(i, o) => {
            let function = analyze_expression(env, None, i)?;

            if let Type::Fun(ref argument, ref result) = function {
                let input = analyze_expression(env, Some(result), o)?;

                if is_assignable(&**argument, &input) {
                    Ok(*result.clone())
                } else {
                    Err(ArgumentsDoNotMatch(format!("Expected argument: {}, found: {}", argument, input)))
                }
            } else {
                Err(NotAFunction(format!("Expected function found: {}, (in: {}, out: {})", function, i, o)))
            }
        }
        Expr::If(cond, a, b) => {
            let cond = analyze_expression(env, Some(&Type::Tag("Bool".s(), vec![])), cond)?;
            let true_branch = analyze_expression(env, expected, a)?;
            let false_branch = analyze_expression(env, expected, b)?;

            if !is_assignable(&Type::Tag("Bool".s(), vec![]), &cond) {
                return Err(IfWithNonBoolCondition(format!("Expected Bool expression but found {}", cond)));
            }

            let branches = vec![true_branch, false_branch];
            let ret_ty = calculate_common_type(&branches);
            match ret_ty {
                Ok(ty) => Ok(ty.clone()),
                Err((a, b)) => Err(
                    IfBranchesDoesntMatch(format!("True Branch: {}, False Branch: {}", a, b))
                )
            }
        }
        Expr::Lambda(patterns, expr) => {
            let (tys, new_vars) = analyze_function_arguments(patterns)?;

            env.enter_block();
            for (name, value) in &new_vars {
                if env.find(name).is_some() {
                    env.exit_block();
                    return Err(VariableNameShadowed(name.clone()));
                }

                env.add(name, value.clone());
            }

            let out = analyze_expression(env, expected, expr);
            env.exit_block();

            let mut var = tys.clone();
            var.push(out?);

            Ok(build_fun_type(&var))
        }
        Expr::List(exprs) => {
            if exprs.is_empty() {
                Ok(Type::Tag("List".s(), vec![Type::Var("a".s())]))
            } else {
                let types: Vec<Type> = exprs.iter()
                    .map(|e| analyze_expression(env, None, e))
                    .collect::<Result<_, _>>()?;

                let ret_ty = calculate_common_type(&types);
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
            for def in defs {
                let def_ty = analyze_function(env, def);

                match def_ty {
                    Ok(ty) => {
                        env.add(&def.1.name, ty);
                    }
                    Err(e) => {
                        env.exit_block();
                        return Err(e);
                    }
                }
            }
            let res = analyze_expression(env, expected, expr);
            env.exit_block();

            res
        }
        Expr::OpChain(exprs, ops) => {
            match create_expr_tree(exprs, ops) {
                Ok(tree) => get_tree_type(env, expected, tree),
                Err(_) => Err(InvalidOperandChain(format!("You cannot mix >> and << without parentheses"))),
            }
        }
        Expr::Record(entries) => {
            let types: Vec<(String, Type)> = entries.iter()
                .map(|(name, expr)| analyze_expression(env, None, expr).map(|ty| (name.clone(), ty)))
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
            let exp_ty = Type::Record(vec![
                (name.clone(), expected.map(|e| e.clone()).unwrap_or(Type::Var("a".s())))
            ]);
            let record = match analyze_expression(env, Some(&exp_ty), expr) {
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
                .map(|e| analyze_expression(env, None, e))
                .collect::<Result<_, _>>()?;

            Ok(Type::Tuple(types))
        }
        Expr::RecordUpdate(name, updates) => {
            let mut update_types = vec![];
            for (name, expr) in updates {
                update_types.push((name.clone(), analyze_expression(env, None, expr)?));
            }
            let exp_ty = Type::Record(update_types);

            let record_type = analyze_expression(env, Some(&exp_ty), &Expr::Ref(name.to_owned()))?;

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
            let mut iter = branches.iter();
            let (_, e) = iter.next().unwrap();
            let first_type = analyze_expression(env, None, e)?;

            while let Some((_, e)) = iter.next() {
                let ret = analyze_expression(env, None, e)?;
                if !is_assignable(&first_type, &ret) {
                    return Err(CaseBranchDontMatchReturnType("".s()));
                }
            }

            // check that the case expression has a valid type
            analyze_expression(env, Some(&first_type), expr)?;

            Ok(first_type)
        }
    }
}

fn get_tree_type(env: &mut StaticEnv, expected: Option<&Type>, tree: ExprTree) -> Result<Type, TypeError> {
    match tree {
        ExprTree::Leaf(e) => analyze_expression(env, expected, &e),
        ExprTree::Branch(op, left, right) => {
            let op_type = analyze_expression(env, None, &Expr::Ref(op.to_owned()))?;

            if let Type::Fun(ref argument, ref next_func) = op_type {
                let left_value = get_tree_type(env, Some(next_func), *left).map(|t| t.clone())?;

                if !is_assignable(&**argument, &left_value) {
                    return Err(ArgumentsDoNotMatch(
                        format!("Expected argument: {}, found: {}", argument, left_value)
                    ));
                }
                if let Type::Fun(ref argument, ref result) = **next_func {
                    let right_value = get_tree_type(env, Some(result), *right).map(|t| t.clone())?;

                    if !is_assignable(&**argument, &right_value) {
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
    use nom::*;
    use nom::verbose_errors::*;
    use parsers::from_code;
    use super::*;
    use tokenizer::tokenize;
    use types::Fun;
    use types::Value;
    use util::builtin_fun_of;

    #[test]
    fn check_unit() {
        let expr = from_code(b"()");
        let mut env = StaticEnv::new();
        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Unit));
    }

    #[test]
    fn check_literal() {
        let expr = from_code(b"123");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_fun() {
        let expr = from_code(b"fun 123");
        let mut env = StaticEnv::new();

        env.add("fun", Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Tag("Int".s(), vec![])),
        ));

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_if() {
        let expr = from_code(b"if True then 1 else 0");
        let mut env = StaticEnv::new();

        env.add("True", Type::Tag("Bool".s(), vec![]));

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_lambda() {
        let expr = from_code(b"\\x -> 1");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Fun(
            Box::new(Type::Var("a".s())),
            Box::new(Type::Tag("Int".s(), vec![])),
        )));
    }

    #[test]
    fn check_list() {
        let expr = from_code(b"[1, 2, 3]");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag(
            "List".s(), vec![Type::Tag("Int".s(), vec![])],
        )));
    }

    #[test]
    fn check_bad_list() {
        let expr = from_code(b"[1, 2, 'a']");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Err(
            ListNotHomogeneous(
                "List of 'Int', but found element 'Char' at index: 2".s()
            )
        ));
    }

    #[test]
    fn check_record() {
        let expr = from_code(b"{ a = 1, b = \"Hi\" }");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(
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

        env.add("+", Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Fun(
                Box::new(Type::Tag("Int".s(), vec![])),
                Box::new(Type::Tag("Int".s(), vec![])),
            )),
        ));

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(
            Type::Tag("Int".s(), vec![])
        ));
    }

    #[test]
    fn check_tuple() {
        let expr = from_code(b"(1, \"a\", ())");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(
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

        env.add("x", record_type.clone());

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(record_type));
    }

    #[test]
    fn check_case() {
        let expr = from_code(b"case 0 of\n 0 -> \"a\"\n _ -> \"b\"");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag("String".s(), vec![])));
    }

    #[test]
    fn check_case2() {
        let expr = from_code(b"case 0 of\n 0 -> 1\n _ -> \"b\"");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Err(CaseBranchDontMatchReturnType("".s())));
    }
}
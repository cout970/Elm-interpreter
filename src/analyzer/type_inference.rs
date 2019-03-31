use std::collections::HashMap;
use std::sync::Arc;

use analyzer::Analyzer;
use analyzer::static_env::StaticEnv;
use ast::*;
use errors::TypeError;
use typed_ast::expr_type;
use types::*;
use util::expression_fold::create_expr_tree;
use util::expression_fold::ExprTree;
use util::qualified_name;

pub fn expr_tree_to_expr(tree: ExprTree) -> Expr {
    match tree {
        ExprTree::Leaf(e) => e,
        ExprTree::Branch(op, left, right) => {
            let left_expr = expr_tree_to_expr(*left);
            let right_expr = expr_tree_to_expr(*right);
            let span = (span(&left_expr).0, span(&right_expr).1);

            Expr::Application(
                span,
                Box::new(Expr::Application(
                    span,
                    Box::new(Expr::Ref(span, op)),
                    Box::new(left_expr),
                )),
                Box::new(right_expr),
            )
        }
    }
}

pub fn type_inference_backtrack_expr(env: &mut StaticEnv, vars: &HashMap<String, Type>, expr: &Expr) {
    match expr {
        Expr::Unit(..) => {}
        Expr::Literal(_, _) => {}
        Expr::RecordAccess(_, _) => {}
        Expr::Tuple(_, items) => {
            items.iter().for_each(|i| type_inference_backtrack_expr(env, vars, i));
        }
        Expr::List(_, items) => {
            items.iter().for_each(|i| type_inference_backtrack_expr(env, vars, i));
        }
        Expr::Record(_, items) => {
            items.iter().for_each(|(_, i)| type_inference_backtrack_expr(env, vars, i));
        }
        Expr::RecordUpdate(_, _, items) => {
            items.iter().for_each(|(_, i)| type_inference_backtrack_expr(env, vars, i));
        }
        Expr::RecordField(_, e, _) => {
            type_inference_backtrack_expr(env, vars, e);
        }
        Expr::If(_, a, b, c) => {
            type_inference_backtrack_expr(env, vars, a);
            type_inference_backtrack_expr(env, vars, b);
            type_inference_backtrack_expr(env, vars, c);
        }
        Expr::Lambda(_, _, e) => {
            type_inference_backtrack_expr(env, vars, e);
        }
        Expr::Application(_, a, b) => {
            type_inference_backtrack_expr(env, vars, a);
            type_inference_backtrack_expr(env, vars, b);
        }
        Expr::Let(_, _, e) => {
            type_inference_backtrack_expr(env, vars, e);
        }
        Expr::Case(_, e, items) => {
            type_inference_backtrack_expr(env, vars, e);
            items.iter().for_each(|(_, i)| type_inference_backtrack_expr(env, vars, i));
        }
        Expr::OpChain(_, exprs, ops) => {
            match create_expr_tree(exprs, ops) {
                Ok(tree) => type_inference_backtrack_expr(env, vars, &expr_tree_to_expr(tree)),
                Err(_) => {}
            }
        }
        Expr::QualifiedRef(_, path, name) => {
            let full_name = qualified_name(path, name);

            type_inference_backtrack_expr(env, vars, &Expr::Ref((0, 0), full_name))
        }
        Expr::Ref(_, variable) => {
            match env.find_definition(variable) {
                Some(variable_type) => {
                    env.replace(variable, type_inference_replace_vars_with_concrete_types(vars, &variable_type))
                }
                None => {}
            }
        }
    }
}

pub fn type_inference_replace_vars_with_concrete_types(vars: &HashMap<String, Type>, ty: &Type) -> Type {
    match ty {
        Type::Var(name) => {
            vars.get(name).unwrap_or(ty).clone()
        }
        Type::Tag(name, sub) => {
            Type::Tag(name.clone(), sub.iter().map(|i| type_inference_replace_vars_with_concrete_types(vars, i)).collect())
        }
        Type::Fun(a, b) => {
            Type::Fun(
                Box::from(type_inference_replace_vars_with_concrete_types(vars, a)),
                Box::from(type_inference_replace_vars_with_concrete_types(vars, b)),
            )
        }
        Type::Unit => {
            Type::Unit
        }
        Type::Tuple(sub) => {
            Type::Tuple(sub.iter().map(|i| type_inference_replace_vars_with_concrete_types(vars, i)).collect())
        }
        Type::Record(sub) => {
            Type::Record(sub.iter().map(|(n, i)| (n.clone(), type_inference_replace_vars_with_concrete_types(vars, i))).collect())
        }
        Type::RecExt(name, sub) => {
            Type::RecExt(name.clone(), sub.iter().map(|(n, i)| (n.clone(), type_inference_replace_vars_with_concrete_types(vars, i))).collect())
        }
    }
}

// Float, number => [number => Float]
// Float -> Float, number -> number => [number => Float]
// number -> Float, number -> number => [number => Float]
pub fn type_inference_find_var_replacements(vars: &mut HashMap<String, Type>, arg: &Type, fun_in: &Type) {
    match fun_in {
        Type::Var(name) => {
            if let Type::Var(_) = arg {
                if name != "number" {
                    vars.insert(name.clone(), arg.clone());
                }
            } else {
                vars.insert(name.clone(), arg.clone());
            }
        }
        Type::Tag(_, sub_in) => {
            if let Type::Tag(_, sub_arg) = arg {
                sub_in.iter().zip(sub_arg).for_each(|(i, a)| type_inference_find_var_replacements(vars, a, i));
            }
        }
        Type::Fun(a_in, b_in) => {
            if let Type::Fun(a_arg, b_arg) = arg {
                type_inference_find_var_replacements(vars, a_arg, a_in);
                type_inference_find_var_replacements(vars, b_arg, b_in);
            }
        }
        Type::Unit => {}
        Type::Tuple(sub_in) => {
            if let Type::Tuple(sub_arg) = arg {
                sub_in.iter().zip(sub_arg).for_each(|(i, a)| type_inference_find_var_replacements(vars, a, i));
            }
        }
        Type::Record(sub_in) => {
            if let Type::Record(sub_arg) = arg {
                // TODO entries may not be ordered
                sub_in.iter().zip(sub_arg).for_each(|((_, i), (_, a))| type_inference_find_var_replacements(vars, a, i));
            }
        }
        Type::RecExt(_, sub_in) => {
            if let Type::RecExt(_, sub_arg) = arg {
                // TODO entries may not be ordered
                sub_in.iter().zip(sub_arg).for_each(|((_, i), (_, a))| type_inference_find_var_replacements(vars, a, i));
            }
        }
    }
}

pub fn type_inference_type_from_expected(env: &mut StaticEnv, expected: &Type, value: &Type) -> Type {
    match value {
        Type::Var(_) => {
            expected.clone()
        }
        Type::Tag(val_name, val_sub) => {
            if let Type::Tag(exp_name, exp_sub) = expected {
                if val_name == exp_name {
                    let sub = exp_sub.iter()
                        .zip(val_sub)
                        .map(|(e, v)| type_inference_type_from_expected(env, e, v))
                        .collect::<Vec<Type>>();

                    Type::Tag(val_name.clone(), sub)
                } else {
                    value.clone()
                }
            } else {
                value.clone()
            }
        }
        Type::Fun(val_in, val_out) => {
            if let Type::Fun(exp_in, exp_out) = expected {
                Type::Fun(
                    Box::from(type_inference_type_from_expected(env, exp_in, val_in)),
                    Box::from(type_inference_type_from_expected(env, exp_out, val_out)),
                )
            } else {
                value.clone()
            }
        }
        Type::Unit => {
            value.clone()
        }
        Type::Tuple(val) => {
            if let Type::Tuple(exp) = expected {
                Type::Tuple(
                    exp.iter().zip(val)
                        .map(|(e, v)| type_inference_type_from_expected(env, e, v))
                        .collect::<Vec<Type>>()
                )
            } else {
                value.clone()
            }
        }
        Type::Record(val) => {
            if let Type::Record(exp) = expected {
                Type::Record(
                    exp.iter().zip(val)
                        .map(|((ne, e), (_, v))| (ne.clone(), type_inference_type_from_expected(env, e, v)))
                        .collect::<Vec<(String, Type)>>()
                )
            } else {
                value.clone()
            }
        }
        Type::RecExt(val_name, val) => {
            if let Type::RecExt(exp_name, exp) = expected {
                if val_name == exp_name {
                    Type::Record(
                        exp.iter().zip(val)
                            .map(|((ne, e), (_, v))| (ne.clone(), type_inference_type_from_expected(env, e, v)))
                            .collect::<Vec<(String, Type)>>()
                    )
                } else {
                    value.clone()
                }
            } else {
                value.clone()
            }
        }
    }
}

pub fn type_inference_rename_variables(env: &mut StaticEnv, vars: &mut HashMap<String, String>, ty: Type) -> Type {
    match ty {
        Type::Unit => {
            Type::Unit
        }
        Type::Var(name) => {
            match vars.get(&name).cloned() {
                Some(name) => {
                    Type::Var(name)
                }
                None => {
                    if name == "number" {
                        Type::Var(name)
                    } else {
                        let new_name = env.next_name();
                        vars.insert(name, new_name.clone());
                        Type::Var(new_name)
                    }
                }
            }
        }
        Type::Tag(name, sub) => {
            Type::Tag(name, sub.into_iter().map(|i| type_inference_rename_variables(env, vars, i)).collect())
        }
        Type::Fun(i, o) => {
            Type::Fun(Box::from(type_inference_rename_variables(env, vars, *i)), Box::from(type_inference_rename_variables(env, vars, *o)))
        }
        Type::Tuple(sub) => {
            Type::Tuple(sub.into_iter().map(|i| type_inference_rename_variables(env, vars, i)).collect())
        }
        Type::Record(sub) => {
            Type::Record(sub.into_iter().map(|(s, i)| (s, type_inference_rename_variables(env, vars, i))).collect())
        }
        Type::RecExt(name, sub) => {
            Type::RecExt(name, sub.into_iter().map(|(s, i)| (s, type_inference_rename_variables(env, vars, i))).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use constructors::type_char;
    use constructors::type_number;
    use test_utils::Test;
    use util::StringConversion;

    use super::*;

    fn analyze_expression(analyzer: &mut Analyzer, expr: &Expr) -> Result<Type, TypeError> {
        let expr = analyzer.analyze_expression(None, expr)?;
        Ok(expr_type(&expr))
    }

    #[test]
    fn check_unit() {
        let (expr, mut analyzer) = Test::expr_analyzer("()");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Unit));
    }

    #[test]
    fn check_literal() {
        let (expr, mut analyzer) = Test::expr_analyzer("123");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Var("number".s())));
    }

    #[test]
    fn check_fun() {
        let (expr, mut analyzer) = Test::expr_analyzer("fun 123");

        analyzer.env.add_definition("fun", Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Tag("Int".s(), vec![])),
        ));

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_if() {
        let (expr, mut analyzer) = Test::expr_analyzer("if True then 1 else 0");

        analyzer.env.add_definition("True", Type::Tag("Bool".s(), vec![]));

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Var("number".s())));
    }

    #[test]
    fn check_lambda() {
        let (expr, mut analyzer) = Test::expr_analyzer("\\x -> 1");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Fun(
            Box::new(Type::Var("a".s())),
            Box::new(Type::Var("number".s())),
        )));
    }

    #[test]
    fn check_list() {
        let (expr, mut analyzer) = Test::expr_analyzer("[1, 2, 3]");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Tag(
            "List".s(), vec![Type::Var("number".s())],
        )));
    }

    #[test]
    fn check_bad_list() {
        let (expr, mut analyzer) = Test::expr_analyzer("[1, 2, 'a']");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Err(
            TypeError::ListNotHomogeneous(span(&expr), type_number(), type_char(), 2)
        ));
    }

    #[test]
    fn check_record() {
        let (expr, mut analyzer) = Test::expr_analyzer("{ a = 1, b = \"Hi\" }");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(
            Type::Record(vec![
                ("a".s(), Type::Var("number".s())),
                ("b".s(), Type::Tag("String".s(), vec![])),
            ])
        ));
    }

    #[test]
    fn check_operator_chain() {
        let (expr, mut analyzer) = Test::expr_analyzer("1 + 2");

        analyzer.env.add_definition("+", Type::Fun(
            Box::new(Type::Var("number".s())),
            Box::new(Type::Fun(
                Box::new(Type::Var("number".s())),
                Box::new(Type::Var("number".s())),
            )),
        ));

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(
            Type::Var("number".s())
        ));
    }

    #[test]
    fn check_tuple() {
        let (expr, mut analyzer) = Test::expr_analyzer("(1, \"a\", ())");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(
            Type::Tuple(vec![
                Type::Var("number".s()),
                Type::Tag("String".s(), vec![]),
                Type::Unit,
            ])
        ));
    }

    #[test]
    fn check_record_update() {
        let (expr, mut analyzer) = Test::expr_analyzer("{ x | a = 0 }");

        // Type of x
        let record_type = Type::Record(vec![
            ("a".s(), Type::Var("number".s())),
            ("b".s(), Type::Var("number".s())),
        ]);

        // Type of expr
        let result_type = Type::RecExt("x".s(), vec![
            ("a".s(), Type::Var("number".s()))
        ]);

        analyzer.env.add_definition("x", record_type.clone());

        let result = analyze_expression(&mut analyzer, &expr);
        assert_eq!(result, Ok(result_type));
    }

    #[test]
    fn check_case() {
        let (expr, mut analyzer) = Test::expr_analyzer("case 0 of\n 0 -> \"a\"\n _ -> \"b\"");

        assert_eq!(analyze_expression(&mut analyzer, &expr), Ok(Type::Tag("String".s(), vec![])));
    }

    #[test]
    fn check_case2() {
        let (expr, mut analyzer) = Test::expr_analyzer("case 0 of\n 0 -> 1\n _ -> \"b\"");

        assert_eq!(
            analyze_expression(&mut analyzer, &expr),
            Err(TypeError::CaseBranchDontMatchReturnType((24, 27), "".s()))
        );
    }
}
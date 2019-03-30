use std::collections::HashMap;
use std::sync::Arc;

use analyzer::Analyser;
use analyzer::pattern_analyzer::analyze_pattern;
use analyzer::pattern_analyzer::analyze_pattern_with_type;
use analyzer::PatternMatchingError;
use analyzer::static_env::StaticEnv;
use analyzer::type_helper::calculate_common_type;
use analyzer::type_helper::get_common_type;
use analyzer::type_helper::is_assignable;
use analyzer::type_of_value;
use ast::*;
use errors::TypeError::*;
use errors::TypeError;
use typed_ast::expr_type;
use types::Adt;
use types::Value;
use util::build_fun_type;
use util::expression_fold::create_expr_tree;
use util::expression_fold::ExprTree;
use util::qualified_name;
use util::StringConversion;

pub fn analyze_expression(env: &mut StaticEnv, expected: Option<&Type>, expr: &Expr) -> Result<Type, TypeError> {
    let expr = Analyser::from(env.clone()).analyze_expression(expected, expr)?;
    Ok(expr_type(&expr))
}

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

pub fn backtrack_expr(env: &mut StaticEnv, vars: &HashMap<String, Type>, expr: &Expr) {
    match expr {
        Expr::Unit(..) => {}
        Expr::Literal(_, _) => {}
        Expr::RecordAccess(_, _) => {}
        Expr::Tuple(_, items) => {
            items.iter().for_each(|i| backtrack_expr(env, vars, i));
        }
        Expr::List(_, items) => {
            items.iter().for_each(|i| backtrack_expr(env, vars, i));
        }
        Expr::Record(_, items) => {
            items.iter().for_each(|(_, i)| backtrack_expr(env, vars, i));
        }
        Expr::RecordUpdate(_, _, items) => {
            items.iter().for_each(|(_, i)| backtrack_expr(env, vars, i));
        }
        Expr::RecordField(_, e, _) => {
            backtrack_expr(env, vars, e);
        }
        Expr::If(_, a, b, c) => {
            backtrack_expr(env, vars, a);
            backtrack_expr(env, vars, b);
            backtrack_expr(env, vars, c);
        }
        Expr::Lambda(_, _, e) => {
            backtrack_expr(env, vars, e);
        }
        Expr::Application(_, a, b) => {
            backtrack_expr(env, vars, a);
            backtrack_expr(env, vars, b);
        }
        Expr::Let(_, _, e) => {
            backtrack_expr(env, vars, e);
        }
        Expr::Case(_, e, items) => {
            backtrack_expr(env, vars, e);
            items.iter().for_each(|(_, i)| backtrack_expr(env, vars, i));
        }
        Expr::OpChain(_, exprs, ops) => {
            match create_expr_tree(exprs, ops) {
                Ok(tree) => backtrack_expr(env, vars, &expr_tree_to_expr(tree)),
                Err(_) => {}
            }
        }
        Expr::QualifiedRef(_, path, name) => {
            let full_name = qualified_name(path, name);

            backtrack_expr(env, vars, &Expr::Ref((0, 0), full_name))
        }
        Expr::Ref(_, variable) => {
            match env.find_definition(variable) {
                Some(variable_type) => {
                    env.replace(variable, replace_vars_with_concrete_types(vars, &variable_type))
                }
                None => {}
            }
        }
    }
}

pub fn get_adt_type(name: &String, vars: &Vec<Value>, adt: Arc<Adt>) -> Type {
    let variant = adt.variants.iter().find(|var| &var.name == name).unwrap();

    let mut var_replacement: HashMap<String, Type> = HashMap::new();
    let value_types: Vec<Type> = vars.iter().map(|v| type_of_value(v)).collect();

    find_var_replacements(&mut var_replacement, &Type::Tuple(value_types), &Type::Tuple(variant.types.clone()));

    let final_types = adt.types.iter()
        .map(|ty| {
            var_replacement.get(ty).cloned().unwrap_or_else(|| Type::Var(ty.clone()))
        })
        .collect();

    Type::Tag(adt.name.clone(), final_types)
}

pub fn replace_vars_with_concrete_types(vars: &HashMap<String, Type>, ty: &Type) -> Type {
    match ty {
        Type::Var(name) => {
            vars.get(name).unwrap_or(ty).clone()
        }
        Type::Tag(name, sub) => {
            Type::Tag(name.clone(), sub.iter().map(|i| replace_vars_with_concrete_types(vars, i)).collect())
        }
        Type::Fun(a, b) => {
            Type::Fun(
                Box::from(replace_vars_with_concrete_types(vars, a)),
                Box::from(replace_vars_with_concrete_types(vars, b)),
            )
        }
        Type::Unit => {
            Type::Unit
        }
        Type::Tuple(sub) => {
            Type::Tuple(sub.iter().map(|i| replace_vars_with_concrete_types(vars, i)).collect())
        }
        Type::Record(sub) => {
            Type::Record(sub.iter().map(|(n, i)| (n.clone(), replace_vars_with_concrete_types(vars, i))).collect())
        }
        Type::RecExt(name, sub) => {
            Type::RecExt(name.clone(), sub.iter().map(|(n, i)| (n.clone(), replace_vars_with_concrete_types(vars, i))).collect())
        }
    }
}

// Float, number => [number => Float]
// Float -> Float, number -> number => [number => Float]
// number -> Float, number -> number => [number => Float]
pub fn find_var_replacements(vars: &mut HashMap<String, Type>, arg: &Type, fun_in: &Type) {
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
                sub_in.iter().zip(sub_arg).for_each(|(i, a)| find_var_replacements(vars, a, i));
            }
        }
        Type::Fun(a_in, b_in) => {
            if let Type::Fun(a_arg, b_arg) = arg {
                find_var_replacements(vars, a_arg, a_in);
                find_var_replacements(vars, b_arg, b_in);
            }
        }
        Type::Unit => {}
        Type::Tuple(sub_in) => {
            if let Type::Tuple(sub_arg) = arg {
                sub_in.iter().zip(sub_arg).for_each(|(i, a)| find_var_replacements(vars, a, i));
            }
        }
        Type::Record(sub_in) => {
            if let Type::Record(sub_arg) = arg {
                // TODO entries may not be ordered
                sub_in.iter().zip(sub_arg).for_each(|((_, i), (_, a))| find_var_replacements(vars, a, i));
            }
        }
        Type::RecExt(_, sub_in) => {
            if let Type::RecExt(_, sub_arg) = arg {
                // TODO entries may not be ordered
                sub_in.iter().zip(sub_arg).for_each(|((_, i), (_, a))| find_var_replacements(vars, a, i));
            }
        }
    }
}

pub fn type_from_expected(env: &mut StaticEnv, expected: &Type, value: &Type) -> Type {
    match value {
        Type::Var(_) => {
            expected.clone()
        }
        Type::Tag(val_name, val_sub) => {
            if let Type::Tag(exp_name, exp_sub) = expected {
                if val_name == exp_name {
                    let sub = exp_sub.iter()
                        .zip(val_sub)
                        .map(|(e, v)| type_from_expected(env, e, v))
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
                    Box::from(type_from_expected(env, exp_in, val_in)),
                    Box::from(type_from_expected(env, exp_out, val_out)),
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
                        .map(|(e, v)| type_from_expected(env, e, v))
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
                        .map(|((ne, e), (_, v))| (ne.clone(), type_from_expected(env, e, v)))
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
                            .map(|((ne, e), (_, v))| (ne.clone(), type_from_expected(env, e, v)))
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

pub fn rename_variables(env: &mut StaticEnv, vars: &mut HashMap<String, String>, ty: Type) -> Type {
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
            Type::Tag(name, sub.into_iter().map(|i| rename_variables(env, vars, i)).collect())
        }
        Type::Fun(i, o) => {
            Type::Fun(Box::from(rename_variables(env, vars, *i)), Box::from(rename_variables(env, vars, *o)))
        }
        Type::Tuple(sub) => {
            Type::Tuple(sub.into_iter().map(|i| rename_variables(env, vars, i)).collect())
        }
        Type::Record(sub) => {
            Type::Record(sub.into_iter().map(|(s, i)| (s, rename_variables(env, vars, i))).collect())
        }
        Type::RecExt(name, sub) => {
            Type::RecExt(name, sub.into_iter().map(|(s, i)| (s, rename_variables(env, vars, i))).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use constructors::type_char;
    use constructors::type_number;
    use test_utils::Test;

    use super::*;

    #[test]
    fn check_unit() {
        let expr = Test::expr("()");
        let mut env = StaticEnv::new();
        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Unit));
    }

    #[test]
    fn check_literal() {
        let expr = Test::expr("123");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Var("number".s())));
    }

    #[test]
    fn check_fun() {
        let expr = Test::expr("fun 123");
        let mut env = StaticEnv::new();

        env.add_definition("fun", Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Tag("Int".s(), vec![])),
        ));

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag("Int".s(), vec![])));
    }

    #[test]
    fn check_if() {
        let expr = Test::expr("if True then 1 else 0");
        let mut env = StaticEnv::new();

        env.add_definition("True", Type::Tag("Bool".s(), vec![]));

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Var("number".s())));
    }

    #[test]
    fn check_lambda() {
        let expr = Test::expr("\\x -> 1");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Fun(
            Box::new(Type::Var("a".s())),
            Box::new(Type::Var("number".s())),
        )));
    }

    #[test]
    fn check_list() {
        let expr = Test::expr("[1, 2, 3]");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag(
            "List".s(), vec![Type::Var("number".s())],
        )));
    }

    #[test]
    fn check_bad_list() {
        let expr = Test::expr("[1, 2, 'a']");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Err(
            ListNotHomogeneous(span(&expr), type_number(), type_char(), 2)
        ));
    }

    #[test]
    fn check_record() {
        let expr = Test::expr("{ a = 1, b = \"Hi\" }");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(
            Type::Record(vec![
                ("a".s(), Type::Var("number".s())),
                ("b".s(), Type::Tag("String".s(), vec![])),
            ])
        ));
    }

    #[test]
    fn check_operator_chain() {
        let expr = Test::expr("1 + 2");
        let mut env = StaticEnv::new();

        env.add_definition("+", Type::Fun(
            Box::new(Type::Var("number".s())),
            Box::new(Type::Fun(
                Box::new(Type::Var("number".s())),
                Box::new(Type::Var("number".s())),
            )),
        ));

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(
            Type::Var("number".s())
        ));
    }

    #[test]
    fn check_tuple() {
        let expr = Test::expr("(1, \"a\", ())");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(
            Type::Tuple(vec![
                Type::Var("number".s()),
                Type::Tag("String".s(), vec![]),
                Type::Unit,
            ])
        ));
    }

    #[test]
    fn check_record_update() {
        let expr = Test::expr("{ x | a = 0 }");
        let mut env = StaticEnv::new();

        // Type of x
        let record_type = Type::Record(vec![
            ("a".s(), Type::Var("number".s())),
            ("b".s(), Type::Var("number".s())),
        ]);

        // Type of expr
        let result_type = Type::RecExt("x".s(), vec![
            ("a".s(), Type::Var("number".s()))
        ]);

        env.add_definition("x", record_type.clone());

        let result = analyze_expression(&mut env, None, &expr);
        assert_eq!(result, Ok(result_type));
    }

    #[test]
    fn check_case() {
        let expr = Test::expr("case 0 of\n 0 -> \"a\"\n _ -> \"b\"");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Ok(Type::Tag("String".s(), vec![])));
    }

    #[test]
    fn check_case2() {
        let expr = Test::expr("case 0 of\n 0 -> 1\n _ -> \"b\"");
        let mut env = StaticEnv::new();

        assert_eq!(analyze_expression(&mut env, None, &expr), Err(CaseBranchDontMatchReturnType((24, 27), "".s())));
    }
}
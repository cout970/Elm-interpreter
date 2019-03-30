use std::collections::HashMap;
use std::ops::Deref;
use std::str::Chars;

use analyzer::expression_analyzer::analyze_expression;
use analyzer::expression_analyzer::backtrack_expr;
use analyzer::expression_analyzer::expr_tree_to_expr;
use analyzer::expression_analyzer::find_var_replacements;
use analyzer::expression_analyzer::get_adt_type;
use analyzer::expression_analyzer::rename_variables;
use analyzer::expression_analyzer::replace_vars_with_concrete_types;
use analyzer::expression_analyzer::type_from_expected;
use analyzer::expression_helper::*;
use analyzer::function_analyzer::analyze_function;
use analyzer::pattern_analyzer::analyze_pattern;
use analyzer::pattern_analyzer::analyze_pattern_with_type;
use analyzer::pattern_analyzer::is_exhaustive;
use analyzer::static_env::StaticEnv;
use analyzer::type_helper::calculate_common_type;
use analyzer::type_helper::get_common_type;
use analyzer::type_helper::is_assignable;
use ast::Definition;
use ast::Expr;
use ast::Float;
use ast::LetDeclaration;
use ast::Literal;
use ast::Pattern;
use ast::span;
use ast::Type;
use constructors::type_bool;
use errors::*;
use source::SourceCode;
use typed_ast::expr_type;
use typed_ast::LetEntry;
use typed_ast::TypedDefinition;
use typed_ast::TypedExpr;
use types::Function;
use types::Value;
use util::build_fun_type;
use util::create_vec_inv;
use util::expression_fold::ExprTreeError;
use util::qualified_name;
use util::StringConversion;

pub mod static_env;
pub mod module_analyser;
mod function_analyzer;
mod expression_analyzer;
mod dependency_sorter;
mod pattern_analyzer;
mod type_helper;

mod expression_helper;

#[derive(Clone, Debug, PartialEq)]
pub enum PatternMatchingError {
    ListPatternsAreNotHomogeneous(Type, Type),
    UnknownOperatorPattern(String),
    UnknownAdtVariant(String),
    ExpectedListType(Type),
    ExpectedUnit(Type),
    ExpectedTuple(Pattern, Type),
    ExpectedRecord(Type),
    ExpectedAdt(String, Type),
    PatternNotExhaustive(Pattern),
    InvalidRecordEntryName(String),
    ExpectedLiteral(String, Type),
}

pub struct Analyser {
    env: StaticEnv
}

impl Analyser {
    pub fn new() -> Self {
        Analyser { env: StaticEnv::new() }
    }

    pub fn from(env: StaticEnv) -> Self {
        Analyser { env }
    }

    pub fn analyze_definition(&mut self, fun: &Definition) -> Result<TypedDefinition, TypeError> {
        println!("analyze_function: {}", fun.name);
        let name_seq = self.env.name_seq.save();

        // Extract function input types and argument variables
        let (argument_types, argument_vars) = self.analyze_function_arguments(&fun.patterns, &fun.header)?;
        self.env.enter_block();

        // Add function arguments
        for (arg_name, ty) in &argument_vars {
            self.env.add_definition(arg_name, ty.clone());
        }

        // Analyse function expression
        let return_expr = if let Some(ty) = &fun.header {
            // Register own type to be able to call the function recursively
            self.env.add_definition(&fun.name, ty.clone());

            // TODO use the number of defined arguments to obtain the [return_type]

            // Convert `a -> b -> c` into vec![a, b, c]
            let fn_types = unpack_types(ty);
            // Extract the last value, `c`
            let return_type = fn_types.last().expect("Expected last value to exist");

            // Analyse function body
            self.analyze_expression(Some(return_type), &fun.expr)
        } else {

            // Create a self function type with a variable as output type, we don't know
            // the return type yet, it must be inferred
            let self_type = create_vec_inv(&argument_types, Type::Var("z".s()));
            // Register own type to be able to call the function recursively
            self.env.add_definition(&fun.name, build_fun_type(&self_type));

            // Analyse function body
            self.analyze_expression(None, &fun.expr)
        };

        // If the expression analysis failed, we return the error
        let return_expr = match return_expr {
            Ok(expr) => expr,
            Err(e) => {
                // The environment must be restored
                self.env.exit_block();
                self.env.name_seq.restore(name_seq);
                return Err(e);
            }
        };

        // Extract or update the final function type
        let fun_type = if fun.header.is_none() {
            let mut final_arg_types: Vec<Type> = vec![];

            // Update argument variable with concrete types
            'outer: for arg in &argument_types {
                if let Type::Var(arg_var_name) = arg {

                    // search in local variables for the type of this variable,
                    // this is needed because the number of arguments and local variables can be different
                    for (name, ty) in &argument_vars {
                        if let Type::Var(local_var_name) = ty {
                            if local_var_name == arg_var_name {
                                if let Some(ty) = self.env.find_definition(name) {
                                    final_arg_types.push(ty);
                                    continue 'outer;
                                }
                            }
                        }
                    }

                    // TODO remove panics in the middle of code
                    panic!("Unable to find variable '{}' in {:?}, for function: {}", &arg, argument_vars, fun.name);
                } else {
                    final_arg_types.push(arg.clone());
                }
            }

            final_arg_types.push(expr_type(&return_expr));
            build_fun_type(&final_arg_types)
        } else {
            fun.header.clone().unwrap()
        };

        self.env.exit_block();
        self.env.name_seq.restore(name_seq);

        // TODO check fun type and expr type are the same

        Ok(TypedDefinition {
            header: fun_type,
            name: fun.name.clone(),
            patterns: fun.patterns.clone(),
            expr: return_expr,
        })
    }

    pub fn analyze_function_arguments(&mut self, patterns: &Vec<Pattern>, func_ty: &Option<Type>) -> Result<(Vec<Type>, Vec<(String, Type)>), TypeError> {
        let mut arguments: Vec<Type> = vec![];
        let mut argument_vars: Vec<(String, Type)> = vec![];

        let iter: Vec<(Option<Type>, &Pattern)> = match func_ty {
            Some(ty) => {
                let list = unpack_types(ty);

                if patterns.len() > list.len() {
                    eprintln!("patterns: {:?}", patterns);
                    eprintln!("list: {:?}", list);
                    return Err(TypeError::InvalidPatternAmount(list.len(), patterns.len()));
                }

                list.into_iter().zip(patterns).map(|(ty, pat)| (Some(ty), pat)).collect()
            }
            _ => {
                patterns.iter().map(|p| (None, p)).collect()
            }
        };

        for (ty, patt) in iter {
            if !is_exhaustive(patt) {
                // TODO
                return Err(TypeError::InvalidPattern((0, 0), PatternMatchingError::PatternNotExhaustive(patt.clone())));
            }

            let (ty, vars) = match ty {
                Some(ty) => {
                    // TODO
                    analyze_pattern_with_type(&mut self.env, patt, ty)
                        .map_err(|e| TypeError::InvalidPattern((0, 0), e))?
                }
                None => {
                    //TODO
                    analyze_pattern(&mut self.env, patt)
                        .map_err(|e| TypeError::InvalidPattern((0, 0), e))?
                }
            };

            arguments.push(ty);
            for pair in vars {
                argument_vars.push(pair);
            }
        }

        Ok((arguments, argument_vars))
    }

    pub fn analyze_expression(&mut self, expected: Option<&Type>, expr: &Expr) -> Result<TypedExpr, TypeError> {
        match expr {
            Expr::Application(_, fun, arg) => {
                self.analyze_application(&**fun, &**arg, expr)
            }
            Expr::Ref(span, name) => {
                self.analyze_expression_ref(expected, *span, name)
            }
            Expr::QualifiedRef(span, path, name) => {
                self.analyze_expression_ref(expected, *span, &qualified_name(path, name))
            }
            Expr::Lambda(span, patterns, expr) => {
                self.analyze_expression_lambda(expected, *span, patterns, expr)
            }
            Expr::List(span, exprs) => {
                self.analyze_expression_list(expected, *span, exprs)
            }
            Expr::Let(span, decls, expr) => {
                self.analyze_expression_let(expected, *span, decls, expr)
            }
            Expr::Record(span, entries) => {
                self.analyze_expression_record(expected, *span, entries)
            }
            Expr::RecordAccess(span, name) => {
                self.analyze_expression_record_access(expected, *span, name)
            }
            Expr::RecordField(span, expr, name) => {
                self.analyze_expression_record_field(expected, *span, expr, name)
            }
            Expr::Tuple(span, items) => {
                self.analyze_expression_tuple(expected, *span, items)
            }
            Expr::RecordUpdate(span, name, updates) => {
                self.analyze_expression_record_update(expected, *span, name, updates)
            }
            Expr::Case(span, expr, branches) => {
                self.analyze_expression_case(expected, *span, &*expr, branches)
            }
            Expr::If(span, cond, a, b) => {
                self.analyze_expression_if(expected, *span, cond, a, b)
            }
            Expr::OpChain(span, exprs, ops) => {
                self.analyze_expression_chain(expected, *span, exprs, ops)
            }
            Expr::Literal(_, lit) => {
                self.analyze_expression_literal(expected, lit)
            }
            Expr::Unit(..) => Ok(TypedExpr::Const(Value::Unit)),
        }
    }
}

fn unpack_types(ty: &Type) -> Vec<Type> {
    let mut curr = ty.clone();
    let mut components = vec![];

    while let Type::Fun(a, b) = curr {
        components.push((*a).clone());
        curr = (*b).clone();
    }
    components.push(curr.clone());
    components
}

pub fn type_check_expression(env: &mut StaticEnv, expr: &Expr) -> Result<Type, TypeError> {
    analyze_expression(env, None, expr)
}

pub fn type_check_function(env: &mut StaticEnv, fun: &Definition) -> Result<Type, TypeError> {
    analyze_function(env, fun)
}

pub fn type_of_value(value: &Value) -> Type {
    match value {
        Value::Unit => {
            Type::Unit
        }
        Value::Number(_) => {
            Type::Var("number".s())
        }
        Value::Int(_) => {
            Type::Tag("Int".s(), vec![])
        }
        Value::Float(_) => {
            Type::Tag("Float".s(), vec![])
        }
        Value::String(_) => {
            Type::Tag("String".s(), vec![])
        }
        Value::Char(_) => {
            Type::Tag("Char".s(), vec![])
        }
        Value::List(items) => {
            if items.is_empty() {
                Type::Tag("List".s(), vec![Type::Var("a".s())])
            } else {
                Type::Tag("List".s(), vec![type_of_value(items.first().unwrap())])
            }
        }
        Value::Tuple(items) => {
            Type::Tuple(items.iter().map(|i| type_of_value(i)).collect())
        }
        Value::Record(items) => {
            Type::Record(items.iter().map(|(s, i)| (s.to_owned(), type_of_value(i))).collect())
        }
        Value::Adt(var_name, items, adt) => {
            get_adt_type(var_name, items, adt.clone())
        }
        Value::Fun { fun, args, .. } => {
            let fun_ty = type_of_function(fun.deref());

            strip_fun_args(args.len(), &fun_ty).clone()
        }
    }
}

fn type_of_function(fun: &Function) -> &Type {
    match fun {
        Function::External(_, _, ty) => ty,
        Function::Wrapper(_, _, ty) => ty,
        Function::Expr(_, _, _, ty) => ty,
    }
}

fn strip_fun_args(args: usize, ty: &Type) -> &Type {
    if args == 0 {
        return ty;
    }

    if let Type::Fun(_, ref output) = ty {
        strip_fun_args(args - 1, output)
    } else {
        ty
    }
}

#[cfg(test)]
mod tests {
    use ast::Statement;
    use constructors::*;
    use core::register_core;
    use test_utils::Test;

    use super::*;

    fn from_code_def(code: &str) -> Definition {
        let stm = Test::statement(code);
        match stm {
            Statement::Def(def) => def,
            _ => panic!("Expected definition but found: {:?}", stm)
        }
    }

    fn format_type(env: &mut StaticEnv, def: &Definition) -> String {
        format!("{}", analyze_function(env, def).expect("Run into type error"))
    }

    #[test]
    fn check_constant() {
        let def = from_code_def("const = 1");
        let mut env = StaticEnv::new();

        assert_eq!(format_type(&mut env, &def), "number");
    }

    #[test]
    fn check_identity() {
        let def = from_code_def("id arg1 = arg1");
        let mut env = StaticEnv::new();

        assert_eq!(format_type(&mut env, &def), "a -> a");
    }

    #[test]
    fn check_var_to_number() {
        let def = from_code_def("sum arg1 arg2 = arg1 + arg2");
        let mut env = StaticEnv::new();

        env.add_definition("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut env, &def), "number -> number -> number");
    }

    #[test]
    fn check_number_to_float() {
        let def = from_code_def("sum arg1 = arg1 + 1.5");
        let mut env = StaticEnv::new();

        env.add_definition("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut env, &def), "Float -> Float");
    }

    #[test]
    fn check_from_number_to_float() {
        let def = from_code_def("sum = (+) 1.5");
        let mut env = StaticEnv::new();

        env.add_definition("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut env, &def), "Float -> Float");
    }

    #[test]
    fn check_list_coercion() {
        let def = from_code_def("my = [1, 1.5]");
        let mut env = StaticEnv::new();

        assert_eq!(format_type(&mut env, &def), "List Float");
    }

    #[test]
    fn check_list_coercion2() {
        let def = from_code_def("my b = [1, 1.5, b]");
        let mut env = StaticEnv::new();

        assert_eq!(format_type(&mut env, &def), "Float -> List Float");
    }

    #[test]
    fn check_variable_separation() {
        let def = from_code_def("my a b = [a, b]");
        let mut env = StaticEnv::new();

        assert_eq!(format_type(&mut env, &def), "a -> a -> List a");
    }

    #[test]
    fn check_variable_separation2() {
        let def = from_code_def("my = (func, func)");
        let mut env = StaticEnv::new();

        env.add_definition("func", Type::Fun(
            Box::from(Type::Var("a".s())),
            Box::from(Type::Var("a".s())),
        ));

        assert_eq!(format_type(&mut env, &def), "( a -> a, b -> b )");
    }

    #[test]
    fn analyze_patterns_1() {
        analyze_pattern_test(
            type_int(),
            pattern_var("a"),
            "Int",
            r#"[("a", Tag("Int", []))]"#,
        );
    }

    #[test]
    #[ignore]
    fn analyze_patterns_2() {
        analyze_pattern_test(
            type_tag_args("Maybe", vec![type_var("item")]),
            pattern_tag_args("Just", vec![pattern_var("a")]),
            "Maybe item",
            r#"[("a", Var("item"))]"#,
        );
    }

    #[test]
    fn analyze_patterns_3() {
        analyze_pattern_test(
            type_int(),
            pattern_wildcard(),
            "Int",
            r#"[]"#,
        );
    }

    #[test]
    fn analyze_patterns_4() {
        analyze_pattern_test(
            type_unit(),
            pattern_unit(),
            "()",
            r#"[]"#,
        );
    }

    #[test]
    fn analyze_patterns_5() {
        analyze_pattern_test(
            type_tuple(vec![type_int(), type_unit()]),
            pattern_tuple(vec![pattern_var("a"), pattern_unit()]),
            "( Int, () )",
            r#"[("a", Tag("Int", []))]"#,
        );
    }

    #[test]
    fn analyze_patterns_6() {
        analyze_pattern_test(
            type_list(type_int()),
            pattern_list(vec![pattern_var("a"), pattern_var("b")]),
            "List Int",
            r#"[("a", Tag("Int", [])), ("b", Tag("Int", []))]"#,
        );
    }

    #[test]
    fn analyze_patterns_7() {
        analyze_pattern_test(
            type_record(vec![("x", type_int())]),
            pattern_record(vec!["x"]),
            "{ x : Int }",
            r#"[("x", Tag("Int", []))]"#,
        );
    }

    #[test]
    fn analyze_patterns_8() {
        analyze_pattern_test(
            type_list(type_int()),
            pattern_cons(pattern_var("x"), pattern_var("xs")),
            "List Int",
            r#"[("x", Tag("Int", [])), ("xs", Tag("List", [Tag("Int", [])]))]"#,
        );
    }

    #[test]
    fn analyze_patterns_9() {
        analyze_pattern_test(
            type_int(),
            pattern_int(1),
            "Int",
            r#"[]"#,
        );
    }

    #[test]
    fn analyze_patterns_10() {
        analyze_pattern_test(
            type_int(),
            pattern_alias(pattern_int(1), "x"),
            "Int",
            r#"[("x", Tag("Int", []))]"#,
        );
    }

    fn analyze_pattern_test(ty: Type, pattern: Pattern, type_str: &str, vars_str: &str) {
        let mut env = StaticEnv::new();
        register_core(&mut env);

        let (res_ty, vars) = analyze_pattern_with_type(&mut env, &pattern, ty)
            .expect("Error");

        assert_eq!(format!("{}", res_ty), type_str);
        assert_eq!(format!("{:?}", vars), vars_str);
    }
}
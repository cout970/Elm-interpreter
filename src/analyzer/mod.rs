use std::collections::HashMap;

use analyzer::inference::{Env, infer_definition_type, tmp_map_patterns};
use analyzer::pattern_analyzer::analyze_pattern;
use analyzer::pattern_analyzer::analyze_pattern_with_type;
use analyzer::pattern_analyzer::is_exhaustive;
use analyzer::static_env::StaticEnv;
use analyzer::type_inference::type_inference_find_var_replacements;
use ast::*;
use ast::Definition;
use ast::Expr;
use builtin::ELM_CORE_MODULES;
use errors::*;
use interpreter::dynamic_env::RuntimeStack;
use loader::AnalyzedModule;
use loader::Declaration;
use loader::declaration_type;
use loader::LoadedModule;
use loader::ModuleImport;
use source::SourceCode;
use typed_ast::expr_type;
use typed_ast::TypedDefinition;
use typed_ast::TypedExpr;
use types::*;
use util::build_fun_type;
use util::create_vec_inv;
use util::qualified_name;

pub mod static_env;
mod type_inference;
mod dependency_sorter;
mod pattern_analyzer;
mod type_helper;

mod expression_helper;
mod statement_helper;
mod module_helper;
mod inference;

#[derive(Debug)]
pub struct Analyzer {
    env: StaticEnv,
    pub e: Env,
    source: SourceCode,
}

impl Analyzer {
    pub fn new(source: SourceCode) -> Self {
        Analyzer { env: StaticEnv::new(), e: Env::new(), source }
    }

    pub fn with(&self, source: SourceCode) -> Self {
        Analyzer { env: self.env.clone(), e: Env::new(), source }
    }

    pub fn add_definition(&mut self, name: &str, var: Type) {
        self.env.add_definition(name, var.clone());
        self.e.set(name, var);
    }

    pub fn analyze_definition(&mut self, fun: &Definition) -> Result<TypedDefinition, TypeError> {
        let name_seq = self.env.name_seq.save();
        return infer_definition_type(&mut self.e, fun);
//
//        // Extract function input types and argument variables
//        let (argument_types, argument_vars) = self.analyze_function_arguments(&fun.patterns, &fun.header)?;
//        self.env.enter_block();
//
//        // Add function arguments
//        for (arg_name, ty) in &argument_vars {
//            self.env.add_definition(arg_name, ty.clone());
//        }
//
//        // Analyse function expression
//        let return_expr = if let Some(ty) = &fun.header {
//            // Register own type to be able to call the function recursively
//            self.env.add_definition(&fun.name, ty.clone());
//
//            // TODO use the number of defined arguments to obtain the [return_type]
//
//            // Convert `a -> b -> c` into vec![a, b, c]
//            let fn_types = unpack_types(ty);
//            // Extract the last value, `c`
//            let return_type = fn_types.last().expect("Expected last value to exist");
//
//            // Analyse function body
//            self.analyze_expression_helper(Some(return_type), &fun.expr)
//        } else {
//
//            // Create a self function type with a variable as output type, we don't know
//            // the return type yet, it must be inferred
//            let self_type = create_vec_inv(&argument_types, Type::Var("z".to_string()));
//            // Register own type to be able to call the function recursively
//            self.env.add_definition(&fun.name, build_fun_type(&self_type));
//
//            // Analyse function body
//            self.analyze_expression_helper(None, &fun.expr)
//        };
//
//        // If the expression analysis failed, we return the error
//        let return_expr = match return_expr {
//            Ok(expr) => expr,
//            Err(e) => {
//                // The environment must be restored
//                self.env.exit_block();
//                self.env.name_seq.restore(name_seq);
//                return Err(e);
//            }
//        };
//
//        // Extract or update the final function type
//        let fun_type = if fun.header.is_none() {
//            let mut final_arg_types: Vec<Type> = vec![];
//
//            // Update argument variable with concrete types
//            'outer: for arg in &argument_types {
//                if let Type::Var(arg_var_name) = arg {
//
//                    // search in local variables for the type of this variable,
//                    // this is needed because the number of arguments and local variables can be different
//                    for (name, ty) in &argument_vars {
//                        if let Type::Var(local_var_name) = ty {
//                            if local_var_name == arg_var_name {
//                                if let Some(ty) = self.env.find_definition(name) {
//                                    final_arg_types.push(ty);
//                                    continue 'outer;
//                                }
//                            }
//                        }
//                    }
//
//                    // TODO remove panics in the middle of code
//                    panic!("Unable to find variable '{}' in {:?}, for function: {}", &arg, argument_vars, fun.name);
//                } else {
//                    final_arg_types.push(arg.clone());
//                }
//            }
//
//            final_arg_types.push(expr_type(&return_expr));
//            build_fun_type(&final_arg_types)
//        } else {
//            fun.header.clone().unwrap()
//        };
//
//        self.env.exit_block();
//        self.env.name_seq.restore(name_seq);
//
//        // TODO check fun type and expr type are the same
//
//        Ok(TypedDefinition {
//            header: fun_type,
//            name: fun.name.clone(),
//            patterns: tmp_map_patterns(&fun.patterns),
//            expr: return_expr,
//        })
    }

    pub fn analyze_function_arguments(&mut self, patterns: &Vec<Pattern>, func_ty: &Option<Type>) -> Result<(Vec<Type>, Vec<(String, Type)>), TypeError> {
        let mut arguments: Vec<Type> = vec![];
        let mut argument_vars: Vec<(String, Type)> = vec![];

        let iter: Vec<(Option<Type>, &Pattern)> = match func_ty {
            Some(ty) => {
                let list = unpack_types(ty);

                if patterns.len() > list.len() {
                    return Err(TypeError::InvalidFunctionPatternAmount {
                        expected: list.len(),
                        found: patterns.len(),
                    });
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
                return Err(TypeError::PatternMatchingError {
                    span: (0, 0),
                    info: PatternMatchingError::PatternNotExhaustive(patt.clone()),
                });
            };

            let (ty, vars) = match ty {
                Some(ty) => {
                    // TODO
                    analyze_pattern_with_type(&mut self.env, patt, ty)
                        .map_err(|info| TypeError::PatternMatchingError { span: (0, 0), info })?
                }
                None => {
                    //TODO
                    analyze_pattern(&mut self.env, patt)
                        .map_err(|info| TypeError::PatternMatchingError { span: (0, 0), info })?
                }
            };

            arguments.push(ty);
            for pair in vars {
                argument_vars.push(pair);
            }
        }

        Ok((arguments, argument_vars))
    }

    pub fn analyze_expression(&mut self, expr: &Expr) -> Result<TypedExpr, ElmError> {
        self.analyze_expression_helper(None, expr)
            .map_err(|e| ElmError::Analyser(self.source.clone(), e))
    }

    pub fn analyze_expression_helper(&mut self, expected: Option<&Type>, expr: &Expr) -> Result<TypedExpr, TypeError> {
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
                self.analyze_expression_list(*span, exprs)
            }
            Expr::Let(span, decls, expr) => {
                self.analyze_expression_let(expected, *span, decls, expr)
            }
            Expr::Record(_, entries) => {
                self.analyze_expression_record(entries)
            }
            Expr::RecordAccess(_, name) => {
                self.analyze_expression_record_access(name)
            }
            Expr::RecordField(_, expr, name) => {
                self.analyze_expression_record_field(expected, expr, name)
            }
            Expr::Tuple(_, items) => {
                self.analyze_expression_tuple(items)
            }
            Expr::RecordUpdate(span, name, updates) => {
                self.analyze_expression_record_update(*span, name, updates)
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
            Expr::Unit(..) => Ok(TypedExpr::Const(Value::Unit.get_type(), Value::Unit)),
        }
    }

    pub fn analyze_statement(&mut self, stm: &Statement) -> Result<Vec<Declaration>, TypeError> {
        let decls = match stm {
            Statement::Alias(name, vars, ty) => {
                self.analyze_statement_typealias(name, vars, ty)?
            }
            Statement::Adt(name, vars, variants) => {
                self.analyze_statement_adt(name, vars, variants)?
            }
            Statement::Port(name, ty) => {
                self.analyze_statement_port(name, ty)?
            }
            Statement::Def(def) => {
                self.analyze_statement_definition(def)?
            }
            Statement::Infix(_, _, name, def) => {
                if let Some(ty) = self.env.find_definition(name) {
                    vec![Declaration::Port(name.clone(), ty.clone()), Declaration::Infix(name.clone(), def.clone(), ty)]
                } else if let Some(ty) = self.env.find_definition(def) {
                    vec![Declaration::Port(name.clone(), ty.clone()), Declaration::Infix(name.clone(), def.clone(), ty)]
                } else {
                    eprintln!("Ignoring infix operator: {}, {}\n {:?}", name, def, self.env);
                    vec![]
                }
            }
        };

        Ok(decls)
    }

    pub fn analyze_module(&mut self, modules: &HashMap<String, AnalyzedModule>, module: &LoadedModule)
                          -> Result<AnalyzedModule, ElmError> {
        let imports = if ELM_CORE_MODULES.contains(&module.src.name.as_str()) {
            self.analyze_module_imports(modules, &module.ast.imports)?
        } else {
            let mut imports = self.get_default_imports(modules)?;
            imports.extend(self.analyze_module_imports(modules, &module.ast.imports)?);
            imports
        };

        // Avoid problems with statement order
        for stm in &module.ast.statements {
            if let Some(ty) = declared_statement_type(stm) {
                self.e.set(declared_statement_name(stm).unwrap(), ty.clone());
                self.env.add_definition(declared_statement_name(stm).unwrap(), ty.clone());
            }
        }

        // Custom behaviour for binary operators
        for stm in &module.ast.statements {
            if let Statement::Infix(_, _, name, def) = stm {
                if let Some(ty) = self.env.find_definition(def) {
                    self.e.set(name, ty.clone());
                    self.env.add_definition(name, ty);
                } else {
                    eprintln!("Infix operator {} where the function {} doesn't have a type header", name, def);
                }
            }
        }

        let declarations = self.analyze_module_declarations(&module.ast.statements)
            .map_err(|list| {
                err_list(&self.source, list, |code, info| ElmError::Analyser(code, info))
            })?;

        Ok(AnalyzedModule {
            name: module.src.name.to_string(),
            dependencies: module.dependencies.clone(),
            all_declarations: declarations,
            imports,
        })
    }
}

fn declared_statement_type(stm: &Statement) -> Option<&Type> {
    match stm {
        Statement::Alias(_, _, _) => None,
        Statement::Adt(_, _, _) => None,
        Statement::Infix(_, _, _, _) => None,
        Statement::Port(name, ty) => Some(ty),
        Statement::Def(def) => {
            if let Some(ty) = &def.header {
                Some(ty)
            } else {
                None
            }
        }
    }
}

fn declared_statement_name(stm: &Statement) -> Option<&str> {
    match stm {
        Statement::Alias(_, _, _) => None,
        Statement::Adt(_, _, _) => None,
        Statement::Infix(_, _, name, _) => Some(name),
        Statement::Port(name, ty) => Some(name),
        Statement::Def(def) => Some(&def.name)
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

// Use Value.get_type() instead
pub fn type_of_value(value: &Value) -> Type {
    match value {
        Value::Unit => {
            Type::Unit
        }
        Value::Number(_) => {
            Type::Var("number".to_string())
        }
        Value::Int(_) => {
            Type::Tag("Int".to_string(), vec![])
        }
        Value::Float(_) => {
            Type::Tag("Float".to_string(), vec![])
        }
        Value::String(_) => {
            Type::Tag("String".to_string(), vec![])
        }
        Value::Char(_) => {
            Type::Tag("Char".to_string(), vec![])
        }
        Value::List(items) => {
            if items.is_empty() {
                Type::Tag("List".to_string(), vec![Type::Var("a".to_string())])
            } else {
                Type::Tag("List".to_string(), vec![type_of_value(items.first().unwrap())])
            }
        }
        Value::Tuple(items) => {
            Type::Tuple(items.iter().map(|i| type_of_value(i)).collect())
        }
        Value::Record(items) => {
            Type::Record(items.iter().map(|(s, i)| (s.to_owned(), type_of_value(i))).collect())
        }
        Value::Adt(name, vars, adt) => {
            let variant = adt.variants.iter().find(|var| &var.name == name).unwrap();

            let mut var_replacement: HashMap<String, Type> = HashMap::new();
            let value_types: Vec<Type> = vars.iter().map(|v| type_of_value(v)).collect();

            type_inference_find_var_replacements(&mut var_replacement, &Type::Tuple(value_types), &Type::Tuple(variant.types.clone()));

            let final_types = adt.types.iter()
                .map(|ty| {
                    var_replacement.get(ty).cloned().unwrap_or_else(|| Type::Var(ty.clone()))
                })
                .collect();

            Type::Tag(adt.name.clone(), final_types)
        }
        Value::Fun { fun, args, .. } => {
            strip_fun_args(args.len(), &fun.get_type()).clone()
        }
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
    use test_utils::Test;
    use util::StringConversion;

    use super::*;

    fn from_code_def(code: &str) -> (Definition, Analyzer) {
        let stm = Test::statement(code);
        match stm {
            Statement::Def(def) => (def, Analyzer::new(SourceCode::from_str(code))),
            _ => panic!("Expected definition but found: {:?}", stm)
        }
    }

    fn format_type(analyzer: &mut Analyzer, def: &Definition) -> String {
        format!("{}", analyzer.analyze_definition(def).expect("Run into type error").header)
    }

    #[test]
    fn check_constant() {
        let (def, mut analyzer) = from_code_def("const = 1");

        assert_eq!(format_type(&mut analyzer, &def), "number");
    }

    #[test]
    fn check_identity() {
        let (def, mut analyzer) = from_code_def("id arg1 = arg1");

        assert_eq!(format_type(&mut analyzer, &def), "a -> a");
    }

    #[test]
    fn check_var_to_number() {
        let (def, mut analyzer) = from_code_def("sum arg1 arg2 = arg1 + arg2");

        analyzer.env.add_definition("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut analyzer, &def), "number -> number -> number");
    }

    #[test]
    fn check_number_to_float() {
        let (def, mut analyzer) = from_code_def("sum arg1 = arg1 + 1.5");

        analyzer.env.add_definition("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut analyzer, &def), "Float -> Float");
    }

    #[test]
    fn check_from_number_to_float() {
        let (def, mut analyzer) = from_code_def("sum = (+) 1.5");

        analyzer.env.add_definition("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut analyzer, &def), "Float -> Float");
    }

    #[test]
    fn check_list_coercion() {
        let (def, mut analyzer) = from_code_def("my = [1, 1.5]");

        assert_eq!(format_type(&mut analyzer, &def), "List Float");
    }

    #[test]
    fn check_list_coercion2() {
        let (def, mut analyzer) = from_code_def("my b = [1, 1.5, b]");

        assert_eq!(format_type(&mut analyzer, &def), "Float -> List Float");
    }

    #[test]
    fn check_variable_separation() {
        let (def, mut analyzer) = from_code_def("my a b = [a, b]");

        assert_eq!(format_type(&mut analyzer, &def), "a -> a -> List a");
    }

    #[test]
    fn check_variable_separation2() {
        let (def, mut analyzer) = from_code_def("my = (func, func)");

        analyzer.env.add_definition("func", Type::Fun(
            Box::from(Type::Var("a".s())),
            Box::from(Type::Var("a".s())),
        ));

        assert_eq!(format_type(&mut analyzer, &def), "( a -> a, b -> b )");
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

        let (res_ty, vars) = analyze_pattern_with_type(&mut env, &pattern, ty)
            .expect("Error");

        assert_eq!(format!("{}", res_ty), type_str);
        assert_eq!(format!("{:?}", vars), vars_str);
    }
}
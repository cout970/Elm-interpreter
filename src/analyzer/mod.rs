use std::collections::HashMap;

use analyzer::env::Env;
use ast::*;
use builtin::ELM_CORE_MODULES;
use errors::*;
use loader::AnalyzedModule;
use loader::Declaration;
use loader::LoadedModule;
use source::SourceCode;
use types::*;

mod statement_analyzer;
mod import_analyzer;
mod definition_analyzer;
mod env;

#[derive(Debug)]
pub struct Analyzer {
    pub e: Env,
    source: SourceCode,
}

impl Analyzer {
    pub fn new(source: SourceCode) -> Self {
        Analyzer { e: Env::new(), source }
    }

    pub fn with(&self, source: SourceCode) -> Self {
        Analyzer { e: self.e.clone(), source }
    }

    pub fn add_port(&mut self, name: &str, var: Type) {
        self.e.set(name, var);
    }

    pub fn add_type_alias(&mut self, alias: TypeAlias) {
        self.e.set_type_alias(alias);
    }

    pub fn add_canonical_type_name(&mut self, name: &str, canonical: &str) {
        self.e.set_canonical_type_name(name, canonical.to_string());
    }

    pub fn analyze_statement(&mut self, stm: &Statement) -> Result<Vec<Declaration>, ElmError> {
        let decls = match stm {
            Statement::Alias(name, vars, ty) => {
                self.analyze_statement_typealias(name, vars, ty)?
            }
            Statement::Adt(name, vars, variants) => {
                self.analyze_statement_adt(name, vars, variants)?
            }
            Statement::Port(span, name, ty) => {
                self.analyze_statement_port(*span, name, ty)?
            }
            Statement::Def(def) => {
                self.analyze_statement_definition(def)?
            }
            Statement::Infix(_, _, name, def) => {
                if let Some(ty) = self.e.get(name) {
                    vec![
                        Declaration::Port(name.clone(), ty.clone()),
                        Declaration::Infix(name.clone(), def.clone(), ty.clone())
                    ]
                } else if let Some(ty) = self.e.get(def) {
                    vec![
                        Declaration::Port(name.clone(), ty.clone()),
                        Declaration::Infix(name.clone(), def.clone(), ty.clone())
                    ]
                } else {
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
            }
        }

        // Custom behaviour for binary operators
        for stm in &module.ast.statements {
            if let Statement::Infix(_, _, name, def) = stm {
                let ty = if let Some(ty) = self.e.get(def) {
                    ty.clone()
                } else {
                    unreachable!("Infix operator {} where the function {} doesn't have a type header", name, def);
                };

                self.e.set(name, ty);
            }
        }

        let declarations = self.analyze_module_declarations(&module.ast.statements)
            .map_err(|list| {
                if list.len() == 1 {
                    list.into_iter().next().unwrap()
                } else {
                    ElmError::List(list)
                }
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
        Statement::Port(_, _, ty) => Some(ty),
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
        Statement::Port(_, name, _) => Some(name),
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
        Value::Adt(_, _, adt) => {
            let final_types = adt.types.iter()
                .map(|ty| Type::Var(ty.clone()))
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
    use constructors::type_of;
    use test_utils::Test;
    use util::{build_fun_type, StringConversion};

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

        assert_eq!(format_type(&mut analyzer, &def), "b -> b");
    }

    #[test]
    fn check_var_to_number() {
        let (def, mut analyzer) = from_code_def("sum arg1 arg2 = arg1 + arg2");

        analyzer.e.set("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut analyzer, &def), "number -> number -> number");
    }

    #[test]
    fn check_number_to_float() {
        let (def, mut analyzer) = from_code_def("sum arg1 = arg1 + 1.5");

        analyzer.e.set("+", build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]));

        assert_eq!(format_type(&mut analyzer, &def), "Float -> Float");
    }

    #[test]
    fn check_from_number_to_float() {
        let (def, mut analyzer) = from_code_def("sum = (+) 1.5");

        analyzer.e.set("+", build_fun_type(&vec![
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

        assert_eq!(format_type(&mut analyzer, &def), "c -> c -> List c");
    }

    #[test]
    fn check_variable_separation2() {
        let (def, mut analyzer) = from_code_def("my = (func, func)");

        analyzer.e.set("func", Type::Fun(
            Box::from(Type::Var("a".s())),
            Box::from(Type::Var("a".s())),
        ));

        assert_eq!(format_type(&mut analyzer, &def), "( c -> c, d -> d )");
    }

    #[test]
    fn check_list() {
        let (expr, mut analyzer) = Test::expr_analyzer("[1, 2, 3.5]");

        assert_eq!(
            type_of("List Float"),
            analyzer.analyze_expression(&expr).unwrap().get_type(),
        );
    }

//    #[test]
//    fn analyze_patterns_1() {
//        analyze_pattern_test(
//            type_int(),
//            pattern_var("a"),
//            "Int",
//            r#"[("a", Tag("Int", []))]"#,
//        );
//    }
//
//    #[test]
//    #[ignore]
//    fn analyze_patterns_2() {
//        analyze_pattern_test(
//            type_tag_args("Maybe", vec![type_var("item")]),
//            pattern_tag_args("Just", vec![pattern_var("a")]),
//            "Maybe item",
//            r#"[("a", Var("item"))]"#,
//        );
//    }
//
//    #[test]
//    fn analyze_patterns_3() {
//        analyze_pattern_test(
//            type_int(),
//            pattern_wildcard(),
//            "Int",
//            r#"[]"#,
//        );
//    }
//
//    #[test]
//    fn analyze_patterns_4() {
//        analyze_pattern_test(
//            type_unit(),
//            pattern_unit(),
//            "()",
//            r#"[]"#,
//        );
//    }
//
//    #[test]
//    fn analyze_patterns_5() {
//        analyze_pattern_test(
//            type_tuple(vec![type_int(), type_unit()]),
//            pattern_tuple(vec![pattern_var("a"), pattern_unit()]),
//            "( Int, () )",
//            r#"[("a", Tag("Int", []))]"#,
//        );
//    }
//
//    #[test]
//    fn analyze_patterns_6() {
//        analyze_pattern_test(
//            type_list(type_int()),
//            pattern_list(vec![pattern_var("a"), pattern_var("b")]),
//            "List Int",
//            r#"[("a", Tag("Int", [])), ("b", Tag("Int", []))]"#,
//        );
//    }
//
//    #[test]
//    fn analyze_patterns_7() {
//        analyze_pattern_test(
//            type_record(vec![("x", type_int())]),
//            pattern_record(vec!["x"]),
//            "{ x : Int }",
//            r#"[("x", Tag("Int", []))]"#,
//        );
//    }
//
//    #[test]
//    fn analyze_patterns_8() {
//        analyze_pattern_test(
//            type_list(type_int()),
//            pattern_cons(pattern_var("x"), pattern_var("xs")),
//            "List Int",
//            r#"[("x", Tag("Int", [])), ("xs", Tag("List", [Tag("Int", [])]))]"#,
//        );
//    }
//
//    #[test]
//    fn analyze_patterns_9() {
//        analyze_pattern_test(
//            type_int(),
//            pattern_int(1),
//            "Int",
//            r#"[]"#,
//        );
//    }
//
//    #[test]
//    fn analyze_patterns_10() {
//        analyze_pattern_test(
//            type_int(),
//            pattern_alias(pattern_int(1), "x"),
//            "Int",
//            r#"[("x", Tag("Int", []))]"#,
//        );
//    }
//
//    fn analyze_pattern_test(ty: Type, pattern: Pattern, type_str: &str, vars_str: &str) {
//        let mut env = StaticEnv::new();
//
//        let (res_ty, vars) = analyze_pattern_with_type(&mut env, &pattern, ty)
//            .expect("Error");
//
//        assert_eq!(format!("{}", res_ty), type_str);
//        assert_eq!(format!("{:?}", vars), vars_str);
//    }
}
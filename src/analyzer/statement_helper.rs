use std::collections::HashSet;
use std::sync::Arc;

use analyzer::Analyzer;
use ast::Definition;
use ast::Type;
use errors::TypeError;
use loader::Declaration;
use types::Adt;
use types::AdtVariant;
use util::build_fun_type;
use util::create_vec_inv;
use util::visitors::type_visitor;

impl Analyzer {
    pub fn analyze_statement_typealias(&mut self, name: &str, decl_vars: &Vec<String>, ty: &Type) -> Result<Vec<Declaration>, TypeError> {
        let mut used_vars: HashSet<String> = HashSet::new();

        type_visitor(&mut used_vars, ty, &|set, node| {
            if let Type::Var(var) = &node {
                set.insert(var.clone());
            }
        });

        if used_vars.len() < decl_vars.len() {
            let unused_vars = decl_vars.into_iter()
                .filter(|t| !used_vars.contains(*t))
                .map(|t| t.clone())
                .collect::<Vec<String>>();

            return Err(TypeError::UnusedTypeVariables(unused_vars));
        }

        if used_vars.len() > decl_vars.len() {
            let unknown_vars = used_vars.into_iter()
                .filter(|t| !decl_vars.contains(t))
                .map(|t| t.clone())
                .collect::<Vec<String>>();

            return Err(TypeError::UndeclaredTypeVariables(unknown_vars));
        }


        let mut decls: Vec<Declaration> = vec![
            Declaration::Alias(name.to_owned(), ty.clone())
        ];

        // If the type alias is for an record, a auxiliary constructor function is created
        if let Type::Record(entries) = ty {
            let mut args: Vec<Type> = entries.iter()
                .map(|(_, ty)| ty.clone())
                .collect();

            args.push(ty.clone());

            decls.push(Declaration::Port(name.to_owned(), build_fun_type(&args)))
        }

        Ok(decls)
    }

    pub fn analyze_statement_adt(&mut self, name: &String, decl_vars: &Vec<String>, variants: &Vec<(String, Vec<Type>)>) -> Result<Vec<Declaration>, TypeError> {
        let vars: Vec<Type> = decl_vars.iter()
            .map(|v| Type::Var(v.to_owned()))
            .collect();

        let adt_variants = variants.iter()
            .map(|(name, types)| {
                AdtVariant {
                    name: name.clone(),
                    types: types.clone(),
                }
            })
            .collect();

        let adt = Arc::new(Adt {
            name: name.to_owned(),
            types: decl_vars.clone(),
            variants: adt_variants,
        });

        let adt_type = Type::Tag(name.to_owned(), vars);
        let mut decls = vec![Declaration::Adt(name.to_owned(), adt.clone())];

        for (variant_name, params) in variants {
            let variant_type = build_fun_type(&create_vec_inv(params, adt_type.clone()));

            decls.push(Declaration::Port(variant_name.clone(), variant_type));
        }

        Ok(decls)
    }

    pub fn analyze_statement_port(&mut self, name: &String, ty: &Type) -> Result<Vec<Declaration>, TypeError> {
        Ok(vec![Declaration::Port(name.to_owned(), ty.clone())])
    }

    pub fn analyze_statement_definition(&mut self, def: &Definition) -> Result<Vec<Declaration>, TypeError> {
        Ok(vec![Declaration::Definition(def.name.clone(), self.analyze_definition(def)?)])
    }
}

#[cfg(test)]
mod tests {
    use source::SourceCode;
    use util::StringConversion;

    use super::*;

    #[test]
    fn check_type_alias_base() {
        let ty = Type::Unit;
        let mut analyzer = Analyzer::new(SourceCode::from_str("typealias A = ()"));
        assert_eq!(
            analyzer.analyze_statement_typealias("A", &vec![], &ty),
            Ok(vec![Declaration::Alias("A".s(), ty)])
        );
    }

    #[test]
    fn check_type_alias_1_var() {
        let ty = Type::Var("a".s());
        let mut analyzer = Analyzer::new(SourceCode::from_str("typealias A a = a"));
        assert_eq!(
            analyzer.analyze_statement_typealias("A", &vec!["a".s()], &ty),
            Ok(vec![Declaration::Alias("A".s(), ty)])
        );
    }

    #[test]
    fn check_type_alias_missing_var() {
        let ty = Type::Var("a".s());
        let mut analyzer = Analyzer::new(SourceCode::from_str("typealias A = a"));
        assert_eq!(
            analyzer.analyze_statement_typealias("A", &vec![], &ty),
            Err(TypeError::UndeclaredTypeVariables(vec!["a".s()]))
        );
    }

    #[test]
    fn check_type_alias_extra_var() {
        let ty = Type::Var("a".s());
        let mut analyzer = Analyzer::new(SourceCode::from_str("typealias A a b = a"));
        assert_eq!(
            analyzer.analyze_statement_typealias("A", &vec!["a".s(), "b".s()], &ty),
            Err(TypeError::UnusedTypeVariables(vec!["b".s()]))
        );
    }
}

use std::collections::HashSet;
use std::sync::Arc;

use analyzer::Analyzer;
use ast::{Definition, Span, TypeAlias};
use ast::Type;
use errors::{ElmError, TypeError};
use loader::Declaration;
use types::Adt;
use types::AdtVariant;
use util::{build_fun_type, VecExt};
use util::create_vec_inv;
use util::visitors::type_visitor;

impl Analyzer {
    pub fn analyze_statement_typealias(&mut self, name: &str, decl_vars: &Vec<String>, ty: &Type) -> Result<Vec<Declaration>, ElmError> {
        let mut used_vars: HashSet<String> = HashSet::new();

        type_visitor(&mut used_vars, ty, &|set, node| {
            if let Type::Var(var) = &node {
                set.insert(var.clone());
            }
            if let Type::RecExt(var, ..) = &node {
                set.insert(var.clone());
            }
        });

        if used_vars.len() < decl_vars.len() {
            let unused_vars = decl_vars.into_iter()
                .filter(|t| !used_vars.contains(*t))
                .map(|t| t.clone())
                .collect::<Vec<String>>();

            return Err(ElmError::Analyser(
                self.source.clone(),
                TypeError::UnusedTypeVariables { name: name.to_string(), values: unused_vars },
            ));
        }

        if used_vars.len() > decl_vars.len() {
            let unknown_vars = used_vars.into_iter()
                .filter(|t| !decl_vars.contains(t))
                .map(|t| t.clone())
                .collect::<Vec<String>>();

            return Err(ElmError::Analyser(
                self.source.clone(),
                TypeError::UndeclaredTypeVariables { name: name.to_string(), values: unknown_vars },
            ));
        }

        let mut decls: Vec<Declaration> = vec![
            Declaration::Alias(TypeAlias {
                name: name.to_string(),
                variables: decl_vars.clone(),
                replacement: ty.clone(),
            })
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

    pub fn analyze_statement_adt(&mut self, name: &String, decl_vars: &Vec<String>, variants: &Vec<(Span, String, Vec<Type>)>) -> Result<Vec<Declaration>, ElmError> {
        let mut decls = vec![];
        let vars: Vec<Type> = decl_vars.iter()
            .map(|v| Type::Var(v.to_owned()))
            .collect();

        let adt_type = Type::Tag(name.to_owned(), vars);

        // Any error inside the block should be returned after exit_block()
        // We cannot use self.e.block(), because we call self.check_type,
        // it needs a immutable reference to self and self.e.block() already
        // has a mutable reference to self
        self.e.enter_block();
        let adt_variants = {
            // Register own name to allow recursive definitions
            self.add_canonical_type_name(name, name);
            let mut adt_variants: Vec<(Span, AdtVariant)> = vec![];

            for (span, name, types) in variants {
                let mut new_types = vec![];

                for ty in types {
                    new_types.push(self.check_type(*span, ty.clone())?);
                }

                adt_variants.push((
                    *span,
                    AdtVariant {
                        name: name.clone(),
                        types: new_types,
                    }
                ));
            }

            Ok(adt_variants)
        };
        self.e.exit_block();
        // Return if Err(_)
        let adt_variants: Vec<(Span, AdtVariant)> = adt_variants?;

        // For each variant a definition is added, this definition is a constructor.
        for (span, variant) in &adt_variants {
            let mut new_variant_types = vec![];

            for ty in &variant.types {
                new_variant_types.push(self.check_type(*span, ty.clone())?);
            }

            let variant_type = if !new_variant_types.is_empty() {
                build_fun_type(&create_vec_inv(&new_variant_types, adt_type.clone()))
            } else {
                adt_type.clone()
            };

            decls.push(Declaration::Port(variant.name.clone(), variant_type));
        }

        let adt = Arc::new(Adt {
            name: name.to_owned(),
            types: decl_vars.clone(),
            variants: adt_variants.map(|(_, a)| a.clone()),
        });

        decls.push(Declaration::Adt(name.to_owned(), adt.clone()));

        Ok(decls)
    }

    pub fn analyze_statement_port(&mut self, span: Span, name: &String, ty: &Type) -> Result<Vec<Declaration>, ElmError> {
        let checked_type = self.check_type(span, ty.clone())?;
        Ok(vec![
            Declaration::Port(name.to_owned(), checked_type)
        ])
    }

    pub fn analyze_statement_definition(&mut self, def: &Definition) -> Result<Vec<Declaration>, ElmError> {
        let typed_def = self.analyze_definition(def)?;
        Ok(vec![
            Declaration::Definition(def.name.clone(), typed_def)
        ])
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
            Ok(vec![Declaration::Alias(TypeAlias {
                name: "A".s(),
                variables: vec![],
                replacement: ty,
            })])
        );
    }

    #[test]
    fn check_type_alias_1_var() {
        let ty = Type::Var("a".s());
        let mut analyzer = Analyzer::new(SourceCode::from_str("typealias A a = a"));
        assert_eq!(
            analyzer.analyze_statement_typealias("A", &vec!["a".s()], &ty),
            Ok(vec![Declaration::Alias(TypeAlias {
                name: "A".s(),
                variables: vec!["a".to_string()],
                replacement: ty,
            })])
        );
    }

    #[test]
    fn check_type_alias_missing_var() {
        let ty = Type::Var("a".s());
        let code = SourceCode::from_str("typealias A = a");
        let mut analyzer = Analyzer::new(code.clone());
        assert_eq!(
            analyzer.analyze_statement_typealias("A", &vec![], &ty),
            Err(ElmError::Analyser(
                code,
                TypeError::UndeclaredTypeVariables { name: "A".to_string(), values: vec!["a".s()] },
            ))
        );
    }

    #[test]
    fn check_type_alias_extra_var() {
        let ty = Type::Var("a".s());
        let code = SourceCode::from_str("typealias A a b = a");
        let mut analyzer = Analyzer::new(code.clone());
        assert_eq!(
            analyzer.analyze_statement_typealias("A", &vec!["a".s(), "b".s()], &ty),
            Err(ElmError::Analyser(
                code,
                TypeError::UnusedTypeVariables { name: "A".to_string(), values: vec!["b".s()] },
            ))
        );
    }
}

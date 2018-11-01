use analyzer::dependency_sorter::sort_statement_dependencies;
use analyzer::function_analyzer::analyze_function_arguments;
use analyzer::static_env::StaticEnv;
use analyzer::TypeError;
use ast::*;
use interpreter::RuntimeError;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use types::Adt;
use types::AdtVariant;
use types::Fun;
use types::Value;
use util::build_fun_type;
use util::create_vec_inv;
use util::qualified_name;
use util::visitors::expr_visitor_block;
use util::visitors::type_visitor;
use analyzer::function_analyzer::analyze_function;

#[derive(Debug, PartialEq)]
enum Registration {
    Def(String, Type),
    Type(String, Type),
}

type Registrations = Vec<Registration>;

pub fn analyze_module(module: &Module) -> Result<(), TypeError> {
    let stms = sort_statement_dependencies(&module.statements);
    let mut env = StaticEnv::new();

    for stm in stms {
        let regs = analyze_statement(&mut env, stm)?;

        for reg in regs.into_iter() {
            match reg {
                Registration::Def(name, ty) => {
                    env.add(&name, ty);
                }
                Registration::Type(name, ty) => {
                    env.add(&name, ty);
                }
            }
        }
    }

    Ok(())
}

fn analyze_statement(env: &mut StaticEnv, stm: &Statement) -> Result<Registrations, TypeError> {
    let regs = match stm {
        Statement::Alias(name, vars, ty) => {
            analyze_type_alias(name, vars, ty)?
        }
        Statement::Adt(name, vars, variants) => {
            analyze_adt(name, vars, variants)?
        }
        Statement::Port(name, ty) => {
            analyze_port(name, ty)?
        }
        Statement::Def(def) => {
            vec![
                Registration::Def(def.name.clone(), analyze_function(env, def)?)
            ]
        }
    };

    Ok(regs)
}

fn analyze_port(name: &str, ty: &Type) -> Result<Registrations, TypeError> {
    Ok(vec![
        Registration::Def(name.to_owned(), ty.clone())
    ])
}

fn analyze_adt(name: &str, decl_vars: &Vec<String>, variants: &Vec<(String, Vec<Type>)>) -> Result<Registrations, TypeError> {
    let vars: Vec<Type> = decl_vars.iter()
        .map(|v| Type::Var(v.to_owned()))
        .collect();

    Ok(vec![
        Registration::Type(name.to_owned(),  Type::Tag(name.to_owned(), vars))
    ])
}

fn analyze_type_alias(name: &str, decl_vars: &Vec<String>, ty: &Type) -> Result<Registrations, TypeError> {
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


    let mut regs: Registrations = vec![
        Registration::Type(name.to_owned(), ty.clone())
    ];

    // If the type alias is for an record, a auxiliary constructor function is added
    if let Type::Record(entries) = ty {
        let mut args: Vec<Type> = entries.iter()
            .map(|(_, ty)| ty.clone())
            .collect();

        args.push(ty.clone());

        regs.push(Registration::Def(name.to_owned(), build_fun_type(&args)))
    }

    Ok(regs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::StringConversion;

    #[test]
    fn check_type_alias_base() {
        let ty = Type::Unit;
        assert_eq!(
            analyze_type_alias("A", &vec![], &ty),
            Ok(vec![Registration::Type("A".s(), ty)])
        );
    }

    #[test]
    fn check_type_alias_1_var() {
        let ty = Type::Var("a".s());
        assert_eq!(
            analyze_type_alias("A", &vec!["a".s()], &ty),
            Ok(vec![Registration::Type("A".s(), ty)])
        );
    }

    #[test]
    fn check_type_alias_missing_var() {
        let ty = Type::Var("a".s());
        assert_eq!(
            analyze_type_alias("A", &vec![], &ty),
            Err(TypeError::UndeclaredTypeVariables(vec!["a".s()]))
        );
    }

    #[test]
    fn check_type_alias_extra_var() {
        let ty = Type::Var("a".s());
        assert_eq!(
            analyze_type_alias("A", &vec!["a".s(), "b".s()], &ty),
            Err(TypeError::UnusedTypeVariables(vec!["b".s()]))
        );
    }
}

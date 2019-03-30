use std::collections::HashMap;
use std::collections::HashSet;

use analyzer::Analyzer;
use analyzer::static_env::StaticEnv;
use ast::*;
use util::qualified_name;
use util::sort::get_acyclic_dependency_graph;
use util::visitors::expr_visitor_block;
use util::visitors::pattern_visitor;
use util::visitors::type_visitor;

pub fn sort_statements(stms: &Vec<Statement>) -> Result<Vec<&Statement>, Vec<String>> {
    // stm name, provided names, dependencies
    let mut stm_map: Vec<(String, Vec<String>, Vec<String>)> = Vec::new();

    for stm in stms {
        let name = get_stm_name(stm).to_owned();
        let provided = get_provided_names(stm);
        let deps = get_stm_dependencies(stm);
        stm_map.push((name, provided, deps));
    }

    let mut graph: HashMap<&String, Vec<&String>> = HashMap::new();

    for (stm, _, deps) in &stm_map {

        // TODO this is really messy and inefficient
        let deps: Vec<&String> = deps.iter()
            .filter(|&dep| {
                dep != stm && stm_map.iter().any(|(_, names, _)| names.contains(dep))
            })
            .map(|dep| {
                &stm_map.iter()
                    .find(|(_, names, _)| names.contains(dep))
                    .unwrap()
                    .0
            })
            .collect();

        graph.insert(stm, deps);
    }

    let sorted_names: Vec<&String> = get_acyclic_dependency_graph(graph)
        .map_err(|e| e.iter().map(|&i| i.clone()).collect::<Vec<String>>())?;

    Ok(sorted_names.iter().
        map(|name| {
            stms.iter().find(|stm| get_stm_name(*stm) == *name).unwrap()
        })
        .collect::<Vec<_>>())
}


fn get_stm_dependencies(def: &Statement) -> Vec<String> {
    match def {
        Statement::Alias(_, _, ty) => { get_type_dependencies(ty) }
        Statement::Port(_, ty) => { get_type_dependencies(ty) }
        Statement::Def(def) => {
            let mut fake_env = StaticEnv::new();
            get_def_dependencies(&mut fake_env, def)
        }
        Statement::Adt(_, _, branches) =>
            branches.iter()
                .map(|(_, tys)| {
                    tys.iter().map(|ty| get_type_dependencies(ty)).flatten()
                })
                .flatten()
                .collect(),
        Statement::Infix(_, _, _, _) => vec![]
    }
}

fn get_def_dependencies(env: &mut StaticEnv, def: &Definition)-> Vec<String>{
    let mut names = add_patterns(env, &def.patterns);

    for x in get_expr_dependencies(env, &def.expr) {
        names.push(x);
    }

    if let Some(ty) = &def.header {
        for x in get_type_dependencies(ty) {
            names.push(x);
        }
    }

    names
}

fn add_patterns(env: &mut StaticEnv, patterns: &Vec<Pattern>) -> Vec<String> {
    let mut analyser = Analyzer::from(env.clone());
    for (_, entries) in analyser.analyze_function_arguments(patterns, &None) {
        for (name, _) in entries {
            env.add_definition(&name, Type::Unit);
        }
    }
    let mut deps = vec![];

    for pattern in patterns {
        pattern_visitor(&mut deps, pattern, &|s: &mut Vec<String>, p: &Pattern| {
            match p {
                Pattern::Var(_) => {}
                Pattern::Adt(name, _) => {
                    s.push(name.to_owned());
                }
                Pattern::Wildcard => {}
                Pattern::Unit => {}
                Pattern::Tuple(_) => {}
                Pattern::List(_) => {}
                Pattern::BinaryOp(op, _, _) => {
                    s.push(op.to_owned());
                }
                Pattern::Record(_) => {}
                Pattern::LitInt(_) => {}
                Pattern::LitString(_) => {}
                Pattern::LitChar(_) => {}
                Pattern::Alias(_, _) => {}
            }
        });
    }
    deps
}

fn get_type_dependencies(ty: &Type) -> Vec<String> {
    let mut refs: HashSet<String> = HashSet::new();

    type_visitor(&mut refs, ty, &|state, sub_ty| {
        match sub_ty {
            Type::Var(_) => {}
            Type::Tag(name, _) => { state.insert(name.clone()); }
            Type::Fun(_, _) => {}
            Type::Unit => {}
            Type::Tuple(_) => {}
            Type::Record(_) => {}
            Type::RecExt(name, _) => { state.insert(name.clone()); }
        }
    });

    refs.into_iter().collect()
}

fn get_expr_dependencies(env: &mut StaticEnv, expr: &Expr) -> Vec<String> {
    let mut local_refs: HashSet<String> = HashSet::new();

    expr_visitor_block(&mut (env, &mut local_refs), expr, &|(env, refs), sub_expr| {
        match sub_expr {
            Expr::RecordUpdate(_, name, _) => {
                if let None = env.find_definition(name) {
                    refs.insert(name.clone());
                }
            }
            Expr::QualifiedRef(_, path, name) => {
                let full_name = qualified_name(path, name);
                if let None = env.find_definition(&full_name) {
                    refs.insert(full_name);
                }
            }
            Expr::OpChain(_, _, ops) => {
                for op in ops {
                    if let None = env.find_definition(op) {
                        refs.insert(op.clone());
                    }
                }
            }
            Expr::Ref(_, name) => {
                if let None = env.find_definition(name) {
                    refs.insert(name.clone());
                }
            }

            Expr::RecordField(_, _, _) => {}
            Expr::RecordAccess(_, _) => {}
            Expr::If(_, _, _, _) => {}
            Expr::Case(_, _, _) => {}
            Expr::Application(_, _, _) => {}
            Expr::Literal(_, _) => {}

            Expr::Lambda(_, patterns, _) => {
                env.enter_block();
                add_patterns(env, patterns);
            }
            Expr::Let(_, decls, _) => {
                env.enter_block();
                for decl in decls {
                    match decl {
                        LetDeclaration::Def(def) => {
                            add_patterns(env, &def.patterns);
                            for x in get_def_dependencies(env, def) {
                                refs.insert(x);
                            }
                        }
                        LetDeclaration::Pattern(pattern, _) => {
                            add_patterns(env, &vec![pattern.clone()]);
                            // TODO
                        }
                    }
                }
            }
            _ => {}
        }
    }, &|(env, _), sub_expr| {
        match sub_expr {
            Expr::Lambda(_, _, _) => {
                env.exit_block();
            }
            Expr::Let(_, _, _) => {
                env.exit_block();
            }
            _ => {}
        }
    });

    local_refs.into_iter().collect()
}

fn get_stm_name(stm: &Statement) -> &str {
    match stm {
        Statement::Alias(name, _, _) => { name }
        Statement::Adt(name, _, _) => { name }
        Statement::Port(name, _) => { name }
        Statement::Def(def) => { &def.name }
        Statement::Infix(_, _, op, _) => { op }
    }
}

fn get_provided_names(stm: &Statement) -> Vec<String> {
    match stm {
        Statement::Alias(name, _, _) => { vec![name.to_owned()] }
        Statement::Adt(name, _, variants) => {
            let mut var_names = variants.iter().map(|(n, _)| n.to_owned()).collect::<Vec<_>>();
            var_names.push(name.to_owned());
            var_names
        }
        Statement::Port(name, _) => { vec![name.to_owned()] }
        Statement::Def(def) => { vec![def.name.to_owned()] }
        Statement::Infix(_, _, op, _) => { vec![op.to_owned()] }
    }
}

#[cfg(test)]
mod tests {
    use test_utils::Test;

    use super::*;

    #[test]
    fn check_expr_dependencies() {
        let module = Test::module("\ny = x + 1\n\nx = 0\n\nz = y");
        let sorted = sort_statements(&module.statements).unwrap();

        let names: Vec<_> = sorted.iter().map(|stm| get_stm_name(stm)).collect();

        assert_eq!(names, vec!["x", "y", "z"]);
    }
}
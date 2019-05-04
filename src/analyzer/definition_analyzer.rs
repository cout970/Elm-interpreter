use hashbrown::HashMap;

use analyzer::{Analyzer, unpack_types};
use analyzer::env::Env;
use ast::{Definition, Expr, LetDeclaration, Span};
use ast::Pattern;
use ast::Type;
use constructors::{type_bool, type_fun, type_list, type_var};
use errors::{ElmError, TypeError};
use typed_ast::{expr_type, LetEntry, TypedPattern};
use typed_ast::TypedDefinition;
use typed_ast::TypedExpr;
use types::Value;
use util::expression_fold::{create_expr_tree, ExprTree};
use util::expression_fold::ExprTreeError;
use util::qualified_name;
use util::ToVec;
use util::VecExt;

// Algorithm inspired by
// https://youtu.be/oPVTNxiMcSU

#[derive(Debug, Clone)]
struct Constraint {
    span: Span,
    left: Type,
    right: Type,
}

#[derive(Debug)]
struct Substitution(HashMap<Type, Type>);

impl Constraint {
    fn new(span: Span, left: &Type, right: &Type) -> Self {
        Constraint { span, left: left.clone(), right: right.clone() }
    }

    fn as_pair(&self) -> (&Type, &Type) {
        (&self.left, &self.right)
    }
}

impl Substitution {
    fn empty() -> Self {
        Substitution(HashMap::new())
    }

    fn var_pair(var: &str, ty: &Type) -> Self {
        let mut map = HashMap::new();
        map.insert(Type::Var(var.to_string()), ty.clone());
        Substitution(map)
    }

    fn merge(self, b: Substitution) -> Substitution {
        let mut map = HashMap::new();

        for (k, v) in self.0 {
            let value = apply_substitution_ty(&b, &v);
            map.insert(k, value);
        }

        for (k, v) in b.0 {
            map.insert(k, v);
        }

        Substitution(map)
    }

    fn replace(&self, ty: Type) -> Type {
        self.0.get(&ty).cloned().unwrap_or(ty)
    }
}

impl Analyzer {
    pub fn analyze_definition(&mut self, fun: &Definition) -> Result<TypedDefinition, ElmError> {
        infer_definition_type(&mut self.e, fun)
            .map_err(|e| ElmError::Analyser(self.source.clone(), e))
    }

    pub fn analyze_expression(&mut self, expr: &Expr) -> Result<TypedExpr, ElmError> {
        infer_expression_type(&mut self.e, expr)
            .map_err(|e| ElmError::Analyser(self.source.clone(), e))
    }

    pub fn check_type(&self, span: Span, ty: Type) -> Result<Type, ElmError> {
        let ty = self.replace_type_alias(ty);
        self.get_canonical_type(span, ty)
            .map_err(|e| ElmError::Analyser(self.source.clone(), e))
    }

    pub fn get_canonical_type(&self, span: Span, ty: Type) -> Result<Type, TypeError> {
        let new_ty = match ty {
            Type::Unit => Type::Unit,
            Type::Var(_) => ty,
            Type::Tag(name, items) => {
                let canonical_name = match self.e.get_canonical_type_name(&name) {
                    Some(name_str) => name_str.to_string(),
                    None => {
                        return Err(TypeError::UnknownType { span, name: name.clone() });
                    }
                };

                Type::Tag(
                    canonical_name,
                    items.into_iter()
                        .map(|it| self.get_canonical_type(span, it)).
                        collect::<Result<Vec<Type>, TypeError>>()?,
                )
            }
            Type::Fun(a, b) => Type::Fun(
                Box::new(self.get_canonical_type(span, *a)?),
                Box::new(self.get_canonical_type(span, *b)?),
            ),
            Type::Tuple(items) => Type::Tuple(
                items.into_iter()
                    .map(|it| self.get_canonical_type(span, it))
                    .collect::<Result<Vec<Type>, TypeError>>()?
            ),
            Type::Record(items) => Type::Record(
                items.into_iter()
                    .map(|(s, it)| self.get_canonical_type(span, it).map(|i| (s, i)))
                    .collect::<Result<Vec<(String, Type)>, TypeError>>()?
            ),
            Type::RecExt(name, items) => Type::RecExt(
                name,
                items.into_iter()
                    .map(|(s, it)| self.get_canonical_type(span, it).map(|i| (s, i)))
                    .collect::<Result<Vec<(String, Type)>, TypeError>>()?,
            ),
        };

        Ok(new_ty)
    }

    pub fn replace_type_alias(&self, ty: Type) -> Type {
        match ty {
            Type::Tag(a, b) => {
                let params: Vec<Type> = b.into_iter()
                    .map(|it| self.replace_type_alias(it))
                    .collect();

                if let Some(alias) = self.e.get_type_alias(&a) {
                    assert_eq!(params.len(), alias.variables.len());
                    let mut map = HashMap::new();

                    for i in 0..params.len() {
                        map.insert(Type::Var(alias.variables[i].clone()), params[i].clone());
                    }

                    let sub = Substitution(map);
                    let new_params: Vec<Type> = params.into_iter().map(|it| sub.replace(it)).collect();

                    Type::Tag(a, new_params)
                } else {
                    Type::Tag(a, params)
                }
            }
            Type::Unit => Type::Unit,
            Type::Var(a) => Type::Var(a),
            Type::Fun(a, b) => Type::Fun(
                Box::new(self.replace_type_alias(*a)),
                Box::new(self.replace_type_alias(*b)),
            ),
            Type::Tuple(a) => Type::Tuple(
                a.into_iter().map(|it| self.replace_type_alias(it)).collect()
            ),
            Type::Record(a) => Type::Record(
                a.into_iter().map(|(s, it)| (s, self.replace_type_alias(it))).collect()
            ),
            Type::RecExt(a, b) => Type::RecExt(
                a,
                b.into_iter().map(|(s, it)| (s, self.replace_type_alias(it))).collect(),
            ),
        }
    }
}

fn infer_expression_type(env: &mut Env, expr: &Expr) -> Result<TypedExpr, TypeError> {
    let mut constraints = vec![];

    let (substitution, annotated_expr) = env.block(|env| {

        // Type Annotation
        let annotated_expr = annotate_expr(env, expr)?;

        // Collect constraints
        collect_expr_constraints(&mut constraints, &annotated_expr);

        // Constraint solutions
        let substitution = match unify_constraints(&constraints) {
            Ok(sub) => sub,
            Err(e) => {
                return Err(e);
            }
        };

        Ok((substitution, annotated_expr))
    })?;

    // Apply solution
    let res_expr = replace_expr_types(&substitution, annotated_expr);

    Ok(res_expr)
}

fn infer_definition_type(env: &mut Env, fun: &Definition) -> Result<TypedDefinition, TypeError> {
    let mut constraints = vec![];

    // exhaustive patterns?
    let (substitution, func_type, annotated_patterns, annotated_expr) = env.block(|env| {

        // Type Annotation
        let func_type = env.next_type();

        env.set(&fun.name, func_type.clone());

        let mut annotated_patterns = vec![];
        for pat in &fun.patterns {
            annotated_patterns.push(annotate_pattern(env, &pat)?);
        }
        for pat in &annotated_patterns {
            add_pattern_vars_to_env(env, pat);
        }

        let annotated_expr = annotate_expr(env, &fun.expr)?;

        // Collect constraints
        // replace_type_alias?
        if let Some(ty) = &fun.header {
            let safe_ty = update_type_variables(env, &mut HashMap::new(), ty.clone());

            collect_type_definition_constraints(&mut constraints, &safe_ty, &annotated_patterns, &annotated_expr);
        };

        for pat in &annotated_patterns {
            collect_pattern_constraints(&mut constraints, pat);
        }

        collect_expr_constraints(&mut constraints, &annotated_expr);

        // Function type
        let mut func_types: Vec<Type> = annotated_patterns.iter()
            .map(|pat| pat.get_type())
            .collect();

        func_types.push(annotated_expr.get_type());

        constraints.push(Constraint::new(
            annotated_expr.get_span(),
            &func_type,
            &type_fun(func_types),
        ));

        // Constraint solutions
        let substitution = match unify_constraints(&constraints) {
            Ok(sub) => sub,
            Err(e) => {
////                // Debug
//                eprintln!("\nFunc {}:\n", &fun.name);
//                for pat in &annotated_patterns {
//                    eprintln!("  pattern:\n{}\n", pat);
//                }
//
//                eprintln!("  expr:\n{}\n", &annotated_expr);
//
//                eprintln!("Constraints: ");
//                for p in &constraints {
//                    eprintln!("{} => {}", p.left, p.right);
//                }
//                eprintln!();

                return Err(e);
            }
        };

        Ok((substitution, func_type, annotated_patterns, annotated_expr))
    })?;

    // Apply solution
    let res_expr = replace_expr_types(&substitution, annotated_expr);
    let res_patterns: Vec<TypedPattern> = annotated_patterns.into_iter()
        .map(|pat| replace_pattern_types(&substitution, pat))
        .collect();

    let def_type = substitution.replace(func_type);

    Ok(TypedDefinition {
        header: def_type,
        name: fun.name.to_string(),
        patterns: res_patterns,
        expr: res_expr,
    })
}

fn update_type_variables(env: &mut Env, dup: &mut HashMap<String, Type>, ty: Type) -> Type {
    match ty {
        Type::Var(name) => {
            match dup.get(&name).cloned() {
                Some(var) => var,
                None => {
                    let new_ty = if name.starts_with("comparable") {
                        env.next_comparable_type()
                    } else if name.starts_with("appendable") {
                        env.next_appendable_type()
                    } else if name.starts_with("number") {
                        env.next_number_type()
                    } else {
                        env.next_type()
                    };
                    dup.insert(name, new_ty.clone());
                    new_ty
                }
            }
        }
        Type::Fun(a, b) => {
            Type::Fun(
                Box::new(update_type_variables(env, dup, *a)),
                Box::new(update_type_variables(env, dup, *b)),
            )
        }
        Type::Tag(name, items) => {
            let vec: Vec<Type> = items.into_iter().map(|e| update_type_variables(env, dup, e)).collect();
            Type::Tag(name, vec)
        }
        Type::Tuple(items) => {
            let vec: Vec<Type> = items.into_iter().map(|e| update_type_variables(env, dup, e)).collect();
            Type::Tuple(vec)
        }
        Type::Record(items) => {
            let vec: Vec<(String, Type)> = items.into_iter().map(|(s, e)| (s, update_type_variables(env, dup, e))).collect();
            Type::Record(vec)
        }
        Type::RecExt(name, items) => {
            let vec: Vec<(String, Type)> = items.into_iter().map(|(s, e)| (s, update_type_variables(env, dup, e))).collect();
            Type::RecExt(name, vec)
        }
        Type::Unit => Type::Unit,
    }
}

fn annotate_pattern(env: &mut Env, pat: &Pattern) -> Result<TypedPattern, TypeError> {
    let typed = match pat {
        Pattern::Var(span, name) => {
            if env.get(name).is_some() {
                return Err(TypeError::VariableNameShadowed { span: pat.get_span(), name: name.clone() });
            }

            TypedPattern::Var(*span, env.next_type(), name.clone())
        }
        Pattern::Adt(span, name, items) => {
            let adt_type = match env.get(name) {
                Some(ty) => ty.clone(),
                None => {
                    return Err(TypeError::MissingDefinition { span: *span, name: name.to_string() });
                }
            };

            let adt_type = update_type_variables(env, &mut HashMap::new(), adt_type);

            TypedPattern::Adt(
                *span,
                env.next_type(),
                adt_type,
                vec_map(env, items, annotate_pattern)?,
            )
        }
        Pattern::Wildcard(span) => {
            TypedPattern::Wildcard(*span)
        }
        Pattern::Unit(span) => {
            TypedPattern::Unit(*span)
        }
        Pattern::Tuple(span, items) => {
            TypedPattern::Tuple(*span,
                                env.next_type(),
                                vec_map(env, items, annotate_pattern)?,
            )
        }
        Pattern::List(span, items) => {
            TypedPattern::List(*span,
                               env.next_type(),
                               vec_map(env, items, annotate_pattern)?,
            )
        }
        Pattern::BinaryOp(span, op, a, b) => {
            TypedPattern::BinaryOp(*span,
                                   env.next_type(),
                                   op.clone(),
                                   Box::new(annotate_pattern(env, a)?),
                                   Box::new(annotate_pattern(env, b)?),
            )
        }
        Pattern::Record(span, items) => {
            TypedPattern::Record(*span,
                                 env.next_type(),
                                 items.clone(),
            )
        }
        Pattern::LitInt(span, val) => {
            TypedPattern::LitInt(*span, *val)
        }
        Pattern::LitString(span, val) => {
            TypedPattern::LitString(*span, val.clone())
        }
        Pattern::LitChar(span, val) => {
            TypedPattern::LitChar(*span, *val)
        }
        Pattern::Alias(span, pat, name) => {
            let ty = annotate_pattern(env, pat)?;
            env.set(name, ty.get_type());
            TypedPattern::Alias(*span, ty.get_type(), Box::new(ty), name.clone())
        }
    };

    Ok(typed)
}

fn annotate_expr(env: &mut Env, expr: &Expr) -> Result<TypedExpr, TypeError> {
    let te = match expr {
        Expr::QualifiedRef(span, base, name) => {
            let name = qualified_name(base, name);
            let ty = env.get(&name).cloned()
                .ok_or_else(|| {
                    eprintln!("{}", env);
                    TypeError::MissingDefinition { span: *span, name: name.to_string() }
                })?;

            let ty = update_type_variables(env, &mut HashMap::new(), ty);
            TypedExpr::Ref(*span, ty, name.clone())
        }
        Expr::Ref(span, name) => {
            let ty = env.get(name).cloned()
                .ok_or_else(|| {
                    TypeError::MissingDefinition { span: *span, name: name.to_string() }
                })?;

            let ty = if let Type::Var(_) = &ty {
                ty
            } else {
                update_type_variables(env, &mut HashMap::new(), ty)
            };
            TypedExpr::Ref(*span, ty, name.clone())
        }
        Expr::Literal(span, lit) => {
            let value: Value = lit.clone().into();
            if let Value::Number(_) = &value {
                TypedExpr::Const(*span, env.next_number_type(), value)
            } else {
                TypedExpr::Const(*span, value.get_type(), value)
            }
        }
        Expr::Unit(span) => {
            TypedExpr::Const(*span, env.next_type(), Value::Unit)
        }
        Expr::Tuple(span, exprs) => {
            TypedExpr::Tuple(*span,
                             env.next_type(),
                             vec_map(env, exprs, annotate_expr)?,
            )
        }
        Expr::List(span, exprs) => {
            TypedExpr::List(*span,
                            env.next_type(),
                            vec_map(env, exprs, annotate_expr)?,
            )
        }
        Expr::Record(span, exprs) => {
            TypedExpr::Record(
                *span,
                env.next_type(),
                vec_pair_map(env, exprs, annotate_expr)?,
            )
        }
        Expr::RecordUpdate(span, name, exprs) => {
            let sub = annotate_expr(env, &Expr::Ref(*span, name.clone()))?;
            TypedExpr::RecordUpdate(
                *span,
                env.next_type(),
                Box::new(sub),
                vec_pair_map(env, exprs, annotate_expr)?,
            )
        }
        Expr::RecordField(span, expr, name) => {
            TypedExpr::RecordField(
                *span,
                env.next_type(),
                Box::new(annotate_expr(env, expr)?),
                name.clone(),
            )
        }
        Expr::RecordAccess(span, name) => {
            TypedExpr::RecordAccess(
                *span,
                env.next_type(),
                name.clone(),
            )
        }
        Expr::If(span, a, b, c) => {
            TypedExpr::If(
                *span,
                env.next_type(),
                Box::new(annotate_expr(env, a)?),
                Box::new(annotate_expr(env, b)?),
                Box::new(annotate_expr(env, c)?),
            )
        }
        Expr::Case(span, expr, branches) => {
            let annotated_expr = annotate_expr(env, expr)?;
            let mut new_branches = vec![];

            for (a, b) in branches {
                env.block(|env| {
                    let pat = annotate_pattern(env, a)?;
                    add_pattern_vars_to_env(env, &pat);

                    new_branches.push((pat, annotate_expr(env, b)?));
                    Ok(())
                })?;
            }

            TypedExpr::Case(
                *span,
                env.next_type(),
                Box::new(annotated_expr),
                new_branches,
            )
        }
        Expr::Lambda(span, pat, expr) => {
            let patterns = vec_map(env, pat, annotate_pattern)?;

            let expr = env.block(|env| {
                for pat in &patterns {
                    add_pattern_vars_to_env(env, pat);
                }
                annotate_expr(env, expr)
            })?;

            TypedExpr::Lambda(
                *span,
                env.next_type(),
                patterns,
                Box::new(expr),
            )
        }
        Expr::Application(span, a, b) => {
            TypedExpr::Application(
                *span,
                env.next_type(),
                Box::new(annotate_expr(env, a)?),
                Box::new(annotate_expr(env, b)?),
            )
        }
        Expr::Let(span, decls, expr) => {
            let (entries, expr) = env.block(|env| {
                let mut entries = vec![];

                for decl in decls {
                    match decl {
                        LetDeclaration::Def(def) => {
                            let typed_def = infer_definition_type(env, def)?;

                            env.set(&def.name, typed_def.header.clone());
                            entries.push(LetEntry::Definition(typed_def));
                        }
                        LetDeclaration::Pattern(pat, expr) => {
                            let pat = annotate_pattern(env, pat)?;
                            add_pattern_vars_to_env(env, &pat);

                            let expr = annotate_expr(env, expr)?;

                            entries.push(LetEntry::Pattern(pat, expr));
                        }
                    }
                }
                Ok((entries, annotate_expr(env, expr)?))
            })?;

            TypedExpr::Let(*span,
                           expr_type(&expr),
                           entries,
                           Box::new(expr),
            )
        }
        Expr::OpChain(span, exprs, ops) => {
            match create_expr_tree(exprs, ops) {
                Ok(tree) => annotate_expr(env, &expr_tree_to_expr(tree))?,
                Err(e) => {
                    let msg = match e {
                        ExprTreeError::InvalidInput => format!("Invalid input"),
                        ExprTreeError::AssociativityError => format!("Associativity error"),
                        ExprTreeError::InternalError(msg) => format!("Internal error: {}", msg),
                    };
                    return Err(TypeError::InvalidOperandChain { span: *span, msg });
                }
            }
        }
    };

    Ok(te)
}

fn collect_type_definition_constraints(res: &mut Vec<Constraint>, ty: &Type, patterns: &Vec<TypedPattern>, expr: &TypedExpr) {
    let list = unpack_types(ty);

    if list.len() <= patterns.len() {
        panic!("Too many patterns: {} patterns and {} arguments", patterns.len(), list.len());
    }

    let mut i = 0;

    while i < patterns.len() {
        res.push(Constraint::new(
            patterns[i].get_span(),
            &patterns[i].get_type(),
            &list[i],
        ));
        i += 1;
    }

    let ret: Vec<_> = list[i..].iter().cloned().collect();
    res.push(Constraint::new(expr.get_span(), &type_fun(ret), &expr.get_type()));
}

fn collect_pattern_constraints(res: &mut Vec<Constraint>, pat: &TypedPattern) {
    match pat {
        TypedPattern::Var(_, _, _) => {}
        TypedPattern::Adt(_, ty, ctor_type, items) => {
            let adt_type = unpack_types(ctor_type).into_iter().last().unwrap();
            let mut ctor = vec![];

            for arg in items {
                ctor.push(arg.get_type());
            }
            ctor.push(adt_type.clone());

            res.push(Constraint::new(
                pat.get_span(),
                ctor_type,
                &type_fun(ctor),
            ));

            res.push(Constraint::new(
                pat.get_span(),
                ty,
                &adt_type,
            ));

            items.for_each(|it| collect_pattern_constraints(res, it));
        }
        TypedPattern::Wildcard(_) => {}
        TypedPattern::Unit(_) => {}
        TypedPattern::Tuple(_, ty, items) => {
            res.push(Constraint::new(pat.get_span(), ty, &Type::Tuple(items.map(|e| e.get_type()))));
            items.for_each(|it| collect_pattern_constraints(res, it));
        }
        TypedPattern::List(_, ty, items) => {
            items.for_each(|it| {
                res.push(Constraint::new(pat.get_span(), ty, &type_list(it.get_type())));
                collect_pattern_constraints(res, it);
            });
        }
        TypedPattern::BinaryOp(_, ty, op, a, b) => {
            assert_eq!("::", op.as_str());
            res.push(Constraint::new(pat.get_span(), ty, &type_list(a.get_type())));
            res.push(Constraint::new(pat.get_span(), &b.get_type(), &type_list(a.get_type())));

            collect_pattern_constraints(res, a);
            collect_pattern_constraints(res, b);
        }
        TypedPattern::Record(_, ty, items) => {
            res.push(Constraint::new(pat.get_span(), ty, &Type::Record(
                items.map(|it| (it.clone(), type_var(it)))
            )));
        }
        TypedPattern::LitInt(_, _) => {}
        TypedPattern::LitString(_, _) => {}
        TypedPattern::LitChar(_, _) => {}
        TypedPattern::Alias(_, _, p, _) => {
            collect_pattern_constraints(res, p);
        }
    }
}

fn collect_expr_constraints(res: &mut Vec<Constraint>, expr: &TypedExpr) {
    match expr {
        TypedExpr::Ref(_, _, _) => { /* ignore */ }
        TypedExpr::Const(_, _, _) => { /* ignore */ }
        TypedExpr::Tuple(_, ty, exprs) => {
            res.push(Constraint::new(expr.get_span(), ty, &Type::Tuple(exprs.map(expr_type))));
            for expr in exprs {
                collect_expr_constraints(res, expr);
            }
        }
        TypedExpr::List(_, ty, exprs) => {
            for expr in exprs {
                res.push(Constraint::new(expr.get_span(), ty, &type_list(expr_type(expr))));
                collect_expr_constraints(res, expr);
            }
        }
        TypedExpr::Record(_, ty, exprs) => {
            res.push(Constraint::new(expr.get_span(), ty, &Type::Record(
                exprs.map(|(s, e)| (s.clone(), expr_type(e)))
            )));

            for (_, expr) in exprs {
                collect_expr_constraints(res, expr);
            }
        }
        TypedExpr::RecordUpdate(_, ty, rec, exprs) => {
            // TODO record must have at least the attributes to update
//            res.push(Constraint::new(expr.get_span(), ty, &Type::RecExt(
//                format!("record"),
//                exprs.map(|(s, e)| (s.clone(), expr_type(e))),
//            )));
            res.push(Constraint::new(rec.get_span(), ty, &rec.get_type()));

            collect_expr_constraints(res, rec);
            for (_, expr) in exprs {
                collect_expr_constraints(res, expr);
            }
        }
        TypedExpr::RecordField(_, ty, record, name) => {
            match record.as_ref() {
                TypedExpr::Record(_, _, fields) => {
                    match fields.iter().find(|(f_name, _)| f_name == name) {
                        Some((_, expr)) => {
                            res.push(Constraint::new(expr.get_span(), ty, &expr_type(expr)));
                        }
                        None => {
//                            Err(TypeError::ExpectingRecordWithName { record: record.clone(), name: name.clone() })
                        }
                    }
                }
                _ => {
//                    Err(TypeError::ExpectingRecordWithName { record: record.clone(), name: name.clone() })
                }
            }

            collect_expr_constraints(res, record);
        }
        TypedExpr::RecordAccess(_, ty, name) => {
            // TODO proper input/output generated names
            res.push(Constraint::new(expr.get_span(), ty, &Type::Fun(
                Box::new(Type::RecExt("input".to_string(), vec![
                    (name.clone(), Type::Var("output".to_string()))
                ])),
                Box::new(Type::Var("output".to_string())),
            )));
        }
        TypedExpr::If(_, ty, a, b, c) => {
            res.push(Constraint::new(expr.get_span(), &expr_type(a), &type_bool()));
            res.push(Constraint::new(expr.get_span(), ty, &expr_type(b)));
            res.push(Constraint::new(expr.get_span(), ty, &expr_type(c)));
            collect_expr_constraints(res, a);
            collect_expr_constraints(res, b);
            collect_expr_constraints(res, c);
        }
        TypedExpr::Case(span, ty, expr, cases) => {
            collect_expr_constraints(res, expr);
            for (pat, expr) in cases {
                collect_pattern_constraints(res, pat);
                collect_expr_constraints(res, expr);

                res.push(Constraint::new(*span, ty, &expr.get_type()));
            }
        }
        TypedExpr::Lambda(span, ty, pat, expr) => {
            let mut chain = vec![];

            for pat in pat {
                chain.push(pat.get_type());
                collect_pattern_constraints(res, pat);
            }

            chain.push(expr.get_type());
            res.push(Constraint::new(*span, ty, &type_fun(chain)));

            collect_expr_constraints(res, expr);
        }
        TypedExpr::Application(_, ty, a, b) => {
            res.push(Constraint::new(expr.get_span(), &expr_type(a), &Type::Fun(
                Box::new(expr_type(b)),
                Box::new(ty.clone()),
            )));
            collect_expr_constraints(res, a);
            collect_expr_constraints(res, b);
        }
        TypedExpr::Let(_, _, _, expr) => {
            collect_expr_constraints(res, expr);
        }
    }
}

fn unify_constraints(constraints: &[Constraint]) -> Result<Substitution, TypeError> {
    if constraints.is_empty() {
        return Ok(Substitution::empty());
    }

    let mut sub = Substitution::empty();
    let mut vec = constraints.to_vec();

    while !vec.is_empty() {
        let new_sub = unify_one(&vec[0])?;
        vec = apply_substitution_set(&new_sub, &vec[1..]);
        sub = sub.merge(new_sub);
    }

    Ok(sub)
}

fn unify_one(constraint: &Constraint) -> Result<Substitution, TypeError> {
    let res = match constraint.as_pair() {
        (Type::Var(a), Type::Var(b)) if b.starts_with("number") && !a.starts_with("number") => {
            unify_var(constraint, a, &constraint.right)?
        }
        (Type::Var(a), Type::Var(b)) if a.starts_with("number") && !b.starts_with("number") => {
            unify_var(constraint, b, &constraint.left)?
        }
        (Type::Var(a), other) | (other, Type::Var(a)) => {
            unify_var(constraint, a, other)?
        }
        (Type::Unit, Type::Unit) => Substitution::empty(),
        (Type::Tag(n1, param1), Type::Tag(n2, param2))
        if n1 == n2 && param1.len() == param2.len() => {
            let c = param1.iter().zip(param2)
                .map(|(a, b)| Constraint::new(constraint.span, a, b))
                .collect::<Vec<_>>();

            unify_constraints(&c)?
        }
        (Type::Fun(arg1, param1), Type::Fun(arg2, param2)) => {
            unify_constraints(&[
                Constraint::new(constraint.span, arg1.as_ref(), arg2.as_ref()),
                Constraint::new(constraint.span, param1.as_ref(), param2.as_ref()),
            ])?
        }
        (Type::Tuple(param1), Type::Tuple(param2))
        if param1.len() == param2.len() => {
            let c = param1.iter().zip(param2)
                .map(|(a, b)| Constraint::new(constraint.span, a, b))
                .collect::<Vec<_>>();

            unify_constraints(&c)?
        }
        (Type::Record(param1), Type::Record(param2))
        if param1.len() == param2.len() => {
            let mut set = vec![];

            for (name1, ty1) in param1 {
                let mut found = false;
                for (name2, ty2) in param2 {
                    if name1 == name2 {
                        set.push(Constraint::new(constraint.span, ty1, ty2));
                        found = true;
                        break;
                    }
                }

                if !found {
                    panic!("Missing: {:?} in {:?}", name1, param2);
                }
            }

            unify_constraints(&set)?
        }
        (Type::RecExt(n1, param1), Type::RecExt(n2, param2))
        if n1 == n2 && param1.len() == param2.len() => {
            let mut set = vec![];

            for (name1, ty1) in param1 {
                let mut found = false;
                for (name2, ty2) in param2 {
                    if name1 == name2 {
                        set.push(Constraint::new(constraint.span, ty1, ty2));
                        found = true;
                        break;
                    }
                }

                if !found {
                    panic!("Missing: {:?} in {:?}", name1, param2);
                }
            }

            unify_constraints(&set)?
        }
        _ => {
            return Err(TypeError::TypeMatchingError {
                span: constraint.span,
                expected: constraint.left.clone(),
                found: constraint.right.clone(),
            });
        }
    };

    Ok(res)
}

enum UnifyError {
    Recursive,
    Incompatible,
}

/// Attempts to create a Substitution for a variable
fn unify_var(constraint: &Constraint, var: &str, ty: &Type) -> Result<Substitution, TypeError> {
    if var == "_" {
        return Ok(Substitution::empty());
    }

    if var.starts_with("number") {
        return match ty {
            Type::Var(var2) if var == var2 => {
                Ok(Substitution::empty())
            }
            Type::Var(var2) if var2.starts_with("number") => {
                Ok(Substitution::var_pair(var, ty))
            }
            Type::Tag(name, _) if name == "Int" || name == "Float" => {
                Ok(Substitution::var_pair(var, ty))
            }
            _ => {
                return Err(TypeError::TypeMatchingError {
                    span: constraint.span,
                    expected: constraint.left.clone(),
                    found: constraint.right.clone(),
                });
            }
        };
    }

    match ty {
        Type::Var(var2) if var == var2 => {
            Ok(Substitution::empty())
        }
        Type::Var(_) => {
            Ok(Substitution::var_pair(var, ty))
        }
        _ if occurs(var, ty) => {
            return Err(TypeError::RecursiveTypeDefinition {
                span: constraint.span,
                var: var.to_string(),
                ty: ty.clone(),
            });
        }
        _ => {
            Ok(Substitution::var_pair(var, ty))
        }
    }
}

fn occurs(var: &str, ty: &Type) -> bool {
    match ty {
        Type::Unit => false,
        Type::Var(var2) => var == var2,
        Type::Fun(a, b) => occurs(var, a) || occurs(var, b),
        Type::Tag(_, items) | Type::Tuple(items) => items.iter().any(|i| occurs(var, i)),
        Type::Record(items) | Type::RecExt(_, items) => items.iter().any(|(_, i)| occurs(var, i))
    }
}

fn apply_substitution_set(sub: &Substitution, cons: &[Constraint]) -> Vec<Constraint> {
    cons.iter().map(|c| apply_substitution_constraint(sub, c)).collect::<Vec<_>>()
}

fn apply_substitution_constraint(sub: &Substitution, cons: &Constraint) -> Constraint {
    Constraint::new(
        cons.span,
        &apply_substitution_ty(sub, &cons.left),
        &apply_substitution_ty(sub, &cons.right),
    )
}

fn apply_substitution_ty(sub: &Substitution, ty: &Type) -> Type {
    sub.0.iter().fold(ty.clone(), |result, (var, sol_ty)| {
        apply_substitution(&result, var, sol_ty)
    })
}

fn apply_substitution(ty: &Type, var: &Type, replacement: &Type) -> Type {
    match ty {
        Type::Unit => ty.clone(),
        Type::Var(_) => {
            if ty == var { replacement.clone() } else { ty.clone() }
        }
        Type::Tag(name, items) => {
            Type::Tag(name.clone(), items.map(|i| apply_substitution(i, var, replacement)))
        }
        Type::Fun(a, b) => {
            Type::Fun(
                Box::new(apply_substitution(a, var, replacement)),
                Box::new(apply_substitution(b, var, replacement)),
            )
        }
        Type::Tuple(items) => {
            Type::Tuple(items.map(|i| apply_substitution(i, var, replacement)))
        }
        Type::Record(items) => {
            Type::Record(items.map(|(s, i)|
                (s.clone(), apply_substitution(i, var, replacement))
            ))
        }
        Type::RecExt(_, items) => {
            Type::Record(items.map(|(s, i)|
                (s.clone(), apply_substitution(i, var, replacement))
            ))
        }
    }
}

fn replace_pattern_types(sub: &Substitution, annotated: TypedPattern) -> TypedPattern {
    match annotated {
        TypedPattern::Var(a, b, c) => {
            TypedPattern::Var(a, sub.replace(b), c)
        }
        TypedPattern::Adt(a, b, c, d) => {
            TypedPattern::Adt(
                a,
                sub.replace(b),
                sub.replace(c),
                d.into_iter().map(|it| replace_pattern_types(sub, it)).collect(),
            )
        }
        TypedPattern::Wildcard(a) => {
            TypedPattern::Wildcard(a)
        }
        TypedPattern::Unit(a) => {
            TypedPattern::Unit(a)
        }
        TypedPattern::Tuple(a, b, c) => {
            TypedPattern::Tuple(
                a,
                sub.replace(b),
                c.into_iter().map(|it| replace_pattern_types(sub, it)).collect(),
            )
        }
        TypedPattern::List(a, b, c) => {
            TypedPattern::List(
                a,
                sub.replace(b),
                c.into_iter().map(|it| replace_pattern_types(sub, it)).collect(),
            )
        }
        TypedPattern::BinaryOp(a, b, c, d, e) => {
            TypedPattern::BinaryOp(
                a,
                sub.replace(b),
                c,
                Box::new(replace_pattern_types(sub, *d)),
                Box::new(replace_pattern_types(sub, *e)),
            )
        }
        TypedPattern::Record(a, b, c) => {
            TypedPattern::Record(a, sub.replace(b), c)
        }
        TypedPattern::LitInt(a, b) => {
            TypedPattern::LitInt(a, b)
        }
        TypedPattern::LitString(a, b) => {
            TypedPattern::LitString(a, b)
        }
        TypedPattern::LitChar(a, b) => {
            TypedPattern::LitChar(a, b)
        }
        TypedPattern::Alias(a, b, c, d) => {
            TypedPattern::Alias(
                a,
                sub.replace(b),
                Box::new(replace_pattern_types(sub, *c)),
                d,
            )
        }
    }
}

fn replace_expr_types(sub: &Substitution, annotated: TypedExpr) -> TypedExpr {
    match annotated {
        TypedExpr::Const(span, ty, a) => {
            TypedExpr::Const(span, sub.replace(ty), a)
        }
        TypedExpr::Tuple(span, ty, a) => {
            TypedExpr::Tuple(
                span,
                sub.replace(ty),
                a.into_iter().map(|a| replace_expr_types(sub, a)).to_vec(),
            )
        }
        TypedExpr::List(span, ty, a) => {
            TypedExpr::List(
                span,
                sub.replace(ty),
                a.into_iter().map(|a| replace_expr_types(sub, a)).to_vec(),
            )
        }
        TypedExpr::Record(span, ty, a) => {
            TypedExpr::Record(
                span,
                sub.replace(ty),
                a.into_iter().map(|(s, a)| (s, replace_expr_types(sub, a))).to_vec(),
            )
        }
        TypedExpr::RecordUpdate(span, ty, a, b) => {
            TypedExpr::RecordUpdate(
                span,
                sub.replace(ty),
                Box::new(replace_expr_types(sub, *a)),
                b.into_iter().map(|(s, a)| (s, replace_expr_types(sub, a))).to_vec(),
            )
        }
        TypedExpr::Ref(span, ty, a) => {
            TypedExpr::Ref(span, sub.replace(ty), a)
        }
        TypedExpr::RecordField(span, ty, a, b) => {
            TypedExpr::RecordField(span, sub.replace(ty), Box::new(replace_expr_types(sub, *a)), b)
        }
        TypedExpr::RecordAccess(span, ty, a) => {
            TypedExpr::RecordAccess(span, sub.replace(ty), a)
        }
        TypedExpr::If(span, ty, a, b, c) => {
            TypedExpr::If(
                span,
                sub.replace(ty),
                Box::new(replace_expr_types(sub, *a)),
                Box::new(replace_expr_types(sub, *b)),
                Box::new(replace_expr_types(sub, *c)),
            )
        }
        TypedExpr::Case(span, ty, a, b) => {
            TypedExpr::Case(
                span,
                sub.replace(ty),
                Box::new(replace_expr_types(sub, *a)),
                b.into_iter().map(|(s, a)| (s, replace_expr_types(sub, a))).to_vec(),
            )
        }
        TypedExpr::Lambda(span, ty, a, b) => {
            TypedExpr::Lambda(
                span,
                sub.replace(ty),
                a,
                Box::new(replace_expr_types(sub, *b)),
            )
        }
        TypedExpr::Application(span, ty, a, b) => {
            TypedExpr::Application(
                span,
                sub.replace(ty),
                Box::new(replace_expr_types(sub, *a)),
                Box::new(replace_expr_types(sub, *b)),
            )
        }
        TypedExpr::Let(span, ty, a, b) => {
            TypedExpr::Let(
                span,
                sub.replace(ty),
                a,
                Box::new(replace_expr_types(sub, *b)))
        }
    }
}

fn add_pattern_vars_to_env(env: &mut Env, pat: &TypedPattern) {
    match pat {
        TypedPattern::Var(_, ty, name) => {
            env.set(name, ty.clone());
        }
        TypedPattern::Adt(_, _, _, items) => {
            items.for_each(|it| add_pattern_vars_to_env(env, it));
        }
        TypedPattern::Wildcard(_) => {}
        TypedPattern::Unit(_) => {}
        TypedPattern::Tuple(_, _, items) => {
            items.for_each(|it| add_pattern_vars_to_env(env, it));
        }
        TypedPattern::List(_, _, items) => {
            items.for_each(|it| add_pattern_vars_to_env(env, it));
        }
        TypedPattern::BinaryOp(_, _, _, a, b) => {
            add_pattern_vars_to_env(env, a);
            add_pattern_vars_to_env(env, b);
        }
        TypedPattern::Record(_, _ty, _fields) => {
            // TODO change fields type for TypedPattern::Var
        }
        TypedPattern::LitInt(_, _) => {}
        TypedPattern::LitString(_, _) => {}
        TypedPattern::LitChar(_, _) => {}
        TypedPattern::Alias(_, ty, pat, name) => {
            add_pattern_vars_to_env(env, pat.as_ref());
            env.set(name, ty.clone());
        }
    }
}

fn is_exhaustive(pattern: &Pattern) -> bool {
    match pattern {
        Pattern::Var(_, _) => true,
        Pattern::Adt(_, _, _) => true,
        Pattern::Wildcard(_, ) => true,
        Pattern::Unit(_, ) => true,
        Pattern::Tuple(_, sub_patterns) => {
            sub_patterns.iter().all(|p| is_exhaustive(p))
        }
        Pattern::List(_, _) => false,
        Pattern::Alias(_, pat, _) => is_exhaustive(pat),
        Pattern::BinaryOp(_, _, _, _) => false,
        Pattern::Record(_, _) => true,
        Pattern::LitInt(_, _) => false,
        Pattern::LitString(_, _) => false,
        Pattern::LitChar(_, _) => false,
    }
}

pub fn expr_tree_to_expr(tree: ExprTree) -> Expr {
    match tree {
        ExprTree::Leaf(e) => e,
        ExprTree::Branch(op, left, right) => {
            let left_expr = expr_tree_to_expr(*left);
            let right_expr = expr_tree_to_expr(*right);
            let span = (left_expr.get_span().0, right_expr.get_span().1);

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

fn vec_map<ENV, F, A, B, E>(env: &mut ENV, vec: &Vec<A>, mut func: F) -> Result<Vec<B>, E>
    where F: FnMut(&mut ENV, &A) -> Result<B, E> {
    let mut result = vec![];

    for a in vec {
        result.push(func(env, a)?);
    }

    Ok(result)
}

fn vec_pair_map<ENV, F, A, B, E, S>(env: &mut ENV, vec: &Vec<(S, A)>, mut func: F) -> Result<Vec<(S, B)>, E>
    where F: FnMut(&mut ENV, &A) -> Result<B, E>,
          S: Clone
{
    let mut result = vec![];

    for (s, a) in vec {
        result.push((s.clone(), func(env, a)?));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use constructors::{type_number_num, type_of, type_string};
    use test_utils::Test;
    use util::StringConversion;

    use super::*;

    #[test]
    fn test_infer_type_of_sum() {
        let expr = Test::definition("a = 1 + 2");
        let mut env = Env::new();
        env.set("+", type_of("Int -> Int -> Int"));

        let typed_expr = infer_definition_type(&mut env, &expr).unwrap();

        assert_eq!(type_of("Int"), typed_expr.header.clone());
    }

    #[test]
    fn test_infer_type_of_closure() {
        let expr = Test::definition("genClosure x = (\\y -> x + y)");
        let mut env = Env::new();
        env.set("+", type_of("Int -> Int -> Int"));

        let typed_expr = infer_definition_type(&mut env, &expr).unwrap();

        assert_eq!(type_of("Int -> Int -> Int"), typed_expr.header.clone());
    }

    #[test]
    fn test_infer_type_of_complex_operation() {
        let expr = Test::definition("a = 1 + 3.2 + (1 + 2)");
        let mut env = Env::new();
        env.set("+", type_of("number -> number -> number"));

        let typed_expr = infer_definition_type(&mut env, &expr).unwrap();

        assert_eq!(type_of("Float"), typed_expr.header.clone());
    }

    #[test]
    fn test_type_error() {
        let expr = Test::definition("a = 1 + 3.2 + (true + 2)");
        let mut env = Env::new();
        env.set("+", type_of("number -> number -> number"));
        env.set("true", type_of("Bool"));

        let typed_expr = infer_definition_type(&mut env, &expr);

        assert_eq!(Err(TypeError::TypeMatchingError {
            span: (15, 23),
            expected: type_of("number3"),
            found: type_of("Bool"),
        }), typed_expr);
    }

    #[test]
    fn test_infer_type_of_duplicated_vars() {
        let expr = Test::definition("a = ((+), (+))");
        let mut env = Env::new();
        env.set("+", type_of("number -> number -> number"));

        let typed_expr = infer_definition_type(&mut env, &expr).unwrap();

        assert_eq!(type_of("(number -> number -> number, number1 -> number1 -> number1)"), typed_expr.header.clone());
    }

    fn analyze_expression(analyzer: &mut Analyzer, expr: &Expr) -> Result<Type, TypeError> {
        let res = analyzer.analyze_expression(expr);
        match res {
            Ok(expr) => {
                eprintln!("Expr:\n{}", expr);
                Ok(expr_type(&expr))
            }
            Err(ElmError::Analyser(_, err)) => {
                Err(err)
            }
            Err(err) => {
                panic!("Error: {}", err);
            }
        }
    }

    #[test]
    fn check_record_update() {
        let (expr, mut analyzer) = Test::expr_analyzer("{ x | a = 0 }");

        // Type of x
        let record_type = type_of("{ a: number, b: number }");

        analyzer.add_port("x", record_type.clone());

        let result = analyze_expression(&mut analyzer, &expr);
        assert_eq!(result, Ok(record_type));
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
            Err(TypeError::TypeMatchingError {
                span: (0, 27),
                expected: type_number_num(1),
                found: type_string(),
            }),
            analyze_expression(&mut analyzer, &expr)
        );
    }
}


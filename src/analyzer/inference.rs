use std::collections::HashMap;

use analyzer::static_env::StaticEnv;
use analyzer::type_inference::expr_tree_to_expr;
use ast::Definition;
use ast::Expr;
use ast::Literal;
use ast::Pattern;
use ast::Type;
use constructors::{type_bool, type_record, type_var};
use constructors::type_list;
use errors::TypeError;
use typed_ast::{expr_type, TypedPattern};
use typed_ast::TypedDefinition;
use typed_ast::TypedExpr;
use types::Value;
use util::expression_fold::create_expr_tree;
use util::expression_fold::ExprTreeError;
use util::name_sequence::NameSequence;
use util::qualified_name;
use util::ToVec;
use util::VecExt;

// https://youtu.be/oPVTNxiMcSU?t=4301
type Constraint = (Type, Type);

#[derive(Debug)]
struct Substitution(HashMap<Type, Type>);

impl Substitution {
    fn empty() -> Self {
        Substitution(HashMap::new())
    }

    fn pair(a: &Type, b: &Type) -> Self {
        let mut map = HashMap::new();
        map.insert(a.clone(), b.clone());
        Substitution(map)
    }

    fn var_pair(var: &str, ty: &Type) -> Self {
        let mut map = HashMap::new();
        map.insert(Type::Var(var.to_string()), ty.clone());
        Substitution(map)
    }

    fn merge(self, b: Substitution) -> Substitution {
        let mut map = HashMap::new();

        map.extend(self.0.into_iter().map(|(k, v)| (k, apply_substitution_ty(&b, &v))));
        map.extend(b.0);

        Substitution(map)
    }

    fn replace(&self, ty: Type) -> Type {
        self.0.get(&ty).cloned().unwrap_or(ty)
    }
}

struct Env {
    blocks: Vec<HashMap<String, Type>>,
    alias: HashMap<String, Type>,
    generator: NameSequence,
    number: NameSequence,
}

impl Env {
    fn new() -> Self {
        Env {
            blocks: vec![HashMap::new()],
            alias: HashMap::new(),
            generator: NameSequence::new(),
            number: NameSequence::new(),
        }
    }

    fn get(&self, name: &str) -> Option<&Type> {
        for block in self.blocks.iter().rev() {
            if let Some(ty) = block.get(name) {
                return Some(ty);
            }
        }

        None
    }

    fn set(&mut self, name: &str, ty: Type) {
        self.blocks.last_mut().unwrap().insert(name.to_string(), ty);
    }

    fn next_type(&mut self) -> Type {
        Type::Var(self.generator.next())
    }

    fn next_number_type(&mut self) -> Type {
        Type::Var(self.number.next_with_prefix("number"))
    }

    pub fn enter_block(&mut self) {
        self.blocks.push(HashMap::new());
    }

    pub fn exit_block(&mut self) {
        self.blocks.pop().expect("Tried to pop the global environment");
    }
}

impl From<Literal> for Value {
    fn from(lit: Literal) -> Self {
        match lit {
            Literal::Int(i) => Value::Number(i),
            Literal::Float(i) => Value::Float(i),
            Literal::String(i) => Value::String(i.clone()),
            Literal::Char(i) => Value::Char(i),
        }
    }
}

fn analyze_definition(env: &mut Env, fun: &Definition) -> Result<TypedDefinition, TypeError> {
    // Type Annotation
    env.enter_block();
    let mut annotated_patterns = vec![];
    for pat in &fun.patterns {
        annotated_patterns.push(annotate_pattern(env, &pat)?);
    }
    let annotated_expr = annotate_expr(env, &fun.expr)?;
    env.exit_block();

    // Constraint collection
    let mut constraints = vec![];

    for pat in &annotated_patterns {
        collect_pattern_constraints(&mut constraints, pat);
    }

    collect_expr_constraints(&mut constraints, &annotated_expr);

    // Constraint solutions
    let substitution = unify_constraints(&constraints);
    let res = replace_types(&substitution, annotated_expr);


    Ok(TypedDefinition {
        header: expr_type(&res),
        name: fun.name.to_string(),
        patterns: fun.patterns.clone(),
        expr: res,
    })
}

fn infer_types(env: &mut Env, expr: &Expr) -> Result<TypedExpr, TypeError> {
    let annotated = annotate_expr(env, expr)?;
    let mut constraints = vec![];

    collect_expr_constraints(&mut constraints, &annotated);

    eprintln!("Tree: \n{}\n", &annotated);

    eprintln!("Constraints: ");
    for (a, b) in &constraints {
        eprintln!("{} => {}", a, b);
    }
    eprintln!();

    let substitution = unify_constraints(&constraints);

    eprintln!("Substitutions: ");
    for (a, b) in &substitution.0 {
        eprintln!("{} => {}", a, b);
    }
    eprintln!();

    let res = replace_types(&substitution, annotated);

    eprintln!("Tree: \n{}\n", &res);

    Ok(res)
}

fn update_vars(env: &mut Env, dup: &mut HashMap<String, Type>, ty: Type) -> Type {
    match ty {
        Type::Var(name) => {
            match dup.get(&name).cloned() {
                Some(var) => var,
                None => {
                    let new_ty = if name.starts_with("number") {
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
                Box::new(update_vars(env, dup, *a)),
                Box::new(update_vars(env, dup, *b)),
            )
        }
        Type::Tag(name, items) => {
            let vec: Vec<Type> = items.into_iter().map(|e| update_vars(env, dup, e)).collect();
            Type::Tag(name, vec)
        }
        Type::Tuple(items) => {
            let vec: Vec<Type> = items.into_iter().map(|e| update_vars(env, dup, e)).collect();
            Type::Tuple(vec)
        }
        Type::Record(items) => {
            let vec: Vec<(String, Type)> = items.into_iter().map(|(s, e)| (s, update_vars(env, dup, e))).collect();
            Type::Record(vec)
        }
        Type::RecExt(name, items) => {
            let vec: Vec<(String, Type)> = items.into_iter().map(|(s, e)| (s, update_vars(env, dup, e))).collect();
            Type::RecExt(name, vec)
        }
        Type::Unit => Type::Unit,
    }
}

fn vec_map<ENV, F, A, B, E>(env: &mut ENV, vec: &Vec<A>, mut func: F) -> Result<Vec<B>, E>
    where F: FnMut(ENV, A) -> Result<B, E> {
    let mut result = vec![];

    for a in vec {
        result.push(func(env, a)?);
    }

    Ok(result)
}

fn vec_pair_map<ENV, F, A, B, E, S>(env: &mut ENV, vec: &Vec<(S, A)>, mut func: F) -> Result<Vec<(S, B)>, E>
    where F: FnMut(ENV, A) -> Result<B, E>,
          S: Clone
{
    let mut result = vec![];

    for (s, a) in vec {
        result.push((s.clone(), func(env, a)?));
    }

    Ok(result)
}

fn map_pair<A, B, E, S, F>(vec: &Vec<(S, A)>, mut func: F) -> Result<Vec<(S, B)>, E>
    where F: FnMut(&A) -> Result<B, E>,
          S: Clone
{
    let mut result = vec![];

    for (s, a) in vec {
        let b = func(a)?;

        result.push((s.clone(), b));
    }

    Ok(result)
}

fn annotate_pattern(env: &mut Env, pat: &Pattern) -> Result<TypedPattern, TypeError> {
    let typed = match pat {
        Pattern::Var(name) => {
            if env.get(name).is_some() {
                return Err(TypeError::VariableNameShadowed { span: (0, 0), name: name.clone() });
            }

            TypedPattern::Var(env.next_type(), name.clone())
        }
        Pattern::Adt(name, items) => {
            TypedPattern::Adt(
                env.next_type(),
                name.clone(),
                vec_map(env, items, annotate_pattern)?,
            )
        }
        Pattern::Wildcard => {
            TypedPattern::Wildcard
        }
        Pattern::Unit => {
            TypedPattern::Unit
        }
        Pattern::Tuple(items) => {
            TypedPattern::Tuple(
                env.next_type(),
                vec_map(env, items, annotate_pattern)?,
            )
        }
        Pattern::List(items) => {
            TypedPattern::List(
                env.next_type(),
                vec_map(env, items, annotate_pattern)?,
            )
        }
        Pattern::BinaryOp(op, a, b) => {
            TypedPattern::BinaryOp(
                env.next_type(),
                op.clone(),
                Box::new(annotate_pattern(env, a)?),
                Box::new(annotate_pattern(env, b)?),
            )
        }
        Pattern::Record(items) => {
            TypedPattern::Record(
                env.next_type(),
                vec_pair_map(env, items, annotate_pattern)?,
            )
        }
        Pattern::LitInt(val) => {
            TypedPattern::LitInt(*val)
        }
        Pattern::LitString(val) => {
            TypedPattern::LitString(val.clone())
        }
        Pattern::LitChar(val) => {
            TypedPattern::LitChar(*val)
        }
        Pattern::Alias(pat, name) => {
            let ty = annotate_pattern(env, pat)?;
            env.set(name, ty.get_type());
            TypedPattern::Alias(ty.get_type(), Box::new(ty), name.clone())
        }
    };

    Ok(typed)
}

fn annotate_expr(env: &mut Env, expr: &Expr) -> Result<TypedExpr, TypeError> {
    let te = match expr {
        Expr::QualifiedRef(span, base, name) => {
            let name = qualified_name(base, name);
            let ty = env.get(&name).cloned()
                .ok_or_else(|| TypeError::MissingDefinition { span: *span, name: name.to_string() })?;

            let ty = update_vars(env, &mut HashMap::new(), ty);
            TypedExpr::Ref(ty, name.clone())
        }
        Expr::Ref(span, name) => {
            let ty = env.get(name).cloned()
                .ok_or_else(|| TypeError::MissingDefinition { span: *span, name: name.to_string() })?;

            let ty = update_vars(env, &mut HashMap::new(), ty);
            TypedExpr::Ref(ty, name.clone())
        }
        Expr::Literal(_, lit) => {
            let value: Value = lit.clone().into();
            if let Value::Number(_) = &value {
                TypedExpr::Const(env.next_number_type(), value)
            } else {
                TypedExpr::Const(value.get_type(), value)
            }
        }
        Expr::Unit(_) => {
            TypedExpr::Const(env.next_type(), Value::Unit)
        }
        Expr::Tuple(_, exprs) => {
            TypedExpr::Tuple(
                env.next_type(),
                vec_map(env, exprs, annotate_expr)?,
            )
        }
        Expr::List(_, exprs) => {
            TypedExpr::List(
                env.next_type(),
                vec_map(env, exprs, annotate_expr)?,
            )
        }
        Expr::Record(_, exprs) => {
            TypedExpr::Record(
                env.next_type(),
                map_pair(exprs, |e| annotate_expr(env, e))?,
            )
        }
        Expr::RecordUpdate(_, name, exprs) => {
            let sub = annotate_expr(env, &Expr::Ref((0, 0), name.clone()))?;
            TypedExpr::RecordUpdate(
                env.next_type(),
                Box::new(sub),
                map_pair(exprs, |e| annotate_expr(env, e))?,
            )
        }
        Expr::RecordField(_, expr, name) => {
            TypedExpr::RecordField(
                env.next_type(),
                Box::new(annotate_expr(env, expr)?),
                name.clone(),
            )
        }
        Expr::RecordAccess(_, name) => {
            TypedExpr::RecordAccess(
                env.next_type(),
                name.clone(),
            )
        }
        Expr::If(_, a, b, c) => {
            TypedExpr::If(
                env.next_type(),
                Box::new(annotate_expr(env, a)?),
                Box::new(annotate_expr(env, b)?),
                Box::new(annotate_expr(env, c)?),
            )
        }
        Expr::Case(_, expr, branches) => {
            TypedExpr::Case(
                env.next_type(),
                Box::new(annotate_expr(env, expr)?),
                map_pair(branches, |e| annotate_expr(env, e))?,
            )
        }
        Expr::Lambda(_, pat, expr) => {
            TypedExpr::Lambda(
                env.next_type(),
                pat.clone(),
                Box::new(annotate_expr(env, expr)?),
            )
        }
        Expr::Application(_, a, b) => {
            TypedExpr::Application(
                env.next_type(),
                Box::new(annotate_expr(env, a)?),
                Box::new(annotate_expr(env, b)?),
            )
        }
        Expr::Let(_, decls, expr) => {
            unimplemented!()
//            TypedExpr::Let(
//                env.next_type(),
//                entries,
//                Box::new(annotate_expr(env, expr))
//            )
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

fn collect_pattern_constraints(res: &mut Vec<Constraint>, pat: &TypedPattern) {
    match pat {
        TypedPattern::Var(_, _) => {}
        TypedPattern::Adt(ty, name, items) => {
            res.push((ty.clone(), Type::Tag(name.clone(), items.map(|e| e.get_type()))));
            items.for_each(|it| collect_pattern_constraints(res, it));
        }
        TypedPattern::Wildcard => {}
        TypedPattern::Unit => {}
        TypedPattern::Tuple(ty, items) => {
            res.push((ty.clone(), Type::Tag(name.clone(), items.map(|e| e.get_type()))));
            items.for_each(|it| collect_pattern_constraints(res, it));
        }
        TypedPattern::List(ty, items) => {
            items.for_each(|it| {
                res.push((ty.clone(), type_list(it.get_type())));
                collect_pattern_constraints(res, it);
            });
        }
        TypedPattern::BinaryOp(ty, op, a, b) => {
            assert_eq!("::", op.as_str());
            res.push((ty.clone(), type_list(a.get_type())));
            res.push((b.get_type(), type_list(a.get_type())));

            collect_pattern_constraints(res, a);
            collect_pattern_constraints(res, b);
        }
        TypedPattern::Record(ty, items) => {
            res.push((
                ty.clone(),
                Type::Record(
                    items.map(|it| (it.clone(), type_var(it)))
                )
            ));
        }
        TypedPattern::LitInt(_) => {}
        TypedPattern::LitString(_) => {}
        TypedPattern::LitChar(_) => {}
        TypedPattern::Alias(_, p, _) => {
            collect_pattern_constraints(res, b);
        }
    }
}

fn collect_expr_constraints(res: &mut Vec<Constraint>, expr: &TypedExpr) {
    match expr {
        TypedExpr::Ref(ty, _) => { /* ignore */ }
        TypedExpr::Const(ty, val) => { /* ignore */ }
        TypedExpr::Tuple(ty, exprs) => {
            res.push((ty.clone(), Type::Tuple(exprs.map(expr_type))));
            for expr in exprs {
                collect_expr_constraints(res, expr);
            }
        }
        TypedExpr::List(ty, exprs) => {
            for expr in exprs {
                res.push((ty.clone(), type_list(expr_type(expr))));
                collect_expr_constraints(res, expr);
            }
        }
        TypedExpr::Record(ty, exprs) => {
            res.push((
                ty.clone(),
                Type::Record(exprs.map(|(s, e)| (s.clone(), expr_type(e))))
            ));

            for (_, expr) in exprs {
                collect_expr_constraints(res, expr);
            }
        }
        TypedExpr::RecordUpdate(ty, rec, exprs) => {
            // TODO change RecExt to use TypeExpr instead of String
            let name: String = if let Type::Var(a) = expr_type(rec) {
                a
            } else {
                unreachable!()
            };

            res.push((
                ty.clone(),
                Type::RecExt(
                    name,
                    exprs.map(|(s, e)| (s.clone(), expr_type(e))),
                )
            ));

            collect_expr_constraints(res, rec);
            for (_, expr) in exprs {
                collect_expr_constraints(res, expr);
            }
        }
        TypedExpr::RecordField(ty, record, name) => {
            match record.as_ref() {
                TypedExpr::Record(_, fields) => {
                    match fields.iter().find(|(f_name, _)| f_name == name) {
                        Some((_, expr)) => {
                            res.push((
                                ty.clone(),
                                expr_type(expr),
                            ));
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
        TypedExpr::RecordAccess(ty, name) => {
            // TODO proper input/output generated names
            res.push((
                ty.clone(),
                Type::Fun(
                    Box::new(Type::RecExt("input".to_string(), vec![
                        (name.clone(), Type::Var("output".to_string()))
                    ])),
                    Box::new(Type::Var("output".to_string())),
                )
            ));
        }
        TypedExpr::If(ty, a, b, c) => {
            res.push((expr_type(a), type_bool()));
            res.push((ty.clone(), expr_type(b)));
            res.push((ty.clone(), expr_type(c)));
            collect_expr_constraints(res, a);
            collect_expr_constraints(res, b);
            collect_expr_constraints(res, c);
        }
        TypedExpr::Case(ty, expr, cases) => {
            collect_expr_constraints(res, expr);
            for (pat, expr) in cases {
//                collect_pattern_constraints(res, pat);
                collect_expr_constraints(res, expr);
            }
        }
        TypedExpr::Lambda(ty, pat, expr) => {
            // todo lambda type constraint
//            for pat in pat {
//                collect_pattern_constraints(res, pat);
//            }
            collect_expr_constraints(res, expr);
        }
        TypedExpr::Application(ty, a, b) => {
            res.push((
                expr_type(a),
                Type::Fun(
                    Box::new(expr_type(b)),
                    Box::new(ty.clone()),
                )
            ));
            collect_expr_constraints(res, a);
            collect_expr_constraints(res, b);
        }
        TypedExpr::Let(ty, _, _) => {
            unimplemented!()
        }
    }
}

fn unify_constraints(constraints: &[Constraint]) -> Substitution {
    if constraints.is_empty() {
        return Substitution::empty();
    }

    let mut sub = Substitution::empty();
    let mut vec = constraints.to_vec();

    while !vec.is_empty() {
        let new_sub = unify_one(&vec[0]);
        vec = apply_substitution_set(&new_sub, &vec[1..]);
        sub = sub.merge(new_sub);
    }

    sub
}

fn unify_one(constraint: &Constraint) -> Substitution {
    match constraint {
        (Type::Unit, Type::Unit) => Substitution::empty(),
        (Type::Var(a), other) | (other, Type::Var(a)) => {
            unify_var(a, other)
        }
        (Type::Tag(n1, param1), Type::Tag(n2, param2))
        if n1 == n2 && param1.len() == param2.len() => {
            let c = param1.iter().zip(param2)
                .map(|(a, b)| (a.clone(), b.clone()))
                .collect::<Vec<_>>();

            unify_constraints(&c)
        }
        (Type::Fun(arg1, param1), Type::Fun(arg2, param2)) => {
            unify_constraints(&[
                (arg1.as_ref().clone(), arg2.as_ref().clone()),
                (param1.as_ref().clone(), param2.as_ref().clone()),
            ])
        }
        (Type::Tuple(param1), Type::Tuple(param2))
        if param1.len() == param2.len() => {
            let c = param1.iter().zip(param2)
                .map(|(a, b)| (a.clone(), b.clone()))
                .collect::<Vec<_>>();

            unify_constraints(&c)
        }
        (Type::Record(param1), Type::Record(param2))
        if param1.len() == param2.len() => {
            let mut set = vec![];

            for (name1, ty1) in param1 {
                let mut found = false;
                for (name2, ty2) in param2 {
                    if name1 == name2 {
                        set.push((ty1.clone(), ty2.clone()));
                        found = true;
                        break;
                    }
                }

                if !found {
                    panic!("Missing: {:?} in {:?}", name1, param2);
                }
            }

            unify_constraints(&set)
        }
        (Type::RecExt(n1, param1), Type::RecExt(n2, param2))
        if n1 == n2 && param1.len() == param2.len() => {
            let mut set = vec![];

            for (name1, ty1) in param1 {
                let mut found = false;
                for (name2, ty2) in param2 {
                    if name1 == name2 {
                        set.push((ty1.clone(), ty2.clone()));
                        found = true;
                        break;
                    }
                }

                if !found {
                    panic!("Missing: {:?} in {:?}", name1, param2);
                }
            }

            unify_constraints(&set)
        }
        _ => panic!("\nType error:\n expected: {}\n    found: {}\n", constraint.0, constraint.1)
    }
}

fn unify_var(var: &str, ty: &Type) -> Substitution {
    match ty {
        Type::Var(var2) if var == var2 => Substitution::empty(),
        Type::Var(var2) => Substitution::var_pair(var, ty),
        _ if occurs(var, ty) => panic!("Recursive type"),
        _ => Substitution::var_pair(var, ty),
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
    (
        apply_substitution_ty(sub, &cons.0),
        apply_substitution_ty(sub, &cons.1),
    )
}

fn apply_substitution_ty(sub: &Substitution, ty: &Type) -> Type {
    sub.0.iter().fold(ty.clone(), |result, (var, solTy)| {
        apply_substitution(&result, var, solTy)
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
        Type::RecExt(name, items) => {
            Type::Record(items.map(|(s, i)|
                (s.clone(), apply_substitution(i, var, replacement))
            ))
        }
    }
}

fn replace_types(sub: &Substitution, annotated: TypedExpr) -> TypedExpr {
    match annotated {
        TypedExpr::Const(ty, a) => {
            TypedExpr::Const(sub.replace(ty), a)
        }
        TypedExpr::Tuple(ty, a) => {
            TypedExpr::Tuple(
                sub.replace(ty),
                a.into_iter().map(|a| replace_types(sub, a)).to_vec(),
            )
        }
        TypedExpr::List(ty, a) => {
            TypedExpr::List(
                sub.replace(ty),
                a.into_iter().map(|a| replace_types(sub, a)).to_vec(),
            )
        }
        TypedExpr::Record(ty, a) => {
            TypedExpr::Record(
                sub.replace(ty),
                a.into_iter().map(|(s, a)| (s, replace_types(sub, a))).to_vec(),
            )
        }
        TypedExpr::RecordUpdate(ty, a, b) => {
            TypedExpr::RecordUpdate(
                sub.replace(ty),
                Box::new(replace_types(sub, *a)),
                b.into_iter().map(|(s, a)| (s, replace_types(sub, a))).to_vec(),
            )
        }
        TypedExpr::Ref(ty, a) => {
            TypedExpr::Ref(sub.replace(ty), a)
        }
        TypedExpr::RecordField(ty, a, b) => {
            TypedExpr::RecordField(sub.replace(ty), Box::new(replace_types(sub, *a)), b)
        }
        TypedExpr::RecordAccess(ty, a) => {
            TypedExpr::RecordAccess(sub.replace(ty), a)
        }
        TypedExpr::If(ty, a, b, c) => {
            TypedExpr::If(
                sub.replace(ty),
                Box::new(replace_types(sub, *a)),
                Box::new(replace_types(sub, *b)),
                Box::new(replace_types(sub, *c)),
            )
        }
        TypedExpr::Case(ty, a, b) => {
            TypedExpr::Case(
                sub.replace(ty),
                Box::new(replace_types(sub, *a)),
                b.into_iter().map(|(s, a)| (s, replace_types(sub, a))).to_vec(),
            )
        }
        TypedExpr::Lambda(ty, a, b) => {
            TypedExpr::Lambda(
                sub.replace(ty),
                a,
                Box::new(replace_types(sub, *b)),
            )
        }
        TypedExpr::Application(ty, a, b) => {
            TypedExpr::Application(
                sub.replace(ty),
                Box::new(replace_types(sub, *a)),
                Box::new(replace_types(sub, *b)),
            )
        }
        TypedExpr::Let(ty, a, b) => {
            TypedExpr::Let(
                sub.replace(ty),
                a,
                Box::new(replace_types(sub, *b)))
        }
    }
}

#[cfg(test)]
mod tests {
    use constructors::type_of;
    use test_utils::Test;

    use super::*;

    #[test]
    fn test_infer_type_of_sum() {
        let expr = Test::expr("1 + 2");
        let mut env = Env::new();
        env.set("+", type_of("Int -> Int -> Int"));

        let typed_expr = infer_types(&mut env, &expr).unwrap();

        assert_eq!(type_of("Int"), expr_type(&typed_expr));
    }

    #[test]
    fn test_infer_type_of_complex_operation() {
        let expr = Test::expr("1 + 3.2 + (1 + 2)");
        let mut env = Env::new();
        env.set("+", type_of("number -> number -> number"));

        let typed_expr = infer_types(&mut env, &expr).unwrap();

        assert_eq!(type_of("Float"), expr_type(&typed_expr));
    }

    #[test]
    fn test_type_error() {
        let expr = Test::expr("1 + 3.2 + (true + 2)");
        let mut env = Env::new();
        env.set("+", type_of("number -> number -> number"));
        env.set("true", type_of("Bool"));

        let typed_expr = infer_types(&mut env, &expr);

        assert_eq!(Err(TypeError::ArgumentsDoNotMatch {
            span: (0, 0),
            expected: type_of("Float"),
            found: type_of("Bool"),
        }), typed_expr);
    }

    #[test]
    fn test_infer_type_of_duplicated_vars() {
        let expr = Test::expr("((+), (+))");
        let mut env = Env::new();
        env.set("+", type_of("number -> number -> number"));

        let typed_expr = infer_types(&mut env, &expr).unwrap();

        assert_eq!(type_of("(number -> number -> number, number1 -> number1 -> number1)"), expr_type(&typed_expr));
    }
}


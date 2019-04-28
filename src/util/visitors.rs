use ast::Expr;
use ast::Pattern;
use ast::Type;

pub fn type_visitor<S, F: Fn(&mut S, &Type)>(state: &mut S, root: &Type, f: &F) {
    f(state, root);

    match root {
        Type::Var(_) => {}
        Type::Tag(_, params) => {
            for param in params {
                type_visitor(state, param, f);
            }
        }
        Type::Fun(a, b) => {
            type_visitor(state, a, f);
            type_visitor(state, b, f);
        }
        Type::Unit => {}
        Type::Tuple(params) => {
            for param in params {
                type_visitor(state, param, f);
            }
        }
        Type::Record(entries) => {
            for (_, ty) in entries {
                type_visitor(state, ty, f);
            }
        }
        Type::RecExt(_, entries) => {
            for (_, ty) in entries {
                type_visitor(state, ty, f);
            }
        }
    }
}

pub fn expr_visitor<S, F: Fn(&mut S, &Expr)>(state: &mut S, root: &Expr, f: &F) {
    expr_visitor_block(state, root, f, &|_, _| ());
}

pub fn expr_visitor_block<S, F: Fn(&mut S, &Expr), G: Fn(&mut S, &Expr)>(state: &mut S, root: &Expr, enter: &F, exit: &G) {
    enter(state, root);

    match root {
        Expr::Unit(..) => {}
        Expr::Tuple(_, items) => {
            for item in items {
                expr_visitor_block(state, item, enter, exit);
            }
        }
        Expr::List(_, items) => {
            for item in items {
                expr_visitor_block(state, item, enter, exit);
            }
        }
        Expr::Record(_, entries) => {
            for (_, entry) in entries {
                expr_visitor_block(state, entry, enter, exit);
            }
        }
        Expr::RecordUpdate(_, _, entries) => {
            for (_, entry) in entries {
                expr_visitor_block(state, entry, enter, exit);
            }
        }
        Expr::QualifiedRef(_, _, _) => {}
        Expr::RecordField(_, record, _) => {
            expr_visitor_block(state, record, enter, exit);
        }
        Expr::RecordAccess(_, _) => {}
        Expr::If(_, a, b, c) => {
            expr_visitor_block(state, a, enter, exit);
            expr_visitor_block(state, b, enter, exit);
            expr_visitor_block(state, c, enter, exit);
        }
        Expr::Case(_, a, entries) => {
            expr_visitor_block(state, a, enter, exit);
            for (_, entry) in entries {
                expr_visitor_block(state, entry, enter, exit);
            }
        }
        Expr::Lambda(_, _, a) => {
            expr_visitor_block(state, a, enter, exit);
        }
        Expr::Application(_, a, b) => {
            expr_visitor_block(state, a, enter, exit);
            expr_visitor_block(state, b, enter, exit);
        }
        Expr::Let(_, _, a) => {
            expr_visitor_block(state, a, enter, exit);
        }
        Expr::OpChain(_, exprs, _) => {
            for e in exprs {
                expr_visitor_block(state, e, enter, exit);
            }
        }
        Expr::Literal(_, _) => {}
        Expr::Ref(_, _) => {}
    }

    exit(state, root);
}


pub fn pattern_visitor<S, F: Fn(&mut S, &Pattern)>(state: &mut S, root: &Pattern, f: &F) {
    pattern_visitor_block(state, root, f, &|_, _| ());
}

pub fn pattern_visitor_block<S, F: Fn(&mut S, &Pattern), G: Fn(&mut S, &Pattern)>(state: &mut S, root: &Pattern, enter: &F, exit: &G) {
    enter(state, root);

    match root {
        Pattern::Var(_, _) => {}
        Pattern::Adt(_, _, items) => {
            for item in items {
                pattern_visitor_block(state, item, enter, exit);
            }
        }
        Pattern::Wildcard(_, ) => {}
        Pattern::Unit(_, ) => {}
        Pattern::Tuple(_, items) => {
            for item in items {
                pattern_visitor_block(state, item, enter, exit);
            }
        }
        Pattern::List(_, items) => {
            for item in items {
                pattern_visitor_block(state, item, enter, exit);
            }
        }
        Pattern::BinaryOp(_, _, left, right) => {
            pattern_visitor_block(state, &*left, enter, exit);
            pattern_visitor_block(state, &*right, enter, exit);
        }
        Pattern::Record(_, _) => {}
        Pattern::LitInt(_, _) => {}
        Pattern::LitString(_, _) => {}
        Pattern::LitChar(_, _) => {}
        Pattern::Alias(_, child, _) => {
            pattern_visitor_block(state, &*child, enter, exit);
        }
    }

    exit(state, root);
}
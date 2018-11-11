use ast::Expr;
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
        Expr::Unit => {}
        Expr::Tuple(items) => {
            for item in items {
                expr_visitor_block(state, item, enter, exit);
            }
        }
        Expr::List(items) => {
            for item in items {
                expr_visitor_block(state, item, enter, exit);
            }
        }
        Expr::Record(entries) => {
            for (_, entry) in entries {
                expr_visitor_block(state, entry, enter, exit);
            }
        }
        Expr::RecordUpdate(_, entries) => {
            for (_, entry) in entries {
                expr_visitor_block(state, entry, enter, exit);
            }
        }
        Expr::QualifiedRef(_, _) => {}
        Expr::RecordField(record, _) => {
            expr_visitor_block(state, record, enter, exit);
        }
        Expr::RecordAccess(_) => {}
        Expr::If(a, b, c) => {
            expr_visitor_block(state, a, enter, exit);
            expr_visitor_block(state, b, enter, exit);
            expr_visitor_block(state, c, enter, exit);
        }
        Expr::Case(a, entries) => {
            expr_visitor_block(state, a, enter, exit);
            for (_, entry) in entries {
                expr_visitor_block(state, entry, enter, exit);
            }
        }
        Expr::Lambda(_, a) => {
            expr_visitor_block(state, a, enter, exit);
        }
        Expr::Application(a, b) => {
            expr_visitor_block(state, a, enter, exit);
            expr_visitor_block(state, b, enter, exit);
        }
        Expr::Let(_, a) => {
            expr_visitor_block(state, a, enter, exit);
        }
        Expr::OpChain(exprs, _) => {
            for e in exprs {
                expr_visitor_block(state, e, enter, exit);
            }
        }
        Expr::Literal(_) => {}
        Expr::Ref(_) => {}
    }

    exit(state, root);
}
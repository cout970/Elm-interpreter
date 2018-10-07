use analyzer::expression_fold::create_expr_tree;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use types::Definition;
use types::Expr;
use types::Literal;
use types::Pattern;
use types::Type;
use types::Value;

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Float(i) => write!(f, "{}", i),
            Literal::String(i) => write!(f, "\"{}\"", i),
            Literal::Char(i) => write!(f, "'{}'", i),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Type::Var(i) => {
                write!(f, "{}", i)?;
            }
            Type::Tag(i, rest) => {
                write!(f, "{}", i)?;
                for i in rest {
                    write!(f, " {}", i)?;
                }
            }
            Type::Fun(a, b) => {
                write!(f, "{} -> {}", a, b)?;
            }
            Type::Unit => {
                write!(f, "()")?;
            }
            Type::Tuple(items) => {
                write!(f, "( ")?;
                for (ind, i) in items.iter().enumerate() {
                    write!(f, "{}", i)?;

                    if ind != items.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, " )")?;
            }
            Type::Record(entries) => {
                write!(f, "{{ ")?;
                for (ind, (a, b)) in entries.iter().enumerate() {
                    write!(f, "{} : {}", a, b)?;

                    if ind != entries.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, " }}")?;
            }
            Type::RecExt(name, entries) => {
                write!(f, "{{ {} | ", name)?;
                for (ind, (a, b)) in entries.iter().enumerate() {
                    write!(f, "{} : {}", a, b)?;

                    if ind != entries.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")?;
            }
        }
        Ok(())
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Expr::Unit => write!(f, "()")?,
            Expr::Tuple(items) => {
                write!(f, "(")?;
                print_vec(f, items)?;
                write!(f, ")")?;
            }
            Expr::List(items) => {
                write!(f, "[")?;
                print_vec(f, items)?;
                write!(f, "]")?;
            }
            Expr::Record(items) => {
                write!(f, "{{ ")?;
                print_pairs(f, items)?;
                write!(f, " }}")?;
            }
            Expr::Adt(name) => write!(f, "{}", name)?,
            Expr::RecordUpdate(name, items) => {
                write!(f, "{{ {} | ", name)?;
                print_pairs(f, items)?;
                write!(f, " }}")?;
            }
            Expr::QualifiedRef(path, name) => {
                for p in path {
                    write!(f, "{}.", p)?
                }
                write!(f, "{}", name)?
            }
            Expr::RecordField(expr, name) => write!(f, "{}.{}", expr, name)?,
            Expr::RecordAccess(name) => write!(f, ".{}", name)?,
            Expr::If(cond, t_branch, f_branch) => {
                write!(f, "if {} then {} else {}", cond, t_branch, f_branch)?
            }
            Expr::Case(expr, branches) => {
                write!(f, "case {} of (", expr)?;
                print_pairs(f, branches)?;
                write!(f, ")")?;
            }
            Expr::Lambda(patt, expr) => {
                write!(f, "\\")?;
                print_vec(f, patt)?;
                write!(f, " -> {}", expr)?
            }
            Expr::Application(a, b) => write!(f, "({} {})", a, b)?,
            Expr::Let(defs, expr) => {
                write!(f, "let (")?;
                print_vec(f, defs)?;
                write!(f, ") in ({})", expr)?;
            }
            Expr::OpChain(exprs, ops) => {
                let tree = create_expr_tree(exprs, ops);
                match tree {
                    Ok(t) => {
                        write!(f, "{:?}", t)?;
                    }
                    Err(e) => {
                        write!(f, "Invalid Tree: {:?}", e)?;
                    }
                }
            }
            Expr::Literal(lit) => write!(f, "{}", lit)?,
            Expr::Ref(name) => write!(f, "{}", name)?,
        }
        Ok(())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::Unit => write!(f, "()")?,
            Value::Int(it) => write!(f, "{}", it)?,
            Value::Float(it) => write!(f, "{}", it)?,
            Value::String(it) => write!(f, "\"{}\"", it)?,
            Value::Char(it) => write!(f, "'{}'", it)?,
            Value::List(items) => {
                write!(f, "[")?;
                print_vec(f, items)?;
                write!(f, "]")?;
            },
            Value::Tuple(items) => {
                write!(f, "(")?;
                print_vec(f, items)?;
                write!(f, ")")?;
            },
            Value::Record(items) => {
                write!(f, "{{ ")?;
                print_pairs(f, items)?;
                write!(f, " }}")?;
            },
            Value::Adt(name, items) => {
                write!(f, "{} ", name)?;
                print_vec(f, items)?;
            },
            Value::Fun(_) => write!(f, "<function>")?,
        }
        Ok(())
    }
}


impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for Definition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

fn print_vec<T: Display>(f: &mut Formatter, v: &Vec<T>) -> Result<(), Error> {
    for (i, item) in v.iter().enumerate() {
        write!(f, "{}", item)?;

        if i != v.len() - 1 {
            write!(f, ", ")?;
        }
    }
    Ok(())
}

fn print_pairs<A: Display, B: Display>(f: &mut Formatter, v: &Vec<(A, B)>) -> Result<(), Error> {
    for (i, item) in v.iter().enumerate() {
        write!(f, "{} = {}", item.0, item.1)?;

        if i != v.len() - 1 {
            write!(f, ", ")?;
        }
    }
    Ok(())
}


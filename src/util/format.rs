use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Write;

use ast::*;
use tokenizer::Token;
use typed_ast::TypedExpr;
use typed_ast::TypedPattern;
use types::ExternalFunc;
use types::Value;
use types::WrapperFunc;
use util::expression_fold::create_expr_tree;

impl Debug for ExternalFunc {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "<external fun '{}'>", self.name)
    }
}

impl Debug for WrapperFunc {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "<wrapper fun '{}'>", self.name)
    }
}

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
                if let Type::Fun(..) = a.as_ref() {
                    write!(f, "({}) -> {}", a, b)?;
                } else {
                    write!(f, "{} -> {}", a, b)?;
                }
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

impl Display for TypedExpr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        print_tree(f, self, 0)
    }
}

fn print_tree(f: &mut Formatter, expr: &TypedExpr, indent: u32) -> Result<(), Error> {
    let mut newline = String::new();
    for _ in 0..indent {
        newline.push(' ');
    }

    match expr {
        TypedExpr::Const(_, ty, val) => {
            write!(f, "{nl}Const {{ {}, {} }}", ty, val, nl = newline)?;
        }
        TypedExpr::Ref(_, ty, val) => {
            write!(f, "{nl}Ref {{ {}, {:?} }}", ty, val, nl = newline)?;
        }
        TypedExpr::Tuple(_, ty, val) => {
            write!(f, "{nl}Tuple {{ {},\n{nl}", ty, nl = newline)?;
            for i in val {
                print_tree(f, i, indent + 1)?;
                write!(f, ",\n{nl}", nl = newline)?;
            }
            write!(f, "{nl}}}", nl = newline)?;
        }
        TypedExpr::List(_, ty, val) => {
            write!(f, "{nl}List {{ {},\n{nl}", ty, nl = newline)?;
            for i in val {
                print_tree(f, i, indent + 1)?;
                write!(f, ",\n{nl}", nl = newline)?;
            }
            write!(f, "{nl}}}", nl = newline)?;
        }
        TypedExpr::Record(_, ty, val) => {
            write!(f, "{nl}Record {{ {},\n{nl}", ty, nl = newline)?;
            for (pat, expr) in val {
                write!(f, "{:?} => ", pat)?;
                print_tree(f, expr, indent + 1)?;
                write!(f, ",\n{nl}", nl = newline)?;
            }
            write!(f, "{nl}}}", nl = newline)?;
        }
        TypedExpr::RecordUpdate(_, ty, val1, val2) => {
            write!(f, "{nl}RecordUpdate {{ {},\n{nl}", ty, nl = newline)?;
            print_tree(f, val1, indent + 1)?;
            write!(f, ",\n{nl}", nl = newline)?;
            for (pat, expr) in val2 {
                write!(f, "{:?} => ", pat)?;
                print_tree(f, expr, indent + 1)?;
                write!(f, ",\n{nl}", nl = newline)?;
            }
            write!(f, "{nl}}}", nl = newline)?;
        }
        TypedExpr::RecordField(_, ty, val1, val2) => {
            write!(f, "{nl}RecordField {{ {},\n{nl}", ty, nl = newline)?;
            print_tree(f, val1.as_ref(), indent + 1)?;
            write!(f, ",\n{nl}{}\n{nl}}}", val2, nl = newline)?;
        }
        TypedExpr::RecordAccess(_, ty, val) => {
            write!(f, "{nl}RecordAccess {{ {},\n{nl}{}\n{nl}}}", ty, val, nl = newline)?;
        }
        TypedExpr::If(_, ty, val1, val2, val3) => {
            write!(f, "{nl}If {{ {},\n{nl}", ty, nl = newline)?;
            print_tree(f, val1.as_ref(), indent + 1)?;
            write!(f, ",\n{nl}", nl = newline)?;
            print_tree(f, val2.as_ref(), indent + 1)?;
            write!(f, ",\n{nl}", nl = newline)?;
            print_tree(f, val3.as_ref(), indent + 1)?;
            write!(f, "\n{nl}}}", nl = newline)?;
        }
        TypedExpr::Case(_, ty, val1, val2) => {
            write!(f, "{nl}Case {{ {},\n{nl}", ty, nl = newline)?;
            print_tree(f, val1.as_ref(), indent + 1)?;
            write!(f, ",\n{nl}", nl = newline)?;
            for (pat, expr) in val2 {
                write!(f, "{:?} => ", pat)?;
                print_tree(f, expr, indent + 1)?;
                write!(f, ",\n{nl}", nl = newline)?;
            }
            write!(f, "{nl}}}", nl = newline)?;
        }
        TypedExpr::Lambda(_, ty, val1, val2) => {
            write!(f, "{nl}Lambda {{ {},\n {:?}\n", ty, val1, nl = newline)?;
            print_tree(f, val2.as_ref(), indent + 1)?;
            write!(f, "\n{nl}}}", nl = newline)?;
        }
        TypedExpr::Application(_, ty, val1, val2) => {
            write!(f, "{nl}Application {{ {},\n{nl}", ty, nl = newline)?;
            print_tree(f, val1.as_ref(), indent + 1)?;
            write!(f, ",\n{nl}", nl = newline)?;
            print_tree(f, val2.as_ref(), indent + 1)?;
            write!(f, "\n{nl}}}", nl = newline)?;
        }
        TypedExpr::Let(_, ty, val1, val2) => {
            write!(f, "{nl}Let {{ {},\n{nl}{:?}\n{nl}", ty, val1, nl = newline)?;
            print_tree(f, val2.as_ref(), indent + 1)?;
            write!(f, "\n{nl}}}", nl = newline)?;
        }
    }

    Ok(())
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Expr::Unit(..) => write!(f, "()")?,
            Expr::Tuple(_, items) => {
                write!(f, "(")?;
                print_vec(f, items)?;
                write!(f, ")")?;
            }
            Expr::List(_, items) => {
                write!(f, "[")?;
                print_vec(f, items)?;
                write!(f, "]")?;
            }
            Expr::Record(_, items) => {
                write!(f, "{{ ")?;
                print_pairs(f, items)?;
                write!(f, " }}")?;
            }
            Expr::RecordUpdate(_, name, items) => {
                write!(f, "{{ {} | ", name)?;
                print_pairs(f, items)?;
                write!(f, " }}")?;
            }
            Expr::QualifiedRef(_, path, name) => {
                for p in path {
                    write!(f, "{}.", p)?
                }
                write!(f, "{}", name)?
            }
            Expr::RecordField(_, expr, name) => write!(f, "{}.{}", expr, name)?,
            Expr::RecordAccess(_, name) => write!(f, ".{}", name)?,
            Expr::If(_, cond, t_branch, f_branch) => {
                write!(f, "if {} then {} else {}", cond, t_branch, f_branch)?
            }
            Expr::Case(_, expr, branches) => {
                write!(f, "case {} of (", expr)?;
                print_pairs(f, branches)?;
                write!(f, ")")?;
            }
            Expr::Lambda(_, patt, expr) => {
                write!(f, "\\")?;
                print_vec(f, patt)?;
                write!(f, " -> {}", expr)?
            }
            Expr::Application(_, a, b) => write!(f, "({} {})", a, b)?,
            Expr::Let(_, decls, expr) => {
                write!(f, "let (")?;
                print_vec(f, decls)?;
                write!(f, ") in ({})", expr)?;
            }
            Expr::OpChain(_, exprs, ops) => {
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
            Expr::Literal(_, lit) => write!(f, "{}", lit)?,
            Expr::Ref(_, name) => write!(f, "{}", name)?,
        }
        Ok(())
    }
}

impl Display for LetDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            LetDeclaration::Def(def) => write!(f, "{}", def),
            LetDeclaration::Pattern(pattern, expr) => write!(f, "{} = {}", pattern, expr),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::Unit => write!(f, "()")?,
            Value::Number(it) => write!(f, "{}", it)?,
            Value::Int(it) => write!(f, "{}", it)?,
            Value::Float(it) => write!(f, "{}", it)?,
            Value::String(it) => write!(f, "\"{}\"", it)?,
            Value::Char(it) => write!(f, "'{}'", it)?,
            Value::List(items) => {
                write!(f, "[")?;
                print_vec(f, items)?;
                write!(f, "]")?;
            }
            Value::Tuple(items) => {
                write!(f, "(")?;
                print_vec(f, items)?;
                write!(f, ")")?;
            }
            Value::Record(items) => {
                write!(f, "{{ ")?;
                print_pairs(f, items)?;
                write!(f, " }}")?;
            }
            Value::Adt(name, items, _) => {
                write!(f, "{}", name)?;
                if !items.is_empty() {
                    write!(f, " ")?;
                }
                print_vec(f, items)?;
            }
            Value::Fun { .. } => write!(f, "<function>")?,
        }
        Ok(())
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for TypedPattern {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        print_pattern(f, self, 0)
    }
}

fn print_pattern(f: &mut Formatter, pat: &TypedPattern, indent: u32) -> Result<(), Error> {
    let mut newline = String::new();
    for _ in 0..indent {
        newline.push(' ');
    }

    match pat {
        TypedPattern::Var(_, ty, name) => {
            write!(f, "{nl}Var {{ {}, {} }}", ty, name, nl = newline)?;
        }
        TypedPattern::Adt(_, ty, own_ty, items) => {
            write!(f, "{nl}Adt {{ {},\n{nl} {},\n{nl}", ty, own_ty, nl = newline)?;
            for item in items {
                print_pattern(f, item, indent + 1)?;
                write!(f, ",\n{nl}", nl = newline)?;
            }
            write!(f, "}}")?;
        }
        TypedPattern::Wildcard(_) => {
            write!(f, "{nl}Wildcard", nl = newline)?;
        }
        TypedPattern::Unit(_) => {
            write!(f, "{nl}Unit", nl = newline)?;
        }
        TypedPattern::Tuple(_, ty, items) => {
            write!(f, "{nl}Tuple {{ {},\n{nl}", ty, nl = newline)?;
            for item in items {
                print_pattern(f, item, indent + 1)?;
                write!(f, ",\n{nl}", nl = newline)?;
            }
            write!(f, "}}")?;
        }
        TypedPattern::List(_, ty, items) => {
            write!(f, "{nl}List {{ {},\n{nl}", ty, nl = newline)?;
            for item in items {
                print_pattern(f, item, indent + 1)?;
                write!(f, ",\n{nl}", nl = newline)?;
            }
            write!(f, "}}")?;
        }
        TypedPattern::BinaryOp(_, ty, op, a, b) => {
            write!(f, "{nl}BinaryOp {{ {},\n{nl} {}\n{nl}", ty, op, nl = newline)?;

            print_pattern(f, a, indent + 1)?;
            write!(f, ",\n{nl}", nl = newline)?;
            print_pattern(f, b, indent + 1)?;
            write!(f, ",\n{nl}", nl = newline)?;

            write!(f, "}}")?;
        }
        TypedPattern::Record(_, ty, items) => {
            write!(f, "{nl}Record {{ {},\n{nl}", ty, nl = newline)?;
            for item in items {
                write!(f, "{},\n{nl}", item, nl = newline)?;
            }
            write!(f, "}}")?;
        }
        TypedPattern::LitInt(_, value) => {
            write!(f, "{nl}Int {{ {} }}", value, nl = newline)?;
        }
        TypedPattern::LitString(_, value) => {
            write!(f, "{nl}String {{ {} }}", value, nl = newline)?;
        }
        TypedPattern::LitChar(_, value) => {
            write!(f, "{nl}Char {{ {} }}", value, nl = newline)?;
        }
        TypedPattern::Alias(_, ty, pat, alias) => {
            write!(f, "{nl}Alias {{ {},\n{nl} {}\n{nl}", ty, alias, nl = newline)?;
            print_pattern(f, pat.as_ref(), indent + 1)?;
            write!(f, ",\n{nl}", nl = newline)?;
            write!(f, "}}")?;
        }
    }

    Ok(())
}

impl Display for Definition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Token::Id(value) => { write!(f, "{}", value) }
            Token::UpperId(value) => { write!(f, "{}", value) }
            Token::BinaryOperator(value) => { write!(f, "{}", value) }
            Token::LitInt(value) => { write!(f, "{}", value) }
            Token::LitFloat(value) => { write!(f, "{}", value) }
            Token::LitChar(value) => { write!(f, "{}", value) }
            Token::LitString(value) => { write!(f, "{}", value) }
            Token::Indent(value) => {
                if *value == 0 {
                    write!(f, "<NewLine>")?;
                } else {
                    write!(f, "<Indentation {} >", *value)?;
                }
                Ok(())
            }
            Token::BackSlash => { write!(f, "\\") }
            Token::PrefixMinus => { write!(f, "-") }
            Token::Let => { write!(f, "let") }
            Token::If => { write!(f, "if") }
            Token::Else => { write!(f, "else") }
            Token::Then => { write!(f, "then") }
            Token::Case => { write!(f, "case") }
            Token::Of => { write!(f, "of") }
            Token::In => { write!(f, "in") }
            Token::ModuleTk => { write!(f, "module") }
            Token::WhereTk => { write!(f, "where") }
            Token::EffectTk => { write!(f, "effect") }
            Token::ExposingTk => { write!(f, "exposing") }
            Token::ImportTk => { write!(f, "import") }
            Token::As => { write!(f, "as") }
            Token::TypeTk => { write!(f, "type") }
            Token::Port => { write!(f, "port") }
            Token::Alias => { write!(f, "alias") }
            Token::InfixTk => { write!(f, "infix") }
            Token::Underscore => { write!(f, "_") }
            Token::Dot => { write!(f, ".") }
            Token::DoubleDot => { write!(f, "..") }
            Token::Comma => { write!(f, ",") }
            Token::LeftParen => { write!(f, "(") }
            Token::RightParen => { write!(f, ")") }
            Token::LeftBracket => { write!(f, "[") }
            Token::RightBracket => { write!(f, "]") }
            Token::LeftBrace => { write!(f, "{{") }
            Token::RightBrace => { write!(f, "}}") }
            Token::Equals => { write!(f, "=") }
            Token::Pipe => { write!(f, "|") }
            Token::RightArrow => { write!(f, "->") }
            Token::LeftArrow => { write!(f, "<-") }
            Token::Colon => { write!(f, ":") }
            Token::Eof => { write!(f, "<Eof>") }
        }
    }
}

pub fn print_vec<T: Display, F: Write>(f: &mut F, v: &[T]) -> Result<(), Error> {
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

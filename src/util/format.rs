use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Write;

use ast::*;
use tokenizer::Token;
use types::BuiltinFunction;
use types::Value;
use util::expression_fold::create_expr_tree;

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
            Expr::Let(decls, expr) => {
                write!(f, "let (")?;
                print_vec(f, decls)?;
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

impl Debug for Box<BuiltinFunction> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "BuiltinFunction")
    }
}
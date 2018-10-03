use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use types::Literal;
use types::Type;

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Float(i) => write!(f, "{}", i),
            Literal::String(i) => write!(f, "{}", i),
            Literal::Char(i) => write!(f, "{}", i),
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
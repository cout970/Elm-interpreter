use ast::*;
use errors::ErrorWrapper;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

pub mod new;
pub mod old;

#[derive(PartialEq, Debug, Clone)]
pub enum SyntaxError {
    Old(old::ParseError),
    New(new::ParseError),
}

pub fn parse_expression(code: &str) -> Result<Expr, ErrorWrapper> {
    old::parse_expression(code)
}

pub fn parse_statement(code: &str) -> Result<Statement, ErrorWrapper> {
    old::parse_statement(code)
}

pub fn parse_module(code: &str) -> Result<Module, ErrorWrapper> {
    new::parse_module(code)
}

#[cfg(test)]
pub fn from_code(code: &[u8]) -> Expr {
//    old::from_code(code)
    let res = new::parse_expression(&String::from_utf8_lossy(code));

    match res {
        Ok(res) => res,
        Err(error) => {
            println!("Error: {}\n", error);
            panic!();
        }
    }
}

#[cfg(test)]
pub fn from_code_stm(code: &[u8]) -> Statement {
//    old::from_code_stm(code)
    let res = new::parse_statement(&String::from_utf8_lossy(code));

    match res {
        Ok(res) => res,
        Err(error) => {
            println!("Error: {}\n", error);
            panic!();
        }
    }
}

#[cfg(test)]
pub fn from_code_mod(code: &[u8]) -> Module {
//    old::from_code_mod(code)
    let res = new::parse_module(&String::from_utf8_lossy(code));

    match res {
        Ok(res) => res,
        Err(error) => {
            println!("Error: {}\n", error);
            panic!();
        }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            SyntaxError::Old(e) => {
                write!(f, "{:?}", e)
            },
            SyntaxError::New(e) => {
                write!(f, "{}", e)
            },
        }
    }
}
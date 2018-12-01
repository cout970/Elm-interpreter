use ast::*;
use errors::ErrorWrapper;

mod parser;
mod util;
mod pattern;
mod expression;
mod types;
mod statement;
mod module;


/// Enum with all possible parsing errors
pub type SyntaxError = parser::ParseError;

/// Generates an abstract syntax tree from an elm expression
pub fn parse_expression(code: &str) -> Result<Expr, ErrorWrapper> {
    parser::parse_expression(code)
}

/// Generates an abstract syntax tree from an elm statement
pub fn parse_statement(code: &str) -> Result<Statement, ErrorWrapper> {
    parser::parse_statement(code)
}

/// Generates an abstract syntax tree from an elm module
pub fn parse_module(code: &str) -> Result<Module, ErrorWrapper> {
    parser::parse_module(code)
}

/// Generates an abstract syntax tree from an elm type definition
pub fn parse_type(code: &str) -> Result<Type, ErrorWrapper> {
    parser::parse_type(code)
}

/// Generates an abstract syntax tree from an elm pattern
pub fn parse_pattern(code: &str) -> Result<Type, ErrorWrapper> {
    parser::parse_type(code)
}

// Utility functions for testing

#[cfg(test)]
pub fn from_code(code: &[u8]) -> Expr {
    let res = parser::parse_expression(&String::from_utf8_lossy(code));

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
    let res = parser::parse_statement(&String::from_utf8_lossy(code));

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
    let res = parser::parse_module(&String::from_utf8_lossy(code));

    match res {
        Ok(res) => res,
        Err(error) => {
            println!("Error: {}\n", error);
            panic!();
        }
    }
}
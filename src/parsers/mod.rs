use parsers::expression::read_expr;
use parsers::statement::read_statement;
use parsers::module::read_module;
use tokenizer::Token;
use tokenizer::tokenize;
use ast::Expr;
use ast::Statement;
use ast::Module;
use parsers::SyntaxError::Unknown;

type Tk<'a> = &'a [Token];

#[macro_use]
mod macros;

mod module;
mod types;
mod statement;
mod expression;
mod pattern;

// TODO
#[derive(PartialEq, Debug, Clone)]
pub enum SyntaxError {
    Unknown
}

pub fn parse_expr(i: &[Token]) -> Result<Expr, SyntaxError> {
    match read_expr(i) {
        Ok((_, e)) => Ok(e),
        Err(_) => Err(Unknown)
    }
}

pub fn parse_statement(i: &[Token]) -> Result<Statement, SyntaxError> {
    match read_statement(i) {
        Ok((_, e)) => Ok(e),
        Err(_) => Err(Unknown)
    }
}

pub fn parse_module(i: &[Token]) -> Result<Module, SyntaxError> {
    match read_module(i) {
        Ok((_, e)) => Ok(e),
        Err(_) => Err(Unknown)
    }
}

pub fn from_code(code: &[u8]) -> Expr {
    use nom::*;

    let stream = tokenize(code).unwrap();
    let expr: IResult<Tk, Expr> = read_expr(&stream);

    match expr {
        Ok((_, e)) => e,
        Err(e) => {
            match e {
                Err::Incomplete(need) => panic!("Tokens needed: {:?}", need),
                Err::Failure(ctx) => panic!("Parsing failure: {:#?}", ctx),
                Err::Error(ctx) => panic!("Syntax error: {:#?}", ctx),
            };
        }
    }
}

pub fn from_code_stm(code: &[u8]) -> Statement {
    use nom::*;

    let stream = tokenize(code).unwrap();
    let stm: IResult<Tk, Statement> = read_statement(&stream);

    match stm {
        Ok((_, e)) => e,
        Err(e) => {
            match e {
                Err::Incomplete(need) => panic!("Tokens needed: {:?}", need),
                Err::Failure(ctx) => panic!("Parsing failure: {:#?}", ctx),
                Err::Error(ctx) => panic!("Syntax error: {:#?}", ctx),
            };
        }
    }
}

pub fn from_code_mod(code: &[u8]) -> Module {
    use nom::*;

    let stream = tokenize(code).unwrap();
    let stm: IResult<Tk, Module> = read_module(&stream);

    match stm {
        Ok((rest, e)) => {
            if rest.len() > 1 {
                panic!("Unreaded part of the input: {:#?}", rest);
            }
            e
        },
        Err(e) => {
            match e {
                Err::Incomplete(need) => panic!("Tokens needed: {:?}", need),
                Err::Failure(ctx) => panic!("Parsing failure: {:#?}", ctx),
                Err::Error(ctx) => panic!("Syntax error: {:#?}", ctx),
            };
        }
    }
}
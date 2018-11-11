use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

use nom::AtEof;
use nom::ErrorKind;
use nom::InputLength;
use nom::Needed;

use ast::Expr;
use ast::Module;
use ast::Statement;
use errors::ErrorWrapper;
use parsers::old::expression::read_expr;
use parsers::old::module::read_module;
use parsers::old::statement::read_statement;
use tokenizer::Token;
use tokenizer::TokenInfo;
use tokenizer::tokenize;
use parsers::SyntaxError;

type Tk<'a> = TokenStream<'a>;

#[macro_use]
mod macros;

mod module;
mod types;
mod statement;
mod expression;
mod pattern;

#[derive(PartialEq, Debug, Clone)]
pub enum ParseError {
    Unknown,
    IncompleteInput(Needed),
    UnableToConsumeAllInput(TokenInfo),
    ExpectedToken(Token, TokenInfo),
    InvalidIndentation(Vec<usize>, usize),
    ExpectedId(TokenInfo),
    ExpectedUpperId(TokenInfo),
    ExpectedBinaryOperator(TokenInfo),
    ExpectedLiteral(TokenInfo),
    Errors(Vec<ParseError>),
}

pub fn parse_expression(code: &str) -> Result<Expr, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    match read_expr(TokenStream::new(&tk)) {
        Ok((_, e)) => Ok(e),
        Err(_) => Err(ErrorWrapper::Syntactic(SyntaxError::Old(ParseError::Unknown))),
    }
}

pub fn parse_statement(code: &str) -> Result<Statement, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    match read_statement(TokenStream::new(&tk)) {
        Ok((_, e)) => Ok(e),
        Err(_) => Err(ErrorWrapper::Syntactic(SyntaxError::Old(ParseError::Unknown))),
    }
}

pub fn parse_module(code: &str) -> Result<Module, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    match read_module(TokenStream::new(&tk)) {
        Ok((_, e)) => Ok(e),
        Err(_) => Err(ErrorWrapper::Syntactic(SyntaxError::Old(ParseError::Unknown))),
    }
}

pub fn from_code(code: &[u8]) -> Expr {
    use nom::*;

    let tokens = tokenize(code).unwrap();
    let expr: IResult<Tk, Expr, ParseError> = read_expr(TokenStream::new(&tokens));

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

    let tokens = tokenize(code).unwrap();
    let stm: IResult<Tk, Statement, ParseError> = read_statement(TokenStream::new(&tokens));

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

    let tokens = tokenize(code).unwrap();
    let stm: IResult<Tk, Module, ParseError> = read_module(TokenStream::new(&tokens));

    match stm {
        Ok((_, e)) => e,
        Err(e) => {
            match e {
                Err::Incomplete(need) => panic!("Tokens needed: {:?}", need),
                Err::Failure(ctx) => panic!("Parsing failure: {:?}", ctx),
                Err::Error(ctx) => {
                    match ctx {
                        Context::Code(input, kind) => {
                            panic!("Syntax error\n{}", format_error(input, kind))
                        }
                        Context::List(all) => {
                            let lines: Vec<String> = all.iter().map(|(input, kind)| format_error(input.clone(), kind.clone())).collect();
                            panic!("Syntax errors:\n{:?}", lines);
                        }
                    }
                }
            };
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct TokenStream<'a> {
    pub all: &'a [TokenInfo],
    pub remaining: &'a [TokenInfo],
}

impl<'a> TokenStream<'a> {
    pub fn new(src: &'a Vec<TokenInfo>) -> TokenStream<'a> {
        TokenStream {
            all: src,
            remaining: src,
        }
    }

    pub fn read_tk(&self) -> Token {
        self.remaining[0].token.clone()
    }

    pub fn read_info(&self) -> TokenInfo {
        self.remaining[0].clone()
    }

    pub fn next(&self, amount: u32) -> TokenStream<'a> {
        TokenStream {
            all: self.all,
            remaining: &self.remaining[(amount as usize)..],
        }
    }

    pub fn len(&self) -> usize {
        self.remaining.len()
    }
}

impl<'a> InputLength for TokenStream<'a> {
    fn input_len(&self) -> usize {
        self.len()
    }
}

impl<'a> AtEof for TokenStream<'a> {
    fn at_eof(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> Display for TokenStream<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for info in self.remaining.iter().take(10) {
            write!(f, "{} ", info.token)?;
        }
        Ok(())
    }
}

impl<'a> Debug for TokenStream<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for info in self.remaining.iter().take(10) {
            write!(f, "{} ", info.token)?;
        }
        Ok(())
    }
}

fn format_error(input: TokenStream, kind: ErrorKind<ParseError>) -> String {
    if let ErrorKind::Custom(info) = kind {
        format!("{:?}", info)
    } else {
        let start = &input.remaining[0].start;
        let pos = input.all.len() - input.remaining.len();
        format!("\n{}:{} Unexpected token: {:?} in:\n{} {} {}\n", start.line + 1, start.column,
                input.read_tk(), input.all[pos - 1].token, input.all[pos].token, input.all[pos + 1].token)
    }
}
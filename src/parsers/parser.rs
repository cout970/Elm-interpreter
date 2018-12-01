use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

use ast::{Expr, Module, Pattern, Statement, Type};
use errors::ErrorWrapper;
use parsers::*;
use parsers::input::Input;
use parsers::util::complete;
use tokenizer::Token;
use tokenizer::tokenize;

#[derive(PartialEq, Debug, Clone)]
pub enum ParseError {
    //@formatter:off
    Expected                    { input: Input, expected: Token, found: Token },
    ExpectedInt                 { input: Input, found: Token },
    ExpectedId                  { input: Input, found: Token },
    ExpectedUpperId             { input: Input, found: Token },
    ExpectedBinaryOperator      { input: Input, found: Token },
    UnmatchedToken              { input: Input, found: Token, options: Vec<Token> },
    ExpectedIndentationLevel    { input: Input, expected: u32, found: u32 },
    ExpectedIndentation         { input: Input, found: Token },
    //@formatter:on
}

pub fn parse_expression(code: &str) -> Result<Expr, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&expression::parse_expr, input)
        .map_err(|e| ErrorWrapper::Syntactic(e))
}

pub fn parse_statement(code: &str) -> Result<Statement, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&statement::parse_statement, input)
        .map_err(|e| ErrorWrapper::Syntactic(e))
}

pub fn parse_module(code: &str) -> Result<Module, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&module::parse_module, input)
        .map_err(|e| ErrorWrapper::Syntactic(e))
}

pub fn parse_type(code: &str) -> Result<Type, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&types::parse_type, input)
        .map_err(|e| ErrorWrapper::Syntactic(e))
}

pub fn parse_pattern(code: &str) -> Result<Pattern, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&pattern::parse_pattern, input)
        .map_err(|e| ErrorWrapper::Syntactic(e))
}


// TODO move to error module
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            ParseError::Expected { input, expected, found } => {
                write!(f, "Expected token '{}', but found '{}': {}\n", expected, found, input)
            }
            ParseError::ExpectedInt { input, found } => {
                write!(f, "Expected integer, but found '{}': {}\n", found, input)
            }
            ParseError::ExpectedId { input, found } => {
                write!(f, "Expected identifier, but found '{}': {}\n", found, input)
            }
            ParseError::ExpectedUpperId { input, found } => {
                write!(f, "Expected capitalized identifier, but found '{}': {}\n", found, input)
            }
            ParseError::ExpectedBinaryOperator { input, found } => {
                write!(f, "Expected binary operator, but found '{}': {}\n", found, input)
            }
            ParseError::UnmatchedToken { input, found, .. } => {
                write!(f, "Found unexpected token '{}': {}\n", found, input)
            }
            ParseError::ExpectedIndentation { input, found } => {
                write!(f, "Expected indentation, but found '{}': {}\n", found, input)
            }
            ParseError::ExpectedIndentationLevel { input, expected, found } => {
                write!(f, "Expected indentation of {}, but found {}: {}\n", expected, found, input)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use parsers::from_code_mod;
    use tokenizer::tokenize;

    #[test]
    fn test_bench_1() {
        from_code_mod(include_bytes!("../../benches/data/tokenizer_1.elm"));
    }

    #[test]
    fn test_bench_2() {
        from_code_mod(include_bytes!("../../benches/data/tokenizer_2.elm"));
    }

    #[test]
    fn test_edge_case() {
        let code = r#"sliceTree shift endIdx tree =
            let
                lastPos =
                    Bitwise.and bitMask <| Bitwise.shiftRightZfBy shift endIdx
            in
                case JsArray.unsafeGet lastPos tree of
                    SubTree sub ->
                        let
                            newSub =
                                sliceTree (shift - shiftStep) endIdx sub
                        in
                            if JsArray.length newSub == 0 then
                                -- The sub is empty, slice it away
                                JsArray.slice 0 lastPos tree
                            else
                                tree
                                    |> JsArray.slice 0 (lastPos + 1)
                                    |> JsArray.unsafeSet lastPos (SubTree newSub)

                     -- This is supposed to be the new tail. Fetched by `fetchNewTail`.
                     -- Slice up to, but not including, this point.
                    Leaf _ ->
                        JsArray.slice 0 lastPos tree"#;

        let tk = tokenize(code.as_bytes()).unwrap();

        for info in tk.iter() {
            println!("|> {}", info.token);
        }
        from_code_mod(code.as_bytes());
    }
}

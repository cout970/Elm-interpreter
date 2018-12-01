use ast::{Expr, Module, Pattern, Statement, Type};
use errors::ErrorWrapper;
use parsers::input::Input;
use parsers::util::complete;
use tokenizer::Token;
use tokenizer::tokenize;

mod input;
mod util;
mod pattern;
mod expression;
mod types;
mod statement;
mod module;

/// Enum with all possible parsing errors
pub type SyntaxError = ParseError;

/// Enum with all possible parsing errors
#[derive(PartialEq, Debug, Clone)]
pub enum ParseError {
    //@formatter:off
    Expected                    { input: Input, expected: Token, found: Token },
    ExpectedInt                 { input: Input, found: Token },
    ExpectedId                  { input: Input, found: Token },
    ExpectedUpperId             { input: Input, found: Token },
    ExpectedBinaryOperator      { input: Input, found: Token },
    ExpectedIndentationLevel    { input: Input, expected: u32, found: u32 },
    ExpectedIndentation         { input: Input, found: Token },
    UnmatchedToken              { input: Input, found: Token, options: Vec<Token> },
    //@formatter:on
}

/// Generates an abstract syntax tree from an elm expression
pub fn parse_expression(code: &str) -> Result<Expr, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&expression::parse_expr, input)
        .map_err(|e| ErrorWrapper::Syntactic(e))
}

/// Generates an abstract syntax tree from an elm statement
pub fn parse_statement(code: &str) -> Result<Statement, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&statement::parse_statement, input)
        .map_err(|e| ErrorWrapper::Syntactic(e))
}

/// Generates an abstract syntax tree from an elm module
pub fn parse_module(code: &str) -> Result<Module, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&module::parse_module, input)
        .map_err(|e| ErrorWrapper::Syntactic(e))
}

/// Generates an abstract syntax tree from an elm type definition
pub fn parse_type(code: &str) -> Result<Type, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&types::parse_type, input)
        .map_err(|e| ErrorWrapper::Syntactic(e))
}

/// Generates an abstract syntax tree from an elm pattern
pub fn parse_pattern(code: &str) -> Result<Pattern, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&pattern::parse_pattern, input)
        .map_err(|e| ErrorWrapper::Syntactic(e))
}

// Utility functions for testing

#[cfg(test)]
pub fn from_code(code: &[u8]) -> Expr {
    let res = parse_expression(&String::from_utf8_lossy(code));

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
    let res = parse_statement(&String::from_utf8_lossy(code));

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
    let res = parse_module(&String::from_utf8_lossy(code));

    match res {
        Ok(res) => res,
        Err(error) => {
            println!("Error: {}\n", error);
            panic!();
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

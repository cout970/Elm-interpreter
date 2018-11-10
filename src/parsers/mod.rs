use ast::Expr;
use ast::Module;
use ast::Statement;
use nom::ErrorKind;
use parsers::expression::read_expr;
use parsers::module::read_module;
use parsers::statement::read_statement;
use parsers::SyntaxError::Unknown;
use tokenizer::Token;
use tokenizer::TokenInfo;
use tokenizer::tokenize;
use tokenizer::TokenStream;
use nom::Needed;

type Tk<'a> = TokenStream<'a>;

#[macro_use]
mod macros;

mod module;
mod types;
mod statement;
mod expression;
mod pattern;
pub mod new;

#[derive(PartialEq, Debug, Clone)]
pub enum SyntaxError {
    Unknown,
    IncompleteInput(Needed),
    UnableToConsumeAllInput(TokenInfo),
    ExpectedToken(Token, TokenInfo),
    InvalidIndentation(Vec<usize>, usize),
    ExpectedId(TokenInfo),
    ExpectedUpperId(TokenInfo),
    ExpectedBinaryOperator(TokenInfo),
    ExpectedLiteral(TokenInfo),
    Errors(Vec<SyntaxError>),
}

pub fn parse_expr(i: TokenStream) -> Result<Expr, SyntaxError> {
    match read_expr(i) {
        Ok((_, e)) => Ok(e),
        Err(_) => Err(Unknown)
    }
}

pub fn parse_statement(i: TokenStream) -> Result<Statement, SyntaxError> {
    match read_statement(i) {
        Ok((_, e)) => Ok(e),
        Err(_) => Err(Unknown)
    }
}

pub fn parse_module(i: TokenStream) -> Result<Module, SyntaxError> {
    match read_module(i) {
        Ok((_, e)) => Ok(e),
        Err(_) => Err(Unknown)
    }
}

pub fn from_code(code: &[u8]) -> Expr {
    use nom::*;

    let tokens = tokenize(code).unwrap();
    let expr: IResult<Tk, Expr, SyntaxError> = read_expr(TokenStream::new(&tokens));

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
    let stm: IResult<Tk, Statement, SyntaxError> = read_statement(TokenStream::new(&tokens));

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
    let stm: IResult<Tk, Module, SyntaxError> = read_module(TokenStream::new(&tokens));

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

fn format_error(input: TokenStream, kind: ErrorKind<SyntaxError>) -> String {
    if let ErrorKind::Custom(info) = kind {
        format!("{:?}", info)
    } else {
        let start = &input.remaining[0].start;
        let pos = input.all.len() - input.remaining.len();
        format!("\n{}:{} Unexpected token: {:?} in:\n{} {} {}\n", start.line + 1, start.column,
                input.read_tk(), input.all[pos - 1].token, input.all[pos].token, input.all[pos + 1].token)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn check_tuple() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Tuple.elm"));
    }

    #[test]
    #[ignore]
    fn check_task() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Task.elm"));
    }

    #[test]
    #[ignore]
    fn check_string() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/String.elm"));
    }

    #[test]
    #[ignore]
    fn check_set() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Set.elm"));
    }

    #[test]
    #[ignore]
    fn check_result() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Result.elm"));
    }

    #[test]
    #[ignore]
    fn check_process() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Process.elm"));
    }

    #[test]
    #[ignore]
    fn check_platform() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Platform.elm"));
    }

    #[test]
    #[ignore]
    fn check_maybe() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Maybe.elm"));
    }

    #[test]
    #[ignore]
    fn check_list() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/List.elm"));
    }

    #[test]
    #[ignore]
    fn check_dict() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Dict.elm"));
    }

    #[test]
    #[ignore]
    fn check_debug() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Debug.elm"));
    }

    #[test]
    #[ignore]
    fn check_char() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Char.elm"));
    }

    #[test]
    #[ignore]
    fn check_bitwise() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Bitwise.elm"));
    }

    #[test]
    #[ignore]
    fn check_basics() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Basics.elm"));
    }

    #[test]
    #[ignore]
    fn check_array() {
        from_code_mod(include_bytes!("/Data/Dev/Elm/core-master/src/Array.elm"));
    }
}
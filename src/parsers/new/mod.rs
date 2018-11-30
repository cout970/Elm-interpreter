use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::Rc;

use ast::Expr;
use ast::Module;
use ast::Statement;
use errors::ErrorWrapper;
use parsers::new::util::complete;
use parsers::SyntaxError;
use tokenizer::Token;
use tokenizer::TokenInfo;
use tokenizer::tokenize;
use ast::Type;

pub mod util;
mod pattern;
mod expression;
mod types;
mod statement;
pub mod module;

#[derive(PartialEq, Debug, Clone)]
pub enum ParseError {
    Expected { input: Input, expected: Token, found: Token },
    ExpectedInt { input: Input, found: Token },
    ExpectedId { input: Input, found: Token },
    ExpectedUpperId { input: Input, found: Token },
    ExpectedBinaryOperator { input: Input, found: Token },
    UnmatchedToken { input: Input, found: Token, options: Vec<Token> },
    ExpectedIndentationLevel { input: Input, expected: u32, found: u32 },
    ExpectedIndentation { input: Input, found: Token },
}

pub fn parse_expression(code: &str) -> Result<Expr, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&expression::parse_expr, input)
        .map_err(|e| ErrorWrapper::Syntactic(SyntaxError::New(e)))
}

pub fn parse_statement(code: &str) -> Result<Statement, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&statement::parse_statement, input)
        .map_err(|e| ErrorWrapper::Syntactic(SyntaxError::New(e)))
}

pub fn parse_module(code: &str) -> Result<Module, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&module::parse_module, input)
        .map_err(|e| ErrorWrapper::Syntactic(SyntaxError::New(e)))
}

pub fn parse_type(code: &str) -> Result<Type, ErrorWrapper> {
    let tk = tokenize(code.as_bytes())
        .map_err(|e| ErrorWrapper::Lexical(e))?;

    let input = Input::new(code.to_owned(), tk);

    complete(&types::parse_type, input)
        .map_err(|e| ErrorWrapper::Syntactic(SyntaxError::New(e)))
}

#[derive(PartialEq, Debug, Clone)]
pub struct Input {
    raw: Rc<RawInput>,
    ptr: usize,
    levels: Rc<Vec<u32>>,
}

#[derive(PartialEq, Debug, Clone)]
struct RawInput {
    string: String,
    tokens: Vec<TokenInfo>,
}

impl Input {
    pub fn new(code_str: String, code: Vec<TokenInfo>) -> Self {
        Input {
            raw: Rc::new(RawInput { string: code_str, tokens: code }),
            ptr: 0,
            levels: Rc::new(vec![0]),
        }
    }

    pub fn next(&self) -> Input {
        let ptr = self.skip_indent();

        Input {
            raw: Rc::clone(&self.raw),
            ptr: ptr + 1,
            levels: self.levels.clone(),
        }
    }

    pub fn enter_level(&self, level: u32) -> Input {
        let mut copy = (&(*self.levels)).clone();
        copy.push(level);
        Input {
            raw: Rc::clone(&self.raw),
            levels: Rc::new(copy),
            ptr: self.ptr,
        }
    }

    pub fn exit_level(&self, level: u32) -> Input {
        let mut copy = (&(*self.levels)).clone();

        if let Some(index) = copy.iter().position(|lv| *lv == level) {
            copy.remove(index);
        }

        Input {
            raw: Rc::clone(&self.raw),
            levels: Rc::new(copy),
            ptr: self.ptr,
        }
    }

    pub fn read(&self) -> Token {
        let ptr = self.skip_indent();
        self.raw.tokens[ptr].token.clone()
    }

    fn skip_indent(&self) -> usize {
        let mut ptr = self.ptr;

        // Ignore indentation that doesn't match a current level
        // defined by a `case of` or `let in` expression
        if ptr < self.raw.tokens.len() {
            while let Token::Indent(level) = &self.raw.tokens[ptr].token {
                if !self.levels.contains(level) {
                    ptr += 1;
                } else {
                    break;
                }
            }
        }

        ptr.min(self.raw.tokens.len() - 1)
    }

    pub fn read_forced(&self) -> Token {
        let ptr = self.ptr.min(self.raw.tokens.len() - 1);
        self.raw.tokens[ptr].token.clone()
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let loc = &self.raw.tokens[self.ptr].start;
        let string = self.raw.string.as_bytes();

        let mut error_pos = 0usize;

        for _ in 0..loc.line {
            while string[error_pos] != b'\n' {
                error_pos += 1;
            }
            error_pos += 1;
        }

        error_pos += loc.column as usize;

        if error_pos >= string.len() {
            error_pos = string.len() - 1;
        }

        let mut line_start = error_pos;
        let mut line_end = error_pos;

        while line_start > 0 {
            if string[line_start] == b'\n' {
                line_start += 1;
                break;
            }
            line_start -= 1;
        }

        while line_end < string.len() {
            if string[line_end] == b'\n' {
                break;
            }
            line_end += 1;
        }

        let mut line = String::new();
        let mut pointer = String::new();
        let mut trail = String::new();

        for index in line_start..line_end {
            if index == error_pos {
                trail.push('┘');
                pointer.push('\u{028C}');
            } else if index < error_pos {
                trail.push('─');
                pointer.push(' ');
            }
            line.push(string[index] as char);
        }

        let line_num = format!("{}", loc.line + 1);
        let mut spaces = String::new();

        for _ in 0..line_num.len() {
            spaces.push(' ');
        }

        write!(f, "\n{} │ {}\n{} │ {}\n{} │ {}", line_num, line, spaces, pointer, spaces, trail)
    }
}

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
        from_code_mod(include_bytes!("../../../benches/data/tokenizer_1.elm"));
    }

    #[test]
    fn test_bench_2() {
        from_code_mod(include_bytes!("../../../benches/data/tokenizer_2.elm"));
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

use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::sync::Arc;
use tokenizer::Token;
use tokenizer::TokenInfo;

mod util;
mod pattern;
mod expression;

#[derive(PartialEq, Debug, Clone)]
pub enum ParseError {
    TODO,
    Expected { input: Input, expected: Token, found: Token },
    ExpectedId { input: Input, found: Token },
    ExpectedUpperId { input: Input, found: Token },
    UnmatchedToken { input: Input, found: Token, options: Vec<Token> },
    ExpectedIndentationLevel { input: Input, expected: u32, found: u32 },
    ExpectedIndentation { input: Input, found: Token },
}

#[derive(PartialEq, Debug, Clone)]
pub struct Input {
    code_str: Arc<String>,
    code: Arc<Vec<TokenInfo>>,
    ptr: usize,
    levels: Vec<u32>,
}

impl Input {
    pub fn new(code_str: String, code: Vec<TokenInfo>) -> Self {
        Input { code_str: Arc::new(code_str), code: Arc::new(code), ptr: 0, levels: vec![0] }
    }

    pub fn next(&self) -> Input {
        let mut ptr = self.ptr;

        // Ignore indentation that doesn't match a current level defined by a `case of` or `let in` expression
        while let Token::Indent(level) = &self.code[ptr].token {
            if !self.levels.contains(level) {
                ptr += 1;
            } else {
                break;
            }
        }

        Input {
            code_str: Arc::clone(&self.code_str),
            code: Arc::clone(&self.code),
            ptr: ptr + 1,
            levels: self.levels.clone(),
        }
    }

    pub fn enter_level(&self, level: u32) -> Input {
        let mut copy = self.clone();
        copy.levels.push(level);
        copy
    }

    pub fn exit_level(&self, level: u32) -> Input {
        let mut copy = self.clone();

        if let Some(index) = self.levels.iter().position(|lv| *lv == level) {
            copy.levels.remove(index);
        }
        copy
    }

    pub fn read(&self) -> Token {
        self.code[self.ptr].token.clone()
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let loc = &self.code[self.ptr].start;

        let mut error_pos = 0usize;

        for _ in 0..loc.line {
            while self.code_str.as_bytes()[error_pos] != b'\n' {
                error_pos += 1;
            }
        }

        error_pos += loc.column as usize;

        if error_pos >= self.code_str.len() {
            error_pos = self.code_str.len() - 1;
        }

        let mut line_start = error_pos;
        let mut line_end = error_pos;

        while line_start > 0 {
            if self.code_str.as_bytes()[line_start] == b'\n' {
                break;
            }
            line_start -= 1;
        }

        while line_end < self.code_str.len() {
            if self.code_str.as_bytes()[line_end] == b'\n' {
                break;
            }
            line_end += 1;
        }

        let mut line = String::new();
        let mut pointer = String::new();
        let mut trail = String::new();

        for index in line_start..line_end {
            if index == error_pos {
                trail.push('\u{2143}');
                pointer.push('\u{028C}');
            } else if index < error_pos {
                trail.push('_');
                pointer.push(' ');
            }
            line.push(self.code_str.as_bytes()[index] as char);
        }

        write!(f, "\n| {}\n| {}\n| {}", line, pointer, trail)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            ParseError::TODO => {
                write!(f, "TODO\n")
            }
            ParseError::Expected { input, expected, found } => {
                write!(f, "Expected token '{}', but found '{}': {}\n", expected, found, input)
            }
            ParseError::ExpectedId { input, found } => {
                write!(f, "Expected identifier, but found '{}': {}\n", found, input)
            }
            ParseError::ExpectedUpperId { input, found } => {
                write!(f, "Expected capitalized identifier, but found '{}': {}\n", found, input)
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

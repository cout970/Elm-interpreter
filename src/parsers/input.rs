use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::Rc;

use ast::Span;
use errors::print_code_location;
use source::SourceCode;
use tokenizer::Token;
use tokenizer::TokenInfo;

#[derive(PartialEq, Debug, Clone)]
pub struct Input {
    raw: Rc<RawInput>,
    ptr: usize,
    levels: Rc<Vec<u32>>,
}

#[derive(PartialEq, Debug, Clone)]
struct RawInput {
    string: SourceCode,
    tokens: Vec<TokenInfo>,
}

impl Input {
    pub fn new(code_str: SourceCode, code: Vec<TokenInfo>) -> Self {
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

    pub fn read(&self) -> Token {
        let ptr = self.skip_indent();
        self.raw.tokens[ptr].token.clone()
    }

    pub fn read_forced(&self) -> Token {
        let ptr = self.ptr.min(self.raw.tokens.len() - 1);
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

    pub fn span(&self) -> Span {
        let ptr = self.skip_indent();
        self.raw.tokens[ptr].span
    }

    pub fn pos(&self) -> u32 {
        let ptr = self.skip_indent();
        self.raw.tokens[ptr].span.0
    }

    pub fn pos_end(&self) -> u32 {
        let ptr = (self.ptr as i32 - 1).max(0) as usize;
        self.raw.tokens[ptr].span.1
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", print_code_location(self.raw.string.as_str(), &self.span()))?;
        Ok(())
    }
}

#[cfg(Test)]
pub fn print_tokens(mut i: Input) {
    while i.read() != Token::Eof {
        println!("Tk: {}", i.read());
        i = i.next();
    }
}

#[cfg(Test)]
pub fn print_token_locations(f: &mut Formatter, i: Input) {
    for tk in i.raw.tokens.iter().skip(i.ptr).take(20) {
        write!(f, "{:03}:{:03} | {}\n", tk.span.0 + 1, tk.span.1 + 1, tk.token).unwrap();
    }
}
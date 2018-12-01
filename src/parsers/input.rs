use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Write;
use std::rc::Rc;

use tokenizer::Location;
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
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (start, end) = self.raw.tokens[self.ptr].span;
        write!(f, "{}", print_code_location(&self.raw.string, start, end))?;
        Ok(())
    }
}

pub fn print_code_location(input: &str, start: Location, end: Location) -> String {
    if input.is_empty() {
        return String::from("Empty");
    }

    let byte_input: &[u8] = input.as_bytes();
    let marker_start = start as usize;
    let marker_end = end as usize;

    let mut line_start = marker_start.min(byte_input.len() - 1).max(0);
    let mut line_end = marker_end.min(byte_input.len() - 1).max(0);

    while line_start > 0 {
        if byte_input[line_start] == b'\n' {
            line_start += 1;
            break;
        }
        line_start -= 1;
    }

    while line_end < byte_input.len() {
        if byte_input[line_end] == b'\n' {
            break;
        }
        line_end += 1;
    }

    let mut line = String::new();
    let mut pointer = String::new();
    let mut trail = String::new();

    for index in line_start..line_end {
        if index == marker_start {
            trail.push('┘');
            pointer.push('\u{028C}');
        } else if index < marker_start {
            trail.push('─');
            pointer.push(' ');
        } else if index < marker_end {
            pointer.push('\u{028C}');
        }
        line.push(byte_input[index] as char);
    }

    let line_num = (&byte_input[0..marker_start]).iter().filter(|&i| *i == b'\n').count();
    let line_num_str = format!("{}", line_num + 1);
    let mut spaces = String::new();

    for _ in 0..line_num_str.len() {
        spaces.push(' ');
    }

    let mut output = String::new();
    write!(&mut output, "\n{} │ {}\n{} │ {}\n{} │ {}", line_num_str, line, spaces, pointer, spaces, trail).unwrap();

    output
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
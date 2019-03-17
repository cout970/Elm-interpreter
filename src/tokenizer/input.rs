use std::collections::VecDeque;
use std::str::Chars;
use std::sync::Arc;

use source::SourceCode;
use tokenizer::Location;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct InputSlice<'a> {
    pub stream: &'a [u8],
    pos: u32,
}

impl<'a> InputSlice<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        InputSlice {
            stream,
            pos: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.stream.len()
    }

    #[inline]
    pub fn advance(&self, n: usize) -> InputSlice<'a> {
        InputSlice {
            stream: &self.stream[n..],
            pos: self.pos + (n as u32),
        }
    }

    pub fn get_location(&self) -> Location {
        self.pos
    }
}

/// Provides indexed access to a iterator-based stream of characters, it limited to the last 5
/// characters to void storing the entire source code
#[derive(Clone)]
struct SourceCodeView<'a> {
    iter: Chars<'a>,
    last: VecDeque<char>,
    pos: u32,
}

impl<'a> SourceCodeView<'a> {
    /// Creates a new view over a sourcecode
    pub fn new(code: &'a SourceCode) -> Self {
        SourceCodeView {
            iter: code.chars(),
            last: VecDeque::with_capacity(5),
            pos: 0,
        }
    }

    /// Save the current internal state
    fn save(&self) -> SourceCodeView {
        self.clone()
    }

    // Restore the internal state from a previous save
    fn restore(&mut self, save: SourceCodeView<'a>) {
        self.iter = save.iter;
        self.last = save.last;
        self.pos = save.pos;
    }

    /// Get a character using a numeric index, this handles UTF-8 variable-byte-length chars but once
    /// you read a character you can't access the characters that are 5 positions bellow that.
    fn get_char(&mut self, pos: i32) -> char {
        let diff = pos - (self.pos as i32);
        let mut character: char = '\0';

        if diff == 0 {
            character = self.iter.next().unwrap_or('\0');
            self.pos += 1;
            self.last.push_front(character);
        } else if diff > 0 {
            for i in 0..diff {
                character = self.iter.next().unwrap_or('\0');
                self.pos += 1;

                // Don't store more than 5 chars back
                if self.last.len() >= 5 {
                    self.last.pop_back().unwrap();
                }

                self.last.push_front(character);
            }
        } else if diff < 0 {
            let pos = self.last.len() as i32 + diff;

            if pos < 0 {
                panic!("Unable to go that far back! diff: {}", diff);
            }

            character = self.last[pos as usize];
        }

        character
    }
}
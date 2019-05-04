use nom::{Err as NomErr, is_space};
use nom::verbose_errors::Context;

use ast::{Float, Int};
use ast::Span;
use errors::*;
use source::SourceCode;
use tokenizer::token_parser::read_token;

mod token_parser;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Id(String),
    UpperId(String),
    BinaryOperator(String),
    LitInt(Int),
    LitFloat(Float),
    LitChar(char),
    LitString(String),
    Indent(u32),
    BackSlash,
    PrefixMinus,
    Let,
    If,
    Else,
    Then,
    Case,
    Of,
    In,
    ModuleTk,
    WhereTk,
    ExposingTk,
    ImportTk,
    EffectTk,
    As,
    TypeTk,
    Port,
    Alias,
    InfixTk,
    Underscore,
    Dot,
    DoubleDot,
    Comma,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Equals,
    Pipe,
    RightArrow,
    LeftArrow,
    Colon,
    Eof,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TokenInfo {
    pub span: Span,
    pub token: Token,
}

#[deprecated(since = "0.0.0", note = "please use `Tokenizer::tokenize` instead")]
pub fn tokenize<'a>(stream: &[u8]) -> Result<Vec<TokenInfo>, ElmError> {
    let code = SourceCode::from_str(String::from_utf8_lossy(stream).as_ref());
    Tokenizer::new(&code).tokenize()
}

pub struct Tokenizer {
    code: SourceCode,
    pos: usize,
}

impl Tokenizer {
    pub fn new(code: &SourceCode) -> Self {
        Tokenizer { code: code.clone(), pos: 0 }
    }

    pub fn source_code(&self) -> SourceCode {
        self.code.clone()
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenInfo>, ElmError> {
        let (tokens, errors) = self.read_all();

        if !errors.is_empty() {
            if errors.len() == 1 {
                Err(errors.into_iter().next().unwrap())
            } else {
                Err(ElmError::List(errors))
            }
        } else {
            Ok(tokens)
        }
    }

    pub fn read_all(&mut self) -> (Vec<TokenInfo>, Vec<ElmError>) {
        let mut errors = vec![];
        let mut tokens = vec![];

        while self.pos < self.code.as_bytes().len() && self.byte(0) != b'\0' {
            match self.next() {
                Ok((span, token)) => tokens.push(TokenInfo { span, token }),
                Err(error) => errors.push(error)
            }
        }

        // Read EOF
        match self.next() {
            Ok((span, token)) => tokens.push(TokenInfo { span, token }),
            Err(error) => errors.push(error)
        }

        (tokens, errors)
    }

    pub fn next(&mut self) -> Result<(Span, Token), ElmError> {
        if self.pos >= self.code.len() {
            return Ok(((self.pos as u32, self.pos as u32), Token::Eof));
        }

        let start = self.pos as u32;
        let opt = self.trim_spaces();

        if let Some(tk) = opt {
            return Ok(((start, self.pos as u32), tk));
        }
        let start = self.pos as u32;
        let remaining_bytes = &self.code.as_bytes()[self.pos..];
        let result: Result<(&[u8], Token), NomErr<&[u8], u32>> = read_token(remaining_bytes);

        match result {
            Ok((rest, tk)) => {
                // Unary minus exception
                let mut replace_tk = false;
                if let Token::BinaryOperator(op) = &tk {
                    if op == "-" {
                        let after_space = self.pos == 0 || is_space(self.code.as_bytes()[(self.pos - 1)]);
                        let before_space = is_space(rest[0]);

                        // space     (-) space      -> binary
                        // not-space (-) space      -> binary
                        // space     (-) not-space  -> unary
                        // not-space (-) not-space  -> binary
                        replace_tk = after_space && !before_space;
                    }
                };

                let final_tk = if replace_tk { Token::PrefixMinus } else { tk };

                let real = self.code.as_bytes().len();
                let consumed = (real - start as usize) - rest.len();
                self.pos += consumed;
                Ok(((start, self.pos as u32), final_tk))
            }
            Err(e) => {
                let result = match e {
                    NomErr::Incomplete(_) => {
                        // This happens when read_token_forced reaches the end of the code
                        // trying to read a string or a large name
                        Err(ElmError::Tokenizer(self.code.clone(), LexicalError::ReachedEnd {
                            pos: self.pos as u32
                        }))
                    }
                    NomErr::Error(ctx) | NomErr::Failure(ctx) => {
                        // Some invalid character or unknown sequence of characters that we are
                        // unable to convert to tokens
                        let input: &[u8] = match ctx {
                            Context::Code(input, _) => input,
                            // TODO remove nom verbose errors, we already handle errors in a custom way
                            Context::List(vec) => vec[0].0.clone()
                        };
                        let new_pos = self.code.as_str().len() - input.len();

                        Err(ElmError::Tokenizer(self.code.clone(), LexicalError::UnableToTokenize {
                            span: (start as u32, new_pos as u32)
                        }))
                    }
                };

                // We ignore the current character and try to tokenize the rest of characters
                self.pos += 1;

                result
            }
        }
    }

    fn byte(&self, ptr: usize) -> u8 {
        self.code.as_bytes()[self.pos + ptr]
    }

    fn trim_spaces(&mut self) -> Option<Token> {
        let mut ptr = 0;
        let mut new_line = false;

        while self.byte(ptr) == b' ' || self.byte(ptr) == b'\n' {
            new_line |= self.byte(ptr) == b'\n';
            ptr += 1;
        }

        if new_line {
            let mut indentation = 0;

            for j in 0..ptr {
                let j = ptr - j - 1;
                if self.byte(j) == b'\n' {
                    break;
                }
                indentation += 1;
            }

            let tk = Token::Indent(indentation);
            self.pos += ptr;
            return Some(tk);
        }

        self.pos += ptr;
        self.trim_comments()
    }

    fn trim_comments(&mut self) -> Option<Token> {
        let offset = self.trim_multiline_comments();

        if offset == 0 {
            self.trim_single_line_comments()
        } else {
            self.pos += offset;
            self.trim_spaces()
        }
    }

    fn trim_multiline_comments(&mut self) -> usize {
        let mut nesting = 0;
        let mut offset = 0;

        loop {
            if self.byte(offset) == b'{' && self.byte(offset + 1) == b'-' {
                nesting += 1;
            }

            if nesting == 0 { break; }

            if self.byte(offset) == b'-' && self.byte(offset + 1) == b'}' {
                nesting -= 1;
                offset += 2;
                if nesting == 0 { break; }
            }

            offset += 1;
        }

        offset
    }

    fn trim_single_line_comments(&mut self) -> Option<Token> {
        // Line starts with --
        if self.byte(0) == b'-' && self.byte(1) == b'-' {
            let mut ptr = 2;

            // Line ends with \n or \r\n
            while self.byte(ptr) != b'\n' && self.byte(ptr) != b'\r' && self.byte(ptr) != b'\0' {
                ptr += 1;
            }

            self.pos += ptr;
            self.trim_spaces()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use source::SourceCode;
    use test_utils::Test;
    use tokenizer::Tokenizer;
    use util::StringConversion;
    use util::VecExt;

    use super::Token::*;

    #[test]
    fn check_tokens() {
        let code = "identifier1234,123.45";
        let tokens = Test::tokens(code).map(|info| info.token.clone());

        assert_eq!(tokens, vec![Id("identifier1234".s()), Comma, LitFloat(123.45), Eof]);
    }

    #[test]
    fn check_multiline_comment() {
        let code = "1 + {- this is my comment -} 2";
        let tokens = Test::tokens(code).map(|info| info.token.clone());
        assert_eq!(tokens, vec![
            LitInt(1),
            BinaryOperator("+".s()),
            LitInt(2),
            Eof
        ]);
    }

    #[test]
    fn check_multiline_comment_recursive() {
        let code = "1 + {- this {- is my -} comment -} 2";
        let tokens = Test::tokens(code).map(|info| info.token.clone());
        assert_eq!(tokens, vec![
            LitInt(1),
            BinaryOperator("+".s()),
            LitInt(2),
            Eof
        ]);
    }

    #[test]
    fn check_line_comment() {
        let code = "1 --this is a comment\n2";
        let tokens = Test::tokens(code).map(|info| info.token.clone());
        assert_eq!(tokens, vec![
            LitInt(1),
            Indent(0),
            LitInt(2),
            Eof
        ]);
    }

    #[test]
    fn check_identifiers() {
        let code = "i, _a, b123, cBAD, aghjh, get_something";
        let tokens = Test::tokens(code).map(|info| info.token.clone());
        assert_eq!(tokens, vec![
            Id("i".s()), Comma,
            Underscore,
            Id("a".s()), Comma,
            Id("b123".s()), Comma,
            Id("cBAD".s()), Comma,
            Id("aghjh".s()), Comma,
            Id("get_something".s()),
            Eof
        ]);
    }

    #[test]
    fn check_indentation_token() {
        let code = "case i of\n  1\n  2\nmy_func";
        let tokens = Test::tokens(code).map(|info| info.token.clone());
        assert_eq!(tokens, vec![
            Case, Id("i".s()), Of,
            Indent(2), LitInt(1),
            Indent(2), LitInt(2),
            Indent(0),
            Id("my_func".s()),
            Eof
        ]);
    }

    #[test]
    fn prefix_minus_edge_case() {
        let code = "(+), (-), (*)";
        let tokens = Test::tokens(code).map(|info| info.token.clone());
        assert_eq!(tokens, vec![
            LeftParen, BinaryOperator("+".s()), RightParen, Comma,
            LeftParen, BinaryOperator("-".s()), RightParen, Comma,
            LeftParen, BinaryOperator("*".s()), RightParen, Eof
        ]);
    }

    #[test]
    fn unary_minus() {
        let code = "n-1";
        let tokens = Test::tokens(code).map(|info| info.token.clone());
        assert_eq!(tokens, vec![
            Id("n".s()), BinaryOperator("-".s()), LitInt(1), Eof
        ]);
    }

    #[test]
    fn force_error() {
        let code = "foo\"bar!@#$%^&*()_+=-[]'\\/.,`test";
        let result = Tokenizer::new(&SourceCode::from_str(code)).tokenize();
        match result {
            Ok(_) => panic!("Error not found!"),
            Err(e) => {
                println!("Found error: \n{}\n", e);
            }
        }
    }
}
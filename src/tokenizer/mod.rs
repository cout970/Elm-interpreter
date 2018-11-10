use ast::{Float, Int};
use nom::*;
use tokenizer::input::Location;
use tokenizer::token_reader::read_tokens;

mod input;
mod token_parser;
mod token_reader;

#[derive(PartialEq, Debug, Clone)]
pub enum LexicalError {
    Incomplete(Needed),
    Error(Location, ErrorKind),
    Failure(Location, ErrorKind),
}

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
    pub start: Location,
    pub end: Location,
    pub token: Token,
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

pub fn tokenize<'a>(stream: &[u8]) -> Result<Vec<TokenInfo>, LexicalError> {
    read_tokens(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Token::*;
    use util::StringConversion;
    use util::VecExt;

    #[test]
    fn check_tokens() {
        let code = b"identifier1234,123.45";
        let tokens = tokenize(code).unwrap().map(|info| info.token.clone());
        assert_eq!(tokens, vec![Id("identifier1234".s()), Comma, LitFloat(123.45), Eof]);
    }

    #[test]
    fn check_multiline_comment() {
        let code = b"1 + {- this is my comment -} 2";
        let tokens = tokenize(code).unwrap().map(|info| info.token.clone());
        assert_eq!(tokens, vec![
            LitInt(1),
            BinaryOperator("+".s()),
            LitInt(2),
            Eof
        ]);
    }

    #[test]
    fn check_multiline_comment_recursive() {
        let code = b"1 + {- this {- is my -} comment -} 2";
        let tokens = tokenize(code).unwrap().map(|info| info.token.clone());
        assert_eq!(tokens, vec![
            LitInt(1),
            BinaryOperator("+".s()),
            LitInt(2),
            Eof
        ]);
    }

    #[test]
    fn check_line_comment() {
        let code = b"1 --this is a comment\n2";
        let tokens = tokenize(code).unwrap().map(|info| info.token.clone());
        assert_eq!(tokens, vec![
            LitInt(1),
            Indent(0),
            LitInt(2),
            Eof
        ]);
    }

    #[test]
    fn check_identifiers() {
        let code = b"i, _a, b123, cBAD, aghjh, get_something";
        let tokens = tokenize(code).unwrap().map(|info| info.token.clone());
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
        let code = b"case i of\n  1\n  2\nmy_func";
        let tokens = tokenize(code).unwrap().map(|info| info.token.clone());
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
    fn prefix_minus_edge_case(){
        let code = b"(+), (-), (*)";
        let tokens = tokenize(code).unwrap().map(|info| info.token.clone());
        assert_eq!(tokens, vec![
            LeftParen, BinaryOperator("+".s()), RightParen, Comma,
            LeftParen, PrefixMinus, RightParen, Comma,
            LeftParen, BinaryOperator("*".s()), RightParen, Eof
        ]);
    }
}
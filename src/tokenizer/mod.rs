use nom::*;
use self::Token::*;
use tokenizer::input::Input;
use tokenizer::token_parser::read_token_forced;
use types::*;
use util::*;

mod input;
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
    Let,
    If,
    Else,
    Then,
    Case,
    Of,
    In,
    ModuleTk,
    Where,
    Exposing,
    ImportTk,
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

pub fn tokenize(stream: &[u8]) -> Vec<Token> {
    let mut stream: Vec<u8> = stream.iter().map(|c| *c).collect();
    stream.push('\0' as u8);
    stream.push('\0' as u8);

    let mut tokens = Vec::new();
    let mut current_input: Input = Input::new(&stream);

    loop {
        let res: Result<(Input, Token), Err<Input, u32>> = next_token(current_input);

        match res {
            Ok((rem, token)) => {
                if token == Token::Eof { break; }
                tokens.push(token);
                current_input = rem;
            }
            Err(_) => {
                panic!("\n{:?}\n", res);
            }
        };
    }
    tokens.push(Token::Eof);

    tokens
}

fn next_token<'a>(i: Input) -> IResult<Input, Token> {
    use nom::verbose_errors::Context;

    if i.stream.len() == 0 {
        return Err(Err::Incomplete(Needed::Size(1)));
    }

    let mut ptr = 0;
    let mut new_line = false;

    while i.stream[ptr] == b' ' || i.stream[ptr] == b'\n' {
        new_line |= i.stream[ptr] == b'\n';
        ptr += 1;
    }

    if new_line {
        let mut indentation = 0;

        for j in 0..ptr {
            let j = ptr - j - 1;
            if i.stream[j] == b'\n' {
                break;
            }
            indentation += 1;
        }

        let tk = Indent(indentation);
        return Ok((i.advance(ptr), tk));
    }

    let (tk, len) = {
        let rest = &i.stream[ptr..];

        let opt_tk = map_result(rest, read_token_forced(rest));

        match opt_tk {
            Some(pair) => pair,
            _ => return Err(Err::Failure(
                Context::Code(i, ErrorKind::Custom(0))
            ))
        }
    };

    Ok((i.advance(ptr + len), tk))
}

fn map_result(input: &[u8], res: IResult<&[u8], Token>) -> Option<(Token, usize)> {
    match res {
        Ok((rest, ret)) => {
            Some((ret, input.len() - rest.len()))
        }
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use nom::*;
    use super::*;

    #[test]
    fn check_tokens() {
        let code = b"identifier1234,123.45";
        let tokens = tokenize(code);
        assert_eq!(tokens, vec![Id("identifier1234".s()), Comma, LitFloat(123.45), Eof]);
    }

    #[test]
    fn check_identifiers() {
        let code = b"i, _a, b123, cBAD, aghjh, get_something";
        let tokens = tokenize(code);
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
        let tokens = tokenize(code);
        assert_eq!(tokens, vec![
            Case, Id("i".s()), Of,
            Indent(2), LitInt(1),
            Indent(2), LitInt(2),
            Indent(0),
            Id("my_func".s()),
            Eof
        ]);
    }
}
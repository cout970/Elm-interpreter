use nom::*;
use self::Token::*;
use tokenizer::input::Input;
use tokenizer::input::Location;
use tokenizer::token_parser::read_token_forced;
use ast::*;

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
    PrefixMinus,
    Let,
    If,
    Else,
    Then,
    Case,
    Of,
    In,
    ModuleTk,
    Where,
    ExposingTk,
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

#[derive(PartialEq, Debug, Clone)]
pub enum LexicalError {
    Incomplete(Needed),
    Error(Location, ErrorKind),
    Failure(Location, ErrorKind),
}

pub fn tokenize(stream: &[u8]) -> Result<Vec<Token>, LexicalError> {
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
            Result::Err(e) => {
                match e {
                    Err::Incomplete(needed) => {
                        return Err(LexicalError::Incomplete(needed));
                    }
                    Err::Error(ctx) => {
                        let (loc, kind) = match ctx {
                            Context::Code(i, kind) => { (i.get_location(), kind.clone()) }
                            Context::List(vec) => {
                                let (i, k) = vec.first().unwrap();
                                (i.get_location(), k.clone())
                            }
                        };
                        return Err(LexicalError::Error(loc, kind));
                    }
                    Err::Failure(ctx) => {
                        let (loc, kind) = match ctx {
                            Context::Code(i, kind) => { (i.get_location(), kind.clone()) }
                            Context::List(vec) => {
                                let (i, k) = vec.first().unwrap();
                                (i.get_location(), k.clone())
                            }
                        };
                        return Err(LexicalError::Failure(loc, kind));
                    }
                }
            }
        };
    }
    tokens.push(Token::Eof);

    Ok(tokens)
}

fn next_token<'a>(i: Input) -> IResult<Input, Token> {
    use nom::verbose_errors::Context;

    if i.stream.len() == 0 {
        return Err(Err::Incomplete(Needed::Size(1)));
    }

    let (i, opt) = trim_spaces(i.clone());

    if let Some(tk) = opt {
        return Ok((i, tk));
    }

    let (tk, len) = {
        let rest = i.stream;

        let opt_tk = map_result(rest, read_token_forced(rest));

        match opt_tk {
            Some(pair) => pair,
            _ => return Err(Err::Failure(
                Context::Code(i, ErrorKind::Custom(0))
            ))
        }
    };

    Ok((i.advance(len), tk))
}

fn trim_spaces(i: Input) -> (Input, Option<Token>) {
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
        return (i.advance(ptr), Some(tk));
    }

    trim_comments(i.advance(ptr))
}

fn trim_comments(i: Input) -> (Input, Option<Token>) {
    let offset = trim_multiline_comments(i.clone());

    if offset == 0 {
        trim_single_line_comments(i)
    } else {
        trim_spaces(i.advance(offset))
    }
}

fn trim_multiline_comments(i: Input) -> usize {
    let rest = i.stream;
    let mut nesting = 0;
    let mut offset = 0;

    loop {
        if rest[offset] == b'{' && rest[offset + 1] == b'-' {
            nesting += 1;
        }

        if nesting == 0 { break; }

        if rest[offset] == b'-' && rest[offset + 1] == b'}' {
            nesting -= 1;
            offset += 2;
            if nesting == 0 { break; }
        }

        offset += 1;
    }

    offset
}

fn trim_single_line_comments(i: Input) -> (Input, Option<Token>) {
    let rest = i.stream;

    if rest[0] == b'-' && rest[1] == b'-' {
        let mut ptr = 2;

        while rest[ptr] != b'\n' && rest[ptr] != b'\r' {
            ptr += 1;
        }

        trim_spaces(i.advance(ptr))
    } else {
        (i, None)
    }
}

fn map_result<T>(input: &[u8], res: IResult<&[u8], T>) -> Option<(T, usize)> {
    match res {
        Ok((rest, ret)) => {
            Some((ret, input.len() - rest.len()))
        }
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::StringConversion;

    #[test]
    fn check_tokens() {
        let code = b"identifier1234,123.45";
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens, vec![Id("identifier1234".s()), Comma, LitFloat(123.45), Eof]);
    }

    #[test]
    fn check_multiline_comment() {
        let code = b"1 + {- this is my comment -} 2";
        let tokens = tokenize(code).unwrap();
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
        let tokens = tokenize(code).unwrap();
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
        let tokens = tokenize(code).unwrap();
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
        let tokens = tokenize(code).unwrap();
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
        let tokens = tokenize(code).unwrap();
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
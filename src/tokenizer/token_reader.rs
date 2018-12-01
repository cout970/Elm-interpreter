use nom::*;
use nom::verbose_errors::Context;

use tokenizer::input::InputSlice;
use tokenizer::LexicalError;
use tokenizer::Token;
use tokenizer::token_parser::read_token_forced;
use tokenizer::TokenInfo;

pub fn read_tokens(stream: &[u8]) -> Result<Vec<TokenInfo>, LexicalError> {
    let mut stream: Vec<u8> = stream.to_vec();
    stream.push('\0' as u8);
    stream.push('\0' as u8);

    let mut tokens: Vec<TokenInfo> = Vec::new();
    let mut current_input: InputSlice = InputSlice::new(&stream);

    loop {
        let res: Result<(InputSlice, Token), Err<InputSlice, u32>> = next_token(current_input.clone());

        match res {
            Ok((rem, token)) => {
                if token == Token::Eof { break; }
                let start = trim_spaces(current_input).0.get_location();
                let end = rem.get_location();

                tokens.push(TokenInfo { token, span: (start, end) });
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
    tokens.push(TokenInfo {
        token: Token::Eof,
        span: (current_input.get_location(), current_input.get_location()),
    });

    Ok(tokens)
}

fn next_token<'a>(i: InputSlice) -> IResult<InputSlice, Token> {
    use nom::verbose_errors::Context;

    if i.len() == 0 {
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

fn trim_spaces(i: InputSlice) -> (InputSlice, Option<Token>) {
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

        let tk = Token::Indent(indentation);
        return (i.advance(ptr), Some(tk));
    }

    trim_comments(i.advance(ptr))
}

fn trim_comments(i: InputSlice) -> (InputSlice, Option<Token>) {
    let offset = trim_multiline_comments(i.clone());

    if offset == 0 {
        trim_single_line_comments(i)
    } else {
        trim_spaces(i.advance(offset))
    }
}

fn trim_multiline_comments(i: InputSlice) -> usize {
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

fn trim_single_line_comments(i: InputSlice) -> (InputSlice, Option<Token>) {
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

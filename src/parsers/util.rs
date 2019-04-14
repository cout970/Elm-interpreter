use ast::Int;
use errors::ParseError;
use parsers::input::Input;
use tokenizer::Token;

pub fn many0<T, F>(func: &F, mut input: Input) -> Result<(Vec<T>, Input), ParseError>
    where F: Fn(Input) -> Result<(T, Input), ParseError> {
    let mut accum: Vec<T> = vec![];

    loop {
        let (res, i) = match func(input.clone()) {
            Ok(pair) => pair,
            Err(_) => {
                break;
            }
        };
        input = i;
        accum.push(res);
    }

    Ok((accum, input))
}

pub fn many1<T, F>(func: &F, input: Input) -> Result<(Vec<T>, Input), ParseError>
    where F: Fn(Input) -> Result<(T, Input), ParseError> {
    let mut accum: Vec<T> = vec![];

    let (first, mut i) = func(input)?;
    accum.push(first);

    loop {
        let (res, _i) = match func(i.clone()) {
            Ok(pair) => pair,
            Err(_) => {
                break;
            }
        };
        i = _i;
        accum.push(res);
    }

    Ok((accum, i))
}

pub fn optional<T, F>(func: &F, input: Input) -> (Option<T>, Input)
    where F: Fn(Input) -> Result<(T, Input), ParseError> {
    match func(input.clone()) {
        Ok((t, i)) => (Some(t), i),
        Err(_) => (None, input)
    }
}

pub fn elem_comma<T, F>(func: &F, input: Input) -> Result<(T, Input), ParseError>
    where F: Fn(Input) -> Result<(T, Input), ParseError> {
    let (res, i) = func(input)?;
    let i = expect(Token::Comma, i)?;
    Ok((res, i))
}

pub fn comma0<T, F>(func: &F, input: Input) -> Result<(Vec<T>, Input), ParseError>
    where F: Fn(Input) -> Result<(T, Input), ParseError> {
    let (first, mut i) = match func(input.clone()) {
        Ok(pair) => pair,
        Err(_) => {
            return Ok((vec![], input));
        }
    };

    let mut accum: Vec<T> = vec![first];

    while let Token::Comma = i.read() {
        let (next, rest) = func(i.next())?;
        accum.push(next);
        i = rest;
    }

    Ok((accum, i))
}

pub fn comma1<T, F>(func: &F, input: Input) -> Result<(Vec<T>, Input), ParseError>
    where F: Fn(Input) -> Result<(T, Input), ParseError> {
    let (first, mut i): (T, Input) = func(input)?;
    let mut accum: Vec<T> = vec![first];

    while let Token::Comma = i.read() {
        let (next, rest) = func(i.next())?;
        accum.push(next);
        i = rest;
    }

    Ok((accum, i))
}

pub fn pipe1<T, F>(func: &F, input: Input) -> Result<(Vec<T>, Input), ParseError>
    where F: Fn(Input) -> Result<(T, Input), ParseError> {
    let (first, mut i): (T, Input) = func(input)?;
    let mut accum: Vec<T> = vec![first];

    while let Token::Pipe = i.read() {
        let (next, rest) = func(i.next())?;
        accum.push(next);
        i = rest;
    }

    Ok((accum, i))
}

pub fn expect(tk: Token, input: Input) -> Result<Input, ParseError> {
    if tk == input.read() {
        Ok(input.next())
    } else {
        let found = input.read();
        Err(ParseError::Expected { span: input.span(), expected: tk, found })
    }
}

pub fn optional_tk(tk: Token, input: Input) -> Input {
    if tk == input.read() {
        input.next()
    } else {
        input
    }
}

pub fn expect_indent(expected: u32, input: Input) -> Result<Input, ParseError> {
    if expected == std::u32::MAX {
        return Ok(input);
    }

    let mut i = if let Token::Indent(found) = input.read() {
        if expected == found {
            input.next()
        } else {
            return Err(ParseError::ExpectedIndentationLevel { span: input.span(), expected, found });
        }
    } else {
        let found = input.read();
        return Err(ParseError::ExpectedIndentation { span: input.span(), found });
    };

    // Ignore all indentations in the same level
    while let Token::Indent(found) = i.read() {
        if found == expected {
            i = i.next()
        } else {
            break;
        }
    }

    Ok(i)
}

pub fn read_indent(input: Input) -> Result<u32, ParseError> {
    if let Token::Indent(found) = input.read_forced() {
        Ok(found)
    } else {
        let found = input.read();
        Err(ParseError::ExpectedIndentation { span: input.span(), found })
    }
}

pub fn read_optional_indent(input: Input) -> u32 {
    if let Token::Indent(found) = input.read_forced() {
        found
    } else {
        std::u32::MAX
    }
}

pub fn expect_int(input: Input) -> Result<(Int, Input), ParseError> {
    if let Token::LitInt(value) = input.read() {
        Ok((value, input.next()))
    } else {
        let found = input.read();
        Err(ParseError::ExpectedInt { span: input.span(), found })
    }
}

pub fn expect_id(input: Input) -> Result<(String, Input), ParseError> {
    if let Token::Id(name) = input.read() {
        Ok((name, input.next()))
    } else {
        let found = input.read();
        Err(ParseError::ExpectedId { span: input.span(), found })
    }
}

pub fn expect_upper(input: Input) -> Result<(String, Input), ParseError> {
    if let Token::UpperId(name) = input.read() {
        Ok((name, input.next()))
    } else {
        let found = input.read();
        Err(ParseError::ExpectedUpperId { span: input.span(), found })
    }
}

pub fn expect_binop(input: Input) -> Result<(String, Input), ParseError> {
    if let Token::BinaryOperator(name) = input.read() {
        Ok((name, input.next()))
    } else if let Token::PrefixMinus = input.read() {
        Ok(("-".to_owned(), input.next()))
    } else {
        let found = input.read();
        Err(ParseError::ExpectedBinaryOperator { span: input.span(), found })
    }
}

pub fn expect_upper_chain(input: Input) -> Result<(String, Input), ParseError> {
    let mut names = vec![];

    let (name, mut i) = expect_upper(input)?;
    names.push(name);

    while let Token::Dot = i.read() {
        let (name, rest) = expect_upper(i.next())?;
        names.push(name);
        i = rest;
    }

    let complete_name = if names.len() == 1 {
        names.into_iter().next().unwrap()
    } else {
        let mut string = String::new();
        let mut iter = names.iter();

        string.push_str(iter.next().unwrap());
        for x in iter {
            string.push('.');
            string.push_str(x);
        }

        string
    };

    Ok((complete_name, i))
}

pub fn complete<T, F>(func: &F, input: Input) -> Result<T, ParseError>
    where F: Fn(Input) -> Result<(T, Input), ParseError> {
    let (t, mut i): (T, Input) = func(input)?;

    // Skip empty lines at the end
    while let Token::Indent(_) = i.read() {
        i = i.next();
    }

    expect(Token::Eof, i)?;
    Ok(t)
}

#[cfg(test)]
pub mod test_utils {
    use std::fmt::Debug;

    use errors::ElmError;
    use source::SourceCode;
    use tokenizer::Tokenizer;

    use super::*;

    fn from(c: &str) -> Input {
        // TODO remove
        let code = SourceCode::from_str(c);
        let tokens = Tokenizer::new(&code).tokenize().expect("Tokenizer error");
        Input::new(code, tokens)
    }

    pub fn test_parser<F, T: Debug>(func: F, code: &str)
        where F: Fn(Input) -> Result<(T, Input), ParseError> {
        let input = from(code);
        let result = complete(&func, input.clone());
        match result {
            Ok(res) => {
                println!("Value: {:?}\n", res);
            }
            Err(error) => {
                println!("Error: {}\n", ElmError::Parser(SourceCode::from_str(code), error));
                panic!();
            }
        }
    }

    pub fn test_parser_result<F, T: Debug + PartialEq>(func: F, code: &str, value: T)
        where F: Fn(Input) -> Result<(T, Input), ParseError> {
        let input = from(code);
        let result = complete(&func, input.clone());
        match result {
            Ok(res) => {
                println!("Value: {:?}\n", res);
                assert_eq!(value, res);
            }
            Err(error) => {
                println!("Error: {}\n", ElmError::Parser(SourceCode::from_str(code), error));
                panic!();
            }
        }
    }

    pub fn test_parser_error<F, T: Debug>(func: F, code: &str)
        where F: Fn(Input) -> Result<(T, Input), ParseError> {
        let input = from(code);
        let result = complete(&func, input.clone());
        match result {
            Ok(res) => {
                println!("Unexpected success: {:?}\n", res);
                panic!();
            }
            Err(error) => {
                println!("Error: {}\n", ElmError::Parser(SourceCode::from_str(code), error));
            }
        }
    }
}

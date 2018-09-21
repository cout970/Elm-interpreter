use nom::*;
use self::Token::*;
use types::*;
use util::*;

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
    LineStart,
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

#[derive(PartialEq, Debug, Clone)]
struct Input<'a> {
    stream: &'a [u8],
    line: u32,
    column: u32,
}

impl<'a> Input<'a> {
    fn new(stream: &'a [u8]) -> Self {
        Input {
            stream,
            line: 0,
            column: 0,
        }
    }

    fn advance(&self, n: usize) -> Input<'a> {
        let skipped: &[u8] = &self.stream[0..n];

        let mut line = self.line;
        let mut column = self.column;

        for c in skipped {
            if *c as char == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }

        Input {
            stream: &self.stream[n..],
            line,
            column,
        }
    }
}

named!(lower<char>, one_of!("abcdefghijklmnopqrstuvwxyz"));

named!(upper<char>, one_of!("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));

named!(number<char>, one_of!("0123456789"));

named!(newline<char>, one_of!("\r\n"));

named!(binop_char<char>, one_of!("~!@#$%^&*-+=<>/?\\._"));

named!(id_char<char>, alt!(lower | upper | number | one_of!("_'")));

named!(read_id<Token>, do_parse!(
    a: lower >>
    b: many0!(id_char) >>
    (from_id(create_vec(a, b).into_iter().collect::<String>()))
));

named!(read_upper_id<Token>, do_parse!(
    a: upper >>
    b: many0!(id_char) >>
    (UpperId(create_vec(a, b).into_iter().collect::<String>()))
));

named!(read_literal<Token>, alt!(
    read_float  |
    read_int    |
    read_string |
    read_char
));

named!(read_int<Token>, do_parse!(
    minus: opt!(char!('-')) >>
    numbers: many1!(number) >>
    (LitInt(parse_int(minus.is_some(), numbers)))
));

named!(read_float<Token>, do_parse!(
    minus: opt!(char!('-')) >>
    integer: many0!(number) >>
    char!('.') >>
    decimal: many1!(number) >>
    (LitFloat(parse_float2(minus.is_some(), integer, decimal)))
));

named!(read_string<Token>, alt!(read_line_string | read_multiline_string));

named!(read_line_string<Token>, do_parse!(
    char!('\"') >>
    c: many0!(string_char) >>
    char!('\"') >>
    (LitString(c.into_iter().collect::<String>()))
));

named!(read_multiline_string<Token>, do_parse!(
    tag!("\"\"\"") >>
    c: many0!(here_doc_char) >>
    tag!("\"\"\"") >>
    (LitString(c.into_iter().collect::<String>()))
));

named!(read_char<Token>, delimited!(char!('\''), char_char, char!('\'')));

named!(char_char<Token>, map!(none_of!("\n\'"), |c| LitChar(c)));

named!(string_char<char>, alt!(
    none_of!("\n\"") | preceded!(char!('\\'), char!('\"'))
));

named!(here_doc_char<char>, alt!(
    none_of!("\"") | terminated!(char!('\"'), not!(tag!("\"\"")))
));

named!(left_paren<Token>, map!(char!('('), |_c| LeftParen));
named!(right_paren<Token>, map!(char!(')'), |_c| RightParen));

named!(left_braket<Token>, map!(char!('['), |_c| LeftBracket));
named!(right_braket<Token>, map!(char!(']'), |_c| RightBracket));

named!(left_brace<Token>, map!(char!('{'), |_c| LeftBrace));
named!(right_brace<Token>, map!(char!('}'), |_c| RightBrace));

named!(colon<Token>, map!(char!(':'), |_c| Colon));

named!(ignore_spaces<()>, do_parse!(
    many0!(alt!(char!(' ') | char!('\t'))) >> (())
));

// the Token::NewLine gets ignored
named!(comment_first_char<()>, not!(alt!(newline | binop_char)));

named!(line_comment<()>, do_parse!(
    tag!("--") >>
    comment_first_char >>
    many0!(none_of!("\n\r")) >>
    newline >>
    ()
));

named!(basic_binop_string<String>, map!(
    many1!(binop_char),
    |v| v.into_iter().collect::<String>()
));

fn basic_binop(input: &[u8]) -> IResult<&[u8], Token> {
    let (o, binop): (&[u8], String) = basic_binop_string(input)?;
    let op = from_binop(binop);
    Ok((o, op))
}

named!(constr_op<Token>, do_parse!(
    a: char!(':') >>
    b: many1!(alt!(binop_char | char!(':'))) >>
    (BinaryOperator(create_vec(a, b).into_iter().collect::<String>()))
));

// note this must go after ".","=","->","<-", "--"
named!(read_binop<Token>, alt!(
    basic_binop |
    constr_op
));

named!(quoted_binop<Token>, do_parse!(
    char!('`') >>
    a: alt!(
        do_parse!(a: lower >> b: many0!(id_char) >> (create_vec(a, b).into_iter().collect::<String>())) |
        do_parse!(a: upper >> b: many0!(id_char) >> (create_vec(a, b).into_iter().collect::<String>()))
    ) >>
    char!('`') >>
    (BinaryOperator(a))
));


named!(underscore<Token>, map!(char!('_'), |_c| Underscore));
named!(dot<Token>, map!(char!('.'), |_c| Dot));
named!(double_dot<Token>, map!(tag!(".."), |_c| DoubleDot));
named!(comma<Token>, map!(char!(','), |_c| Comma));
named!(equals<Token>, map!(char!('='), |_c| Equals));
named!(pipe<Token>, map!(char!('|'), |_c| Pipe));
named!(left_arrow<Token>, map!(tag!("<-"), |_c| LeftArrow));
named!(right_arrow<Token>, map!(tag!("->"), |_c| RightArrow));
named!(eof_marker<Token>, alt!(map!(eof!(), |_c| Eof) | map!(char!('\0'), |_c| Eof)));

named!(read_token_forced<Token>, alt!(
    read_binop
    | colon
    | comma
    | pipe
    | read_id
    | read_upper_id
    | read_literal
    | left_paren
    | right_paren
    | left_braket
    | right_braket
    | left_brace
    | right_brace
    | eof_marker
));

fn from_id(id: String) -> Token {
    match id.as_bytes() {
        b"let" => Let,
        b"if" => If,
        b"else" => Else,
        b"then" => Then,
        b"case" => Case,
        b"of" => Of,
        b"in" => In,
        b"module" => ModuleTk,
        b"where" => Where,
        b"exposing" => Exposing,
        b"import" => ImportTk,
        b"as" => As,
        b"type" => TypeTk,
        b"port" => Port,
        b"alias" => Alias,
        _ => Id(id)
    }
}

fn from_binop(id: String) -> Token {
    match id.as_bytes() {
        b"_" => Underscore,
        b"." => Dot,
        b".." => DoubleDot,
        b"=" => Equals,
        b"<-" => LeftArrow,
        b"->" => RightArrow,
        _ => BinaryOperator(id)
    }
}

fn map_result(input: &[u8], res: IResult<&[u8], Token>) -> Option<(Token, usize)> {
    match res {
        Ok((rest, ret)) => {
            Some((ret, input.len() - rest.len()))
        }
        _ => None
    }
}

fn read_token<'a>(i: Input) -> IResult<Input, Token> {
    use nom::simple_errors::Context;

    if i.stream.len() == 0 {
        return Err(Err::Incomplete(Needed::Size(1)));
    }

//    let mut i = i;
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

        if indentation == 0 {
            return Ok((i.advance(ptr), LineStart));
        } else {
            return Ok((i.advance(ptr), Indent(indentation)));
        }
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

pub fn get_all_tokens(stream: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut stream: Vec<u8> = stream.iter().map(|c| c.to_owned()).collect();
    stream.push('\0' as u8);
    stream.push('\0' as u8);

    let mut input: Input = Input::new(&stream);
    loop {
        let res = read_token(input);

        match res {
            Ok((rem, token)) => {
                tokens.push(token.clone());
                if token == Token::Eof { break; }
                input = rem;
            }
            Err(_) => {
                panic!("\n{:?}\n", res);
            }
        };
    }

    tokens
}

#[cfg(test)]
mod tests {
    use nom::*;
    use super::*;

    #[test]
    fn check_line_string() {
        assert_ok!(
            read_line_string("\"Hello World\"".as_bytes()),
            LitString("Hello World".s())
        );
    }

    #[test]
    fn check_multiline_string() {
        assert_ok!(
            read_multiline_string(b"\"\"\"Hello\nWorld\"\"\""),
            LitString("Hello\nWorld".s())
        );
    }

    #[test]
    fn check_char() {
        assert_ok!(
            read_char(b"'H'"),
            LitChar('H')
        );
    }

    #[test]
    fn check_int() {
        assert_ok!(read_int(b"0|"), LitInt(0));
        assert_ok!(read_int(b"-1|"), LitInt(-1));
        assert_ok!(read_int(b"1|"), LitInt(1));
        assert_ok!(read_int(b"99999|"), LitInt(99999));
        assert_ok!(read_int(b"-1234|"), LitInt(-1234));
    }

    #[test]
    fn check_float() {
        assert_ok!(read_float(b"0.0|"), LitFloat(0.0));
        assert_ok!(read_float(b"-1.0|"), LitFloat(-1.0));
        assert_ok!(read_float(b".0|"), LitFloat(0.0));
        assert_ok!(read_float(b"-.0|"), LitFloat(0.0));
        assert_ok!(read_float(b"1.2|"), LitFloat(1.2));
        assert_ok!(read_float(b"99999.0|"), LitFloat(99999.0));
        assert_ok!(read_float(b"-1234.0|"), LitFloat(-1234.0));
    }

    #[test]
    fn check_comment() {
        assert_ok!(line_comment(b"-- \n"), ());
        assert_ok!(line_comment(b"-- aghf\n"), ());
        assert_ok!(line_comment(b"--sss\n"), ());
        assert_ok!(line_comment(b"--srtga\n"), ());
        assert_ok!(line_comment(b"-- er ert -- eyr\n"), ());
    }

    #[test]
    fn check_binop() {
        assert_ok!(read_token_forced(b"= "), Equals);
        assert_ok!(read_token_forced(b"== "), BinaryOperator("==".s()));
        assert_ok!(read_token_forced(b"=== "), BinaryOperator("===".s()));
        assert_ok!(read_token_forced(b"- "), BinaryOperator("-".s()));
        assert_ok!(read_token_forced(b"-- "), BinaryOperator("--".s()));
        assert_ok!(read_token_forced(b"--- "), BinaryOperator("---".s()));
        assert_ok!(read_token_forced(b". "), Dot);
        assert_ok!(read_token_forced(b".. "), DoubleDot);
        assert_ok!(read_token_forced(b"... "), BinaryOperator("...".s()));
        assert_ok!(read_token_forced(b"-> "), RightArrow);
        assert_ok!(read_token_forced(b"<- "), LeftArrow);
        assert_ok!(read_token_forced(b"<-- "), BinaryOperator("<--".s()));
    }

    #[test]
    fn check_tokens() {
        let code = b"identifier1234,123.45";
        let tokens = get_all_tokens(code);
        assert_eq!(tokens, vec![Id("identifier1234".s()), Comma, LitFloat(123.45), Eof]);
    }

    #[test]
    fn check_identifiers() {
        let code = b"i, _a, b123, cBAD, aghjh, get_something";
        let tokens = get_all_tokens(code);
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
        let tokens = get_all_tokens(code);
        assert_eq!(tokens, vec![
            Case, Id("i".s()), Of,
            Indent(2), LitInt(1),
            Indent(2), LitInt(2),
            LineStart,
            Id("my_func".s()),
            Eof
        ]);
    }
}
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

named!(lower<char>, one_of!("abcdefghijklmnopqrstuvwxyz"));

named!(upper<char>, one_of!("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));

named!(number<char>, one_of!("0123456789"));

named!(newline<char>, one_of!("\n\r"));

named!(binop_char<char>, one_of!("~!@#$%^&*-+=<>/?\\._"));

named!(id_char<char>, alt!(lower | upper | number | one_of!("_'")));

named!(read_id<Token>, do_parse!(
    a: lower >>
    b: many0!(id_char) >>
    (Id(create_vec(a, b).into_iter().collect::<String>()))
));

named!(read_upper_id<Token>, do_parse!(
    a: upper >>
    b: many0!(id_char) >>
    (UpperId(create_vec(a, b).into_iter().collect::<String>()))
));

named!(read_literal<Token>, alt!(
    read_int    |
    read_float  |
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
    many0!(alt!( char!(' ') | char!('\t') )) >> (())
));

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

    let op = match binop.as_bytes() {
        b"." => Dot,
        b".." => DoubleDot,
        b"=" => Equals,
        b"<-" => LeftArrow,
        b"->" => RightArrow,
        _ => BinaryOperator(binop)
    };

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


named!(dot<Token>, map!(char!('.'), |_c| Dot));
named!(double_dot<Token>, map!(tag!(".."), |_c| DoubleDot));
named!(comma<Token>, map!(char!(','), |_c| Comma));
named!(equals<Token>, map!(char!('='), |_c| Equals));
named!(pipe<Token>, map!(char!('|'), |_c| Pipe));
named!(left_arrow<Token>, map!(tag!("<-"), |_c| LeftArrow));
named!(right_arrow<Token>, map!(tag!("->"), |_c| RightArrow));
named!(eof_marker<Token>, alt!(map!(eof!(), |_c| Eof) | map!(char!('\0'), |_c| Eof)));

named!(read_token_forced<Token>, alt!(
    read_binop          |
    colon               |
    comma               |
    pipe                |
    read_id             |
    read_upper_id       |
    read_literal        |
    left_paren          |
    right_paren         |
    left_braket         |
    right_braket        |
    left_brace          |
    right_brace         |
    eof_marker
));

named!(pub read_token<Token>, do_parse!(
    ignore_spaces >>
    many0!(line_comment) >>
    token: read_token_forced >>
    many0!(newline) >>
    (token)
));

pub fn get_all_tokens(stream: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut stream: Vec<u8> = stream.iter().map(|c| c.to_owned()).collect();
    stream.push('\0' as u8);
    stream.push('\0' as u8);

    let mut ptr: &[u8] = &stream;
    loop {
        let res: IResult<&[u8], Token> = read_token(ptr);

        match res {
            Ok((rem, token)) => {
                tokens.push(token.clone());
                if token == Token::Eof { break; }
                ptr = rem;
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
            LitString("Hello World".to_string())
        );
    }

    #[test]
    fn check_multiline_string() {
        assert_ok!(
            read_multiline_string(b"\"\"\"Hello\nWorld\"\"\""),
            LitString("Hello\nWorld".to_string())
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
        assert_ok!(read_binop(b"= "), Equals);
        assert_ok!(read_binop(b"== "), BinaryOperator("==".to_string()));
        assert_ok!(read_binop(b"=== "), BinaryOperator("===".to_string()));
        assert_ok!(read_binop(b"- "), BinaryOperator("-".to_string()));
        assert_ok!(read_binop(b"-- "), BinaryOperator("--".to_string()));
        assert_ok!(read_binop(b"--- "), BinaryOperator("---".to_string()));
        assert_ok!(read_binop(b". "), Dot);
        assert_ok!(read_binop(b".. "), DoubleDot);
        assert_ok!(read_binop(b"... "), BinaryOperator("...".to_string()));
        assert_ok!(read_binop(b"-> "), RightArrow);
        assert_ok!(read_binop(b"<- "), LeftArrow);
        assert_ok!(read_binop(b"<-- "), BinaryOperator("<--".to_string()));
    }
}
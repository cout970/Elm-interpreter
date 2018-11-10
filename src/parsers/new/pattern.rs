use ast::Literal;
use ast::Pattern;
use parsers::new::Input;
use parsers::new::ParseError;
use parsers::new::util::comma0;
use parsers::new::util::expect;
use parsers::new::util::expect_id;
use parsers::new::util::many0;
use tokenizer::Token;

pub fn parse_pattern(input: Input) -> Result<(Pattern, Input), ParseError> {
    let (patt, i) = match input.read() {
        Token::Id(name) => (Pattern::Var(name.to_owned()), input.next()),
        Token::UpperId(name) => {
            let (params, i) = many0(&parse_pattern, input.next())?;
            (Pattern::Adt(name.to_owned(), params), i)
        }
        Token::LeftParen => {
            // Unit => ()
            // Tuple => (a,) (a, b,) (a, b, c)

            let input = input.next();
            match input.read() {
                Token::RightParen => {
                    (Pattern::Unit, input.next())
                }
                _ => {
                    let (values, i) = comma0(&parse_pattern, input)?;
                    let i = expect(Token::RightParen, i)?;
                    (Pattern::Tuple(values), i)
                }
            }
        }
        Token::LeftBracket => {
            let (values, i) = comma0(&parse_pattern, input.next())?;
            let i = expect(Token::RightBracket, i)?;
            (Pattern::List(values), i)
        }
        Token::LeftBrace => {
            let (values, i) = comma0(&expect_id, input.next())?;
            let i = expect(Token::RightBrace, i)?;
            (Pattern::Record(values), i)
        }
        Token::Underscore => (Pattern::Wildcard, input.next()),
        Token::LitInt(value) => (Pattern::Literal(Literal::Int(value)), input.next()),
        Token::LitFloat(value) => (Pattern::Literal(Literal::Float(value)), input.next()),
        Token::LitChar(value) => (Pattern::Literal(Literal::Char(value)), input.next()),
        Token::LitString(value) => (Pattern::Literal(Literal::String(value)), input.next()),
        _ => {
            let found = input.read();
            return Err(ParseError::UnmatchedToken { input, found, options: vec![] });
        }
    };

    if let Token::BinaryOperator(op) = i.read() {
        let (patt2, i) = parse_pattern(i.next())?;

        return Ok((Pattern::BinaryOp(op, Box::from(patt), Box::from(patt2)), i));
    }

    Ok((patt, i))
}


#[cfg(test)]
mod tests {
    use parsers::new::util::test_parser;
    use parsers::new::util::test_parser_error;

    use super::*;

    #[test]
    fn pattern_test() {
        test_parser(parse_pattern, "a");
        test_parser(parse_pattern, "A");
        test_parser(parse_pattern, "A a");
        test_parser(parse_pattern, "()");
        test_parser(parse_pattern, "(a)");
        test_parser(parse_pattern, "(a, a)");
        test_parser(parse_pattern, "(a, a, a)");
        test_parser(parse_pattern, "A ()");
        test_parser(parse_pattern, "_");
        test_parser(parse_pattern, "[]");
        test_parser(parse_pattern, "[a]");
        test_parser(parse_pattern, "x::xs");
        test_parser(parse_pattern, "[x::xs]");
        test_parser(parse_pattern, "{}");
        test_parser(parse_pattern, "{ x }");
        test_parser(parse_pattern, "{ x, y }");
    }

    #[test]
    fn pattern_error_test() {
        test_parser_error(parse_pattern, "(a,)");
        test_parser_error(parse_pattern, "(a, a,)");
        test_parser_error(parse_pattern, "[a,]");
        test_parser_error(parse_pattern, "{ x, y, }");
    }
}

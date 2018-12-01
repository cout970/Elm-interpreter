use ast::Pattern;
use parsers::parser::Input;
use parsers::parser::ParseError;
use parsers::util::comma0;
use parsers::util::expect;
use parsers::util::expect_id;
use parsers::util::many0;
use tokenizer::Token;
use util::create_vec;

pub fn parse_pattern_expr(input: Input) -> Result<(Pattern, Input), ParseError> {
    let (mut patt, mut i) = parse_pattern(input)?;

    if let Token::BinaryOperator(_) = i.read() {
        while let Token::BinaryOperator(op) = i.read() {
            let (patt2, rest) = parse_pattern(i.next())?;
            patt = Pattern::BinaryOp(op, Box::from(patt), Box::from(patt2));
            i = rest;
        }
    } else if let Token::As = i.read() {
        let (alias, i) = expect_id(i.next())?;
        return Ok((Pattern::Alias(Box::from(patt), alias), i));
    }

    Ok((patt, i))
}

pub fn parse_pattern(input: Input) -> Result<(Pattern, Input), ParseError> {
    let (patt, i) = match input.read() {
        Token::Id(name) => (Pattern::Var(name.to_owned()), input.next()),
        Token::UpperId(name) => {
            let (params, i) = many0(&parse_pattern, input.next())?;
            (Pattern::Adt(name.to_owned(), params), i)
        }
        Token::LeftParen => {
            // Unit => ()
            // Parens => (a)
            // Tuple => (a,) (a, b,) (a, b, c)

            let input = input.next();
            match input.read() {
                Token::RightParen => {
                    // Unit ()
                    (Pattern::Unit, input.next())
                }
                _ => {
                    let (first, i) = parse_pattern_expr(input)?;
                    match i.read() {
                        Token::RightParen => {
                            // Parens (a)
                            (first, i.next())
                        }
                        _ => {
                            // Tuple (a,)
                            let i = expect(Token::Comma, i)?;
                            let (rest, i) = comma0(&parse_pattern_expr, i)?;
                            let i = expect(Token::RightParen, i)?;
                            (Pattern::Tuple(create_vec(first, rest)), i)
                        }
                    }
                }
            }
        }
        Token::LeftBracket => {
            let (values, i) = comma0(&parse_pattern_expr, input.next())?;
            let i = expect(Token::RightBracket, i)?;
            (Pattern::List(values), i)
        }
        Token::LeftBrace => {
            let (values, i) = comma0(&expect_id, input.next())?;
            let i = expect(Token::RightBrace, i)?;
            (Pattern::Record(values), i)
        }
        Token::Underscore => (Pattern::Wildcard, input.next()),
        Token::LitInt(value) => (Pattern::LitInt(value), input.next()),
        Token::LitChar(value) => (Pattern::LitChar(value), input.next()),
        Token::LitString(value) => (Pattern::LitString(value), input.next()),
        _ => {
            let found = input.read();
            return Err(ParseError::UnmatchedToken { input, found, options: vec![] });
        }
    };

    Ok((patt, i))
}


#[cfg(test)]
mod tests {
    use parsers::util::test_parser;
    use parsers::util::test_parser_error;
    use parsers::util::test_parser_result;
    use util::StringConversion;

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
        test_parser(parse_pattern, "(x::xs)");
        test_parser(parse_pattern, "[x::xs]");
        test_parser(parse_pattern, "{}");
        test_parser(parse_pattern, "{ x }");
        test_parser(parse_pattern, "{ x, y }");
        test_parser(parse_pattern, "Leaf _");
    }

    #[test]
    fn pattern_error_test() {
//        test_parser_error(parse_pattern, "(a,)");
        test_parser_error(parse_pattern, "(a, a,)");
        test_parser_error(parse_pattern, "[a,]");
        test_parser_error(parse_pattern, "{ x, y, }");
    }


    #[test]
    fn check_literal() {
        test_parser_result(parse_pattern, "1", Pattern::LitInt(1));
    }

    #[test]
    fn check_variable() {
        test_parser_result(parse_pattern, "variable", Pattern::Var("variable".s()));
    }

    #[test]
    fn check_algebraic_data_type() {
        test_parser_result(parse_pattern, "List a", Pattern::Adt(
            "List".s(),
            vec![Pattern::Var("a".s())],
        ));
    }

    #[test]
    fn check_wildcard() {
        test_parser_result(parse_pattern, "_", Pattern::Wildcard);
    }

    #[test]
    fn check_unit() {
        test_parser_result(parse_pattern, "()", Pattern::Unit);
    }

    #[test]
    fn check_tuple() {
        test_parser_result(parse_pattern, "(a, b)", Pattern::Tuple(vec![
            Pattern::Var("a".s()), Pattern::Var("b".s())
        ]));
    }

    #[test]
    fn check_empty_list() {
        test_parser_result(parse_pattern, "[]", Pattern::List(vec![]));
    }

    #[test]
    fn check_list() {
        test_parser_result(parse_pattern, "[a, b]", Pattern::List(vec![
            Pattern::Var("a".s()), Pattern::Var("b".s())
        ]));
    }

    #[test]
    fn check_record() {
        test_parser_result(parse_pattern, "{ a, b }", Pattern::Record(
            vec!["a".s(), "b".s()]
        ));
    }
}


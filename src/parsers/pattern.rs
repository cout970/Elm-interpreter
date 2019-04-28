use ast::Pattern;
use parsers::input::Input;
use parsers::ParseError;
use parsers::util::comma0;
use parsers::util::expect;
use parsers::util::expect_id;
use parsers::util::many0;
use tokenizer::Token;
use util::create_vec;

pub fn parse_pattern_expr(input: Input) -> Result<(Pattern, Input), ParseError> {
    let start = input.pos();
    let (mut patt, mut i) = parse_pattern(input)?;

    if let Token::BinaryOperator(_) = i.read() {
        let mut chain: Vec<(Pattern, String)> = vec![];
        let mut last_patt = patt;

        // Read chain of a::(b::(c::(d)))
        while let Token::BinaryOperator(op) = i.read() {
            let (patt2, rest) = parse_pattern(i.next())?;
            chain.push((last_patt, op));
            last_patt = patt2;
            i = rest;
        }

        patt = chain.into_iter().rev().fold(last_patt, |accum, (p, op)| {
            Pattern::BinaryOp((p.get_span().0, accum.get_span().1), op, Box::from(p), Box::from(accum))
        });
    } else if let Token::As = i.read() {
        let (alias, i) = expect_id(i.next())?;
        return Ok((Pattern::Alias((start, i.pos_end()), Box::from(patt), alias), i));
    }

    Ok((patt, i))
}

pub fn parse_pattern(input: Input) -> Result<(Pattern, Input), ParseError> {
    parse_pattern_helper(input, true)
}

pub fn parse_pattern_without_adt(input: Input) -> Result<(Pattern, Input), ParseError> {
    parse_pattern_helper(input, false)
}

fn parse_pattern_helper(input: Input, adt: bool) -> Result<(Pattern, Input), ParseError> {
    let start = input.pos();
    let (patt, i) = match input.read() {
        Token::Id(name) => (Pattern::Var(input.span(), name.to_owned()), input.next()),
        Token::UpperId(name) => {
            if adt {
                let (params, i) = many0(&parse_pattern_without_adt, input.next())?;
                (Pattern::Adt((start, i.pos_end()), name.to_owned(), params), i)
            } else {
                (Pattern::Adt(input.span(), name.to_owned(), vec![]), input.next())
            }
        }
        Token::LeftParen => {
            // Unit => ()
            // Parens => (a)
            // Tuple => (a,) (a, b,) (a, b, c)

            let input = input.next();
            match input.read() {
                Token::RightParen => {
                    // Unit ()
                    (Pattern::Unit((start, start + 2)), input.next())
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
                            (Pattern::Tuple((start, i.pos_end()), create_vec(first, rest)), i)
                        }
                    }
                }
            }
        }
        Token::LeftBracket => {
            let (values, i) = comma0(&parse_pattern_expr, input.next())?;
            let i = expect(Token::RightBracket, i)?;
            (Pattern::List((start, i.pos_end()), values), i)
        }
        Token::LeftBrace => {
            let (values, i) = comma0(&expect_id, input.next())?;
            let i = expect(Token::RightBrace, i)?;
            (Pattern::Record((start, i.pos_end()), values), i)
        }
        Token::Underscore => (Pattern::Wildcard((start, input.next().pos_end())), input.next()),
        Token::LitInt(value) => (Pattern::LitInt((start, input.next().pos_end()), value), input.next()),
        Token::LitChar(value) => (Pattern::LitChar((start, input.next().pos_end()), value), input.next()),
        Token::LitString(value) => (Pattern::LitString((start, input.next().pos_end()), value), input.next()),
        _ => {
            let found = input.read();
            return Err(ParseError::UnmatchedToken { span: input.span(), found, options: vec![] });
        }
    };

    Ok((patt, i))
}


#[cfg(test)]
mod tests {
    use parsers::util::test_utils::*;
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
        test_parser_result(parse_pattern, "1", Pattern::LitInt((0, 1), 1));
    }

    #[test]
    fn check_variable() {
        test_parser_result(parse_pattern, "variable", Pattern::Var((0, 0), "variable".s()));
    }

    #[test]
    fn check_algebraic_data_type() {
        test_parser_result(parse_pattern, "List a", Pattern::Adt(
            (0, 0),
            "List".s(),
            vec![Pattern::Var((0, 0), "a".s())],
        ));
    }

    #[test]
    fn check_wildcard() {
        test_parser_result(parse_pattern, "_", Pattern::Wildcard((0, 0)));
    }

    #[test]
    fn check_unit() {
        test_parser_result(parse_pattern, "()", Pattern::Unit((0, 0)));
    }

    #[test]
    fn check_tuple() {
        test_parser_result(parse_pattern, "(a, b)", Pattern::Tuple(
            (0, 0), vec![
                Pattern::Var((0, 0), "a".s()), Pattern::Var((0, 0), "b".s())
            ]));
    }

    #[test]
    fn check_empty_list() {
        test_parser_result(parse_pattern, "[]", Pattern::List((0, 0), vec![]));
    }

    #[test]
    fn check_list() {
        test_parser_result(parse_pattern, "[a, b]", Pattern::List(
            (0, 0), vec![
                Pattern::Var((0, 0), "a".s()), Pattern::Var((0, 0), "b".s())
            ]));
    }

    #[test]
    fn check_record() {
        test_parser_result(parse_pattern, "{ a, b }", Pattern::Record(
            (0, 0),
            vec!["a".s(), "b".s()],
        ));
    }

    #[test]
    fn check_operator_associativity() {
        test_parser_result(parse_pattern_expr, "a::b::c::d", Pattern::BinaryOp(
            (0, 0),
            "::".s(),
            Box::from(Pattern::Var((0, 0), "a".s())),
            Box::from(Pattern::BinaryOp(
                (0, 0),
                "::".s(),
                Box::from(Pattern::Var((0, 0), "b".s())),
                Box::from(Pattern::BinaryOp(
                    (0, 0),
                    "::".s(),
                    Box::from(Pattern::Var((0, 0), "c".s())),
                    Box::from(Pattern::Var((0, 0), "d".s())),
                )),
            )),
        ));
    }
}


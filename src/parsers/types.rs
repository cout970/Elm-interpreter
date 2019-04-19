use ast::Type;
use parsers::input::Input;
use parsers::ParseError;
use parsers::util::comma0;
use parsers::util::expect;
use parsers::util::expect_id;
use parsers::util::expect_upper;
use parsers::util::many0;
use parsers::util::optional_tk;
use tokenizer::Token;
use util::create_vec;
use util::qualified_name;
use util::uncons;

pub fn parse_type(input: Input) -> Result<(Type, Input), ParseError> {
    let (ty, i) = parse_type_with_adt(input)?;
    let (rest, i) = many0(&parse_type_chain, i)?;

    Ok((create_fun(ty, rest), i))
}

pub fn parse_type_without_adt(input: Input) -> Result<(Type, Input), ParseError> {
    parse_type_base(input, false)
}

pub fn parse_type_with_adt(input: Input) -> Result<(Type, Input), ParseError> {
    parse_type_base(input, true)
}

pub fn parse_type_base(input: Input, adt: bool) -> Result<(Type, Input), ParseError> {
    let (ty, i) = match input.read() {
        Token::Id(name) => (Type::Var(name.to_owned()), input.next()),
        Token::UpperId(name) => {
            let i = input.next();
            let (name, i) = match i.read() {
                Token::Dot => {

                    let (second, mut i) = expect_upper(i.next())?;
                    let mut accum = vec![second];

                    while let Token::Dot = i.read() {
                        let (next, rest) = expect_upper(i.next())?;
                        accum.push(next);
                        i = rest;
                    }

                    let names = create_vec(name.to_owned(), accum);
                    let (list, last) = uncons(names);
                    (qualified_name(&list, &last), i)
                }
                _ => (name.to_owned(), i)
            };

            if adt {
                let (params, i) = many0(&parse_type_with_adt, i)?;
                (Type::Tag(name, params), i)
            } else {
                (Type::Tag(name, vec![]), i)
            }
        }
        Token::LeftParen => {
            // () => Unit
            // (a) => Paren
            // (a,) (a, b,) (a, b, c) => Tuple

            let input = input.next();
            match input.read() {
                Token::RightParen => {
                    (Type::Unit, input.next())
                }
                _ => {
                    let (first, i) = parse_type(input)?;
                    match i.read() {
                        Token::RightParen => {
                            (first, i.next())
                        }
                        _ => {
                            let i = expect(Token::Comma, i)?;
                            let (rest, i) = comma0(&parse_type, i)?;
                            let i = expect(Token::RightParen, i)?;
                            (Type::Tuple(create_vec(first, rest)), i)
                        }
                    }
                }
            }
        }
        Token::LeftBrace => {
            // Options
            // { x : Int } => Record
            // { a | x : Int } => Record Update

            let i = input.next();
            match i.read() {
                Token::RightBrace => {
                    // {}
                    (Type::Record(vec![]), i.next())
                }
                Token::Id(name) => {
                    let i = i.next();
                    match i.read() {
                        Token::Colon => {
                            // { x : Int }

                            let (expr, i) = parse_type(i.next())?;
                            let i = optional_tk(Token::Comma, i);
                            let (values, i) = comma0(&parse_record_entry, i)?;
                            let i = expect(Token::RightBrace, i)?;
                            (Type::Record(create_vec((name, expr), values)), i)
                        }
                        Token::Pipe => {
                            // { a | x : Int }
                            let (values, i) = comma0(&parse_record_entry, i.next())?;
                            let i = expect(Token::RightBrace, i)?;
                            (Type::RecExt(name, values), i)
                        }
                        _ => {
                            let input = i;
                            let found = input.read();
                            return Err(ParseError::UnmatchedToken { span: input.span(), found, options: vec![Token::Equals, Token::Pipe] });
                        }
                    }
                }
                _ => {
                    let input = i;
                    let found = input.read();
                    return Err(ParseError::UnmatchedToken { span: input.span(), found, options: vec![Token::RightBrace, Token::Id(String::from("variable"))] });
                }
            }
        }
        _ => {
            let found = input.read();
            return Err(ParseError::UnmatchedToken { span: input.span(), found, options: vec![] });
        }
    };

    Ok((ty, i))
}

fn parse_record_entry(input: Input) -> Result<((String, Type), Input), ParseError> {
    let (name, i) = expect_id(input)?;
    let i = expect(Token::Colon, i)?;
    let (ty, i) = parse_type(i)?;

    Ok(((name, ty), i))
}

fn parse_type_chain(input: Input) -> Result<(Type, Input), ParseError> {
    let i = expect(Token::RightArrow, input)?;
    let (ty, i) = parse_type(i)?;

    Ok((ty, i))
}

fn create_fun(a: Type, b: Vec<Type>) -> Type {
    if b.is_empty() { return a; }

    let c = create_vec(a, b);
    let mut iter = c.into_iter().rev();

    iter.next().map(|first| iter.fold(first, |acc, b|
        Type::Fun(Box::new(b), Box::new(acc)),
    )).unwrap()
}


#[cfg(test)]
mod tests {
    use parsers::util::test_utils::*;
    use util::StringConversion;

    use super::*;

    #[test]
    fn expr_test() {
        test_parser(parse_type, "Int");
        test_parser(parse_type, "()");
        test_parser(parse_type, "(Int)");
        test_parser(parse_type, "(Int, Int)");
        test_parser(parse_type, "{}");
        test_parser(parse_type, "{ x: Int }");
        test_parser(parse_type, "{ x: Int, y: Int }");
        test_parser(parse_type, "List a");
        test_parser(parse_type, "List (Int, a)");
        test_parser(parse_type, "Int -> Int");
        test_parser(parse_type, "Int -> Int -> (Int, Int)");
    }

    #[test]
    fn expr_error_test() {
        test_parser_error(parse_type, "{ Int : x }");
        test_parser_error(parse_type, "Int ->");
        test_parser_error(parse_type, "-> Int");
    }

    #[test]
    fn check_maybe_priority() {
        let expected = Type::Fun(
            Box::from(Type::Tag("Maybe".s(), vec![Type::Var("a".s())])),
            Box::from(Type::Fun(
                Box::from(Type::Tag("Maybe".s(), vec![Type::Var("b".s())])),
                Box::from(Type::Tag("Maybe".s(), vec![Type::Var("value".s())])),
            ))
        );

        test_parser_result(parse_type, "Maybe a -> Maybe b -> Maybe value", expected);
    }

    #[test]
    fn check_unit() {
        test_parser_result(parse_type, "()", Type::Unit);
    }

    #[test]
    fn check_variable() {
        test_parser_result(parse_type, "a", Type::Var("a".s()));
    }

    #[test]
    fn check_tag() {
        test_parser_result(parse_type, "List a", Type::Tag(
            "List".s(),
            vec![Type::Var("a".s())],
        ));
    }

    #[test]
    fn check_tuple2() {
        test_parser_result(parse_type, "(a,b)", Type::Tuple(vec![
            Type::Var("a".s()), Type::Var("b".s())
        ]));
    }

    #[test]
    fn check_tuple6() {
        test_parser_result(parse_type, "(a,b,c,d,e,f)", Type::Tuple(vec![
            Type::Var("a".s()),
            Type::Var("b".s()),
            Type::Var("c".s()),
            Type::Var("d".s()),
            Type::Var("e".s()),
            Type::Var("f".s()),
        ]));
    }

    #[test]
    fn check_empty_record() {
        test_parser_result(parse_type, "{}", Type::Record(vec![]));
    }

    #[test]
    fn check_record() {
        test_parser_result(parse_type, "{ a: b }", Type::Record(
            vec![("a".s(), Type::Var("b".s()))]
        ));
    }

    #[test]
    fn check_ext_record() {
        test_parser_result(parse_type, "{ list | a: b }", Type::RecExt(
            "list".s(),
            vec![("a".s(), Type::Var("b".s()))],
        ));
    }

    #[test]
    fn check_paren() {
        test_parser_result(parse_type, "(a)", Type::Var("a".s()));
    }

    #[test]
    fn check_function() {
        test_parser_result(parse_type, "Int -> Float -> a", Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Fun(
                Box::new(Type::Tag("Float".s(), vec![])),
                Box::new(Type::Var("a".s())),
            )),
        ));
    }
}

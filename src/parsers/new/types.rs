use ast::Type;
use parsers::new::Input;
use parsers::new::ParseError;
use parsers::new::util::comma0;
use parsers::new::util::expect;
use parsers::new::util::expect_id;
use parsers::new::util::many0;
use tokenizer::Token;
use parsers::new::util::optional_tk;
use util::create_vec;

pub fn parse_type(input: Input) -> Result<(Type, Input), ParseError> {
    let (ty, i) = parse_type_base(input)?;
    let (rest, i) = many0(&parse_type_chain, i)?;

    Ok((create_fun(ty, rest), i))
}
pub fn parse_type_base(input: Input) -> Result<(Type, Input), ParseError> {
    let (ty, i) = match input.read() {
        Token::Id(name) => (Type::Var(name.to_owned()), input.next()),
        Token::UpperId(name) => {
            let (params, i) = many0(&parse_type, input.next())?;
            (Type::Tag(name.to_owned(), params), i)
        }
        Token::LeftParen => {
            // Unit => ()
            // Tuple => (a,) (a, b,) (a, b, c)

            let input = input.next();
            match input.read() {
                Token::RightParen => {
                    (Type::Unit, input.next())
                }
                _ => {
                    let (values, i) = comma0(&parse_type, input)?;
                    let i = expect(Token::RightParen, i)?;
                    (Type::Tuple(values), i)
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
                            return Err(ParseError::UnmatchedToken { input, found, options: vec![Token::Equals, Token::Pipe] });
                        }
                    }
                }
                _ => {
                    let input = i;
                    let found = input.read();
                    return Err(ParseError::UnmatchedToken { input, found, options: vec![Token::RightBrace, Token::Id(String::from("variable"))] });
                }
            }
        }
        _ => {
            let found = input.read();
            return Err(ParseError::UnmatchedToken { input, found, options: vec![] });
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
        Type::Fun(Box::new(b), Box::new(acc))
    )).unwrap()
}


#[cfg(test)]
mod tests {
    use parsers::new::util::test_parser;
    use parsers::new::util::test_parser_error;
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
}
use ast::Definition;
use ast::Expr;
use ast::Literal;
use ast::Pattern;
use parsers::new::Input;
use parsers::new::ParseError;
use parsers::new::pattern::parse_pattern;
use parsers::new::statement::parse_definition;
use parsers::new::util::comma0;
use parsers::new::util::expect;
use parsers::new::util::expect_binop;
use parsers::new::util::expect_id;
use parsers::new::util::expect_indent;
use parsers::new::util::expect_upper;
use parsers::new::util::many0;
use parsers::new::util::many1;
use parsers::new::util::optional_tk;
use parsers::new::util::read_indent;
use tokenizer::Token;
use util::create_vec;

pub fn parse_expr(input: Input) -> Result<(Expr, Input), ParseError> {
    let (first, i) = parse_expr_application(input)?;
    let (rest, i) = many0(&binop_expr, i)?;

    Ok((create_binop_chain(first, rest), i))
}

fn parse_expr_application(input: Input) -> Result<(Expr, Input), ParseError> {
    let (exprs, i): (Vec<Expr>, Input) = many1(&parse_expr_base, input)?;
    let mut iter = exprs.into_iter();
    let first = iter.next().unwrap();
    let tree = iter.fold(first, |acc, b| Expr::Application(Box::new(acc), Box::new(b)));

    Ok((tree, i))
}

fn parse_expr_base(input: Input) -> Result<(Expr, Input), ParseError> {
    let (expr, i) = match input.read() {
        Token::LitInt(value) => (Expr::Literal(Literal::Int(value)), input.next()),
        Token::LitFloat(value) => (Expr::Literal(Literal::Float(value)), input.next()),
        Token::LitChar(value) => (Expr::Literal(Literal::Char(value)), input.next()),
        Token::LitString(value) => (Expr::Literal(Literal::String(value)), input.next()),
        Token::Id(name) => (Expr::Ref(name), input.next()),
        Token::Dot => {
            let (name, i) = expect_id(input.next())?;
            (Expr::RecordAccess(name), i)
        }
        Token::UpperId(first) => {
            // Options:
            // True => Ref
            // List.map => QualifiedRef

            let i = input.next();
            match i.read() {
                Token::Dot => {
                    // Parsed: Upper.
                    let (rest, i) = many0(&parse_dot_name, i)?;
                    // Parsed: Upper.A.B.C
                    let i = expect(Token::Dot, i)?;
                    let (name, i) = expect_id(i)?;
                    // Parsed: Upper.A.B.C.func
                    (Expr::QualifiedRef(create_vec(first, rest), name), i)
                }
                _ => {
                    (Expr::Ref(first), input.next())
                }
            }
        }
        Token::If => {
            let (condition, i) = parse_expr(input.next())?;
            let i = expect(Token::Then, i)?;
            let (true_branch, i) = parse_expr(i)?;
            let i = expect(Token::Else, i)?;
            let (false_branch, i) = parse_expr(i)?;

            (Expr::If(Box::from(condition), Box::from(true_branch), Box::from(false_branch)), i)
        }
        Token::LeftParen => {
            // Options:
            // () => Unit
            // (1) => Literal
            // (1, 2) => Tuple

            let i = input.next();
            match i.read() {
                Token::RightParen => {
                    // ()
                    (Expr::Unit, i.next())
                }
                _ => {
                    let (value, i) = parse_expr(i)?;
                    match i.read() {
                        Token::RightParen => {
                            // (1)
                            (value, i.next())
                        }
                        _ => {
                            // (1, 2)
                            let i = expect(Token::Comma, i)?;
                            let (rest, i) = comma0(&parse_expr, i)?;
                            let i = expect(Token::RightParen, i)?;
                            (Expr::Tuple(create_vec(value, rest)), i)
                        }
                    }
                }
            }
        }
        Token::LeftBracket => {
            let i = input.next();
            let (values, i) = comma0(&parse_expr, i)?;
            let i = expect(Token::RightBracket, i)?;
            (Expr::List(values), i)
        }
        Token::LeftBrace => {
            // Options
            // { x = 0 } => Record
            // { a | x = 0 } => Record Update

            let i = input.next();
            match i.read() {
                Token::RightBrace => {
                    // {}
                    (Expr::Record(vec![]), i.next())
                }
                Token::Id(name) => {
                    let i = i.next();
                    match i.read() {
                        Token::Equals => {
                            // { x = 0 }
                            let (expr, i) = parse_expr(i.next())?;
                            let i = optional_tk(Token::Comma, i);
                            let (values, i) = comma0(&parse_record_entry, i)?;
                            let i = expect(Token::RightBrace, i)?;
                            (Expr::Record(create_vec((name, expr), values)), i)
                        }
                        Token::Pipe => {
                            // { a | x = 0}
                            let (values, i) = comma0(&parse_record_entry, i.next())?;
                            let i = expect(Token::RightBrace, i)?;
                            (Expr::RecordUpdate(name, values), i)
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
        Token::BackSlash => {
            let (pats, i) = many1(&parse_pattern, input.next())?;
            let i = expect(Token::RightArrow, i)?;
            let (expr, i) = parse_expr(i)?;

            (Expr::Lambda(pats, Box::from(expr)), i)
        }
        Token::Let => {
            let i = input.next();
            let level = read_indent(i.clone())?;

            let i = i.enter_level(level);

            let (defs, i) = many1(&|i| {
                parse_definition(level, expect_indent(level, i)?)
            }, i)?;

            let i = i.exit_level(level);

            let i = expect(Token::In, i)?;
            let (expr, i) = parse_expr(i)?;

            (Expr::Let(defs, Box::from(expr)), i)
        }
        Token::Case => {
            let (cond, i) = parse_expr(input.next())?;
            let i = expect(Token::Of, i)?;
            let level = read_indent(i.clone())?;
            let i = i.enter_level(level);
            let (branches, i) = many1(&|i| parse_case_branch(level, i), i)?;
            let i = i.exit_level(level);
            (Expr::Case(Box::from(cond), branches), i)
        }
        Token::PrefixMinus => {
            let (expr, i) = parse_expr(input.next())?;
            (Expr::Application(Box::from(Expr::Ref(String::from("-"))), Box::from(expr)), i)
        }
        _ => {
            let found = input.read();
            return Err(ParseError::UnmatchedToken { input, found, options: vec![] });
        }
    };

    // Check for trailing record access `.x`
    // TODO repeat? `{ x = { y = 0 } }.x.y`
    let (expr, i) = if i.read() == Token::Dot {
        let (name, i) = expect_id(i.next())?;
        (Expr::RecordField(Box::from(expr), name), i)
    } else {
        (expr, i)
    };


    Ok((expr, i))
}

fn parse_case_branch(indent: u32, input: Input) -> Result<((Pattern, Expr), Input), ParseError> {
    let i = expect_indent(indent, input)?;
    let (pat, i) = parse_pattern(i)?;
    let i = expect(Token::RightArrow, i)?;
    let (expr, i) = parse_expr(i)?;

    Ok(((pat, expr), i))
}

fn parse_dot_name(input: Input) -> Result<(String, Input), ParseError> {
    let i = expect(Token::Dot, input)?;
    let (name, i) = expect_upper(i)?;

    Ok((name, i))
}

fn parse_record_entry(input: Input) -> Result<((String, Expr), Input), ParseError> {
    let (id, i) = expect_id(input)?;
    let i = expect(Token::Equals, i)?;
    let (expr, i) = parse_expr(i)?;

    Ok(((id, expr), i))
}

fn binop_expr(input: Input) -> Result<((String, Expr), Input), ParseError> {
    let (op, i) = expect_binop(input)?;
    let (expr, i) = parse_expr(i)?;

    Ok(((op, expr), i))
}

fn create_binop_chain(first: Expr, rest: Vec<(String, Expr)>) -> Expr {
    if rest.is_empty() { return first; }

    let mut exprs = Vec::new();
    let mut ops = Vec::new();

    exprs.push(first);

    for (op, expr) in rest {
        ops.push(op);
        exprs.push(expr);
    }

    Expr::OpChain(exprs, ops)
}


#[cfg(test)]
mod tests {
    use parsers::new::util::test_parser;
    use parsers::new::util::test_parser_error;
    use super::*;

    #[test]
    fn expr_test() {
        test_parser(parse_expr, "123");
        test_parser(parse_expr, "123.123");
        test_parser(parse_expr, "'a'");
        test_parser(parse_expr, "\"Hello World\"");
        test_parser(parse_expr, "if 1 then 2 else 3");
        test_parser(parse_expr, "()");
        test_parser(parse_expr, "(1)");
        test_parser(parse_expr, "(1,)");
        test_parser(parse_expr, "(1,2)");
        test_parser(parse_expr, "(1,2,)");
        test_parser(parse_expr, "[]");
        test_parser(parse_expr, "[1]");
        test_parser(parse_expr, "[1,]");
        test_parser(parse_expr, "[1,2]");
        test_parser(parse_expr, "[1,2,]");
        test_parser(parse_expr, "{}");
        test_parser(parse_expr, "{ x = 1 }");
        test_parser(parse_expr, "{ x = 1, }");
        test_parser(parse_expr, "{ x = 1, y = 0 }");
        test_parser(parse_expr, "{ x = 1, y = 0, }");
        test_parser(parse_expr, "a");
        test_parser(parse_expr, "my_var_name123");
        test_parser(parse_expr, "True");
        test_parser(parse_expr, "{ a | x = 0 }");
        test_parser(parse_expr, "{ a | x = 0, }");
        test_parser(parse_expr, "{ a | x = 0, y = 1 }");
        test_parser(parse_expr, "{ a | x = 0, y = 1, }");
        test_parser(parse_expr, "List");
        test_parser(parse_expr, "List.map");
        test_parser(parse_expr, "List.A.B.C.a");
        test_parser(parse_expr, "{ x = 0 }.x");
        test_parser(parse_expr, ".x");
        test_parser(parse_expr, "\\x -> 1");
        test_parser(parse_expr, "\\x y z -> [x, y, z]");
        test_parser(parse_expr, "case myList of\n [] -> 0\n _ -> 1");
        test_parser(parse_expr, "sum 1 2");
        test_parser(parse_expr, "1 + 2");
        test_parser(parse_expr, "-42");
        test_parser(parse_expr, "-(1+2)");
    }

    #[test]
    fn expr_error_test() {
        test_parser_error(parse_expr, "123.123.1");
        test_parser_error(parse_expr, "if 1 then 2");
        test_parser_error(parse_expr, "(");
        test_parser_error(parse_expr, ")");
        test_parser_error(parse_expr, "(,)");
        test_parser_error(parse_expr, "case myList of\n  [] -> 0\n   _ -> 1");
        test_parser_error(parse_expr, "+1");
        test_parser_error(parse_expr, "1 +");
        test_parser_error(parse_expr, "1 + 2 +");
        test_parser_error(parse_expr, "- 42");
        test_parser_error(parse_expr, "- (1+2)");
    }
}

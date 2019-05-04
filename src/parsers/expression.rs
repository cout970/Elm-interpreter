use ast::Expr;
use ast::LetDeclaration;
use ast::Literal;
use ast::Pattern;
use ast::span;
use parsers::input::Input;
use parsers::ParseError;
use parsers::pattern::parse_pattern;
use parsers::pattern::parse_pattern_expr;
use parsers::statement::parse_definition;
use parsers::util::*;
use tokenizer::Token;
use util::create_vec;

pub fn parse_expr(input: Input) -> Result<(Expr, Input), ParseError> {
    let (first, i) = parse_expr_application(input)?;
    let (rest, i) = many0(&binop_expr, i)?;

    Ok((create_binop_chain(first, rest), i))
}

fn parse_expr_application(input: Input) -> Result<(Expr, Input), ParseError> {
    let start = input.pos();
    let (exprs, i): (Vec<Expr>, Input) = many1(&parse_expr_base, input)?;
    let end = i.pos_end();
    let mut iter = exprs.into_iter();
    let first = iter.next().unwrap();
    let tree = iter.fold(first, |acc, b| Expr::Application((start, end), Box::new(acc), Box::new(b)));

    Ok((tree, i))
}

fn parse_expr_base(input: Input) -> Result<(Expr, Input), ParseError> {
    let (expr, i) = match input.read() {
        Token::LitInt(value) => (Expr::Literal(input.span(), Literal::Int(value)), input.next()),
        Token::LitFloat(value) => (Expr::Literal(input.span(), Literal::Float(value)), input.next()),
        Token::LitChar(value) => (Expr::Literal(input.span(), Literal::Char(value)), input.next()),
        Token::LitString(value) => (Expr::Literal(input.span(), Literal::String(value)), input.next()),
        Token::Id(name) => (Expr::Ref(input.span(), name), input.next()),
        Token::Dot => {
            let (name, i) = expect_id(input.next())?;
            (Expr::RecordAccess((input.pos(), i.pos()), name), i)
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
                    (Expr::QualifiedRef((input.pos(), i.pos()), create_vec(first, rest), name), i)
                }
                _ => {
                    (Expr::Ref((input.pos(), i.pos()), first), input.next())
                }
            }
        }
        Token::If => {
            let (condition, i) = parse_expr(input.next())?;
            let i = expect(Token::Then, i)?;
            let (true_branch, i) = parse_expr(i)?;
            let i = expect(Token::Else, i)?;
            let (false_branch, i) = parse_expr(i)?;

            (Expr::If((input.pos(), i.pos()), Box::from(condition), Box::from(true_branch), Box::from(false_branch)), i)
        }
        Token::LeftParen => {
            // Options:
            // () => Unit
            // (+) => Ref
            // (1) => Literal
            // (1, 2) => Tuple

            let i = input.next();
            match i.read() {
                Token::RightParen => {
                    // ()
                    let i = i.next();
                    (Expr::Unit((input.pos(), i.pos())), i)
                }
                Token::BinaryOperator(op) => {
                    // (+)
                    let i = expect(Token::RightParen, i.next())?;
                    (Expr::Ref((input.pos(), i.pos()), op), i)
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
                            let (rest, i) = comma1(&parse_expr, i)?;
                            let i = expect(Token::RightParen, i)?;
                            (Expr::Tuple((input.pos(), i.pos()), create_vec(value, rest)), i)
                        }
                    }
                }
            }
        }
        Token::LeftBracket => {
            let i = input.next();
            let (values, i) = comma0(&parse_expr, i)?;
            let i = expect(Token::RightBracket, i)?;
            (Expr::List((input.pos(), i.pos()), values), i)
        }
        Token::LeftBrace => {
            // Options
            // { x = 0 } => Record
            // { a | x = 0 } => Record Update

            let i = input.next();
            match i.read() {
                Token::RightBrace => {
                    // {}
                    let i = i.next();
                    (Expr::Record((input.pos(), i.pos()), vec![]), i)
                }
                Token::Id(name) => {
                    let i = i.next();
                    match i.read() {
                        Token::Equals => {
                            // { x = 0 }
                            let (expr, i) = parse_expr(i.next())?;
                            let (values, i) = many0(&parse_record_entry, i)?;

                            let i = expect(Token::RightBrace, i)?;
                            (Expr::Record((input.pos(), i.pos()), create_vec((name, expr), values)), i)
                        }
                        Token::Pipe => {
                            // { a | x = 0 }
                            let (first_name, i) = expect_id(i.next())?;
                            let i = expect(Token::Equals, i)?;
                            let (first_expr, i) = parse_expr(i)?;
                            let (values, i) = comma0(&parse_record_entry, i)?;

                            let i = expect(Token::RightBrace, i)?;
                            (Expr::RecordUpdate((input.pos(), i.pos()), name, create_vec((first_name, first_expr), values)), i)
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
        Token::BackSlash => {
            let (pats, i) = many1(&parse_pattern, input.next())?;
            let i = expect(Token::RightArrow, i)?;
            let (expr, i) = parse_expr(i)?;

            (Expr::Lambda((input.pos(), i.pos()), pats, Box::from(expr)), i)
        }
        Token::Let => {
            let i = input.next();
            let level = read_optional_indent(i.clone());

            let i = i.enter_level(level);

            let (defs, i) = many1(&|i| {
                parse_let_declaration(level, expect_indent(level, i)?)
            }, i)?;

            let i = i.exit_level(level);

            let i = expect(Token::In, i)?;
            let (expr, i) = parse_expr(i)?;

            (Expr::Let((input.pos(), i.pos()), defs, Box::from(expr)), i)
        }
        Token::Case => {
            let (cond, i) = parse_expr(input.next())?;
            let i = expect(Token::Of, i)?;
            let level = read_indent(i.clone())?;
            let i = i.enter_level(level);
            let (branches, i) = many1(&|i| parse_case_branch(level, i), i)?;
            let i = i.exit_level(level);
            (Expr::Case((input.pos(), i.pos()), Box::from(cond), branches), i)
        }
        Token::PrefixMinus => {
            let (expr, i) = parse_expr_base(input.next())?;
            (Expr::Application((input.pos(), i.pos()), Box::from(Expr::Ref((input.pos(), input.pos() + 1), String::from("__internal__minus"))), Box::from(expr)), i)
        }
        _ => {
            let found = input.read();
            return Err(ParseError::UnmatchedToken { span: input.span(), found, options: vec![] });
        }
    };

    // Check for trailing record access `.x`
    // TODO repeat? `{ x = { y = 0 } }.x.y`
    let (expr, i) = if i.read() == Token::Dot {
        let (name, i) = expect_id(i.next())?;
        (Expr::RecordField((input.pos(), i.pos()), Box::from(expr), name), i)
    } else {
        (expr, i)
    };


    Ok((expr, i))
}

fn parse_let_declaration(indent: u32, input: Input) -> Result<(LetDeclaration, Input), ParseError> {
    match input.read() {
        Token::Id(_) => {
            let (def, i) = parse_definition(indent, input)?;

            Ok((LetDeclaration::Def(def), i))
        }
        _ => {
            let (pat, i) = parse_pattern(input)?;
            let i = expect(Token::Equals, i)?;
            let (expr, i) = parse_expr(i)?;

            Ok((LetDeclaration::Pattern(pat, expr), i))
        }
    }
}

fn parse_case_branch(indent: u32, input: Input) -> Result<((Pattern, Expr), Input), ParseError> {
    let i = expect_indent(indent, input)?;
    let (pat, i) = parse_pattern_expr(i)?;
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
    let i = expect(Token::Comma, input)?;
    let (id, i) = expect_id(i)?;
    let i = expect(Token::Equals, i)?;
    let (expr, i) = parse_expr(i)?;

    Ok(((id, expr), i))
}

fn binop_expr(input: Input) -> Result<((String, Expr), Input), ParseError> {
    let (op, i) = expect_binop(input)?;
    let (expr, i) = parse_expr_application(i)?;

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

    Expr::OpChain((span(exprs.first().unwrap()).0, span(exprs.last().unwrap()).1), exprs, ops)
}


#[cfg(test)]
mod tests {
    use ast::Definition;
    use parsers::util::test_utils::*;
    use util::StringConversion;

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
        test_parser(parse_expr, "(1,2)");
        test_parser(parse_expr, "[]");
        test_parser(parse_expr, "[1]");
        test_parser(parse_expr, "[1,2]");
        test_parser(parse_expr, "{}");
        test_parser(parse_expr, "{ x = 1 }");
        test_parser(parse_expr, "{ x = 1, y = 0 }");
        test_parser(parse_expr, "a");
        test_parser(parse_expr, "my_var_name123");
        test_parser(parse_expr, "True");
        test_parser(parse_expr, "{ a | x = 0 }");
        test_parser(parse_expr, "{ a | x = 0, y = 1 }");
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
        test_parser(parse_expr, "let\n x = 0\n in\n x * x");
        test_parser(parse_expr, "let\n x = 10\n y = 5\n in\n x * y");
        test_parser(parse_expr, "let x = 0 in x * x");
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
        test_parser_error(parse_expr, "(1,)");
        test_parser_error(parse_expr, "(1,2,)");
        test_parser_error(parse_expr, "[1,]");
        test_parser_error(parse_expr, "[1,2,]");
        test_parser_error(parse_expr, "{ x = 1, }");
        test_parser_error(parse_expr, "{ x = 1, y = 0, }");
        test_parser_error(parse_expr, "{ a | x = 0, }");
        test_parser_error(parse_expr, "{ a | x = 0, y = 1, }");
    }

    #[test]
    fn check_unit() {
        test_parser_result(parse_expr, "()", Expr::Unit((0, 0)));
    }

    #[test]
    fn check_parens() {
        test_parser_result(parse_expr, "(a)", Expr::Ref((0, 0), "a".s()));
    }

    #[test]
    fn check_tuple() {
        test_parser_result(parse_expr, "(a, b)", Expr::Tuple(
            (0, 0),
            vec![
                Expr::Ref((0, 0), "a".s()),
                Expr::Ref((0, 0), "b".s())
            ],
        ));
    }

    #[test]
    fn check_list() {
        test_parser_result(parse_expr, "[a, b]", Expr::List(
            (0, 0),
            vec![
                Expr::Ref((0, 0), "a".s()),
                Expr::Ref((0, 0), "b".s())],
        ));
    }

    #[test]
    fn check_empty_list() {
        test_parser_result(parse_expr, "[]", Expr::List((0, 0), vec![]));
    }

    #[test]
    fn check_if() {
        test_parser_result(parse_expr, "if a then b else c", Expr::If(
            (0, 0),
            Box::new(Expr::Ref((0, 0), "a".s())),
            Box::new(Expr::Ref((0, 0), "b".s())),
            Box::new(Expr::Ref((0, 0), "c".s())),
        ));
    }

    #[test]
    fn check_lambda() {
        test_parser_result(parse_expr, "\\x -> x", Expr::Lambda(
            (0, 7),
            vec![Pattern::Var((1, 2), "x".s())],
            Box::new(Expr::Ref((6, 7), "x".s())),
        ));
    }

    #[test]
    fn check_case() {
        test_parser_result(parse_expr, "case x of\n  [] -> 0\n  _ -> 1", Expr::Case(
            (0, 28),
            Box::new(Expr::Ref((5, 6), "x".s())),
            vec![(
                     Pattern::List((12, 14), vec![]),
                     Expr::Literal((18, 19), Literal::Int(0))
                 ), (
                     Pattern::Wildcard((22, 23)),
                     Expr::Literal((27, 28), Literal::Int(1))
                 )],
        ));
    }

    #[test]
    fn check_let() {
        test_parser_result(parse_expr, "let x = 5 in 3", Expr::Let(
            (0, 0),
            vec![
                LetDeclaration::Def(Definition {
                    header: None,
                    name: "x".s(),
                    patterns: vec![],
                    expr: Expr::Literal((0, 0), Literal::Int(5)),
                })
            ],
            Box::new(Expr::Literal((0, 0), Literal::Int(3))),
        ));
    }

    #[test]
    fn check_binop_chain() {
        test_parser_result(parse_expr, "1 + 2 + 3 + 4", Expr::OpChain(
            (0, 0),
            vec![
                Expr::Literal((0, 0), Literal::Int(1)),
                Expr::Literal((0, 0), Literal::Int(2)),
                Expr::Literal((0, 0), Literal::Int(3)),
                Expr::Literal((0, 0), Literal::Int(4)),
            ],
            vec!["+".s(), "+".s(), "+".s()],
        ));
    }

    #[test]
    fn check_binop_chain_multiline() {
        test_parser_result(parse_expr, "1 + \n 2 + \n 3 + \n 4", Expr::OpChain(
            (0, 0),
            vec![
                Expr::Literal((0, 0), Literal::Int(1)),
                Expr::Literal((0, 0), Literal::Int(2)),
                Expr::Literal((0, 0), Literal::Int(3)),
                Expr::Literal((0, 0), Literal::Int(4)),
            ],
            vec!["+".s(), "+".s(), "+".s()],
        ));
    }

    #[test]
    fn check_priorities() {
        test_parser_result(parse_expr, "1 * 2 + 3 * 4", Expr::OpChain(
            (0, 0),
            vec![
                Expr::Literal((0, 0), Literal::Int(1)),
                Expr::Literal((0, 0), Literal::Int(2)),
                Expr::Literal((0, 0), Literal::Int(3)),
                Expr::Literal((0, 0), Literal::Int(4)),
            ],
            vec!["*".s(), "+".s(), "*".s()],
        ));
    }

    #[test]
    fn check_record_update() {
        test_parser_result(parse_expr, "{ a | b = 0 }", Expr::RecordUpdate(
            (0, 0),
            "a".s(),
            vec![("b".s(), Expr::Literal((0, 0), Literal::Int(0)))],
        ));
    }

    #[test]
    fn check_record_update2() {
        test_parser_result(parse_expr, "{ a | b = 0, c = 1 }", Expr::RecordUpdate(
            (0, 0),
            "a".s(),
            vec![
                ("b".s(), Expr::Literal((0, 0), Literal::Int(0))),
                ("c".s(), Expr::Literal((0, 0), Literal::Int(1))),
            ],
        ));
    }

    #[test]
    fn check_record_access() {
        test_parser_result(parse_expr, ".x", Expr::RecordAccess((0, 0), "x".s()));
    }

    #[test]
    fn check_record_field() {
        test_parser_result(parse_expr, "{}.x", Expr::RecordField(
            (0, 0),
            Box::new(Expr::Record((0, 0), vec![])),
            "x".s(),
        ));
    }

    #[test]
    fn check_qualified_ref() {
        test_parser_result(parse_expr, "List.map", Expr::QualifiedRef(
            (0, 0),
            vec!["List".s()],
            "map".s(),
        ));
    }

    #[test]
    fn check_function_application() {
        test_parser_result(parse_expr, "my_fun 1", Expr::Application(
            (0, 0),
            Box::new(Expr::Ref((0, 0), "my_fun".s())),
            Box::new(Expr::Literal((0, 0), Literal::Int(1))),
        ));
    }

    #[test]
    fn check_function_application2() {
        test_parser_result(parse_expr, "my_fun 1 2", Expr::Application(
            (0, 0),
            Box::new(Expr::Application(
                (0, 0),
                Box::new(Expr::Ref((0, 0), "my_fun".s())),
                Box::new(Expr::Literal((0, 0), Literal::Int(1))),
            )),
            Box::new(Expr::Literal((0, 0), Literal::Int(2))),
        ));
    }

    #[test]
    fn check_function_application_priority() {
        test_parser_result(parse_expr, "my_fun 1 2 + 3", Expr::OpChain(
            (0, 0),
            vec![
                Expr::Application(
                    (0, 0),
                    Box::new(Expr::Application(
                        (0, 0),
                        Box::new(Expr::Ref((0, 0), "my_fun".s())),
                        Box::new(Expr::Literal((0, 0), Literal::Int(1))),
                    )),
                    Box::new(Expr::Literal((0, 0), Literal::Int(2))),
                ),
                Expr::Literal((0, 0), Literal::Int(3))
            ],
            vec!["+".s()],
        ));
    }

    #[test]
    fn check_multiline_expr() {
        test_parser_result(parse_expr, "my_fun []\n  []",
                           Expr::Application(
                               (0, 0),
                               Box::new(Expr::Application(
                                   (0, 0),
                                   Box::new(Expr::Ref((0, 0), "my_fun".s())),
                                   Box::new(Expr::List((0, 0), vec![])),
                               )),
                               Box::new(Expr::List((0, 0), vec![])),
                           ),
        );
    }

    #[test]
    fn check_case_indentation() {
        let code = "\
case msg of\n    Increment ->\n        model + 1\n    Decrement ->\n        model - 1\
        ";

        test_parser_result(parse_expr, code, Expr::Case(
            (0, 81),
            Box::new(Expr::Ref((5, 8), "msg".s())),
            vec![
                (
                    Pattern::Adt((16, 25), "Increment".s(), vec![]),
                    Expr::OpChain(
                        (0, 0),
                        vec![Expr::Ref((0, 0), "model".s()), Expr::Literal((0, 0), Literal::Int(1))],
                        vec!["+".s()],
                    )
                ),
                (
                    Pattern::Adt((51, 60), "Decrement".s(), vec![]),
                    Expr::OpChain(
                        (0, 0),
                        vec![Expr::Ref((0, 0), "model".s()), Expr::Literal((0, 0), Literal::Int(1))],
                        vec!["-".s()],
                    )
                )
            ],
        ),
        );
    }

    #[test]
    fn check_prefix_minus() {
        test_parser_result(parse_expr, "-(1+2)", Expr::Application(
            (0, 0),
            Box::from(Expr::Ref((0, 0), "__internal__minus".s())),
            Box::from(Expr::OpChain(
                (0, 0),
                vec![Expr::Literal((0, 0), Literal::Int(1)), Expr::Literal((0, 0), Literal::Int(2))],
                vec!["+".s()],
            )),
        ));
    }

    #[test]
    fn check_infix_minus() {
        test_parser_result(parse_expr, "1 - 2", Expr::OpChain(
            (0, 0),
            vec![Expr::Literal((0, 0), Literal::Int(1)), Expr::Literal((0, 0), Literal::Int(2))],
            vec!["-".s()],
        ));
    }

    #[test]
    fn check_infix_minus_precedence() {
        test_parser_result(parse_expr, "1 -2", Expr::Application(
            (0, 0),
            Box::new(Expr::Literal((0, 0), Literal::Int(1))),
            Box::new(Expr::Application(
                (0, 0),
                Box::new(Expr::Ref((0, 0), "__internal__minus".s())),
                Box::new(Expr::Literal((0, 0), Literal::Int(2))),
            )),
        ));
    }

    #[test]
    fn check_infix_minus_validity() {
        test_parser_result(parse_expr, "1- 2", Expr::OpChain(
            (0, 0),
            vec![Expr::Literal((0, 0), Literal::Int(1)), Expr::Literal((0, 0), Literal::Int(2))],
            vec!["-".s()],
        ));
    }

    /**
     * This is a weird behavior of the lang, it's uncommon, so I will just ignore it
     * Also someone commented that this shouldn't be a allowed because it doesn't follow the
     * format guidelines of elm
     **/
    #[test]
    fn check_infix_minus_edge_case() {
        test_parser_result(parse_expr, "1-2", Expr::OpChain(
            (0, 0),
            vec![Expr::Literal((0, 0), Literal::Int(1)), Expr::Literal((0, 0), Literal::Int(2))],
            vec!["-".s()],
        ));
    }

    #[test]
    fn check_infix_minus_edge_case2() {
        test_parser_result(parse_expr, "-n string", Expr::Application(
            (0, 0),
            Box::from(Expr::Application(
                (0, 3),
                Box::from(Expr::Ref((0, 1), "__internal__minus".s())),
                Box::from(Expr::Ref((1, 2), "n".s())),
            )),
            Box::from(Expr::Ref((0, 0), "string".s())),
        ));
    }

    #[test]
    fn check_multiline_expr2() {
        let code = "Browser.element \
        \n    { init = init\
        \n    , view = view\
        \n    , update = update\
        \n    , subscriptions = subscriptions\
        \n    }\
        ";

        test_parser_result(parse_expr, code, Expr::Application(
            (0, 0),
            Box::from(Expr::QualifiedRef(
                (0, 0),
                vec![
                    "Browser".s()
                ],
                "element".s(),
            )),
            Box::from(Expr::Record(
                (0, 0),
                vec![
                    ("init".s(), Expr::Ref((0, 0), "init".s())),
                    ("view".s(), Expr::Ref((0, 0), "view".s())),
                    ("update".s(), Expr::Ref((0, 0), "update".s())),
                    ("subscriptions".s(), Expr::Ref((0, 0), "subscriptions".s()))
                ],
            )),
        ));
    }

    #[test]
    fn check_multiline_expr3() {
        let code = "\
            let \
            \n     row x = \
            \n        List.range 0 x \
            \n        |> List.map (\\y -> Cell Dirt ) \
            \n     column x y = \
            \n         List.range 0 y \
            \n         |> List.map (\\s -> row x) \
            \n in \
            \n    { cells = (column size size) \
            \n    , entities = [] \
            \n    }\
            ";

        test_parser_result(parse_expr, code, Expr::Let(
            (0, 0),
            vec![
                LetDeclaration::Def(Definition {
                    header: None,
                    name: "row".s(),
                    patterns: vec![
                        Pattern::Var((14, 15), "x".s())
                    ],
                    expr: Expr::OpChain(
                        (27, 81),
                        vec![
                            Expr::Application(
                                (27, 41),
                                Box::from(Expr::Application(
                                    (27, 41),
                                    Box::from(Expr::QualifiedRef((27, 38), vec!["List".s()], "range".s())),
                                    Box::from(Expr::Literal((38, 39), Literal::Int(0))),
                                )),
                                Box::from(Expr::Ref((40, 41), "x".s())),
                            ),
                            Expr::Application(
                                (54, 81),
                                Box::from(Expr::QualifiedRef((54, 63), vec!["List".s()], "map".s())),
                                Box::from(Expr::Lambda(
                                    (64, 80),
                                    vec![Pattern::Var((65, 66), "y".s())],
                                    Box::from(Expr::Application(
                                        (70, 79),
                                        Box::from(Expr::Ref((70, 75), "Cell".s())),
                                        Box::from(Expr::Ref((75, 80), "Dirt".s())),
                                    )),
                                )),
                            )
                        ],
                        vec!["|>".s()],
                    ),
                }),
                LetDeclaration::Def(Definition {
                    header: None,
                    name: "column".s(),
                    patterns: vec![
                        Pattern::Var((95, 96), "x".s()),
                        Pattern::Var((97, 98), "y".s())
                    ],
                    expr: Expr::OpChain(
                        (111, 161),
                        vec![
                            Expr::Application(
                                (111, 125),
                                Box::from(Expr::Application(
                                    (111, 125),
                                    Box::from(Expr::QualifiedRef((111, 122), vec!["List".s()], "range".s())),
                                    Box::from(Expr::Literal((122, 123), Literal::Int(0))),
                                )),
                                Box::from(Expr::Ref((124, 125), "y".s())),
                            ),
                            Expr::Application(
                                (139, 161),
                                Box::from(Expr::QualifiedRef((139, 148), vec!["List".s()], "map".s())),
                                Box::from(Expr::Lambda(
                                    (149, 160),
                                    vec![Pattern::Var((150, 151), "s".s())],
                                    Box::from(Expr::Application(
                                        (155, 160),
                                        Box::from(Expr::Ref((155, 158), "row".s())),
                                        Box::from(Expr::Ref((159, 160), "x".s())),
                                    )),
                                )),
                            )
                        ],
                        vec!["|>".s()],
                    ),
                })
            ],
            Box::from(Expr::Record((172, 228), vec![
                (
                    "cells".s(),
                    Expr::Application(
                        (183, 199),
                        Box::from(Expr::Application(
                            (183, 199),
                            Box::from(Expr::Ref((183, 189), "column".s())),
                            Box::from(Expr::Ref((190, 194), "size".s())),
                        )),
                        Box::from(Expr::Ref((195, 199), "size".s())),
                    )
                ),
                (
                    "entities".s(),
                    Expr::List((219, 227), vec![])
                )
            ],
            )),
        ));
    }
}




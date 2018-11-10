use ast::Expr;
use ast::Literal;
use ast::Pattern;
use parsers::new::Input;
use parsers::new::ParseError;
use parsers::new::pattern::parse_pattern;
use parsers::new::statement::parse_definition;
use parsers::new::util::*;
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
                            let (rest, i) = comma1(&parse_expr, i)?;
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
                            let (values, i) = many0(&parse_record_entry, i)?;

                            let i = expect(Token::RightBrace, i)?;
                            (Expr::Record(create_vec((name, expr), values)), i)
                        }
                        Token::Pipe => {
                            // { a | x = 0 }
                            let (first_name, i) = expect_id(i.next())?;
                            let i = expect(Token::Equals, i)?;
                            let (first_expr, i) = parse_expr(i)?;
                            let (values, i) = comma0(&parse_record_entry, i)?;

                            let i = expect(Token::RightBrace, i)?;
                            (Expr::RecordUpdate(name, create_vec((first_name, first_expr), values)), i)
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
            let level = read_optional_indent(i.clone());

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

    Expr::OpChain(exprs, ops)
}


#[cfg(test)]
mod tests {
    use ast::Definition;
    use parsers::new::util::test_parser;
    use parsers::new::util::test_parser_error;
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
        test_parser_result(parse_expr, "()", Expr::Unit);
    }

    #[test]
    fn check_parens() {
        test_parser_result(parse_expr, "(a)", Expr::Ref("a".s()));
    }

    #[test]
    fn check_tuple() {
        test_parser_result(parse_expr, "(a, b)", Expr::Tuple(vec![
            Expr::Ref("a".s()),
            Expr::Ref("b".s())]),
        );
    }

    #[test]
    fn check_list() {
        test_parser_result(parse_expr, "[a, b]", Expr::List(vec![
            Expr::Ref("a".s()),
            Expr::Ref("b".s())]),
        );
    }

    #[test]
    fn check_empty_list() {
        test_parser_result(parse_expr, "[]", Expr::List(vec![]));
    }

    #[test]
    fn check_if() {
        test_parser_result(parse_expr, "if a then b else c", Expr::If(
            Box::new(Expr::Ref("a".s())),
            Box::new(Expr::Ref("b".s())),
            Box::new(Expr::Ref("c".s())),
        ));
    }

    #[test]
    fn check_lambda() {
        test_parser_result(parse_expr, "\\x -> x", Expr::Lambda(
            vec![Pattern::Var("x".s())],
            Box::new(Expr::Ref("x".s())),
        ));
    }

    #[test]
    fn check_case() {
        test_parser_result(parse_expr, "case x of\n  [] -> 0\n  _ -> 1", Expr::Case(
            Box::new(Expr::Ref("x".s())),
            vec![(
                     Pattern::List(vec![]),
                     Expr::Literal(Literal::Int(0))
                 ), (
                     Pattern::Wildcard,
                     Expr::Literal(Literal::Int(1))
                 )],
        ));
    }

    #[test]
    fn check_let() {
        test_parser_result(parse_expr, "let x = 5 in 3", Expr::Let(
            vec![
                Definition {
                    header: None,
                    name: "x".s(),
                    patterns: vec![],
                    expr: Expr::Literal(Literal::Int(5)),
                }
            ],
            Box::new(Expr::Literal(Literal::Int(3))),
        ));
    }

    #[test]
    fn check_binop_chain() {
        test_parser_result(parse_expr, "1 + 2 + 3 + 4", Expr::OpChain(vec![
            Expr::Literal(Literal::Int(1)),
            Expr::Literal(Literal::Int(2)),
            Expr::Literal(Literal::Int(3)),
            Expr::Literal(Literal::Int(4)),
        ], vec!["+".s(), "+".s(), "+".s()],
        ));
    }


    #[test]
    fn check_binop_chain_multiline() {
        test_parser_result(parse_expr, "1 + \n 2 + \n 3 + \n 4", Expr::OpChain(vec![
            Expr::Literal(Literal::Int(1)),
            Expr::Literal(Literal::Int(2)),
            Expr::Literal(Literal::Int(3)),
            Expr::Literal(Literal::Int(4)),
        ], vec!["+".s(), "+".s(), "+".s()],
        ));
    }

    #[test]
    fn check_priorities() {
        test_parser_result(parse_expr, "1 * 2 + 3 * 4", Expr::OpChain(vec![
            Expr::Literal(Literal::Int(1)),
            Expr::Literal(Literal::Int(2)),
            Expr::Literal(Literal::Int(3)),
            Expr::Literal(Literal::Int(4)),
        ], vec!["*".s(), "+".s(), "*".s()],
        ));
    }

    #[test]
    fn check_record_update() {
        test_parser_result(parse_expr, "{ a | b = 0 }", Expr::RecordUpdate(
            "a".s(),
            vec![("b".s(), Expr::Literal(Literal::Int(0)))],
        ));
    }

    #[test]
    fn check_record_update2() {
        test_parser_result(parse_expr, "{ a | b = 0, c = 1 }", Expr::RecordUpdate(
            "a".s(),
            vec![
                ("b".s(), Expr::Literal(Literal::Int(0))),
                ("c".s(), Expr::Literal(Literal::Int(1))),
            ],
        ));
    }

    #[test]
    fn check_record_access() {
        test_parser_result(parse_expr, ".x", Expr::RecordAccess("x".s()));
    }

    #[test]
    fn check_record_field() {
        test_parser_result(parse_expr, "{}.x", Expr::RecordField(
            Box::new(Expr::Record(vec![])),
            "x".s(),
        ));
    }

    #[test]
    fn check_qualified_ref() {
        test_parser_result(parse_expr, "List.map", Expr::QualifiedRef(
            vec!["List".s()],
            "map".s(),
        ));
    }

    #[test]
    fn check_function_application() {
        test_parser_result(parse_expr, "my_fun 1", Expr::Application(
            Box::new(Expr::Ref("my_fun".s())),
            Box::new(Expr::Literal(Literal::Int(1))),
        ));
    }

    #[test]
    fn check_function_application2() {
        test_parser_result(parse_expr, "my_fun 1 2", Expr::Application(
            Box::new(Expr::Application(
                Box::new(Expr::Ref("my_fun".s())),
                Box::new(Expr::Literal(Literal::Int(1))),
            )),
            Box::new(Expr::Literal(Literal::Int(2))),
        ));
    }

    #[test]
    fn check_function_application_priority() {
        test_parser_result(parse_expr, "my_fun 1 2 + 3", Expr::OpChain(
            vec![
                Expr::Application(
                    Box::new(Expr::Application(
                        Box::new(Expr::Ref("my_fun".s())),
                        Box::new(Expr::Literal(Literal::Int(1))),
                    )),
                    Box::new(Expr::Literal(Literal::Int(2))),
                ),
                Expr::Literal(Literal::Int(3))
            ],
            vec!["+".s()],
        ));
    }

    #[test]
    fn check_multiline_expr() {
        test_parser_result(parse_expr, "my_fun []\n  []",
                           Expr::Application(
                               Box::new(Expr::Application(
                                   Box::new(Expr::Ref("my_fun".s())),
                                   Box::new(Expr::List(vec![])),
                               )),
                               Box::new(Expr::List(vec![])),
                           ),
        );
    }

    #[test]
    fn check_case_indentation() {
        test_parser_result(parse_expr, "\
case msg of\n    Increment ->\n        model + 1\n    Decrement ->\n        model - 1\
        ", Expr::Case(
            Box::new(Expr::Ref("msg".s())),
            vec![
                (
                    Pattern::Adt("Increment".s(), vec![]),
                    Expr::OpChain(
                        vec![Expr::Ref("model".s()), Expr::Literal(Literal::Int(1))],
                        vec!["+".s()],
                    )
                ),
                (
                    Pattern::Adt("Decrement".s(), vec![]),
                    Expr::OpChain(
                        vec![Expr::Ref("model".s()), Expr::Literal(Literal::Int(1))],
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
            Box::from(Expr::Ref("-".s())),
            Box::from(Expr::OpChain(
                vec![Expr::Literal(Literal::Int(1)), Expr::Literal(Literal::Int(2))],
                vec!["+".s()],
            )),
        ));
    }

    #[test]
    fn check_infix_minus() {
        test_parser_result(parse_expr, "1 - 2", Expr::OpChain(
            vec![Expr::Literal(Literal::Int(1)), Expr::Literal(Literal::Int(2))],
            vec!["-".s()],
        ));
    }

    #[test]
    fn check_infix_minus_precedence() {
        test_parser_result(parse_expr, "1 -2", Expr::Application(
            Box::new(Expr::Literal(Literal::Int(1))),
            Box::new(Expr::Application(
                Box::new(Expr::Ref("-".s())),
                Box::new(Expr::Literal(Literal::Int(2))),
            )),
        ));
    }

    #[test]
    fn check_infix_minus_validity() {
        test_parser_result(parse_expr, "1- 2", Expr::OpChain(
            vec![Expr::Literal(Literal::Int(1)), Expr::Literal(Literal::Int(2))],
            vec!["-".s()],
        ));
    }

    /**
     * This is a weird behavior of the lang, it's uncommon, so I will just ignore it
     * Also someone commented that this shouldn't be a allowed because it doesn't follow the
     * format guidelines of elm
     **/
    #[test]
    #[ignore]
    fn check_infix_minus_edge_case() {
        test_parser_result(parse_expr, "1-2", Expr::OpChain(
            vec![Expr::Literal(Literal::Int(1)), Expr::Literal(Literal::Int(2))],
            vec!["-".s()],
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
            Box::from(Expr::QualifiedRef(
                vec![
                    "Browser".s()
                ],
                "element".s(),
            )),
            Box::from(Expr::Record(
                vec![
                    ("init".s(), Expr::Ref("init".s())),
                    ("view".s(), Expr::Ref("view".s())),
                    ("update".s(), Expr::Ref("update".s())),
                    ("subscriptions".s(), Expr::Ref("subscriptions".s()))
                ]
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
            vec![
                Definition {
                    header: None,
                    name: "row".s(),
                    patterns: vec![
                        Pattern::Var("x".s())
                    ],
                    expr: Expr::OpChain(
                        vec![
                            Expr::Application(
                                Box::from(Expr::Application(
                                    Box::from(Expr::QualifiedRef(vec!["List".s()], "range".s())),
                                    Box::from(Expr::Literal(Literal::Int(0))),
                                )),
                                Box::from(Expr::Ref("x".s())),
                            ),
                            Expr::Application(
                                Box::from(Expr::QualifiedRef(vec!["List".s()], "map".s())),
                                Box::from(Expr::Lambda(
                                    vec![Pattern::Var("y".s())],
                                    Box::from(Expr::Application(
                                        Box::from(Expr::Ref("Cell".s())),
                                        Box::from(Expr::Ref("Dirt".s())),
                                    )),
                                )),
                            )
                        ],
                        vec!["|>".s()],
                    ),
                },
                Definition {
                    header: None,
                    name: "column".s(),
                    patterns: vec![
                        Pattern::Var("x".s()),
                        Pattern::Var("y".s())
                    ],
                    expr: Expr::OpChain(
                        vec![
                            Expr::Application(
                                Box::from(Expr::Application(
                                    Box::from(Expr::QualifiedRef(vec!["List".s()], "range".s())),
                                    Box::from(Expr::Literal(Literal::Int(0))),
                                )),
                                Box::from(Expr::Ref("y".s())),
                            ),
                            Expr::Application(
                                Box::from(Expr::QualifiedRef(vec!["List".s()], "map".s())),
                                Box::from(Expr::Lambda(
                                    vec![Pattern::Var("s".s())],
                                    Box::from(Expr::Application(
                                        Box::from(Expr::Ref("row".s())),
                                        Box::from(Expr::Ref("x".s())),
                                    )),
                                )),
                            )
                        ],
                        vec!["|>".s()],
                    ),
                }
            ],
            Box::from(Expr::Record(vec![
                (
                    "cells".s(),
                    Expr::Application(
                        Box::from(Expr::Application(
                            Box::from(Expr::Ref("column".s())),
                            Box::from(Expr::Ref("size".s())),
                        )),
                        Box::from(Expr::Ref("size".s())),
                    )
                ),
                (
                    "entities".s(),
                    Expr::List(vec![])
                )
            ]
            )),
        ));
    }
}




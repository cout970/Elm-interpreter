use ast::Definition;
use ast::Statement;
use ast::Type;
use parsers::expression::parse_expr;
use parsers::input::Input;
use parsers::parser::ParseError;
use parsers::pattern::parse_pattern;
use parsers::types::parse_type;
use parsers::util::expect;
use parsers::util::expect_binop;
use parsers::util::expect_id;
use parsers::util::expect_indent;
use parsers::util::expect_int;
use parsers::util::expect_upper;
use parsers::util::many0;
use parsers::util::pipe1;
use tokenizer::Token;

pub fn parse_statement(input: Input) -> Result<(Statement, Input), ParseError> {
    let (stm, i) = match input.read() {
        Token::TypeTk => {
            let i = input.next();
            if let Token::Alias = i.read() {
                //type alias
                let (name, i) = expect_upper(i.next())?;
                let (params, i) = many0(&expect_id, i)?;
                let i = expect(Token::Equals, i)?;
                let (ty, i) = parse_type(i)?;

                (Statement::Alias(name, params, ty), i)
            } else {
                //type
                let (name, i) = expect_upper(i)?;
                let (params, i) = many0(&expect_id, i)?;
                let i = expect(Token::Equals, i)?;
                let (branches, i) = pipe1(&parse_adt_branch, i)?;

                (Statement::Adt(name, params, branches), i)
            }
        }
        Token::Port => {
            let (name, i) = expect_id(input.next())?;
            let i = expect(Token::Colon, i)?;
            let (ty, i) = parse_type(i)?;

            (Statement::Port(name, ty), i)
        }
        Token::Id(_) => {
            let (def, i) = parse_definition(0, input)?;

            (Statement::Def(def), i)
        }
        Token::InfixTk => {
            let (direction, i) = expect_id(input.next())?;
            let (level, i) = expect_int(i)?;
            let i = expect(Token::LeftParen, i)?;
            let (op, i) = expect_binop(i)?;
            let i = expect(Token::RightParen, i)?;
            let i = expect(Token::Equals, i)?;
            let (func, i) = expect_id(i)?;

            (Statement::Infix(direction, level, op, func), i)
        }
        _ => {
            let found = input.read();
            return Err(ParseError::UnmatchedToken { input, found, options: vec![] });
        }
    };

    Ok((stm, i))
}

pub fn parse_definition(indent: u32, input: Input) -> Result<(Definition, Input), ParseError> {
    let (name, i) = expect_id(input)?;

    let (header, i) = match i.read() {
        Token::Colon => {
            let (ty, i) = parse_type(i.next())?;
            let i = expect_indent(indent, i)?;
            let (f_name, i) = expect_id(i)?;
            assert_eq!(f_name, name);

            (Some(ty), i)
        }
        _ => (None, i)
    };

    let (patterns, i) = many0(&parse_pattern, i)?;
    let i = expect(Token::Equals, i)?;
    let (expr, i) = parse_expr(i)?;

    Ok((Definition { header, name, patterns, expr }, i))
}

fn parse_adt_branch(input: Input) -> Result<((String, Vec<Type>), Input), ParseError> {
    let (name, i) = expect_upper(input)?;
    let (params, i) = many0(&parse_type, i)?;

    Ok(((name, params), i))
}

#[cfg(test)]
mod tests {
    use ast::Expr;
    use ast::Literal;
    use ast::Pattern;
    use parsers::util::test_parser;
    use parsers::util::test_parser_error;
    use parsers::util::test_parser_result;
    use util::StringConversion;

    use super::*;

    #[test]
    fn expr_test() {
        test_parser(parse_statement, "type Bool = True | False");
        test_parser(parse_statement, "type List a = Cons a List | Nil");
        test_parser(parse_statement, "type alias EmptySet = {}");
        test_parser(parse_statement, "type alias Set a = { all: List a, inside: List a }");
        test_parser(parse_statement, "port sum : Int -> Int -> Int");
        test_parser(parse_statement, "x = 0");
        test_parser(parse_statement, "func (a, b) = a + b");
        test_parser(parse_statement, "func x = x");
        test_parser(parse_statement, "func : Int -> Int\nfunc x = x");

        test_parser(parse_statement, "type Bool\n = True\n | False");
        test_parser(parse_statement, "type List a\n = Cons a List\n | Nil");
        test_parser(parse_statement, "type alias EmptySet =\n {}");
        test_parser(parse_statement, "type alias Set a = {\n all: List a,\n inside: List a\n }");
        test_parser(parse_statement, "port sum\n  : Int\n -> Int\n -> Int");
        test_parser(parse_statement, "x =\n 0");
        test_parser(parse_statement, "func\n x\n =\n x");
        test_parser(parse_statement, "func (a, b) = a + b");
    }

    #[test]
    fn expr_error_test() {
        test_parser_error(parse_statement, "type Bool");
        test_parser_error(parse_statement, "type Bool \n= True \n| False");
        test_parser_error(parse_statement, "type List a \n= Cons a List \n| Nil");
        test_parser_error(parse_statement, "type alias EmptySet = \n{}");
        test_parser_error(parse_statement, "type alias Set a = { \nall: List a, \ninside: List a \n}");
        test_parser_error(parse_statement, "port sum");
    }

    #[test]
    fn check_type_alias() {
        test_parser_result(parse_statement, "type alias Html = MyHtml", Statement::Alias(
            "Html".s(), vec![],
            Type::Tag("MyHtml".s(), vec![]),
        ));
    }

    #[test]
    fn check_adt() {
        test_parser_result(parse_statement, "type Boolean = True | False", Statement::Adt(
            "Boolean".s(), vec![],
            vec![("True".s(), vec![]), ("False".s(), vec![])],
        ));
    }

    #[test]
    fn check_port() {
        test_parser_result(parse_statement, "port js_function : Int -> Int", Statement::Port(
            "js_function".s(),
            Type::Fun(
                Box::new(Type::Tag("Int".s(), vec![])),
                Box::new(Type::Tag("Int".s(), vec![])),
            ),
        ));
    }

    #[test]
    fn check_def() {
        test_parser_result(parse_statement, "my_fun x = ()", Statement::Def(
            Definition {
                header: None,
                name: "my_fun".s(),
                patterns: vec![Pattern::Var("x".s())],
                expr: Expr::Unit,
            }
        ));
    }

    #[test]
    fn check_def2() {
        test_parser_result(parse_statement, "x = 5", Statement::Def(
            Definition {
                header: None,
                name: "x".s(),
                patterns: vec![],
                expr: Expr::Literal(Literal::Int(5)),
            }
        ));
    }

    #[test]
    fn check_def3() {
        test_parser_result(parse_statement, "my_fun: Int\nmy_fun = 5", Statement::Def(
            Definition {
                header: Some(Type::Tag("Int".s(), vec![])),
                name: "my_fun".s(),
                patterns: vec![],
                expr: Expr::Literal(Literal::Int(5)),
            }
        ));
    }

    //    #[test]
    fn check_def4() {
        let code = "\
update msg model =\n    case msg of\n    Increment ->\n        model + 1\n    Decrement ->\n        model - 1\
        ";

        test_parser_result(parse_statement, code, Statement::Def(
            Definition {
                header: None,
                name: "update".s(),
                patterns: vec![Pattern::Var("msg".s()), Pattern::Var("model".s())],
                expr: Expr::Case(
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
            }
        ),
        );
    }

    #[test]
    fn check_function_header() {
        test_parser_result(parse_statement, "init: () -> (Model, Cmd Msg)\ninit flags = ({ grid = createGrid 32}, loadMap \"/src/map.txt\")", Statement::Def(Definition {
            header: Some(Type::Fun(
                Box::from(Type::Unit),
                Box::from(Type::Tuple(vec![
                    Type::Tag("Model".s(), vec![]),
                    Type::Tag("Cmd".s(), vec![
                        Type::Tag("Msg".s(), vec![])
                    ]),
                ])),
            )),
            name: "init".s(),
            patterns: vec![
                Pattern::Var("flags".s())
            ],
            expr: Expr::Tuple(
                vec![
                    Expr::Record(
                        vec![
                            (
                                "grid".s(),
                                Expr::Application(
                                    Box::from(Expr::Ref("createGrid".s())),
                                    Box::from(Expr::Literal(Literal::Int(32))),
                                )
                            )
                        ]
                    ),
                    Expr::Application(
                        Box::from(Expr::Ref("loadMap".s())),
                        Box::from(Expr::Literal(Literal::String("/src/map.txt".s()))),
                    )
                ]
            ),
        }));
    }
}
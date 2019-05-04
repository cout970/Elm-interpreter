use ast::{Definition, Span};
use ast::Statement;
use ast::Type;
use parsers::expression::parse_expr;
use parsers::input::Input;
use parsers::ParseError;
use parsers::pattern::parse_pattern;
use parsers::types::parse_type;
use parsers::types::parse_type_without_adt;
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
    let start = input.pos();
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

            (Statement::Port((start, i.pos_end()), name, ty), i)
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
            return Err(ParseError::UnmatchedToken { span: input.span(), found, options: vec![] });
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

fn parse_adt_branch(input: Input) -> Result<((Span, String, Vec<Type>), Input), ParseError> {
    let start = input.pos();
    let (name, i) = expect_upper(input)?;
    let (params, i) = many0(&parse_type_without_adt, i)?;

    Ok((((start, i.pos_end()), name, params), i))
}

#[cfg(test)]
mod tests {
    use ast::Expr;
    use ast::Literal;
    use ast::Pattern;
    use parsers::util::test_utils::*;
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

        test_parser(parse_statement, "type Dict k v \n = RBNode_elm_builtin NColor k v (Dict k v) (Dict k v)\n | RBEmpty_elm_builtin");
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
            vec![((15, 19), "True".s(), vec![]), ((22, 27), "False".s(), vec![])],
        ));
    }

    #[test]
    fn check_port() {
        test_parser_result(parse_statement, "port js_function : Int -> Int", Statement::Port(
            (0, 29),
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
                patterns: vec![Pattern::Var((0, 0), "x".s())],
                expr: Expr::Unit((0, 0)),
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
                expr: Expr::Literal((0, 0), Literal::Int(5)),
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
                expr: Expr::Literal((0, 0), Literal::Int(5)),
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
                patterns: vec![Pattern::Var((0, 0), "msg".s()), Pattern::Var((0, 0), "model".s())],
                expr: Expr::Case(
                    (0, 0),
                    Box::new(Expr::Ref((0, 0), "msg".s())),
                    vec![
                        (
                            Pattern::Adt((0, 0), "Increment".s(), vec![]),
                            Expr::OpChain(
                                (0, 0),
                                vec![Expr::Ref((0, 0), "model".s()), Expr::Literal((0, 0), Literal::Int(1))],
                                vec!["+".s()],
                            )
                        ),
                        (
                            Pattern::Adt((0, 0), "Decrement".s(), vec![]),
                            Expr::OpChain(
                                (0, 0),
                                vec![Expr::Ref((0, 0), "model".s()), Expr::Literal((0, 0), Literal::Int(1))],
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
                Pattern::Var((0, 0), "flags".s())
            ],
            expr: Expr::Tuple(
                (0, 0),
                vec![
                    Expr::Record(
                        (0, 0),
                        vec![
                            (
                                "grid".s(),
                                Expr::Application(
                                    (0, 0),
                                    Box::from(Expr::Ref((0, 0), "createGrid".s())),
                                    Box::from(Expr::Literal((0, 0), Literal::Int(32))),
                                )
                            )
                        ],
                    ),
                    Expr::Application(
                        (0, 0),
                        Box::from(Expr::Ref((0, 0), "loadMap".s())),
                        Box::from(Expr::Literal((0, 0), Literal::String("/src/map.txt".s()))),
                    )
                ],
            ),
        }));
    }
}
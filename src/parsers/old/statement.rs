use ast::*;
use parsers::old::expression::read_expr;
use parsers::old::pattern::read_pattern;
use parsers::old::Tk;
use parsers::old::types::read_type;
use tokenizer::Token::*;
use parsers::old::ParseError;

// Definitions

rule!(pub top_level_statement<Statement>, do_parse!(
    many0!(indent!()) >>
    s: read_statement >>
    (s)
));

rule!(pub read_statement<Statement>, alt!(
    alias
    | adt
    | port
    | definition
));

rule!(indentation<()>, do_parse!(
    many0!(indent_except!(vec![0])) >>
    (())
));

rule!(definition<Statement>, map!(
    read_definition, |c| Statement::Def(c)
));

rule!(pub read_definition<Definition>, do_parse!(
    t: opt!(read_type_def) >>
    many0!(indent!()) >>
    a: id!() >>
    p: many0!(read_pattern) >>
    tk!(Equals) >>
    indentation >>
    e: read_expr >>
    (Definition {
        header: t.map(|e| e.1),
        name: a,
        patterns: p,
        expr: e,
    })
));

rule!(read_type_def<(String, Type)>, do_parse!(
    name: read_type_def_name >>
    indentation >>
    tk!(Colon) >>
    indentation >>
    ty: read_type >>
    indentation >>
    alt!(indent!(0) | map!(tk!(Eof), |_|())) >>
    ((name, ty))
));

rule!(read_type_def_name<String>, alt!(
    id!() | delimited!(tk!(LeftParen), binop!(), tk!(RightParen))
));

rule!(adt<Statement>, do_parse!(
    tk!(TypeTk) >>
    a: upper_id!() >>
    b: many0!(id!()) >>
    indentation >>
    tk!(Equals) >>
    entries: separated_nonempty_list!(pipe_separator, adt_def) >>
    (Statement::Adt(a, b, entries))
));

rule!(pipe_separator<()>, do_parse!(
    indentation >>
    tk!(Pipe) >>
    (())
));

rule!(adt_def<(String, Vec<Type>)>, do_parse!(
    indentation >>
    n: upper_id!() >>
    ty: many0!(read_type) >>
    ((n, ty))
));

rule!(port<Statement>, do_parse!(
    many0!(indent!()) >>
    tk!(Port) >>
    t: read_type_def >>
    (Statement::Port(t.0, t.1))
));

rule!(alias<Statement>, do_parse!(
    tk!(TypeTk) >>
    tk!(Alias) >>
    a: upper_id!() >>
    b: many0!(id!()) >>
    indentation >>
    tk!(Equals) >>
    indentation >>
    ty: read_type >>
    (Statement::Alias(a, b, ty))
));

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use tokenizer::tokenize;
//    use util::StringConversion;
//    use tokenizer::TokenStream;
//
//    #[test]
//    fn check_type_alias() {
//        let tokens = tokenize(b"type alias Html = MyHtml").unwrap();
//        let m = read_statement(TokenStream::new(&tokens));
//        assert_ok!(m, Statement::Alias(
//            "Html".s(), vec![],
//            Type::Tag("MyHtml".s(), vec![])
//        ));
//    }
//
//    #[test]
//    fn check_adt() {
//        let tokens = tokenize(b"type Boolean = True | False").unwrap();
//        let m = read_statement(TokenStream::new(&tokens));
//        assert_ok!(m, Statement::Adt(
//            "Boolean".s(), vec![],
//            vec![("True".s(), vec![]), ("False".s(), vec![])],
//        ));
//    }
//
//    #[test]
//    fn check_port() {
//        let tokens = tokenize(b"port js_function : Int -> Int").unwrap();
//        let m = read_statement(TokenStream::new(&tokens));
//        assert_ok!(m, Statement::Port(
//            "js_function".s(),
//            Type::Fun(
//                Box::new(Type::Tag("Int".s(), vec![])),
//                Box::new(Type::Tag("Int".s(), vec![])),
//            )
//        ));
//    }
//
//    #[test]
//    fn check_def() {
//        let tokens = tokenize(b"my_fun x = ()").unwrap();
//        let m = read_statement(TokenStream::new(&tokens));
//        assert_ok!(m, Statement::Def(
//            Definition {
//                header: None,
//                name: "my_fun".s(),
//                patterns: vec![Pattern::Var("x".s())],
//                expr: Expr::Unit,
//            }
//        ));
//    }
//
//    #[test]
//    fn check_def2() {
//        let tokens = tokenize(b"x = 5").unwrap();
//        let m = read_statement(TokenStream::new(&tokens));
//        assert_ok!(m, Statement::Def(
//            Definition {
//                header:None,
//                name: "x".s(),
//                patterns: vec![],
//                expr: Expr::Literal(Literal::Int(5))
//            }
//        ));
//    }
//
//    #[test]
//    fn check_def3() {
//        let tokens = tokenize(b"my_fun: Int\nmy_fun = 5").unwrap();
//        let m = read_statement(TokenStream::new(&tokens));
//        assert_ok!(m,
//            Statement::Def(
//                Definition {
//                    header:Some(Type::Tag("Int".s(), vec![])),
//                    name: "my_fun".s(),
//                    patterns: vec![],
//                    expr: Expr::Literal(Literal::Int(5))
//                }
//            )
//        );
//    }
//
//    //    #[test]
//    fn check_def4() {
//        let tokens = tokenize(b"\
//update msg model =\n    case msg of\n    Increment ->\n        model + 1\n    Decrement ->\n        model - 1\
//        ").unwrap();
//        let m = read_statement(TokenStream::new(&tokens));
//        assert_ok!(m,
//            Statement::Def(
//                Definition{
//                    header: None,
//                    name: "update".s(),
//                    patterns: vec![Pattern::Var("msg".s()), Pattern::Var("model".s())],
//                    expr: Expr::Case(
//                        Box::new(Expr::Ref("msg".s())),
//                        vec![
//                            (
//                                Pattern::Adt("Increment".s(), vec![]),
//                                Expr::OpChain(
//                                    vec![Expr::Ref("model".s()), Expr::Literal(Literal::Int(1))],
//                                    vec!["+".s()]
//                                )
//                            ),
//                            (
//                                Pattern::Adt("Decrement".s(), vec![]),
//                                Expr::OpChain(
//                                    vec![Expr::Ref("model".s()), Expr::Literal(Literal::Int(1))],
//                                    vec!["-".s()]
//                                )
//                            )
//                        ]
//                    )
//                }
//            )
//        );
//    }
//
//
//    #[test]
//    fn check_function_header() {
//        let tokens = tokenize(b"init: () -> (Model, Cmd Msg)\ninit flags = ({ grid = createGrid 32}, loadMap \"/src/map.txt\")").unwrap();
//        let (rest, ty) = read_type_def(TokenStream::new(&tokens)).unwrap();
//        let m = read_definition(rest);
//
//        assert_ok!(m, Definition {
//            header: None,
//            name: "init".s(),
//            patterns: vec![
//                Pattern::Var("flags".s())
//            ],
//            expr: Expr::Tuple(
//                vec![
//                    Expr::Record(
//                        vec![
//                            (
//                                "grid".s(),
//                                Expr::Application(
//                                    Box::from(Expr::Ref("createGrid".s())),
//                                    Box::from(Expr::Literal(Literal::Int(32))),
//                                )
//                            )
//                        ]
//                    ),
//                    Expr::Application(
//                        Box::from(Expr::Ref("loadMap".s())),
//                        Box::from(Expr::Literal(Literal::String("/src/map.txt".s()))),
//                    )
//                ]
//            ),
//        });
//    }
//}

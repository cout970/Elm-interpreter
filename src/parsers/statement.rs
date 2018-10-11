use *;
use parsers::expression::read_expr;
use parsers::pattern::read_pattern;
use parsers::types::read_type;
use tokenizer::Token::*;

// Definitions

named!(pub top_level_statement<Tk, Statement>, do_parse!(
    tk!(LineStart) >>
    s: read_statement >>
    (s)
));

named!(pub read_statement<Tk, Statement>, alt!(
    alias
    | adt
    | port
    | definition
));

named!(pub read_definition<Tk, Definition>, do_parse!(
    t: opt!(read_type_def) >>
    v: read_value_def >>
    (Definition(t.map(|e| e.1), v))
));

named!(read_type_def<Tk, (String, Type)>, do_parse!(
    name: read_type_def_name >>
    tk!(Colon) >>
    ty: read_type >>
    opt!(tk!(LineStart)) >>
    ((name, ty))
));

named!(read_type_def_name<Tk, String>, alt!(
    id!() | delimited!(tk!(LeftParen), binop!(), tk!(RightParen))
));

named!(read_value_def<Tk, ValueDefinition>, do_parse!(
    a: id!() >>
    p: many0!(read_pattern) >>
    tk!(Equals) >>
    many0!(indent!()) >>
    e: read_expr >>
    (ValueDefinition { name: a, patterns: p, expr: e })
));

named!(adt<Tk, Statement>, do_parse!(
    tk!(TypeTk) >>
    a: upper_id!() >>
    b: many0!(id!()) >>
    many0!(indent!()) >>
    tk!(Equals) >>
    entries: separated_nonempty_list!(pipe_separator, adt_def) >>
    (Statement::Adt(create_vec(a, b), entries))
));

named!(pipe_separator<Tk, ()>, do_parse!(
    many0!(indent!()) >>
    tk!(Pipe) >>
    (())
));

named!(adt_def<Tk, (String, Vec<Type>)>, do_parse!(
    many0!(indent!()) >>
    n: upper_id!() >>
    ty: many0!(read_type) >>
    ((n, ty))
));

named!(port<Tk, Statement>, do_parse!(
    tk!(Port) >>
    t: read_type_def >>
    (Statement::Port(t.0, t.1))
));

named!(definition<Tk, Statement>, map!(
    read_definition, |c| Statement::Def(c)
));

named!(alias<Tk, Statement>, do_parse!(
    tk!(TypeTk) >>
    tk!(Alias) >>
    a: upper_id!() >>
    b: many0!(id!()) >>
    many0!(indent!()) >>
    tk!(Equals) >>
    many0!(indent!()) >>
    ty: read_type >>
    (Statement::Alias(create_vec(a, b), ty))
));

#[cfg(test)]
mod tests {
    use nom::*;
    use super::*;
    use tokenizer::tokenize;

    #[test]
    fn check_type_alias() {
        let stream = tokenize(b"\ntype alias Html = MyHtml");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Alias(
            vec!["Html".s()],
            Type::Tag("MyHtml".s(), vec![])
        ));
    }

    #[test]
    fn check_adt() {
        let stream = tokenize(b"\ntype Boolean = True | False");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Adt(
            vec!["Boolean".s()],
            vec![("True".s(), vec![]), ("False".s(), vec![])],
        ));
    }

    #[test]
    fn check_port() {
        let stream = tokenize(b"\nport js_function : Int -> Int");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Port(
            "js_function".s(),
            Type::Fun(
                Box::new(Type::Tag("Int".s(), vec![])),
                Box::new(Type::Tag("Int".s(), vec![])),
            )
        ));
    }

    #[test]
    fn check_def() {
        let stream = tokenize(b"\nmy_fun x = ()");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Def(
            Definition(
                None,
                ValueDefinition {
                    name: "my_fun".s(),
                    patterns: vec![Pattern::Var("x".s())],
                    expr: Expr::Unit
                }
            )
        ));
    }

    #[test]
    fn check_def2() {
        let stream = tokenize(b"\nx = 5");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Def(
            Definition(
                None,
                ValueDefinition{
                    name: "x".s(),
                    patterns: vec![],
                    expr: Expr::Literal(Literal::Int(5))
                }
            )
        ));
    }

    #[test]
    fn check_def3() {
        let stream = tokenize(b"\nmy_fun: Int\nmy_fun = 5");
        let m = top_level_statement(&stream);
        assert_ok!(m,
            Statement::Def(
                Definition(
                    Some(Type::Tag("Int".s(), vec![])),
                    ValueDefinition {
                        name: "my_fun".s(),
                        patterns: vec![],
                        expr: Expr::Literal(Literal::Int(5))
                    }
                )
            )
        );
    }

    //    #[test]
    fn check_def4() {
        let stream = tokenize(b"\n\
update msg model =\n    case msg of\n    Increment ->\n        model + 1\n    Decrement ->\n        model - 1\
        ");
        let m = top_level_statement(&stream);
        assert_ok!(m,
            Statement::Def(
                Definition(
                    None,
                    ValueDefinition{
                        name: "update".s(),
                        patterns: vec![Pattern::Var("msg".s()), Pattern::Var("model".s())],
                        expr: Expr::Case(
                            Box::new(Expr::Ref("msg".s())),
                            vec![
                                (
                                    Pattern::Adt("Increment".s(), vec![]),
                                    Expr::OpChain(
                                        vec![Expr::Ref("model".s()), Expr::Literal(Literal::Int(1))],
                                        vec!["+".s()]
                                    )
                                ),
                                (
                                    Pattern::Adt("Decrement".s(), vec![]),
                                    Expr::OpChain(
                                        vec![Expr::Ref("model".s()), Expr::Literal(Literal::Int(1))],
                                        vec!["-".s()]
                                    )
                                )
                            ]
                        )
                    }
                )
            )
        );
    }
}

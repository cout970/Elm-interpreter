use *;
use parsers::expression::read_expr;
use parsers::pattern::read_pattern;
use parsers::spaces;
use parsers::types::read_type;
use tokenizer::Token::*;

// Definitions

named!(pub top_level_statement<Tk, Statement>, do_parse!(
    tk!(LineStart) >>
    s: statement_item >>
    (s)
));

named!(statement_item<Tk, Statement>, alt!(
    alias |
    adt   |
    port  |
    definition
));

named!(adt<Tk, Statement>, do_parse!(
    tk!(TypeTk) >>
    a: upper_id!() >>
    b: many0!(id!()) >>
    spaces >>
    tk!(Equals) >>
    entries: separated_nonempty_list!(pipe_separator, adt_def) >>
    (Statement::Adt(create_vec(a, b), entries))
));

named!(pipe_separator<Tk, ()>, do_parse!(
    spaces >>
    tk!(Pipe) >>
    (())
));

named!(adt_def<Tk, (String, Vec<Type>)>, do_parse!(
    spaces >>
    n: upper_id!() >>
    ty: many0!(read_type) >>
    ((n, ty))
));

named!(port<Tk, Statement>, do_parse!(
    tk!(Port) >>
    t: read_type_def >>
    (Statement::Port(t))
));

named!(definition<Tk, Statement>, map!(
    read_definition, |c| Statement::Def(c)
));

named!(pub read_definition<Tk, Definition>, do_parse!(
    t: opt!(read_type_def) >>
    v: read_value_def >>
    (Definition(t, v))
));

named!(read_type_def<Tk, TypeDefinition>, do_parse!(
    name: read_type_def_name >>
    tk!(Colon) >>
    ty: read_type >>
    opt!(tk!(LineStart)) >>
    (TypeDefinition(name, ty))
));

named!(read_type_def_name<Tk, String>, alt!(
    id!() | delimited!(tk!(LeftParen), binop!(), tk!(RightParen))
));

named!(read_value_def<Tk, ValueDefinition>, alt!(
    prefix_value_def |
    infix_value_def |
    name_value_def
));

named!(prefix_value_def<Tk, ValueDefinition>, do_parse!(
    tk!(LeftParen) >>
    b: binop!() >>
    tk!(RightParen) >>
    p: many0!(read_pattern) >>
    tk!(Equals) >>
    spaces >>
    e: read_expr >>
    (ValueDefinition::PrefixOp(b, p, e))
));

named!(infix_value_def<Tk, ValueDefinition>, do_parse!(
    a: read_pattern >>
    b: binop!() >>
    p: many0!(read_pattern) >>
    tk!(Equals) >>
    spaces >>
    e: read_expr >>
    (ValueDefinition::InfixOp(a, b, p, e))
));

named!(name_value_def<Tk, ValueDefinition>, do_parse!(
    a: id!() >>
    p: many0!(read_pattern) >>
    tk!(Equals) >>
    spaces >>
    e: read_expr >>
    (ValueDefinition::Name(a, p, e))
));

named!(alias<Tk, Statement>, do_parse!(
    tk!(TypeTk) >>
    tk!(Alias) >>
    a: upper_id!() >>
    b: many0!(id!()) >>
    spaces >>
    tk!(Equals) >>
    spaces >>
    ty: read_type >>
    (Statement::Alias(create_vec(a, b), ty))
));

#[cfg(test)]
mod tests {
    use nom::*;
    use super::*;
    use tokenizer::get_all_tokens;

    #[test]
    fn check_type_alias() {
        let stream = get_all_tokens(b"\ntype alias Html = MyHtml");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Alias(
            vec!["Html".s()],
            Type::Tag("MyHtml".s(), vec![])
        ));
    }

    #[test]
    fn check_adt() {
        let stream = get_all_tokens(b"\ntype Boolean = True | False");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Adt(
            vec!["Boolean".s()],
            vec![("True".s(), vec![]), ("False".s(), vec![])],
        ));
    }

    #[test]
    fn check_port() {
        let stream = get_all_tokens(b"\nport js_function : Int -> Int");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Port(
            TypeDefinition("js_function".s(),
                Type::Fun(
                    Box::new(Type::Tag("Int".s(), vec![])),
                    Box::new(Type::Tag("Int".s(), vec![])),
                )
            )
        ));
    }

    #[test]
    fn check_def() {
        let stream = get_all_tokens(b"\nmy_fun x = ()");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Def(
            Definition(
                None,
                ValueDefinition::Name("my_fun".s(), vec![Pattern::Var("x".s())], Expr::Unit)
            )
        ));
    }

    #[test]
    fn check_def2() {
        let stream = get_all_tokens(b"\nx = 5");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Def(
            Definition(
                None,
                ValueDefinition::Name("x".s(), vec![], Expr::Literal(Literal::Int(5)))
            )
        ));
    }

    #[test]
    fn check_def3() {
        let stream = get_all_tokens(b"\nmy_fun: Int\nmy_fun = 5");
        let m = top_level_statement(&stream);
        assert_ok!(m,
            Statement::Def(
                Definition(
                    Some(TypeDefinition("my_fun".s(), Type::Tag("Int".s(), vec![]))),
                    ValueDefinition::Name("my_fun".s(), vec![], Expr::Literal(Literal::Int(5)))
                )
            )
        );
    }

//    #[test]
    fn check_def4() {
        let stream = get_all_tokens(b"\n\
update msg model =\n    case msg of\n    Increment ->\n        model + 1\n    Decrement ->\n        model - 1\
        ");
        let m = top_level_statement(&stream);
        assert_ok!(m,
            Statement::Def(
                Definition(
                    None,
                    ValueDefinition::Name("update".s(),
                        vec![Pattern::Var("msg".s()), Pattern::Var("model".s())],
                        Expr::Case(
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
                    )
                )
            )
        );
    }
}

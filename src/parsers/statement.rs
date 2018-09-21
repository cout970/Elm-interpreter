use *;
use parsers::expression::read_expr;
use parsers::pattern::read_pattern;
use parsers::types::read_type;
use tokenizer::Token::*;

// Definitions

named!(pub top_level_statement<Tk, Statement>, alt!(
    alias |
    adt   |
    port  |
    definition
));

named!(adt<Tk, Statement>, do_parse!(
    opt!(tk!(LineStart)) >>
    tk!(TypeTk) >>
    a: upper_id!() >>
    b: many0!(id!()) >>
    tk!(Equals) >>
    entries: separated_nonempty_list!(tk!(Pipe), adt_def) >>
    (Statement::Adt(create_vec(a, b), entries))
));

named!(adt_def<Tk, (String, Vec<Type>)>, do_parse!(
    n: upper_id!() >>
    ty: many0!(read_type) >>
    ((n, ty))
));

named!(port<Tk, Statement>, do_parse!(
    opt!(tk!(LineStart)) >>
    tk!(Port) >>
    t: read_type_def >>
    (Statement::Port(t))
));

named!(definition<Tk, Statement>, map!(read_definition, |c| Statement::Def(c)));

named!(pub read_definition<Tk, Definition>, do_parse!(
    opt!(tk!(LineStart)) >>
    t: opt!(read_type_def) >>
    v: read_value_def >>
    (Definition(t, v))
));

named!(read_type_def<Tk, TypeDefinition>, alt!(
    do_parse!(n: id!() >> tk!(Colon) >> t: read_type >> (TypeDefinition(n, t))) |
    do_parse!(tk!(LeftParen) >> n: binop!() >> tk!(RightParen) >> tk!(Colon) >> t: read_type >> (TypeDefinition(n, t)))
));

named!(read_value_def<Tk, ValueDefinition>, alt!(
    prefix_value_def |
    infix_value_def |
    name_value_def
));

named!(prefix_value_def<Tk, ValueDefinition>, do_parse!(
    opt!(tk!(LineStart)) >>
    tk!(LeftParen) >>
    b: binop!() >>
    tk!(RightParen) >>
    p: many0!(read_pattern) >>
    tk!(Equals) >>
    e: read_expr >>
    (ValueDefinition::PrefixOp(b, p, e))
));

named!(infix_value_def<Tk, ValueDefinition>, do_parse!(
    opt!(tk!(LineStart)) >>
    a: read_pattern >>
    b: binop!() >>
    p: many0!(read_pattern) >>
    tk!(Equals) >>
    e: read_expr >>
    (ValueDefinition::InfixOp(a, b, p, e))
));

named!(name_value_def<Tk, ValueDefinition>, do_parse!(
    opt!(tk!(LineStart)) >>
    a: id!() >>
    p: many0!(read_pattern) >>
    tk!(Equals) >>
    e: read_expr >>
    (ValueDefinition::Name(a, p, e))
));

named!(alias<Tk, Statement>, do_parse!(
    opt!(tk!(LineStart)) >>
    tk!(TypeTk) >>
    tk!(Alias) >>
    a: upper_id!() >>
    b: many0!(id!()) >>
    tk!(Equals) >>
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
        let m = name_value_def(&stream);
        assert_ok!(m, ValueDefinition::Name("x".s(), vec![], Expr::Literal(Literal::Int(5))));
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
}
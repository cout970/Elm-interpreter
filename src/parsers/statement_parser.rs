use *;
use parsers::module_parser::space;
use parsers::expression_parser::read_expr;
use parsers::pattern_parser::read_pattern;
use parsers::type_parser::read_type;
use tokenizer::Token::*;

// Definitions

named!(pub top_level_statement<Tk, Statement>, alt!(
    alias |
    adt   |
    port  |
    definition
));

named!(adt<Tk, Statement>, do_parse!(
    tk!(TypeTk) >>
    a: upper_id!() >>
    b: many0!(id!()) >>
    tk!(Equals) >>
    entries: separated_nonempty_list!(tk!(Pipe), adt_def) >>
    tk!(NewLine) >>
    (Statement::Adt(create_vec(a, b), entries))
));

named!(adt_def<Tk, (String, Vec<Type>)>, do_parse!(
    n: upper_id!() >>
    ty: many0!(read_type) >>
    ((n, ty))
));

named!(port<Tk, Statement>, do_parse!(
    tk!(Port) >>
    t: read_type_def >>
    (Statement::Port(t))
));

named!(definition<Tk, Statement>, map!(read_definition, |c| Statement::Def(c)));

named!(pub read_definition<Tk, Definition>, do_parse!(
    t: opt!(read_type_def) >>
    v: read_value_def >>
    (Definition(t, v))
));

named!(read_type_def<Tk, TypeDefinition>, alt!(
    do_parse!(n: id!() >> tk!(Colon) >> t: read_type >> tk!(NewLine) >> (TypeDefinition(n, t))) |
    do_parse!(tk!(LeftParen) >> n: binop!() >> tk!(RightParen) >> t: read_type >> tk!(NewLine) >> (TypeDefinition(n, t)))
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
       e: read_expr >>
       (ValueDefinition::PrefixOp(b, p, e))
));

named!(infix_value_def<Tk, ValueDefinition>, do_parse!(
       a: read_pattern >>
       b: binop!() >>
       p: many0!(read_pattern) >>
       tk!(Equals) >>
       e: read_expr >>
       (ValueDefinition::InfixOp(a, b, p, e))
));

named!(name_value_def<Tk, ValueDefinition>, do_parse!(
       a: id!() >>
       p: many0!(read_pattern) >>
       tk!(Equals) >>
       e: read_expr >>
       (ValueDefinition::Name(a, p, e))
));

named!(alias<Tk, Statement>, do_parse!(
    tk!(TypeTk) >>
    tk!(Alias) >>
    a: upper_id!() >>
    b: many0!(id!()) >>
    tk!(Equals) >>
    ty: read_type >>
    tk!(NewLine) >>
    (Statement::Alias(create_vec(a, b), ty))
));

#[cfg(test)]
mod tests {
    use nom::*;
    use super::*;
    use tokenizer::get_all_tokens;

    #[test]
    fn check_type_alias() {
        let stream = get_all_tokens(b"type alias Html = MyHtml\n");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Alias(
            vec!["Html".s()],
            Type::Tag("MyHtml".s(), vec![])
        ));
    }

    #[test]
    fn check_adt() {
        let stream = get_all_tokens(b"type Boolean = True | False\n");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Adt(
            vec!["Boolean".s()],
            vec![("True".s(), vec![]), ("False".s(), vec![])],
        ));
    }

    #[test]
    fn check_port() {
        let stream = get_all_tokens(b"port js_function : Int -> Int\n");
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
        let stream = get_all_tokens(b"my_fun x = ()\n");
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
        let stream = get_all_tokens(b"x = 5\n");
        let m = name_value_def(&stream);
        assert_ok!(m, ValueDefinition::Name("x".s(), vec![], Expr::Literal(Literal::Int(5))));
    }

    #[test]
    fn check_def3() {
        let stream = get_all_tokens(b"my_fun: Int\nmy_fun = 5\n");
        let m = read_type_def(&stream);
        assert_ok!(m, TypeDefinition("my_fun".s(), Type::Tag("Int".s(), vec![]))
//        Statement::Def(
//            Definition(
//                Some(TypeDefinition("my_fun".s(), Type::Tag("Int".s(), vec![]))),
//                ValueDefinition::Name("my_fun".s(), vec![], Expr::Literal(Literal::Int(5)))
//            )
//        )
        );
    }
}

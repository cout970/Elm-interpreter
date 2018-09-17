use *;
use tokenizer::Token::*;
use parsers::type_parser::read_type;
use parsers::pattern_parser::read_pattern;

// Definitions

named!(pub top_level_statement<Tk, Statement>, alt!(
    alias |
    adt   |
    port  |
    definition
));

named!(adt<Tk, Statement>, do_parse!(
    id!("type") >>
    a: upper_id!() >>
    b: many0!(id!()) >>
    tk!(Equals) >>
    entries: separated_nonempty_list!(tk!(Pipe), adt_def) >>
    (Statement::Adt(create_vec(a, b), entries))
));

named!(adt_def<Tk, (String, Vec<Type>)>, do_parse!(
    n: upper_id!() >>
    ty: many1!(read_type) >>
    ((n, ty))
));

named!(port<Tk, Statement>, do_parse!(
    id!("port") >>
    t: read_type_def >>
    id!("port") >>
    v: read_value_def >>
    (Statement::Port(t, v))
));

named!(definition<Tk, Statement>, map!(read_definition, |c| Statement::Def(c)));

named!(pub read_definition<Tk, Definition>, do_parse!(
    t: opt!(read_type_def) >>
    v: read_value_def >>
    (Definition(t, v))
));

named!(read_type_def<Tk, TypeDefinition>, alt!(
    do_parse!(n: id!() >> id!(":") >> t: read_type >> (TypeDefinition(n, t))) |
    do_parse!(tk!(LeftParen) >> n: binop!() >> tk!(RightParen) >> t: read_type >> (TypeDefinition(n, t)))
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
    id!("type") >>
    id!("alias") >>
    a: upper_id!() >>
    b: many0!(id!()) >>
    tk!(Equals) >>
    ty: read_type >>
    (Statement::Alias(create_vec(a, b), ty))
));

named!(read_expr<Tk, Expr>, map!(tk!(LeftParen), |_c| Expr::Unit));

#[cfg(test)]
mod tests {
    use nom::*;
    use super::*;
    use tokenizer::get_all_tokens;

    #[test]
    fn check_type_alias() {
        let stream = get_all_tokens(b"type alias Html list = ()");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Alias(vec!["Html".to_string(), "list".to_string()], Type::Unit));
    }

    #[test]
    fn check_def() {
        let stream = get_all_tokens(b"my_fun x = ()");
        let m = top_level_statement(&stream);
        assert_ok!(m, Statement::Def(
            Definition(
                None,
                ValueDefinition::Name("my_fun".s(), vec![Pattern::Var("x".s())], Expr::Unit)
            )
        ));
    }
}

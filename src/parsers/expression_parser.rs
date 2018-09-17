use *;
use tokenizer::Token::*;
use parsers::module_parser::read_ref;
use parsers::pattern_parser::read_pattern;
use parsers::statement_parser::read_definition;

// Expresions

//named!(pub pr_expr<Tk, Expr>, call!(pr_binop_chain));
//
//
//named!(pub pr_binop_chain<Expr>, do_parse!(
//    head: pr_expr_part >> s >>
//    chain: many0!(preceded!(s, tuple!(pr_bin_op, pr_expr_part))) >>
//    (chain.into_iter().rev().fold(head, |a, b| Expr::BinaryOp(Box::new(a), b.0, Box::new(b.1))))
//));
//
named!(pub read_expr<Tk, Expr>, alt!(
    unit  |
    tuple |
    unit_tuple |
    list  |
    range |
    adt   |
    read_if |
    read_lambda |
    read_case   |
    read_let    |
    map!(literal!(), |c| Expr::Literal(c)) |
    map!(read_ref,   |c| Expr::Ref(c)) |
    delimited!(tk!(LeftParen), read_expr, tk!(RightParen))
));

named!(unit<Tk, Expr>, do_parse!(
    tk!(LeftParen) >> tk!(RightParen) >> (Expr::Unit)
));

named!(tuple<Tk, Expr>, do_parse!(
    tk!(LeftParen) >>
    a: read_expr >>
    tk!(Comma) >>
    list: separated_nonempty_list!(tk!(Comma), read_expr) >>
    tk!(RightParen) >>
    (Expr::Tuple(create_vec(a, list)))
));

named!(unit_tuple<Tk, Expr>, do_parse!(
    tk!(LeftParen) >>
    list: many1!(tk!(Comma)) >>
    tk!(RightParen) >>
    (Expr::Tuple(create_vec(Expr::Unit, list.into_iter().map(|_c| Expr::Unit).collect())))
));

named!(list<Tk, Expr>, do_parse!(
    tk!(LeftBracket) >>
    list: separated_list!(tk!(Comma), read_expr) >>
    tk!(RightBracket) >>
    (Expr::List(list))
));

named!(range<Tk, Expr>, do_parse!(
    tk!(LeftBracket) >>
    a: read_expr >>
    tk!(DoubleDot) >>
    b: read_expr >>
    tk!(RightBracket) >>
    (Expr::Range(Box::new(a), Box::new(b)))
));

named!(adt<Tk, Expr>, do_parse!(
    a: upper_id!() >> (Expr::Adt(a))
));

named!(read_if<Tk, Expr>, do_parse!(
    id!("if") >>
    cond: read_expr >>
    id!("then") >>
    tru: read_expr >>
    id!("else") >>
    fal: read_expr >>
    (Expr::If(Box::new(cond), Box::new(tru), Box::new(fal)))
));

named!(read_lambda<Tk, Expr>, do_parse!(
    binop!("\\") >>
    p: many1!(read_pattern) >>
    tk!(RightArrow) >>
    expr: read_expr >>
    (Expr::Lambda(p, Box::new(expr)))
));

named!(read_case<Tk, Expr>, do_parse!(
    id!("case") >>
    e: read_expr >>
    id!("of") >>
    b: many1!(do_parse!(p: read_pattern >> tk!(RightArrow) >> ex: read_expr >> ((p, ex)))) >>
    (Expr::Case(Box::new(e), b))
));

named!(read_let<Tk, Expr>, do_parse!(
    id!("let") >>
    a: many1!(read_definition) >>
    id!("in") >>
    b: read_expr >>
    (Expr::Let(a, Box::new(b)))
));

#[cfg(test)]
mod tests {
    use nom::*;
    use super::*;
    use tokenizer::get_all_tokens;

    #[test]
    fn check_unit() {
        let stream = get_all_tokens(b"()");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::Unit);
    }

    #[test]
    fn check_parens() {
        let stream = get_all_tokens(b"(a)");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::Ref(Ref::Name("a".s())));
    }

    #[test]
    fn check_tuple() {
        let stream = get_all_tokens(b"(a, b)");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::Tuple(vec![
            Expr::Ref(Ref::Name("a".s())),
            Expr::Ref(Ref::Name("b".s()))])
        );
    }

    #[test]
    fn check_list() {
        let stream = get_all_tokens(b"[a, b]");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::List(vec![
            Expr::Ref(Ref::Name("a".s())),
            Expr::Ref(Ref::Name("b".s()))])
        );
    }

    #[test]
    fn check_empty_list() {
        let stream = get_all_tokens(b"[]");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::List(vec![]));
    }

    #[test]
    fn check_range() {
        let stream = get_all_tokens(b"[a..b]");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::Range(
            Box::new(Expr::Ref(Ref::Name("a".s()))),
            Box::new(Expr::Ref(Ref::Name("b".s())))
        ));
    }

    #[test]
    fn check_unit_tuple() {
        let stream = get_all_tokens(b"(,)");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::Tuple(vec![
            Expr::Unit,
            Expr::Unit
        ]));
    }

    #[test]
    fn check_if() {
        let stream = get_all_tokens(b"if a then b else c");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::If(
            Box::new(Expr::Ref(Ref::Name("a".s()))),
            Box::new(Expr::Ref(Ref::Name("b".s()))),
            Box::new(Expr::Ref(Ref::Name("c".s()))),
        ));
    }

    #[test]
    fn check_lambda() {
        let stream = get_all_tokens(b"\\x -> x");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::Lambda(
            vec![Pattern::Var("x".s())],
            Box::new(Expr::Ref(Ref::Name("x".s()))),
        ));
    }

    #[test]
    fn check_case() {
        let stream = get_all_tokens(b"case x of [] -> 0");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::Case(
            Box::new(Expr::Ref(Ref::Name("x".s()))),
            vec![(
                Pattern::List(vec![]),
                Expr::Literal(Literal::Int(0))
            )]
        ));
    }

    #[test]
    fn check_let() {
        let stream = get_all_tokens(b"let x = 5 in x");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::Let(
            vec![
                Definition(None, ValueDefinition::Name(
                    "x".s(),
                    vec![],
                    Expr::Literal(Literal::Int(5))
                ))
            ],
            Box::new(Expr::Ref(Ref::Name("x".s())))
        ));
    }
}
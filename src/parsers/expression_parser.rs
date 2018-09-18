use *;
use parsers::module_parser::read_ref;
use parsers::module_parser::upper_ids;
use parsers::pattern_parser::read_pattern;
use parsers::statement_parser::read_definition;
use tokenizer::Token::*;
use types::Expr;

// Expresions

named!(pub read_expr<Tk, Expr>, do_parse!(
    first: read_expr_app >>
    rest: many0!(tuple!(binop!(), read_expr_app)) >>
    (create_binop_chain(first, rest))
));

named!(read_expr_app<Tk, Expr>, do_parse!(
    first: read_expr_aux >>
    rest: many0!(read_expr_aux) >>
    (rest.into_iter().fold(first, |acc, b| Expr::Application(Box::new(acc), Box::new(b))))
));

//named!(pub read_expr<Tk, Expr>, do_parse!(
//    first: read_expr_chain >>
//    rest: many0!(read_expr_chain) >>
//    (rest.into_iter().fold(first, |acc, b| Expr::Application(Box::new(acc), Box::new(b))))
//));

//named!(read_expr_chain<Tk, Expr>, do_parse!(
//    first: read_expr_aux >>
//    rest: many0!(tuple!(binop!(), read_expr_aux)) >>
//    (create_binop_chain(first, rest))
//));

named!(read_expr_aux<Tk, Expr>, alt!(
    record_field | read_non_rec_field_expr
));

named!(read_non_rec_field_expr<Tk, Expr>, alt!(
    unit            |
    tuple           |
    unit_tuple      |
    list            |
    range           |
    qualified_ref   |
    adt             |
    read_if         |
    read_lambda     |
    read_case       |
    read_let        |
    record          |
    record_update   |
    record_access   |
    map!(literal!(), |c| Expr::Literal(c)) |
    map!(read_ref,   |c| Expr::Ref(c))     |
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

named!(record<Tk, Expr>, do_parse!(
    tk!(LeftBrace) >>
    entries: separated_list!(tk!(Comma), do_parse!(
        id: id!() >>
        tk!(Equals) >>
        expr: read_expr >>
        ((id, expr))
    )) >>
    tk!(RightBrace) >>
    (Expr::Record(entries))
));

named!(read_if<Tk, Expr>, do_parse!(
    tk!(If) >>
    cond: read_expr >>
    tk!(Then) >>
    tru: read_expr >>
    tk!(Else) >>
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
    tk!(Case) >>
    e: read_expr >>
    tk!(Of) >>
    b: many1!(do_parse!(p: read_pattern >> tk!(RightArrow) >> ex: read_expr >> ((p, ex)))) >>
    (Expr::Case(Box::new(e), b))
));

named!(read_let<Tk, Expr>, do_parse!(
    tk!(Let) >>
    a: many1!(read_definition) >>
    tk!(In) >>
    b: read_expr >>
    (Expr::Let(a, Box::new(b)))
));

named!(record_update<Tk, Expr>, do_parse!(
    tk!(LeftBrace) >>
    id: id!() >>
    tk!(Pipe) >>
    updates: separated_nonempty_list!(tk!(Comma), do_parse!(
        id: id!() >>
        tk!(Equals) >>
        expr: read_expr >>
        ((id, expr))
    )) >>
    tk!(RightBrace) >>
    (Expr::RecordUpdate(id, updates))
));

named!(record_access<Tk, Expr>, do_parse!(
    tk!(Dot) >>
    id: id!() >>
    (Expr::RecordAccess(id))
));

named!(record_field<Tk, Expr>, do_parse!(
    e: read_non_rec_field_expr >>
    tk!(Dot) >>
    id: id!() >>
    (Expr::RecordField(Box::new(e), id))
));

named!(qualified_ref<Tk, Expr>, do_parse!(
    e: upper_ids >>
    tk!(Dot) >>
    id: id!() >>
    (Expr::QualifiedRef(e, id))
));

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
        let stream = get_all_tokens(b"let x = 5 in 3");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::Let(
            vec![
                Definition(None, ValueDefinition::Name(
                    "x".s(),
                    vec![],
                    Expr::Literal(Literal::Int(5))
                ))
            ],
            Box::new(Expr::Literal(Literal::Int(3)))
        ));
    }

    #[test]
    fn check_binop_chain() {
        let stream = get_all_tokens(b"1 + 2 + 3 + 4");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::OpChain(vec![
            Expr::Literal(Literal::Int(1)),
            Expr::Literal(Literal::Int(2)),
            Expr::Literal(Literal::Int(3)),
            Expr::Literal(Literal::Int(4)),
        ], vec!["+".s(), "+".s(), "+".s()]
        ));
    }

    #[test]
    fn check_priorities() {
        let stream = get_all_tokens(b"1 * 2 + 3 * 4");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::OpChain(vec![
           Expr::Literal(Literal::Int(1)),
           Expr::Literal(Literal::Int(2)),
           Expr::Literal(Literal::Int(3)),
           Expr::Literal(Literal::Int(4)),
        ], vec!["*".s(), "+".s(), "*".s()]
        ));
    }

    #[test]
    fn check_record_update() {
        let stream = get_all_tokens(b"{ a | b = 0 }");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::RecordUpdate(
            "a".s(),
            vec![("b".s(), Expr::Literal(Literal::Int(0)))]
        ));
    }

    #[test]
    fn check_record_update2() {
        let stream = get_all_tokens(b"{ a | b = 0, c = 1 }");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::RecordUpdate(
            "a".s(),
            vec![
                ("b".s(), Expr::Literal(Literal::Int(0))),
                ("c".s(), Expr::Literal(Literal::Int(1))),
            ]
        ));
    }

    #[test]
    fn check_record_access() {
        let stream = get_all_tokens(b".x");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::RecordAccess("x".s()));
    }

    #[test]
    fn check_record_field() {
        let stream = get_all_tokens(b"{}.x");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::RecordField(
            Box::new(Expr::Record(vec![])),
            "x".s()
        ));
    }

    #[test]
    fn check_qualified_ref() {
        let stream = get_all_tokens(b"List.map");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::QualifiedRef(
            vec!["List".s()],
            "map".s()
        ));
    }

    #[test]
    fn check_function_application() {
        let stream = get_all_tokens(b"my_fun 1");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::Application(
            Box::new(Expr::Ref(Ref::Name("my_fun".s()))),
            Box::new(Expr::Literal(Literal::Int(1)))
        ));
    }

    #[test]
    fn check_function_application2() {
        let stream = get_all_tokens(b"my_fun 1 2");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::Application(
            Box::new(Expr::Application(
                Box::new(Expr::Ref(Ref::Name("my_fun".s()))),
                Box::new(Expr::Literal(Literal::Int(1)))
            )),
            Box::new(Expr::Literal(Literal::Int(2)))
        ));
    }

    #[test]
    fn check_function_application_priority() {
        let stream = get_all_tokens(b"my_fun 1 2 + 3");
        let m = read_expr(&stream);
        assert_ok!(m, Expr::OpChain(
            vec![
                Expr::Application(
                    Box::new(Expr::Application(
                        Box::new(Expr::Ref(Ref::Name("my_fun".s()))),
                        Box::new(Expr::Literal(Literal::Int(1)))
                    )),
                    Box::new(Expr::Literal(Literal::Int(2)))
                ),
                Expr::Literal(Literal::Int(3))
            ],
            vec!["+".s()]
        ));
    }
}
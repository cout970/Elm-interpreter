use ast::*;
use nom::*;
use parsers::old::module::read_ref;
use parsers::old::module::upper_ids;
use parsers::old::pattern::read_pattern;
use parsers::old::statement::read_definition;
use parsers::old::Tk;
use tokenizer::Token::*;
use util::create_vec;
use parsers::old::ParseError;

// Expressions

struct ExprParser {
    indent: Vec<usize>
}

impl ExprParser {
    pub fn new() -> Self {
        ExprParser { indent: vec![0] }
    }

    fn push(self, n: usize) -> Self {
        let mut indent = self.indent;
        indent.push(n);

        ExprParser { indent }
    }

    method_rule!(spaces<ExprParser, ()>, self, do_parse!(
        many0!(indent_except!(self.indent)) >> (())
    ));

    method_rule!(read_expr<ExprParser, Expr>, mut self,
        do_parse!(
            call_m!(self.spaces) >>
            first: call_m!(self.read_expr_app) >>
            rest: many0!(call_m!(self.binop_item)) >>
            (create_binop_chain(first, rest))
        )
    );

    method_rule!(binop_item<ExprParser, (String, Expr)>, mut self,
        do_parse!(
            call_m!(self.spaces) >>
            op: binop!() >>
            call_m!(self.spaces) >>
            ex: call_m!(self.read_expr_app) >>
            ((op, ex))
        )
    );

    method_rule!(read_expr_app<ExprParser, Expr>, mut self,
        do_parse!(
            first: call_m!(self.read_expr_aux) >>
            rest: many0!(call_m!(self.read_next_arg)) >>
            (rest.into_iter().fold(first, |acc, b| Expr::Application(Box::new(acc), Box::new(b))))
        )
    );

    method_rule!(read_next_arg<ExprParser, Expr>, mut self,
        do_parse!(
            call_m!(self.spaces) >>
            e: call_m!(self.read_expr_aux)
            >> (e)
        )
    );

    method_rule!(read_expr_aux<ExprParser, Expr>, mut self,
        alt!( call_m!(self.record_field)
            | call_m!(self.read_non_rec_field_expr)
        )
    );

    method_rule!(record_field<ExprParser, Expr>, mut self,
        do_parse!(
            e: call_m!(self.read_non_rec_field_expr) >>
            tk!(Dot) >>
            id: id!() >>
            (Expr::RecordField(Box::new(e), id))
        )
    );

    method_rule!(read_non_rec_field_expr<ExprParser, Expr>, mut self,
        alt!( call_m!(self.unit)
            | call_m!(self.tuple)
            | call_m!(self.unit_tuple)
            | call_m!(self.list)
            | call_m!(self.qualified_ref)
            | call_m!(self.adt)
            | call_m!(self.read_if)
            | call_m!(self.read_lambda)
            | call_m!(self.read_case)
            | call_m!(self.read_let)
            | call_m!(self.record)
            | call_m!(self.record_update)
            | call_m!(self.record_access)
            | map!(literal!(), |c| Expr::Literal(c))
            | map!(read_ref,   |c| Expr::Ref(c))
            | call_m!(self.unary_minus)
            | do_parse!(tk!(LeftParen) >> e: call_m!(self.read_expr) >> tk!(RightParen) >> (e))
        )
    );

    method_rule!(read_case<ExprParser, Expr>, mut self,
        do_parse!(
            tk!(Case) >>
            e: call_m!(self.read_expr) >>
            tk!(Of) >>
            count: do_parse!(s: indent!() >> ({self = self.push(s as usize); s})) >>
            first: call_m!(self.case_branch) >>
            rest: many0!(do_parse!(indent!(count) >> b: call_m!(self.case_branch) >> (b))) >>
            (Expr::Case(Box::new(e), create_vec(first, rest)))
        )
    );

    method_rule!(case_branch<ExprParser, (Pattern, Expr)>, mut self, do_parse!(
        p: read_pattern >>
        tk!(RightArrow) >>
        ex: call_m!(self.read_expr) >>
        ((p, ex))
    ));

    method_rule!(unit<ExprParser, Expr>, self, do_parse!(
        tk!(LeftParen) >> tk!(RightParen) >> (Expr::Unit)
    ));

    method_rule!(tuple<ExprParser, Expr>, mut self, do_parse!(
        tk!(LeftParen) >>
        a: call_m!(self.read_expr) >>
        call_m!(self.comma_separator) >>
        list: separated_nonempty_list!(call_m!(self.comma_separator), call_m!(self.read_expr)) >>
        tk!(RightParen) >>
        (Expr::Tuple(create_vec(a, list)))
    ));

    method_rule!(unit_tuple<ExprParser, Expr>, mut self, do_parse!(
        tk!(LeftParen) >>
        list: one_or_more!(call_m!(self.comma_separator)) >>
        tk!(RightParen) >>
        (Expr::Tuple(create_vec(Expr::Unit, list.into_iter().map(|_c| Expr::Unit).collect())))
    ));

    method_rule!(comma_separator<ExprParser, ()>, mut self, do_parse!(
        call_m!(self.spaces) >>
        tk!(Comma) >>
        call_m!(self.spaces) >>
        (())
    ));

    method_rule!(list<ExprParser, Expr>, mut self, do_parse!(
        tk!(LeftBracket) >>
        list: separated_list!(call_m!(self.comma_separator), call_m!(self.read_expr)) >>
        call_m!(self.spaces) >>
        tk!(RightBracket) >>
        (Expr::List(list))
    ));

    method_rule!(adt<ExprParser, Expr>, self, do_parse!(
        a: upper_id!() >> (Expr::Adt(a))
    ));

    method_rule!(record<ExprParser, Expr>, mut self, do_parse!(
        tk!(LeftBrace) >>
        call_m!(self.spaces) >>
        entries: separated_list!(call_m!(self.comma_separator), do_parse!(
            call_m!(self.spaces) >>
            id: id!() >>
            tk!(Equals) >>
            expr: call_m!(self.read_expr) >>
            ((id, expr))
        )) >>
        call_m!(self.spaces) >>
        tk!(RightBrace) >>
        (Expr::Record(entries))
    ));

    method_rule!(read_if<ExprParser, Expr>, mut self, do_parse!(
        tk!(If) >>
        cond: call_m!(self.read_expr) >>
        tk!(Then) >>
        tru: call_m!(self.read_expr) >>
        tk!(Else) >>
        fal: call_m!(self.read_expr) >>
        (Expr::If(Box::new(cond), Box::new(tru), Box::new(fal)))
    ));

    method_rule!(read_lambda<ExprParser, Expr>, mut self, do_parse!(
        tk!(BackSlash) >>
        p: one_or_more!(read_pattern) >>
        tk!(RightArrow) >>
        expr: call_m!(self.read_expr) >>
        (Expr::Lambda(p, Box::new(expr)))
    ));

    method_rule!(read_let<ExprParser, Expr>, mut self, do_parse!(
        call_m!(self.spaces) >>
        tk!(Let) >>
        call_m!(self.spaces) >>
        a: one_or_more!( do_parse!(
            call_m!(self.spaces) >>
            e: read_definition >>
            (e)
        )) >>
        call_m!(self.spaces) >>
        tk!(In) >>
        call_m!(self.spaces) >>
        b: call_m!(self.read_expr) >>
        (Expr::Let(a, Box::new(b)))
    ));

    method_rule!(record_update<ExprParser, Expr>, mut self, do_parse!(
        tk!(LeftBrace) >>
        id: id!() >>
        tk!(Pipe) >>
        updates: separated_nonempty_list!(call_m!(self.comma_separator), do_parse!(
            id: id!() >>
            tk!(Equals) >>
            expr: call_m!(self.read_expr) >>
            ((id, expr))
        )) >>
        tk!(RightBrace) >>
        (Expr::RecordUpdate(id, updates))
    ));

    method_rule!(record_access<ExprParser, Expr>, self, do_parse!(
        tk!(Dot) >>
        id: id!() >>
        (Expr::RecordAccess(id))
    ));

    method_rule!(unary_minus<ExprParser, Expr>, mut self, do_parse!(
        minus!() >>
        e: call_m!(self.read_expr) >>
        (Expr::Application(
            Box::new(Expr::Ref(String::from("-"))),
            Box::new(e),
        ))
    ));


    method_rule!(qualified_ref<ExprParser, Expr>, self, do_parse!(
        e: upper_ids >>
        tk!(Dot) >>
        id: id!() >>
        (Expr::QualifiedRef(e, id))
    ));
}

// independent methods

pub fn read_expr(i: Tk) -> IResult<Tk, Expr, ParseError> {
    let (_, m) = ExprParser::new().read_expr(i);
    m
}

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
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use tokenizer::tokenize;
//    use util::StringConversion;
//    use tokenizer::TokenStream;
//
//    #[test]
//    fn check_unit() {
//        let p = ExprParser::new();
//        let tokens = tokenize(b"()").unwrap();
//        let (_, m) = p.read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Unit);
//    }
//
//    #[test]
//    fn check_parens() {
//        let tokens = tokenize(b"(a)").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Ref("a".s()));
//    }
//
//    #[test]
//    fn check_tuple() {
//        let tokens = tokenize(b"(a, b)").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Tuple(vec![
//            Expr::Ref("a".s()),
//            Expr::Ref("b".s())])
//        );
//    }
//
//    #[test]
//    fn check_list() {
//        let tokens = tokenize(b"[a, b]").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::List(vec![
//            Expr::Ref("a".s()),
//            Expr::Ref("b".s())])
//        );
//    }
//
//    #[test]
//    fn check_empty_list() {
//        let tokens = tokenize(b"[]").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::List(vec![]));
//    }
//
//    #[test]
//    fn check_unit_tuple() {
//        let tokens = tokenize(b"(,)").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Tuple(vec![
//            Expr::Unit,
//            Expr::Unit
//        ]));
//    }
//
//    #[test]
//    fn check_if() {
//        let tokens = tokenize(b"if a then b else c").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::If(
//            Box::new(Expr::Ref("a".s())),
//            Box::new(Expr::Ref("b".s())),
//            Box::new(Expr::Ref("c".s())),
//        ));
//    }
//
//    #[test]
//    fn check_lambda() {
//        let tokens = tokenize(b"\\x -> x").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Lambda(
//            vec![Pattern::Var("x".s())],
//            Box::new(Expr::Ref("x".s())),
//        ));
//    }
//
//    #[test]
//    fn check_case() {
//        let tokens = tokenize(b"case x of\n  [] -> 0\n  _ -> 1").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Case(
//            Box::new(Expr::Ref("x".s())),
//            vec![(
//                Pattern::List(vec![]),
//                Expr::Literal(Literal::Int(0))
//            ),(
//                Pattern::Wildcard,
//                Expr::Literal(Literal::Int(1))
//            )]
//        ));
//    }
//
//    #[test]
//    fn check_let() {
//        let tokens = tokenize(b"let x = 5 in 3").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Let(
//            vec![
//                Definition {
//                    header: None,
//                    name: "x".s(),
//                    patterns: vec![],
//                    expr: Expr::Literal(Literal::Int(5))
//                }
//            ],
//            Box::new(Expr::Literal(Literal::Int(3)))
//        ));
//    }
//
//    #[test]
//    fn check_binop_chain() {
//        let tokens = tokenize(b"1 + 2 + 3 + 4").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::OpChain(vec![
//            Expr::Literal(Literal::Int(1)),
//            Expr::Literal(Literal::Int(2)),
//            Expr::Literal(Literal::Int(3)),
//            Expr::Literal(Literal::Int(4)),
//        ], vec!["+".s(), "+".s(), "+".s()]
//        ));
//    }
//
//
//    #[test]
//    fn check_binop_chain_multiline() {
//        let tokens = tokenize(b"1 + \n 2 + \n 3 + \n 4").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::OpChain(vec![
//            Expr::Literal(Literal::Int(1)),
//            Expr::Literal(Literal::Int(2)),
//            Expr::Literal(Literal::Int(3)),
//            Expr::Literal(Literal::Int(4)),
//        ], vec!["+".s(), "+".s(), "+".s()]
//        ));
//    }
//
//    #[test]
//    fn check_priorities() {
//        let tokens = tokenize(b"1 * 2 + 3 * 4").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::OpChain(vec![
//           Expr::Literal(Literal::Int(1)),
//           Expr::Literal(Literal::Int(2)),
//           Expr::Literal(Literal::Int(3)),
//           Expr::Literal(Literal::Int(4)),
//        ], vec!["*".s(), "+".s(), "*".s()]
//        ));
//    }
//
//    #[test]
//    fn check_record_update() {
//        let tokens = tokenize(b"{ a | b = 0 }").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::RecordUpdate(
//            "a".s(),
//            vec![("b".s(), Expr::Literal(Literal::Int(0)))]
//        ));
//    }
//
//    #[test]
//    fn check_record_update2() {
//        let tokens = tokenize(b"{ a | b = 0, c = 1 }").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::RecordUpdate(
//            "a".s(),
//            vec![
//                ("b".s(), Expr::Literal(Literal::Int(0))),
//                ("c".s(), Expr::Literal(Literal::Int(1))),
//            ]
//        ));
//    }
//
//    #[test]
//    fn check_record_access() {
//        let tokens = tokenize(b".x").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::RecordAccess("x".s()));
//    }
//
//    #[test]
//    fn check_record_field() {
//        let tokens = tokenize(b"{}.x").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::RecordField(
//            Box::new(Expr::Record(vec![])),
//            "x".s()
//        ));
//    }
//
//    #[test]
//    fn check_qualified_ref() {
//        let tokens = tokenize(b"List.map").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::QualifiedRef(
//            vec!["List".s()],
//            "map".s()
//        ));
//    }
//
//    #[test]
//    fn check_function_application() {
//        let tokens = tokenize(b"my_fun 1").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Application(
//            Box::new(Expr::Ref("my_fun".s())),
//            Box::new(Expr::Literal(Literal::Int(1)))
//        ));
//    }
//
//    #[test]
//    fn check_function_application2() {
//        let tokens = tokenize(b"my_fun 1 2").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Application(
//            Box::new(Expr::Application(
//                Box::new(Expr::Ref("my_fun".s())),
//                Box::new(Expr::Literal(Literal::Int(1)))
//            )),
//            Box::new(Expr::Literal(Literal::Int(2)))
//        ));
//    }
//
//    #[test]
//    fn check_function_application_priority() {
//        let tokens = tokenize(b"my_fun 1 2 + 3").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::OpChain(
//            vec![
//                Expr::Application(
//                    Box::new(Expr::Application(
//                        Box::new(Expr::Ref("my_fun".s())),
//                        Box::new(Expr::Literal(Literal::Int(1)))
//                    )),
//                    Box::new(Expr::Literal(Literal::Int(2)))
//                ),
//                Expr::Literal(Literal::Int(3))
//            ],
//            vec!["+".s()]
//        ));
//    }
//
//    #[test]
//    fn check_multiline_expr() {
//        let tokens = tokenize(b"my_fun []\n  []").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m,
//        Expr::Application(
//            Box::new(Expr::Application(
//                Box::new(Expr::Ref("my_fun".s())),
//                Box::new(Expr::List(vec![]))
//            )),
//            Box::new(Expr::List(vec![]))
//        )
//        );
//    }
//
//    #[test]
//    fn check_case_indentation() {
//        let tokens = tokenize(b"\
//case msg of\n    Increment ->\n        model + 1\n    Decrement ->\n        model - 1\
//        ").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Case(
//                Box::new(Expr::Ref("msg".s())),
//                vec![
//                    (
//                        Pattern::Adt("Increment".s(), vec![]),
//                        Expr::OpChain(
//                            vec![Expr::Ref("model".s()), Expr::Literal(Literal::Int(1))],
//                            vec!["+".s()]
//                        )
//                    ),
//                    (
//                        Pattern::Adt("Decrement".s(), vec![]),
//                        Expr::OpChain(
//                            vec![Expr::Ref("model".s()), Expr::Literal(Literal::Int(1))],
//                            vec!["-".s()]
//                        )
//                    )
//                ]
//            )
//        );
//    }
//
//    #[test]
//    fn check_prefix_minus() {
//        let tokens = tokenize(b"-(1+2)").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Application(
//            Box::from(Expr::Ref("-".s())),
//            Box::from(Expr::OpChain(
//                vec![Expr::Literal(Literal::Int(1)), Expr::Literal(Literal::Int(2))],
//                vec!["+".s()],
//            ))
//        ));
//    }
//
//    #[test]
//    fn check_infix_minus() {
//        let tokens = tokenize(b"1 - 2").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::OpChain(
//            vec![Expr::Literal(Literal::Int(1)), Expr::Literal(Literal::Int(2))],
//            vec!["-".s()],
//        ));
//    }
//
//    #[test]
//    fn check_infix_minus_precedence() {
//        let tokens = tokenize(b"1 -2").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Application(
//            Box::new(Expr::Literal(Literal::Int(1))),
//            Box::new(Expr::Application(
//                Box::new(Expr::Ref("-".s())),
//                Box::new(Expr::Literal(Literal::Int(2))),
//            )),
//        ));
//    }
//
//    #[test]
//    fn check_infix_minus_validity() {
//        let tokens = tokenize(b"1- 2").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::OpChain(
//            vec![Expr::Literal(Literal::Int(1)), Expr::Literal(Literal::Int(2))],
//            vec!["-".s()],
//        ));
//    }
//
//    /**
//     * This is a weird behavior of the lang, it's uncommon, so I will just ignore it
//     **/
//    #[test]
//    #[ignore]
//    fn check_infix_minus_edge_case() {
//        let tokens = tokenize(b"1-2").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::OpChain(
//            vec![Expr::Literal(Literal::Int(1)), Expr::Literal(Literal::Int(2))],
//            vec!["-".s()],
//        ));
//    }
//
//    #[test]
//    fn check_multiline_expr2() {
//        let tokens = tokenize(b"Browser.element\n    { init = init\n    , view = view\n    , update = update\n    , subscriptions = subscriptions\n    }\n").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Application(
//            Box::from(Expr::QualifiedRef(
//                vec![
//                    "Browser".s()
//                ],
//                "element".s(),
//            )),
//            Box::from(Expr::Record(
//                vec![
//                    ("init".s(), Expr::Ref("init".s())),
//                    ("view".s(), Expr::Ref("view".s())),
//                    ("update".s(), Expr::Ref("update".s())),
//                    ("subscriptions".s(), Expr::Ref("subscriptions".s()))
//                ]
//            ))
//        ));
//    }
//
//    #[test]
//    #[ignore]
//    fn check_multiline_expr3() {
//        let tokens = tokenize(b"let \
//     row x = \
//        List.range 0 x \
//        |> List.map (\\y -> Cell Dirt ) \
// \
//     column x y = \
//         List.range 0 y \
//         |> List.map (\\s -> row x) \
// in \
//    { cells = (column size size) \
//    , entities = [] \
//    }").unwrap();
//        let m = read_expr(TokenStream::new(&tokens));
//        assert_ok!(m, Expr::Unit);
//    }
//}
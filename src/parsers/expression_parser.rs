use *;

// Expresions

//named!(pub pr_expr<Expr>, call!(pr_binop_chain));
//
//
//named!(pub pr_binop_chain<Expr>, do_parse!(
//    head: pr_expr_part >> s >>
//    chain: many0!(preceded!(s, tuple!(pr_bin_op, pr_expr_part))) >>
//    (chain.into_iter().rev().fold(head, |a, b| Expr::BinaryOp(Box::new(a), b.0, Box::new(b.1))))
//));
//
//named!(pub pr_expr_part<Expr>, alt!(
//    unit |
//    braket |
//    do_parse!(s >> char!('(') >> e: separated_nonempty_list!(char!(','), preceded!(s, pr_expr)) >> char!(')') >> (Expr::Tuple(e))) |
//    do_parse!(s >> char!('[') >> e: separated_list!(char!(','), preceded!(s, pr_expr)) >> char!(']') >> (Expr::List(e))) |
//    do_parse!(s >> char!('[') >> a: pr_expr >> tag!("..") >> b: pr_expr >> char!(']') >> (Expr::Range(Box::new(a), Box::new(b)))) |
//    do_parse!(s >> char!('{') >> e: separated_list!(char!(','), preceded!(s, pr_record_binding)) >> char!('}') >> (Expr::Record(e))) |
//    do_parse!(s >> char!('(') >> e: many1!(preceded!(s, char!(','))) >> char!(')') >> (Expr::Tuple(create_vec(Expr::Unit, e.into_iter().map(|_i| Expr::Unit).collect())))) |
//
//    do_parse!(s >> tag!(" .") >> a: pr_id >> (Expr::Getter(a))) |
//    do_parse!(s >> a: pr_upper_ids >> char!('.') >> b: pr_id >> (Expr::QualifiedRef(a, b))) |
//
//    do_parse!(s >> tag!("if") >> s >> c: pr_expr >> s >> tag!("then") >> s >> t: pr_expr >> s >> tag!("else") >> s >> f: pr_expr >> (Expr::If(Box::new(c), Box::new(t), Box::new(f)))) |
//
//    do_parse!(s >> e: pr_upper_id >> (Expr::ADT(e))) |
//    do_parse!(s >> e: pr_literal >> (Expr::Literal(e))) |
//    do_parse!(s >> e: pr_ref >> (Expr::Ref(e)))
//));
//
//named!(unit<Expr>, do_parse!(
//    char!('(') >> char!(')') >> (Expr::Unit)
//));
//
//named!(braket<Expr>, do_parse!(
//    char!('(') >> e: pr_expr >> char!(')') >> (e)
//));
//
//named!(tuple<Expr>, do_parse!(
//    char!('(') >>
//    e: separated_nonempty_list!(char!(','), preceded!(s, pr_expr)) >>
//    char!(')') >>
//    (Expr::Tuple(e))
//));
//
//
//
//
//named!(pub pr_record_binding<(String, Expr)>, do_parse!(
//    i: pr_id >>
//    char!('=') >>
//    e: pr_expr >>
//    ((i, e))
//));
use *;
use tokenizer::Token;
use tokenizer::Token::*;
use types::*;
use util::create_vec;

named!(pub read_pattern<Tk, Pattern>,  alt!(
    wildcard |
    variable |
    adt      |
    unit     |
    tuple    |
    list     |
    record   |
    literal
));

named!(variable<Tk, Pattern>, map!(id!(), |l| Pattern::Var(l)));

named!(adt<Tk, Pattern>, do_parse!(
    id: upper_id!() >>
    rest: many0!(read_pattern) >>
    (Pattern::Adt(id, rest))
));

named!(wildcard<Tk, Pattern>, map!(tk!(Underscore), |_c| Pattern::Wildcard));

named!(unit<Tk, Pattern>, do_parse!(
    tk!(LeftParen) >> tk!(RightParen) >> (Pattern::Unit)
));

named!(tuple<Tk, Pattern>, do_parse!(
    tk!(LeftParen) >>
    a: read_pattern >>
    comma_separator >>
    b: separated_nonempty_list!(comma_separator, read_pattern) >>
    tk!(RightParen) >>
    (Pattern::Tuple(create_vec(a, b)))
));

named!(list<Tk, Pattern>, do_parse!(
    tk!(LeftBracket) >>
    list: separated_list!(comma_separator, read_pattern) >>
    tk!(RightBracket) >>
    (Pattern::List(list))
));

named!(record<Tk, Pattern>, do_parse!(
    tk!(LeftBrace) >>
    list: separated_list!(comma_separator, id!()) >>
    tk!(RightBrace) >>
    (Pattern::Record(list))
));

// TODO binop pattern Ex. first :: rest
named!(comma_separator<Tk, ()>, do_parse!(
   spaces >>
   tk!(Comma) >>
   spaces >>
   (())
));

named!(spaces<Tk, ()>, do_parse!(
    many0!(indent!()) >> (())
));

named!(literal<Tk, Pattern>, map!(literal!(), |l| Pattern::Literal(l)));

#[cfg(test)]
mod tests {
    use nom::*;
    use super::*;
    use tokenizer::get_all_tokens;

    #[test]
    fn check_literal() {
        let stream = get_all_tokens(b"1");
        let m = read_pattern(&stream);
        assert_ok!(m, Pattern::Literal(Literal::Int(1)));
    }

    #[test]
    fn check_variable() {
        let stream = get_all_tokens(b"variable");
        let m = read_pattern(&stream);
        assert_ok!(m, Pattern::Var("variable".s()));
    }

    #[test]
    fn check_algebraic_data_type() {
        let stream = get_all_tokens(b"List a");
        let m = read_pattern(&stream);
        assert_ok!(m, Pattern::Adt("List".s(), vec![Pattern::Var("a".s())]));
    }

    #[test]
    fn check_wildcard() {
        let stream = get_all_tokens(b"_");
        let m = read_pattern(&stream);
        assert_ok!(m, Pattern::Wildcard);
    }

    #[test]
    fn check_unit() {
        let stream = get_all_tokens(b"()");
        let m = read_pattern(&stream);
        assert_ok!(m, Pattern::Unit);
    }

    #[test]
    fn check_tuple() {
        let stream = get_all_tokens(b"(a, b)");
        let m = read_pattern(&stream);
        assert_ok!(m, Pattern::Tuple(vec![
            Pattern::Var("a".s()), Pattern::Var("b".s())
        ]));
    }

    #[test]
    fn check_empty_list() {
        let stream = get_all_tokens(b"[]");
        let m = read_pattern(&stream);
        assert_ok!(m, Pattern::List(vec![]));
    }

    #[test]
    fn check_list() {
        let stream = get_all_tokens(b"[a, b]");
        let m = read_pattern(&stream);
        assert_ok!(m, Pattern::List(vec![
            Pattern::Var("a".s()), Pattern::Var("b".s())
        ]));
    }

    #[test]
    fn check_record() {
        let stream = get_all_tokens(b"{ a, b }");
        let m = read_pattern(&stream);
        assert_ok!(m, Pattern::Record(
            vec!["a".s(), "b".s()]
        ));
    }
}

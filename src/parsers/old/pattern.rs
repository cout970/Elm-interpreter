use tokenizer::Token::*;
use ast::*;
use parsers::old::Tk;
use util::create_vec;
use parsers::old::ParseError;

rule!(pub read_pattern<Pattern>,  alt!(
    wildcard |
    variable |
    adt      |
    unit     |
    tuple    |
    list     |
    record   |
    literal
));

rule!(variable<Pattern>, map!(id!(), |l| Pattern::Var(l)));

rule!(adt<Pattern>, do_parse!(
    id: upper_id!() >>
    rest: many0!(read_pattern) >>
    (Pattern::Adt(id, rest))
));

rule!(wildcard<Pattern>, map!(tk!(Underscore), |_c| Pattern::Wildcard));

rule!(unit<Pattern>, do_parse!(
    tk!(LeftParen) >> tk!(RightParen) >> (Pattern::Unit)
));

rule!(tuple<Pattern>, do_parse!(
    tk!(LeftParen) >>
    a: read_pattern >>
    comma_separator >>
    b: separated_nonempty_list!(comma_separator, read_pattern) >>
    tk!(RightParen) >>
    (Pattern::Tuple(create_vec(a, b)))
));

rule!(list<Pattern>, do_parse!(
    tk!(LeftBracket) >>
    list: separated_list!(comma_separator, read_pattern) >>
    tk!(RightBracket) >>
    (Pattern::List(list))
));

rule!(record<Pattern>, do_parse!(
    tk!(LeftBrace) >>
    list: separated_list!(comma_separator, id!()) >>
    tk!(RightBrace) >>
    (Pattern::Record(list))
));

// TODO binop pattern Ex. first :: rest
rule!(comma_separator<()>, do_parse!(
   spaces >>
   tk!(Comma) >>
   spaces >>
   (())
));

rule!(spaces<()>, do_parse!(
    many0!(indent!()) >> (())
));

// Removed Pattern::Literal and added LitInt, LitString, LitChar
// This is old code and it's not worth updating
rule!(literal<Pattern>, map!(literal!(), |l| Pattern::LitInt(1)));

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use tokenizer::tokenize;
//    use util::StringConversion;
//    use tokenizer::TokenStream;
//
//    #[test]
//    fn check_literal() {
//        let tokens = tokenize(b"1").unwrap();
//        let m = read_pattern(TokenStream::new(&tokens));
//        assert_ok!(m, Pattern::Literal(Literal::Int(1)));
//    }
//
//    #[test]
//    fn check_variable() {
//        let tokens = tokenize(b"variable").unwrap();
//        let m = read_pattern(TokenStream::new(&tokens));
//        assert_ok!(m, Pattern::Var("variable".s()));
//    }
//
//    #[test]
//    fn check_algebraic_data_type() {
//        let tokens = tokenize(b"List a").unwrap();
//        let m = read_pattern(TokenStream::new(&tokens));
//        assert_ok!(m, Pattern::Adt("List".s(), vec![Pattern::Var("a".s())]));
//    }
//
//    #[test]
//    fn check_wildcard() {
//        let tokens = tokenize(b"_").unwrap();
//        let m = read_pattern(TokenStream::new(&tokens));
//        assert_ok!(m, Pattern::Wildcard);
//    }
//
//    #[test]
//    fn check_unit() {
//        let tokens = tokenize(b"()").unwrap();
//        let m = read_pattern(TokenStream::new(&tokens));
//        assert_ok!(m, Pattern::Unit);
//    }
//
//    #[test]
//    fn check_tuple() {
//        let tokens = tokenize(b"(a, b)").unwrap();
//        let m = read_pattern(TokenStream::new(&tokens));
//        assert_ok!(m, Pattern::Tuple(vec![
//            Pattern::Var("a".s()), Pattern::Var("b".s())
//        ]));
//    }
//
//    #[test]
//    fn check_empty_list() {
//        let tokens = tokenize(b"[]").unwrap();
//        let m = read_pattern(TokenStream::new(&tokens));
//        assert_ok!(m, Pattern::List(vec![]));
//    }
//
//    #[test]
//    fn check_list() {
//        let tokens = tokenize(b"[a, b]").unwrap();
//        let m = read_pattern(TokenStream::new(&tokens));
//        assert_ok!(m, Pattern::List(vec![
//            Pattern::Var("a".s()), Pattern::Var("b".s())
//        ]));
//    }
//
//    #[test]
//    fn check_record() {
//        let tokens = tokenize(b"{ a, b }").unwrap();
//        let m = read_pattern(TokenStream::new(&tokens));
//        assert_ok!(m, Pattern::Record(
//            vec!["a".s(), "b".s()]
//        ));
//    }
//}

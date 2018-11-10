use *;
use tokenizer::Token::*;
use ast::*;
use parsers::Tk;
use util::create_vec;
use parsers::SyntaxError;

// Types

rule!(pub read_type<Type>, do_parse!(
    a: func_less_type >>
    b: many0!(do_parse!(tk!(RightArrow) >> f: func_less_type >> (f))) >>
    (create_fun(a, b))
));

rule!(func_less_type<Type>, alt!(
    tag         |
    variable    |
    unit        |
    tuple       |
    record      |
    ext_record  |
    do_parse!(tk!(LeftParen) >> t: read_type >> tk!(RightParen) >> (t))
));

rule!(unit<Type>, do_parse!(
    tk!(LeftParen) >> tk!(RightParen) >> (Type::Unit)
));

rule!(tuple<Type>, do_parse!(
    tk!(LeftParen) >>
    a: read_type >>
    tk!(Comma) >>
    b: separated_nonempty_list!(tk!(Comma), read_type) >>
    tk!(RightParen) >>
    (Type::Tuple(create_vec(a, b)))
));

rule!(variable<Type>, do_parse!(
    i: id!() >> (Type::Var(i))
));

rule!(tag<Type>, do_parse!(
    i: upper_id!() >>
    t: many0!(read_type) >>
    (Type::Tag(i, t))
));

rule!(record<Type>, do_parse!(
    tk!(LeftBrace) >>
    l: separated_list!(tk!(Comma), record_binding) >>
    tk!(RightBrace) >>
    (Type::Record(l))
));

rule!(ext_record<Type>, do_parse!(
    tk!(LeftBrace) >>
    i: id!() >>
    tk!(Pipe) >>
    l: separated_list!(tk!(Comma), record_binding) >>
    tk!(RightBrace) >>
    (Type::RecExt(i, l))
));


rule!(record_binding<(String, Type)>, do_parse!(
    a: id!() >> tk!(Colon) >> b: read_type >> ((a, b))
));

fn create_fun(a: Type, b: Vec<Type>) -> Type {
    if b.is_empty() { return a; }

    let c = create_vec(a, b);
    let mut iter = c.into_iter().rev();

    iter.next().map(|first| iter.fold(first, |acc, b|
        Type::Fun(Box::new(b), Box::new(acc))
    )).unwrap()
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use tokenizer::tokenize;
//    use util::StringConversion;
//    use tokenizer::TokenStream;
//
//    #[test]
//    fn check_unit() {
//        let tokens = tokenize(b"()").unwrap();
//        let m = read_type(TokenStream::new(&tokens));
//        assert_ok!(m, Type::Unit);
//    }
//
//    #[test]
//    fn check_variable() {
//        let tokens = tokenize(b"a").unwrap();
//        let m = read_type(TokenStream::new(&tokens));
//        assert_ok!(m, Type::Var("a".s()));
//    }
//
//    #[test]
//    fn check_tag() {
//        let tokens = tokenize(b"List a").unwrap();
//        let m = read_type(TokenStream::new(&tokens));
//        assert_ok!(m, Type::Tag("List".s(), vec![Type::Var("a".s())]));
//    }
//
//    #[test]
//    fn check_tuple2() {
//        let tokens = tokenize(b"(a,b)").unwrap();
//        let m = read_type(TokenStream::new(&tokens));
//        assert_ok!(m, Type::Tuple(vec![Type::Var("a".s()), Type::Var("b".s())]));
//    }
//
//    #[test]
//    fn check_tuple6() {
//        let tokens = tokenize(b"(a,b,c,d,e,f)").unwrap();
//        let m = read_type(TokenStream::new(&tokens));
//        assert_ok!(m, Type::Tuple(vec![
//            Type::Var("a".s()),
//            Type::Var("b".s()),
//            Type::Var("c".s()),
//            Type::Var("d".s()),
//            Type::Var("e".s()),
//            Type::Var("f".s()),
//        ]));
//    }
//
//    #[test]
//    fn check_empty_record() {
//        let tokens = tokenize(b"{}").unwrap();
//        let m = read_type(TokenStream::new(&tokens));
//        assert_ok!(m, Type::Record(vec![]));
//    }
//
//    #[test]
//    fn check_record() {
//        let tokens = tokenize(b"{ a: b }").unwrap();
//        let m = read_type(TokenStream::new(&tokens));
//        assert_ok!(m, Type::Record(vec![("a".s(), Type::Var("b".s()))]));
//    }
//
//    #[test]
//    fn check_ext_record() {
//        let tokens = tokenize(b"{ list | a: b }").unwrap();
//        let m = read_type(TokenStream::new(&tokens));
//        assert_ok!(m, Type::RecExt("list".s(), vec![("a".s(), Type::Var("b".s()))]));
//    }
//
//    #[test]
//    fn check_paren() {
//        let tokens = tokenize(b"(a)").unwrap();
//        let m = read_type(TokenStream::new(&tokens));
//        assert_ok!(m, Type::Var("a".s()));
//    }
//
//    #[test]
//    fn check_function() {
//        let tokens = tokenize(b"Int -> Float -> a").unwrap();
//        let m = read_type(TokenStream::new(&tokens));
//        assert_ok!(m, Type::Fun(
//            Box::new(Type::Tag("Int".s(), vec![])),
//            Box::new(Type::Fun(
//                Box::new(Type::Tag("Float".s(), vec![])),
//                Box::new(Type::Var("a".s()))
//            ))
//        ));
//    }
//}

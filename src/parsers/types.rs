use *;
use tokenizer::Token::*;
use types::*;
use parsers::Tk;
use util::create_vec;

// Types

named!(pub read_type<Tk, Type>, do_parse!(
    a: func_less_type >>
    b: many0!(do_parse!(tk!(RightArrow) >> f: func_less_type >> (f))) >>
    (create_fun(a, b))
));

named!(func_less_type<Tk, Type>, alt!(
    tag         |
    variable    |
    unit        |
    tuple       |
    record      |
    ext_record  |
    do_parse!(tk!(LeftParen) >> t: read_type >> tk!(RightParen) >> (t))
));

named!(unit<Tk, Type>, do_parse!(
    tk!(LeftParen) >> tk!(RightParen) >> (Type::Unit)
));

named!(tuple<Tk, Type>, do_parse!(
    tk!(LeftParen) >>
    a: read_type >>
    tk!(Comma) >>
    b: separated_nonempty_list!(tk!(Comma), read_type) >>
    (Type::Tuple(create_vec(a, b)))
));

named!(variable<Tk, Type>, do_parse!(
    i: id!() >> (Type::Var(i))
));

named!(tag<Tk, Type>, do_parse!(
    i: upper_id!() >>
    t: many0!(read_type) >>
    (Type::Tag(i, t))
));

named!(record<Tk, Type>, do_parse!(
    tk!(LeftBrace) >>
    l: separated_list!(tk!(Comma), record_binding) >>
    tk!(RightBrace) >>
    (Type::Record(l))
));

named!(ext_record<Tk, Type>, do_parse!(
    tk!(LeftBrace) >>
    i: id!() >>
    tk!(Pipe) >>
    l: separated_list!(tk!(Comma), record_binding) >>
    tk!(RightBrace) >>
    (Type::RecExt(i, l))
));


named!(record_binding<Tk, (String, Type)>, do_parse!(
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

#[cfg(test)]
mod tests {
    use nom::*;
    use super::*;
    use tokenizer::tokenize;
    use util::StringConversion;

    #[test]
    fn check_unit() {
        let stream = tokenize(b"()");
        let m = read_type(&stream);
        assert_ok!(m, Type::Unit);
    }

    #[test]
    fn check_variable() {
        let stream = tokenize(b"a");
        let m = read_type(&stream);
        assert_ok!(m, Type::Var("a".s()));
    }

    #[test]
    fn check_tag() {
        let stream = tokenize(b"List a");
        let m = read_type(&stream);
        assert_ok!(m, Type::Tag("List".s(), vec![Type::Var("a".s())]));
    }

    #[test]
    fn check_tuple2() {
        let stream = tokenize(b"(a,b)");
        let m = read_type(&stream);
        assert_ok!(m, Type::Tuple(vec![Type::Var("a".s()), Type::Var("b".s())]));
    }

    #[test]
    fn check_tuple6() {
        let stream = tokenize(b"(a,b,c,d,e,f)");
        let m = read_type(&stream);
        assert_ok!(m, Type::Tuple(vec![
            Type::Var("a".s()),
            Type::Var("b".s()),
            Type::Var("c".s()),
            Type::Var("d".s()),
            Type::Var("e".s()),
            Type::Var("f".s()),
        ]));
    }

    #[test]
    fn check_empty_record() {
        let stream = tokenize(b"{}");
        let m = read_type(&stream);
        assert_ok!(m, Type::Record(vec![]));
    }

    #[test]
    fn check_record() {
        let stream = tokenize(b"{ a: b }");
        let m = read_type(&stream);
        assert_ok!(m, Type::Record(vec![("a".s(), Type::Var("b".s()))]));
    }

    #[test]
    fn check_ext_record() {
        let stream = tokenize(b"{ list | a: b }");
        let m = read_type(&stream);
        assert_ok!(m, Type::RecExt("list".s(), vec![("a".s(), Type::Var("b".s()))]));
    }

    #[test]
    fn check_paren() {
        let stream = tokenize(b"(a)");
        let m = read_type(&stream);
        assert_ok!(m, Type::Var("a".s()));
    }

    #[test]
    fn check_function() {
        let stream = tokenize(b"Int -> Float -> a");
        let m = read_type(&stream);
        assert_ok!(m, Type::Fun(
            Box::new(Type::Tag("Int".s(), vec![])),
            Box::new(Type::Fun(
                Box::new(Type::Tag("Float".s(), vec![])),
                Box::new(Type::Var("a".s()))
            ))
        ));
    }
}

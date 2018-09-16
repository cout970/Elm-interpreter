use *;
use tokenizer::Token;
use tokenizer::Token::*;
use types::*;

named!(pub read_pattern<Tk, Pattern>, alt!(
    variable |
    adt |
    literal
));

named!(variable<Tk, Pattern>, map!(id!(), |l| Pattern::Var(l)));

named!(adt<Tk, Pattern>, do_parse!(
    id: upper_id!() >>
    rest: many0!(read_pattern) >>
    (Pattern::Adt(id, rest))
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
}

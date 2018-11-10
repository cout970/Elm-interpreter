use ast::Definition;
use ast::Statement;
use ast::Type;
use parsers::new::expression::parse_expr;
use parsers::new::Input;
use parsers::new::ParseError;
use parsers::new::pattern::parse_pattern;
use parsers::new::types::parse_type;
use parsers::new::util::expect;
use parsers::new::util::expect_id;
use parsers::new::util::expect_indent;
use parsers::new::util::expect_upper;
use parsers::new::util::many0;
use parsers::new::util::pipe1;
use tokenizer::Token;

pub fn parse_statement(input: Input) -> Result<(Statement, Input), ParseError> {
    let (stm, i) = match input.read() {
        Token::TypeTk => {
            let i = input.next();
            if let Token::Alias = i.read() {
                //type alias
                let (name, i) = expect_upper(i.next())?;
                let (params, i) = many0(&expect_id, i)?;
                let i = expect(Token::Equals, i)?;
                let (ty, i) = parse_type(i)?;

                (Statement::Alias(name, params, ty), i)
            } else {
                //type
                let (name, i) = expect_upper(i)?;
                let (params, i) = many0(&expect_id, i)?;
                let i = expect(Token::Equals, i)?;
                let (branches, i) = pipe1(&parse_adt_branch, i)?;

                (Statement::Adt(name, params, branches), i)
            }
        }
        Token::Port => {
            let (name, i) = expect_id(input.next())?;
            let i = expect(Token::Colon, i)?;
            let (ty, i) = parse_type(i)?;

            (Statement::Port(name, ty), i)
        }
        Token::Id(name) => {
            let (def, i) = parse_definition(0, input)?;

            (Statement::Def(def), i)
        }
        _ => {
            let found = input.read();
            return Err(ParseError::UnmatchedToken { input, found, options: vec![] });
        }
    };

    Ok((stm, i))
}

pub fn parse_definition(indent: u32, input: Input) -> Result<(Definition, Input), ParseError> {
    let (name, i) = expect_id(input)?;

    let (header, i) = match i.read() {
        Token::Colon => {
            let (ty, i) = parse_type(i.next())?;
            let i = expect_indent(indent, i)?;
            let (f_name, i) = expect_id(i)?;
            assert_eq!(f_name, name);

            (Some(ty), i)
        }
        _ => (None, i)
    };

    let (patterns, i) = many0(&parse_pattern, i)?;
    let i = expect(Token::Equals, i)?;
    let (expr, i) = parse_expr(i)?;

    Ok((Definition { header, name, patterns, expr }, i))
}

fn parse_adt_branch(input: Input) -> Result<((String, Vec<Type>), Input), ParseError> {
    let (name, i) = expect_upper(input)?;
    let (params, i) = many0(&parse_type, i)?;

    Ok(((name, params), i))
}

#[cfg(test)]
mod tests {
    use parsers::new::util::test_parser;
    use parsers::new::util::test_parser_error;
    use super::*;

    #[test]
    fn expr_test() {
        test_parser(parse_statement, "type Bool = True | False");
        test_parser(parse_statement, "type List a = Cons a List | Nil");
        test_parser(parse_statement, "type alias EmptySet = {}");
        test_parser(parse_statement, "type alias Set a = { all: List a, inside: List a }");
        test_parser(parse_statement, "port sum : Int -> Int -> Int");
        test_parser(parse_statement, "x = 0");
        test_parser(parse_statement, "func (a, b) = a + b");
        test_parser(parse_statement, "func x = x");
        test_parser(parse_statement, "func : Int -> Int\nfunc x = x");

        test_parser(parse_statement, "type Bool\n = True\n | False");
        test_parser(parse_statement, "type List a\n = Cons a List\n | Nil");
        test_parser(parse_statement, "type alias EmptySet =\n {}");
        test_parser(parse_statement, "type alias Set a = {\n all: List a,\n inside: List a\n }");
        test_parser(parse_statement, "port sum\n  : Int\n -> Int\n -> Int");
        test_parser(parse_statement, "x =\n 0");
        test_parser(parse_statement, "func\n x\n =\n x");
        test_parser(parse_statement, "func (a, b) = a + b");
    }

    #[test]
    fn expr_error_test() {
        test_parser_error(parse_statement, "type Bool");
        test_parser_error(parse_statement, "type Bool \n= True \n| False");
        test_parser_error(parse_statement, "type List a \n= Cons a List \n| Nil");
        test_parser_error(parse_statement, "type alias EmptySet = \n{}");
        test_parser_error(parse_statement, "type alias Set a = { \nall: List a, \ninside: List a \n}");
        test_parser_error(parse_statement, "port sum");
    }
}
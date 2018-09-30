use types::Expr;

#[derive(Clone, Debug, PartialEq)]
pub enum ExprTree {
    Leaf(Expr),
    Branch(String, Box<ExprTree>, Box<ExprTree>),
}

#[derive(Clone, Debug, PartialEq)]
enum ETk {
    Expr(Expr),
    Op(String),
}

pub fn token_stream(exprs: Vec<Expr>, ops: Vec<String>) -> Vec<ETk> {
    assert_eq!(exprs.len(), ops.len() + 1);
    let mut list = Vec::new();

    let mut e = exprs.iter();

    list.push(ETk::Expr(e.next().unwrap().clone()));

    ops.iter().for_each(|o| {
        list.push(ETk::Op(o.clone()));
        list.push(ETk::Expr(e.next().unwrap().clone()));
    });

    list
}

fn create_tree(mut tk: &[ETk], level: i32) -> (&[ETk], ExprTree) {
    if level == 10 {
        match &tk[0] {
            &ETk::Expr(ref e) => return (&tk[1..], ExprTree::Leaf(e.clone())),
            _ => panic!("create_tree illegal state, tk: {:#?}", tk)
        }
    }

    let (_tk, first) = create_tree(tk, level + 1);
    tk = _tk;

    let mut items: Vec<(String, ExprTree)> = vec![];

    while !tk.is_empty() {
        let op = match &tk[0] {
            &ETk::Op(ref e) => e.clone(),
            _ => panic!("create_tree illegal state")
        };

        if get_operator_priority(&op) != level {
            break;
        }

        let (_tk, item) = create_tree(&tk[1..], level + 1);
        items.push((op, item));

        tk = _tk;
    }


    let expr = items
        .into_iter()
        .fold(first, |a, (op, b)| ExprTree::Branch(op, Box::new(a), Box::new(b)));

    (tk, expr)
}

// default priorities
pub fn get_operator_priority(op: &str) -> i32 {
    match op {
        ">>" | "<<" => 9,
        "^" => 8,
        "*" | "/" | "//" | "%" | "rem" => 7,
        "+" | "-" => 6,
        "++" | "::" => 5,
        "==" | "/=" | "<" | ">" | "<=" | ">=" => 4,
        "&&" => 3,
        "||" => 2,
        "|>" | "<|" => 0,
        _ => 1
    }
}

enum Associativity {
    Left,
    Right,
    None,
}

pub fn get_operator_associativity(op: &str) -> Associativity {
    match op {
        "|>" | ">>" | "*" | "/" | "//" | "%" | "rem" | "+" | "-" => Associativity::Left,
        "<|" | "<<" | "^" | "++" | "::" | "&&" | "||" => Associativity::Right,
        "==" | "/=" | "<" | ">" | "<=" | ">=" => Associativity::None,
        _ => Associativity::Left
    }
}

#[cfg(test)]
mod tests {
    use interpreter::expression_fold::ExprTree::*;
    use nom::*;
    use nom::verbose_errors::*;
    use parsers::expression::read_expr;
    use super::*;
    use tokenizer::get_all_tokens;
    use types::Expr;
    use types::Expr::Ref;
    use util::StringConversion;
    use util::Tk;

    fn from_code(code: &[u8]) -> Expr {
        let stream = get_all_tokens(code);
        let expr: IResult<Tk, Expr> = read_expr(&stream);

        match expr {
            Ok((_, e)) => e,
            Err(e) => {
                match e {
                    Err::Incomplete(need) => panic!("Tokens needed: {:?}", need),
                    Err::Failure(ctx) => panic!("Parsing failure: {:#?}", ctx),
                    Err::Error(ctx) => panic!("Syntax error: {:#?}", ctx),
                };
            }
        }
    }

    #[test]
    fn check_operator_precedence() {
        let expr = from_code(b"a + b * c / d - f");
        match expr {
            Expr::OpChain(exprs, ops) => {
                let stream = token_stream(exprs, ops);
                let (stream, tree) = create_tree(&stream, 0);
                assert_eq!(stream.len(), 0);
                assert_eq!(tree, Branch(
                    "-".s(),
                    Box::new(Branch(
                        "+".s(),
                        Box::new(Leaf(Ref("a".s()))),
                        Box::new(Branch(
                            "/".s(),
                            Box::new(Branch(
                                "*".s(),
                                Box::new(Leaf(Ref("b".s()))),
                                Box::new(Leaf(Ref("c".s()))),
                            )),
                            Box::new(Leaf(Ref("d".s()))),
                        )),
                    )),
                    Box::new(Leaf(Ref("f".s()))),
                ));
            }
            _ => panic!("Invalid type")
        }
    }
}
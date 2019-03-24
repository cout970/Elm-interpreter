use ast::Expr;
use util::expression_fold::ExprTreeError::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ExprTree {
    Leaf(Expr),
    Branch(String, Box<ExprTree>, Box<ExprTree>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprTreeError {
    InvalidInput,
    AssociativityError,
    InternalError(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Associativity {
    Left,
    Right,
    None,
}

#[derive(Clone, Debug, PartialEq)]
enum ETk {
    Expr(Expr),
    Op(String),
}

pub fn create_expr_tree(exprs: &Vec<Expr>, ops: &Vec<String>) -> Result<ExprTree, ExprTreeError> {
    let tokens = token_stream(exprs, ops)?;
    let (rest, tree) = create_tree(&tokens, 0)?;
    assert_eq!(rest.len(), 0);

    Ok(tree)
}

fn token_stream(exprs: &Vec<Expr>, ops: &Vec<String>) -> Result<Vec<ETk>, ExprTreeError> {
    if exprs.len() != ops.len() + 1 {
        return Err(ExprTreeError::InvalidInput);
    }

    let mut list = Vec::new();
    let mut e = exprs.iter();

    list.push(ETk::Expr(e.next().unwrap().clone()));

    ops.iter().for_each(|o| {
        list.push(ETk::Op(o.clone()));
        list.push(ETk::Expr(e.next().unwrap().clone()));
    });

    Ok(list)
}

fn create_tree(mut tk: &[ETk], level: i32) -> Result<(&[ETk], ExprTree), ExprTreeError> {
    if level == 10 {
        return match &tk[0] {
            &ETk::Expr(ref e) => Ok((&tk[1..], ExprTree::Leaf(e.clone()))),
            _ => Err(InternalError(format!("create_tree illegal state, tk: {:#?}", tk)))
        };
    }

    let (_tk, first) = create_tree(tk, level + 1)?;
    tk = _tk;

    let mut ops: Vec<String> = vec![];
    let mut exprs: Vec<ExprTree> = vec![];

    exprs.push(first);

    while !tk.is_empty() {
        let op = match &tk[0] {
            &ETk::Op(ref e) => e.clone(),
            _ => panic!("create_tree illegal state")
        };

        if get_operator_priority(&op) != level {
            break;
        }

        let (_tk, item) = create_tree(&tk[1..], level + 1)?;
        exprs.push(item);
        ops.push(op);

        tk = _tk;
    }

    if ops.is_empty() {
        return Ok((tk, exprs[0].clone()));
    }

    let first_op = ops.first().unwrap();
    let assoc = get_operator_associativity(first_op);

    match assoc {
        Associativity::Left => {
            let mut index: usize = 0;
            let mut current_tree = exprs[index].clone();
            index += 1;

            for op in ops.iter() {
                if get_operator_associativity(op) != assoc {
                    return Err(AssociativityError);
                }
                current_tree = ExprTree::Branch(
                    op.clone(),
                    Box::new(current_tree),
                    Box::new(exprs[index].clone()),
                );
                index += 1;
            }

            Ok((tk, current_tree))
        }
        Associativity::Right => {
            let mut index: isize = (exprs.len() - 1) as isize;
            let mut current_tree = exprs[index as usize].clone();
            index -= 1;

            for op in ops.iter() {
                if get_operator_associativity(op) != assoc {
                    return Err(AssociativityError);
                }
                current_tree = ExprTree::Branch(
                    op.clone(),
                    Box::new(exprs[index as usize].clone()),
                    Box::new(current_tree),
                );
                index -= 1;
            }

            Ok((tk, current_tree))
        }
        Associativity::None => {
            if ops.len() == 1 {
                Ok((tk, ExprTree::Branch(
                    ops[0].clone(),
                    Box::new(exprs[0].clone()),
                    Box::new(exprs[1].clone()),
                )))
            } else {
                Err(AssociativityError)
            }
        }
    }
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
    use ast::Expr;
    use ast::Expr::Ref;
    use test_utils::Test;
    use util::StringConversion;

    use super::*;
    use super::ExprTree::*;

    #[test]
    fn check_operator_precedence() {
        let expr = Test::expr("a + b * c / d - f");
        match expr {
            Expr::OpChain(_, exprs, ops) => {
                let tree = create_expr_tree(&exprs, &ops);
                assert_eq!(tree, Ok(Branch(
                    "-".s(),
                    Box::new(Branch(
                        "+".s(),
                        Box::new(Leaf(Ref((0, 0), "a".s()))),
                        Box::new(Branch(
                            "/".s(),
                            Box::new(Branch(
                                "*".s(),
                                Box::new(Leaf(Ref((0, 0), "b".s()))),
                                Box::new(Leaf(Ref((0, 0), "c".s()))),
                            )),
                            Box::new(Leaf(Ref((0, 0), "d".s()))),
                        )),
                    )),
                    Box::new(Leaf(Ref((0, 0), "f".s()))),
                )));
            }
            _ => panic!("Invalid type")
        }
    }

    #[test]
    fn check_operator_associativity_1() {
        let expr = Test::expr("a >> b >> c"); // (a >> b) >> c
        match expr {
            Expr::OpChain(_, exprs, ops) => {
                let tree = create_expr_tree(&exprs, &ops);
                assert_eq!(tree, Ok(Branch(
                    ">>".s(),
                    Box::new(Branch(
                        ">>".s(),
                        Box::new(Leaf(Ref((0, 0), "a".s()))),
                        Box::new(Leaf(Ref((0, 0), "b".s()))),
                    )),
                    Box::new(Leaf(Ref((0, 0), "c".s()))),
                )));
            }
            _ => panic!("Invalid type")
        }
    }

    #[test]
    fn check_operator_associativity_2() {
        let expr = Test::expr("a << b << c"); // a << (b << c)
        match expr {
            Expr::OpChain(_, exprs, ops) => {
                let tree = create_expr_tree(&exprs, &ops);
                assert_eq!(tree, Ok(Branch(
                    "<<".s(),
                    Box::new(Leaf(Ref((0, 0), "a".s()))),
                    Box::new(Branch(
                        "<<".s(),
                        Box::new(Leaf(Ref((0, 0), "b".s()))),
                        Box::new(Leaf(Ref((0, 0), "c".s()))),
                    )),
                )));
            }
            _ => panic!("Invalid type")
        }
    }

    #[test]
    fn check_operator_associativity_3() {
        let expr = Test::expr("a >> b << c"); // Error
        match expr {
            Expr::OpChain(_, exprs, ops) => {
                let tree = create_expr_tree(&exprs, &ops);
                assert_eq!(tree, Err(AssociativityError));
            }
            _ => panic!("Invalid type")
        }
    }

    #[test]
    fn check_operator_associativity_4() {
        let expr = Test::expr("a == b == c"); // Error
        match expr {
            Expr::OpChain(_, exprs, ops) => {
                let tree = create_expr_tree(&exprs, &ops);
                assert_eq!(tree, Err(AssociativityError));
            }
            _ => panic!("Invalid type")
        }
    }

    #[test]
    fn check_operator_associativity_5() {
        let expr = Test::expr("a == b");
        match expr {
            Expr::OpChain(_, exprs, ops) => {
                let tree = create_expr_tree(&exprs, &ops);
                assert_eq!(tree, Ok(Branch(
                    "==".s(),
                    Box::new(Leaf(Ref((0, 0), "a".s()))),
                    Box::new(Leaf(Ref((0, 0), "b".s()))),
                )));
            }
            _ => panic!("Invalid type")
        }
    }
}
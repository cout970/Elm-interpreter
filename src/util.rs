use tokenizer::Token;
use types::*;

pub type Tk<'a> = &'a [Token];


#[macro_export]
macro_rules! tk {
    ($i:expr, $token: expr) => (
        {
            use nom::*;
            use nom::simple_errors::Context;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = &$i[0];

                if look == &$token {
                  Ok((&$i[1..], look.clone()))
                } else {
                  Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                }
            }
        }
    );
}

#[macro_export]
macro_rules! id {
    ($i:expr,) => (
        {
            use nom::*;
            use nom::simple_errors::Context;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = &$i[0];

                if let Token::Id(str) = look {
                  Ok((&$i[1..], str.clone()))
                } else {
                  Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                }
            }
        }
    );
}

#[macro_export]
macro_rules! upper_id {
    ($i:expr,) => (
        {
            use nom::*;
            use nom::simple_errors::Context;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = &$i[0];

                if let Token::UpperId(str) = look {
                  Ok((&$i[1..], str.clone()))
                } else {
                  Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                }
            }
        }
    );
}

#[macro_export]
macro_rules! binop {
    ($i:expr,) => (
        {
            use nom::*;
            use nom::simple_errors::Context;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = &$i[0];

                if let Token::BinaryOperator(str) = look {
                  Ok((&$i[1..], str.clone()))
                } else {
                  Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                }
            }
        }
    );
}

#[macro_export]
macro_rules! literal {
    ($i:expr,) => (
        {
            use nom::*;
            use nom::simple_errors::Context;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = &$i[0];

                match look {
                   LitInt(value) => Ok((&$i[1..], Literal::Int(*value))),
                   LitFloat(value) => Ok((&$i[1..], Literal::Float(*value))),
                   LitChar(value) => Ok((&$i[1..], Literal::Char(*value))),
                   LitString(value) => Ok((&$i[1..], Literal::String(value.clone()))),
                   _ => Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                }
            }
        }
    );
}

#[macro_export]
macro_rules! indent {
    ($i:expr, $count: expr) => (
    {
        use nom::*;
        use nom::simple_errors::Context;

        if $i.len() == 0 {
            Err(Err::Incomplete(Needed::Size(1)))
        } else {
            let look = &$i[0];

            if look == &Token::Indent($count) {
                Ok((&$i[1..], ()))
            } else {
                Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
            }
        }
    }
    );

    ($i:expr,) => (
    {
        use nom::*;
        use nom::simple_errors::Context;

        if $i.len() == 0 {
            Err(Err::Incomplete(Needed::Size(1)))
        } else {
            let look = &$i[0];

            if let Token::Indent(count) = look {
                Ok((&$i[1..], *count))
            } else {
                Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
            }
        }
    }
    );
}

macro_rules! indent_except {
    ($i:expr, $levels: expr) => (
    {
        use nom::*;
        use nom::simple_errors::Context;

        if $i.len() > 0 {
            let look = &$i[0];

            if let Token::Indent(count) = look {
                if $levels.contains( &(*count as usize)) {
                    Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                } else {
                    Ok((&$i[1..], * count))
                }
            } else {
                Err(Err::Error(Context::Code( $i, ErrorKind::Custom(0))))
            }
        } else {
            Err(Err::Incomplete(Needed::Size(1)))
        }
    }
    );
}

#[cfg(test)]
macro_rules! assert_ok {
   ($r: expr, $tk: expr) => {
       match &$r {
           Ok((rem, item)) => {
               assert_eq!(*item, $tk, "Remaining: {:?}", rem);
           }
           Err(_) => {
               panic!("{:?}", $r);
           }
       }
   }
}

pub trait StringConversion {
    fn s(&self) -> String;
}

impl StringConversion for str {
    fn s(&self) -> String {
        self.to_string()
    }
}

// default priorities
fn get_operator_priority(op: &str) -> i32 {
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

pub fn to_string(v: &[u8]) -> String {
    v.into_iter().map(|c| *c as char).collect::<String>()
}

pub fn create_vec<T>(first: T, rest: Vec<T>) -> Vec<T> {
    let mut vec: Vec<T> = Vec::new();
    vec.push(first);
    for i in rest.into_iter() {
        vec.push(i);
    }
    vec
}

pub fn parse_int(negative: bool, digits: Vec<char>) -> Int {
    let s: String = digits.into_iter().collect();
    let value = s.parse::<Int>().unwrap();
    if negative { -value } else { value }
}

pub fn parse_float(integer_part: Vec<char>, decimal_part: Vec<char>) -> Float {
    let int_part: String = integer_part.into_iter().collect();
    let dec_part: String = decimal_part.into_iter().collect();
    format!("{}.{}", int_part, dec_part).parse::<Float>().unwrap()
}

pub fn parse_float2(minus: bool, integer_part: Vec<char>, decimal_part: Vec<char>) -> Float {
    let int_part: String = integer_part.into_iter().collect();
    let dec_part: String = decimal_part.into_iter().collect();
    let value = format!("{}.{}", int_part, dec_part).parse::<Float>().unwrap();
    if minus { -value } else { value }
}
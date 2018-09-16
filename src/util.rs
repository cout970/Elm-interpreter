use tokenizer::Token;
use types::*;

pub type Tk<'a> = &'a [Token];

#[macro_export]
macro_rules! tk {
    ($i:expr, $token: expr) => (
        {
            use nom::*;
            use nom::simple_errors::Context;

            if $i.len() == 0 { return Err(Err::Incomplete(Needed::Size(1))); }
            let look = &$i[0];
            if look == &$token {
              Ok((&$i[1..], look.clone()))
            }else {
              Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
            }
        }
    );
}

#[macro_export]
macro_rules! id {
    ($i:expr, $string: expr) => (
        {
            use nom::*;
            use nom::simple_errors::Context;

            if $i.len() == 0 { return Err(Err::Incomplete(Needed::Size(1))); }
            let look = &$i[0];
            if look == &Token::Id($string.to_string()) {
              Ok((&$i[1..], $string.to_string()))
            } else {
              Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
            }
        }
    );

    ($i:expr,) => (
        {
            use nom::*;
            use nom::simple_errors::Context;

            if $i.len() == 0 { return Err(Err::Incomplete(Needed::Size(1))); }
            let look = &$i[0];

            if let Token::Id(str) = look {
              Ok((&$i[1..], str.clone()))
            } else {
              Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
            }
        }
    );
}

#[macro_export]
macro_rules! upper_id {
    ($i:expr, $string: expr) => (
        {
            use nom::*;
            use nom::simple_errors::Context;

            if $i.len() == 0 { return Err(Err::Incomplete(Needed::Size(1))); }
            let look = &$i[0];
            if look == &Token::UpperId($string.to_string()) {
              Ok((&$i[1..], $string.to_string()))
            } else {
              Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
            }
        }
    );

    ($i:expr,) => (
        {
            use nom::*;
            use nom::simple_errors::Context;

            if $i.len() == 0 { return Err(Err::Incomplete(Needed::Size(1))); }
            let look = &$i[0];

            if let Token::UpperId(str) = look {
              Ok((&$i[1..], str.clone()))
            } else {
              Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
            }
        }
    );
}

#[macro_export]
macro_rules! binop {
    ($i:expr, $string: expr) => (
        {
            use nom::*;
            use nom::simple_errors::Context;

            if $i.len() == 0 { return Err(Err::Incomplete(Needed::Size(1))); }
            let look = &$i[0];
            if look == &Token::BinaryOperator($string.to_string()) {
              Ok((&$i[1..], $string.to_string()))
            } else {
              Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
            }
        }
    );

    ($i:expr,) => (
        {
            use nom::*;
            use nom::simple_errors::Context;

            if $i.len() == 0 { return Err(Err::Incomplete(Needed::Size(1))); }
            let look = &$i[0];

            if let Token::BinaryOperator(str) = look {
              Ok((&$i[1..], str.clone()))
            } else {
              Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
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

            if $i.len() == 0 { return Err(Err::Incomplete(Needed::Size(1))); }
            let look = &$i[0];

            let lit = match look {
               LitInt(value) => Literal::Int(*value),
               LitFloat(value) => Literal::Float(*value),
               LitChar(value) => Literal::Char(*value),
               LitString(value) => Literal::String(value.clone()),
               _ => {return Err(Err::Error(Context::Code($i, ErrorKind::Custom(0)))); }
            };

            Ok((&$i[1..], lit))
        }
    );
}

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
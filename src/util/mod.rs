use std::intrinsics::transmute;
use std::sync::Arc;

use ast::*;
use types::*;

pub mod name_sequence;
pub mod format;
pub mod expression_fold;
pub mod visitors;
pub mod sort;

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

pub trait OptionExt<A> {
    fn zip<B>(self, other: Option<B>) -> Option<(A, B)>;
}

impl<A> OptionExt<A> for Option<A> {
    fn zip<B>(self, other: Option<B>) -> Option<(A, B)> {
        match self {
            Some(a) => {
                match other {
                    Some(b) => {
                        Some((a, b))
                    }
                    None => None
                }
            }
            None => None
        }
    }
}

pub trait VecExt<A> {
    fn for_each<F: FnMut(&A)>(&self, f: F);
    fn map<B, F: FnMut(&A) -> B>(&self, f: F) -> Vec<B>;
    fn join_vec(&self, other: &[A]) -> Vec<A>;
}

pub trait ToVec<A> {
    fn to_vec(&mut self) -> Vec<A>;
}

impl<T: ?Sized, A: Sized> ToVec<A> for T where T: Iterator<Item=A> {
    fn to_vec(&mut self) -> Vec<A> {
        self.collect()
    }
}

impl<A: Clone> VecExt<A> for Vec<A> {
    fn for_each<F: FnMut(&A)>(&self, f: F) {
        self.iter().for_each(f);
    }

    fn map<B, F: FnMut(&A) -> B>(&self, f: F) -> Vec<B> {
        self.iter().map(f).collect()
    }

    fn join_vec(&self, other: &[A]) -> Vec<A> {
        let mut res: Vec<A> = Vec::new();
        for a in self {
            res.push(a.clone());
        }
        for b in other {
            res.push(b.clone());
        }
        res
    }
}

pub fn builtin_fun_of(func: ExternalFunc, ty: Type) -> Value {
    Value::Fun {
        args: vec![],
        arg_count: arg_count(&ty),
        fun: Arc::new(Function::External(next_fun_id(), func, ty)),
    }
}

pub fn arg_count(ty: &Type) -> u32 {
    match ty {
        Type::Fun(_, ref out) => {
            1 + arg_count(out)
        }
        _ => 0
    }
}

pub fn to_string(v: &[u8]) -> String {
    v.into_iter().map(|c| *c as char).collect::<String>()
}

pub fn uncons<T>(mut list: Vec<T>) -> (Vec<T>, T) {
    assert!(!list.is_empty());
    let last = list.pop().unwrap();
    (list, last)
}


pub fn create_vec<T>(first: T, rest: Vec<T>) -> Vec<T> {
    let mut vec: Vec<T> = Vec::new();
    vec.push(first);
    for i in rest.into_iter() {
        vec.push(i);
    }
    vec
}

pub fn create_vec_inv<T: Clone>(start: &[T], last: T) -> Vec<T> {
    let mut vec: Vec<T> = start.to_vec();
    vec.push(last);
    vec
}

pub fn parse_int(negative: bool, digits: Vec<char>) -> Int {
    let s: String = digits.into_iter().collect();
    let value = s.parse::<Int>().unwrap();
    if negative { -value } else { value }
}

pub fn parse_int_base(base: u32, digits: Vec<char>) -> Int {
    let s: String = digits.into_iter().collect();
    i64::from_str_radix(&s, base).unwrap() as Int
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

pub fn build_fun_type(types: &[Type]) -> Type {
    assert!(!types.is_empty());

    if types.len() == 1 {
        return types[0].clone();
    }

    if types.len() == 2 {
        Type::Fun(
            Box::from(types[0].clone()),
            Box::from(types[1].clone()),
        )
    } else {
        Type::Fun(
            Box::from(types[0].clone()),
            Box::from(build_fun_type(&types[1..])),
        )
    }
}

pub fn qualified_name(path: &[String], name: &str) -> String {
    let mut full_name = String::new();
    for x in path {
        full_name.push_str(x);
        full_name.push('.');
    }
    full_name.push_str(name);

    full_name
}

#[cfg(test)]
pub fn test_resource(path: &str) -> String {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/test/");
    d.push(path);
    d.to_string_lossy().to_string()
}

pub fn resource_path(path: &str) -> String {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/");
    d.push(path);
    d.to_string_lossy().to_string()
}

pub fn transmute_float_to_int(x: f32) -> i32 {
    unsafe { transmute::<f32, i32>(x) }
}
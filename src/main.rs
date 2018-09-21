#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate pretty_assertions;

use nom::*;
use nom::simple_errors::Context;
use parsers::module::*;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use tokenizer::*;
use types::*;
use util::*;
use std::io::stdin;
use parsers::statement::top_level_statement;
use parsers::expression::read_expr;
use std::io::stdout;
use std::io::BufRead;

mod types;
#[macro_use]
mod util;
mod parsers;
mod tokenizer;

fn load_file() -> Vec<u8> {
    let mut file = File::open("example.elm").expect("Example file not found");
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();

    data
}

fn use_file() -> bool { false }

fn main() {
    if use_file() {
        let file = load_file();
        let tokens = get_all_tokens(&file);
        let result = read_module(&tokens);

        if let Ok((rest, module)) = result {
            println!("Remaining: {:?}\n", rest);
            println!("Output: \n{:#?}", module);
        } else {
            println!("{:?}", result);
        }
    } else {
        let mut input = String::new();

        print!("> ");
        stdout().flush();
        let stdin = stdin();

        for line in stdin.lock().lines() {
            let tokens = get_all_tokens(&line.unwrap().as_bytes());

            let result = top_level_statement(&tokens);

            if let Ok((rest, module)) = result {
                println!("Output: \n{:#?}", module);
            } else {
                println!("Error: {:?}", result);
            }

            println!();
        }
    }
}

fn print<T: std::fmt::Debug>(r: IResult<&[u8], T>) {
    match r {
        Ok((str, t)) => {
            println!("{:?}, rest: '{}'", t, to_string(str));
        }
        Err(e) => {
            match e {
                Err::Error(ctx) => {
                    match ctx {
                        Context::Code(c, ..) => {
                            println!("Erro, rest: {:?}", to_string(c));
                        }
                    }
                }
                Err::Incomplete(needed) => {
                    println!("Inco: {:?}", needed);
                }
                Err::Failure(ctx) => {
                    println!("Fail: {:?}", ctx);
                }
            }
        }
    }
}

//#[cfg(test)]
//mod tests {
//    use nom::*;
//    use super::*;
//    use tokenizer::get_all_tokens;
//
////    #[test]
//    fn check_snippet_1() {
//        let stream = get_all_tokens(b"\n\
//        module MyModule.MySubModule () where\n\
//        ");
//        let m = read_module(&stream);
//        assert_ok!(m, Module {
//            header: Some(ModuleHeader {
//                name: vec!["MyModule".s(), "MySubModule".s()],
//                exports: vec![]
//            }),
//            ..Module::default()
//        });
//    }
//
////    #[test]
//    fn check_snippet_2() {
//        let stream = get_all_tokens(b"\n\
//        update msg model = \n\
//          case msg of\n\
//            Increment ->\n\
//              model + 1\n\
//        \n\
//            Decrement ->\n\
//              model - 1\n\
//        ");
//        let m = read_module(&stream);
//        assert_ok!(m, Module {
//            statements: vec![],
//            ..Module::default()
//        });
//    }
//}

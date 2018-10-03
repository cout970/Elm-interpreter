// Development only {
// cargo watch -s 'clear && cargo test'
#![allow(dead_code, unused_imports)]
// }


#[macro_use]
extern crate nom;
#[macro_use]
extern crate pretty_assertions;

use nom::*;
use nom::verbose_errors::Context;
use parsers::module::*;
use parsers::statement::top_level_statement;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use types::*;
use util::*;
use parsers::expression::read_expr;
use tokenizer::*;

mod types;
#[macro_use]
mod util;
mod parsers;
mod tokenizer;
mod analyzer;
mod interpreter;

fn main() {
    interpret_stdin();
}

fn interpret_stdin(){
    print!("> ");
    stdout().flush().unwrap();
    let stdin = stdin();

    for line in stdin.lock().lines() {
        let tokens = get_all_tokens(&line.unwrap().as_bytes());

        let result = read_expr(&tokens);

        if let Ok((_, module)) = result {
            println!("Output: \n{:#?}", module);
        } else {
            println!("Error: {:?}", result);
        }

        println!();
    }
}

fn load_file() -> Vec<u8> {
    let mut file = File::open("example.elm").expect("Example file not found");
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();

    data
}

fn interpret_file(){
    let file = load_file();
    let tokens = get_all_tokens(&file);
//        println!("Tokens: \n{:#?}\n", tokens);

    let result = read_module(&tokens);

    if let Ok((rest, module)) = result {
        println!("Remaining: {:?}\n", rest);
        println!("Output: \n{:#?}", module);
    } else {
        println!("{:?}", result);
    }
}
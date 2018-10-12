// Development only {
// cargo watch -s 'clear && cargo test'
#![allow(dead_code, unused_imports)]
// }

#[macro_use]
extern crate nom;
#[macro_use]
extern crate pretty_assertions;

use analyzer::environment::default_lang_env;
use analyzer::environment::Environment;
use analyzer::environment::expand_env;
use analyzer::type_resolution::get_value_type;
use interpreter::eval;
use nom::ExtendInto;
use nom::IResult;
use nom::verbose_errors::Context;
use parsers::parse_expr;
use parsers::parse_statement;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use tokenizer::*;
use types::*;
use util::*;
use analyzer::type_check_expression;
use analyzer::static_env::StaticEnv;

mod types;
#[macro_use]
mod util;
#[macro_use]
mod parsers;
mod tokenizer;
mod analyzer;
mod interpreter;

fn main() {
    interpret_stdin();
}

fn interpret_stdin() {
    print!("> ");
    stdout().flush().unwrap();
    let stdin = stdin();
    let mut env = default_lang_env();

    for line in stdin.lock().lines() {
        if let Err(s) = run_line(&mut env, &line.unwrap().as_bytes()) {
            println!("Error: {}", s);
        }
        print!("> ");
        stdout().flush().unwrap();
    }
}

pub fn run_line(env: &mut Environment, line: &[u8]) -> Result<String, String> {
    use nom::*;
    let tokens = tokenize(line);
    let stm = parse_statement(&tokens).map_err(|e| format!("{:?}", e));

    match stm {
        Ok(statement) => {
            match statement {
                Statement::Alias(path, ty) => { Ok(format!("type alias {:?} = {}", path, ty)) }
                Statement::Adt(def, variants) => { Ok(format!("type {:?} = {:?}", def, variants)) }
                Statement::Port(name, ty) => { Ok(format!("port {} = {}", name, ty)) }
                Statement::Def(ref def) => {
                    expand_env(env, vec![def]).map_err(|e| format!("{:?}", e))?;
                    Ok(format!("def {:?}", def))
                }
            }
        }
        Err(_) => {
            let mut static_env = StaticEnv::new();
            let expr = parse_expr(&tokens).map_err(|e| format!("{:?}", e))?;
            // check expr type
            type_check_expression(&mut static_env, &expr).map_err(|e| format!("{:?}", e))?;
            env.enter_block();
            let value = eval(env, &expr);
            env.exit_block();

            let value = value?;

            Ok(format!("{} : {}", &value, get_value_type(&value)))
        }
    }
}

fn load_file() -> Vec<u8> {
    let mut file = File::open("example.elm").expect("Example file not found");
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();

    data
}

//fn interpret_file() {
//    let file = load_file();
//    let tokens = tokenize(&file);
//        println!("Tokens: \n{:#?}\n", tokens);

//    let result = read_module(&tokens);
//
//    if let Ok((rest, module)) = result {
//        println!("Remaining: {:?}\n", rest);
//        println!("Output: \n{:#?}", module);
//    } else {
//        println!("{:?}", result);
//    }
//}
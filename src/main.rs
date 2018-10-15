// Development only {
// cargo watch -s 'clear && cargo test'
#![allow(dead_code, unused_imports)]
// }

#[macro_use]
extern crate nom;
#[macro_use]
extern crate pretty_assertions;

use analyzer::static_env::StaticEnv;
use analyzer::type_check_expression;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::eval_expression;
use interpreter::eval_statement;
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
use analyzer::type_of_value;

mod types;
#[macro_use]
mod util;
#[macro_use]
mod parsers;
mod tokenizer;
mod analyzer;
mod interpreter;
/*
fib num = case num of \
 0 -> 0 \
 1 -> 1 \
 _ -> fib (num - 1) + fib (num - 2)
*/

fn main() {
    let mut env = DynamicEnv::default_lang_env();
    loop {
        // Read
        let line = read_terminal_line().unwrap_or(String::from(""));

        if line.is_empty() { continue; }

        // Eval
        let result = eval_statement(&mut env, &line);

        // Print
        match result {
            Ok(opt) => {
                match opt {
                    Some(value) => {
                        println!("{} : {}", value, type_of_value(&value));
                    }
                    None => {
                        // No error
                    }
                }
            }
            Err(_) => {
                env.eval_calls = 0;
                let result = eval_expression(&mut env, &line);

                match result {
                    Ok(value) => {
                        println!("{} : {} (in {} evals)", value, type_of_value(&value), env.eval_calls);
                    }
                    Err(error) => {
                        println!("Error: {:#?}", error);
                    }
                }
            }
        }

        // Loop back to the start
    }
}

fn read_terminal_line() -> Result<String, ()> {
    let stdin = stdin();
    let mut line = String::new();

    print!("> ");
    stdout().flush().unwrap();

    loop {
        stdin.lock().read_line(&mut line).map_err(|_| ())?;
        if line.len() < 2 {
            return Err(());
        }

        if line.as_bytes()[line.len() - 2] != b'\\' {
            break;
        }

        line.pop().unwrap();
        line.pop().unwrap();
        line.push('\n');

        print!("| ");
        stdout().flush().unwrap();
    }

    Ok(line)
}


//fn interpret_stdin() {
//    print!("> ");
//    stdout().flush().unwrap();
//    let stdin = stdin();
//    let mut env = default_lang_env();
//
//    for line in stdin.lock().lines() {
//        if let Err(s) = run_line(&mut env, &line.unwrap().as_bytes()) {
//            println!("Error: {}", s);
//        }
//        print!("> ");
//        stdout().flush().unwrap();
//    }
//}
//
//pub fn run_line(env: &mut Environment, line: &[u8]) -> Result<String, String> {
//    use nom::*;
//    let tokens = tokenize(line);
//    let stm = parse_statement(&tokens).map_err(|e| format!("{:?}", e));
//
//    match stm {
//        Ok(statement) => {
//            match statement {
//                Statement::Alias(path, ty) => { Ok(format!("type alias {:?} = {}", path, ty)) }
//                Statement::Adt(def, variants) => { Ok(format!("type {:?} = {:?}", def, variants)) }
//                Statement::Port(name, ty) => { Ok(format!("port {} = {}", name, ty)) }
//                Statement::Def(ref def) => {
//                    expand_env(env, vec![def]).map_err(|e| format!("{:?}", e))?;
//                    Ok(format!("def {:?}", def))
//                }
//            }
//        }
//        Err(_) => {
//            let mut static_env = StaticEnv::new();
//            let expr = parse_expr(&tokens).map_err(|e| format!("{:?}", e))?;
//            // check expr type
//            type_check_expression(&mut static_env, &expr).map_err(|e| format!("{:?}", e))?;
//            env.enter_block();
//            let value = eval(env, &expr);
//            env.exit_block();
//
//            let value = value?;
//
//            Ok(format!("{} : {}", &value, get_value_type(&value)))
//        }
//    }
//}
//
//fn load_file() -> Vec<u8> {
//    let mut file = File::open("example.elm").expect("Example file not found");
//    let mut data: Vec<u8> = Vec::new();
//    file.read_to_end(&mut data).unwrap();
//
//    data
//}

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
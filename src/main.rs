// Development only {
// cargo watch -s 'clear && cargo test'
#![allow(dead_code, unused_imports)]
// }


extern crate mylib;

use mylib::analyzer::type_of_value;
use mylib::interpreter::dynamic_env::DynamicEnv;
use mylib::interpreter::eval_expression;
use mylib::interpreter::eval_statement;
use std::io::BufRead;
use std::io::Read;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use mylib::tokenizer::tokenize;

/*
fib num = case num of \
 0 -> 0 \
 1 -> 1 \
 _ -> fib (num - 1) + fib (num - 2)
*/

fn main() {
    let code: &'static [u8] = include_bytes!("../benches/data/tokenizer_2.elm");
    let result = tokenize(code);
    match result {
        Ok(_) => {},
        Err(e) => {
            println!("{:#?}", e);
        },
    }
}

fn repl() {
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


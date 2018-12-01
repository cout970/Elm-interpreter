extern crate elm_interpreter;

use std::io::BufRead;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use elm_interpreter::analyzer::type_of_value;
use elm_interpreter::Interpreter;

/*
fib num = case num of \
 0 -> 0 \
 1 -> 1 \
 _ -> fib (num - 1) + fib (num - 2)
*/

fn main() {
    repl();
}

fn repl() {
    let mut engine = Interpreter::new();
    loop {
        // Read
        let line = read_terminal_line().unwrap_or(String::from(""));

        if line.is_empty() { continue; }

        // Eval
        let result = engine.eval_statement(&line);

        // Print
        match result {
            Ok(opt) => {
                if let Some(value) = opt {
                    println!("{} : {}", value, type_of_value(&value));
                }
            }
            Err(_) => {
                let result = engine.eval_expr(&line);

                match result {
                    Ok(value) => {
                        println!("{} : {}", value, type_of_value(&value));
                    }
                    Err(error) => {
                        println!("{}", error);
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


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
use analyzer::type_of_value;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::eval_expression;
use interpreter::eval_statement;
use parsers::parse_expr;
use parsers::parse_statement;
use tokenizer::*;
use types::*;
use util::*;
use nom::ExtendInto;
use nom::IResult;
use nom::verbose_errors::Context;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

pub mod types;
#[macro_use]
pub mod util;
pub mod tokenizer;
#[macro_use]
pub mod parsers;
pub mod analyzer;
pub mod interpreter;


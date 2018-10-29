// Development only {
// cargo watch -s 'clear && cargo test --color always 2>&1'
#![allow(dead_code)]
// }

#[macro_use]
extern crate nom;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod ast;
pub mod types;
#[macro_use]
mod util;
pub mod tokenizer;
#[macro_use]
pub mod parsers;
pub mod analyzer;
pub mod interpreter;
pub mod errors;


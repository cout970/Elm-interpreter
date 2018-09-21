use *;
use tokenizer::Token::*;

pub mod module;
pub mod types;
pub mod statement;
pub mod expression;
pub mod pattern;

named!(spaces<Tk, ()>, do_parse!(
    many0!(indent!()) >>
    (())
));

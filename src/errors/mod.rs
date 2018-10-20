use analyzer::TypeError;
use interpreter::RuntimeError;
use parsers::SyntaxError;
use tokenizer::LexicalError;

#[derive(PartialEq, Debug, Clone)]
pub enum ErrorWrapper {
    Lexical(LexicalError),
    Syntactic(SyntaxError),
    Type(TypeError),
    Runtime(RuntimeError),
}

pub fn format_error(error: ErrorWrapper) -> String {
    match error {
        ErrorWrapper::Lexical(it) => { format_lexical_error(it) }
        ErrorWrapper::Syntactic(it) => { format_syntactic_error(it) }
        ErrorWrapper::Type(it) => { format_type_error(it) }
        ErrorWrapper::Runtime(it) => { format_runtime_error(it) }
    }
}

pub fn format_lexical_error(error: LexicalError) -> String {
    format!("\
-- PARSE ERROR ------------------------------------------------------------- elm\n\
{:?}\
"
        , error)
}

pub fn format_syntactic_error(error: SyntaxError) -> String {
    format!("\
-- PARSE ERROR ------------------------------------------------------------- elm\n\
{:?}\
"
            , error)
}

pub fn format_type_error(error: TypeError) -> String {
    format!("{:?}", error)
}

pub fn format_runtime_error(error: RuntimeError) -> String {
    format!("{:?}", error)
}

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Write;

use analyzer::TypeError;
use interpreter::RuntimeError;
use parsers::ParseError;
use rust_interop::InteropError;
use tokenizer::LexicalError;
use util::format::print_vec;

#[derive(PartialEq, Clone)]
pub enum ErrorWrapper {
    LexicalError(LexicalError),
    ParseError(String, ParseError),
    TypeError(TypeError),
    RuntimeError(RuntimeError),
    InteropError(InteropError),
}

pub fn format_error(error: &ErrorWrapper) -> String {
    match error {
        ErrorWrapper::LexicalError(it) => { format_lexical_error(it) }
        ErrorWrapper::ParseError(code, it) => { format_parse_error(code, it) }
        ErrorWrapper::TypeError(it) => { format_type_error(it) }
        ErrorWrapper::RuntimeError(it) => { format_runtime_error(it) }
        ErrorWrapper::InteropError(it) => { format_interop_error(it) }
    }
}

pub fn format_lexical_error(error: &LexicalError) -> String {
    let mut msg = String::new();
    msg.push_str("-- PARSE ERROR ------------------------------------------------------------- elm\n");

    write!(&mut msg, "{:?}", error).unwrap();
    msg
}

pub fn format_parse_error(code: &str, error: &ParseError) -> String {
    let mut msg = String::new();
    msg.push_str("-- PARSE ERROR ------------------------------------------------------------- elm\n");

    match error {
        ParseError::Expected { span, expected, found } => {
            let loc = print_code_location(code, span.0, span.1);
            write!(&mut msg, "Expected token '{}', but found '{}': {}\n", expected, found, loc).unwrap()
        }
        ParseError::ExpectedInt { span, found } => {
            let loc = print_code_location(code, span.0, span.1);
            write!(&mut msg, "Expected integer, but found '{}': {}\n", found, loc).unwrap()
        }
        ParseError::ExpectedId { span, found } => {
            let loc = print_code_location(code, span.0, span.1);
            write!(&mut msg, "Expected identifier, but found '{}': {}\n", found, loc).unwrap()
        }
        ParseError::ExpectedUpperId { span, found } => {
            let loc = print_code_location(code, span.0, span.1);
            write!(&mut msg, "Expected capitalized identifier, but found '{}': {}\n", found, loc).unwrap()
        }
        ParseError::ExpectedBinaryOperator { span, found } => {
            let loc = print_code_location(code, span.0, span.1);
            write!(&mut msg, "Expected binary operator, but found '{}': {}\n", found, loc).unwrap()
        }
        ParseError::UnmatchedToken { span, found, .. } => {
            let loc = print_code_location(code, span.0, span.1);
            write!(&mut msg, "Found unexpected token '{}': {}\n", found, loc).unwrap()
        }
        ParseError::ExpectedIndentation { span, found } => {
            let loc = print_code_location(code, span.0, span.1);
            write!(&mut msg, "Expected indentation, but found '{}': {}\n", found, loc).unwrap()
        }
        ParseError::ExpectedIndentationLevel { span, expected, found } => {
            let loc = print_code_location(code, span.0, span.1);
            write!(&mut msg, "Expected indentation of {}, but found {}: {}\n", expected, found, loc).unwrap()
        }
    }
    msg
}

pub fn format_type_error(error: &TypeError) -> String {
    let mut msg = String::new();
    match error {
//        TypeError::MissingAdt(name) => {
//            write!(&mut msg, "-- NAMING ERROR ------------------------------------------------------------ elm\n\n").unwrap();
//            write!(&mut msg, "I cannot find a `{}` constructor:\n", name).unwrap();
////            write!(&mut msg, "Hint: Read <https://elm-lang.org/0.19.0/imports> to see how `import` declarations work in Elm.").unwrap();
//        }
        TypeError::MissingDefinition(_, name) => {
            write!(&mut msg, "-- NAMING ERROR ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "I cannot find a `{}` variable:\n", name).unwrap();
//            write!(&mut msg, "Hint: Read <https://elm-lang.org/0.19.0/imports> to see how `import` declarations work in Elm.").unwrap();
        }
//        TypeError::ListNotHomogeneous(_) => {},
//        TypeError::IfWithNonBoolCondition(_) => {},
//        TypeError::IfBranchesDoesntMatch(_) => {},
//        TypeError::ArgumentsDoNotMatch(_) => {},
//        TypeError::NotAFunction(_) => {},
//        TypeError::InvalidOperandChain(_) => {},
//        TypeError::RecordUpdateOnNonRecord(_) => {},
//        TypeError::RecordUpdateUnknownField(_) => {},
//        TypeError::CaseBranchDontMatchReturnType(_) => {},
//        TypeError::DefinitionTypeAndReturnTypeMismatch => {},
//        TypeError::InvalidPattern(_) => {},
//        TypeError::ConstantEvaluationError(_) => {},
//        TypeError::VariableAlreadyDeclared(_) => {},
//        TypeError::UnableToCalculateFunctionType(_) => {},
//        TypeError::VariableNameShadowed(_) => {},
//        TypeError::InternalError => {},
        TypeError::List(errors) => {
            let len = errors.len();
            for e in errors {
                msg.push_str(&format_type_error(e));
                msg.push('\n');
            }

            write!(&mut msg, "\nFound {} errors\n", len).unwrap();
        }
        _ => {
            write!(&mut msg, "-- TYPE ERROR ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "{:?}", error).unwrap();
        }
    }
    msg
}

pub fn format_runtime_error(error: &RuntimeError) -> String {
    let mut msg = String::new();
    match error {
        RuntimeError::MissingDefinition(name, _env) => {
            write!(&mut msg, "-- NAMING ERROR ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "I cannot find a `{}` variable:\n", name).unwrap();
            write!(&mut msg, "Hint: Read <https://elm-lang.org/0.19.0/imports> to see how `import` declarations work in Elm.").unwrap();
        }
        RuntimeError::IncorrectDefType(e) => {
            return format_type_error(e);
        }
        RuntimeError::RecordUpdateOnNonRecord(field, value) => {
            write!(&mut msg, "-- TYPE MISMATCH ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a record with the field `{}` but found:\n\n{}\n\n", field, value).unwrap();
            write!(&mut msg, "Maybe you forgot some code?").unwrap();
        }
        RuntimeError::InvalidIfCondition(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "This `if` condition does not evaluate to a boolean value, True or False.\n\n").unwrap();
            write!(&mut msg, "It is a value of type:\n\n{}\n\nBut I need this `if` condition to be a Bool value.", value).unwrap();
            write!(&mut msg, "Hint: Elm does not have “truthiness” such that ints and strings and lists are \
                              automatically converted to booleans. Do that conversion explicitly!").unwrap();
        }
        RuntimeError::RecordFieldNotFound(field, value) => {
            write!(&mut msg, "-- TYPE MISMATCH ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "This record does not have a `{}` field:\n\n{}\n\n", field, value).unwrap();
            write!(&mut msg, "This is usually a typo.").unwrap();
        }
        RuntimeError::CaseExpressionNonExhaustive(value, branches) => {
            write!(&mut msg, "-- MISSING PATTERNS -------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "This `case` does not have branches for all possibilities:\n\n{}\n\n", value).unwrap();
            write!(&mut msg, "Is not included in the existing branches:\n\n").unwrap();
            print_vec(&mut msg, &branches).unwrap();
            write!(&mut msg, "\n\nHint: If you want to write the code for each branch later, use `Debug.todo` as a \
                              placeholder. Read <https://elm-lang.org/0.19.0/missing-patterns> for more \
                              guidance on this workflow.").unwrap();
        }
        RuntimeError::ExpectedRecord(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a record but found:\n\n{}\n\n", value).unwrap();
        }
        RuntimeError::ExpectedFunction(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a function but found:\n\n{}\n\n", value).unwrap();
        }
        RuntimeError::ExpectedAdt(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a adt but found:\n\n{}\n\n", value).unwrap();
        }
        RuntimeError::ExpectedTuple(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a tuple but found:\n\n{}\n\n", value).unwrap();
        }
        RuntimeError::ExpectedList(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a list but found:\n\n{}\n\n", value).unwrap();
        }
        RuntimeError::ExpectedFloat(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a float but found:\n\n{}\n\n", value).unwrap();
        }
        RuntimeError::ExpectedInt(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a int but found:\n\n{}\n\n", value).unwrap();
        }
        RuntimeError::ExpectedNumber(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a number but found:\n\n{}\n\n", value).unwrap();
        }
        RuntimeError::FunArgumentSizeMismatch(expected, found) => {
            write!(&mut msg, "-- TOO MANY ARGS ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "The `fun` function expects {} argument, but it got {} instead.\n", expected, found).unwrap();
            write!(&mut msg, "Are there any missing commas? Or missing parentheses?").unwrap();
        }
        RuntimeError::ExpectedNonEmptyList(value) => {
            write!(&mut msg, "-- PATTERN MATCHING ERROR -------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a non empty list, but found:\n\n{}\n\n", value).unwrap();
            write!(&mut msg, "Try adding a extra branch for []").unwrap();
        }
        RuntimeError::UnknownOperatorPattern(name) => {
            write!(&mut msg, "-- PARSE ERROR ------------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I cannot use the `{}` operator\n\n", name).unwrap();
            write!(&mut msg, "I was expecting:\n\n\
                              - the `as` keyword\n\
                              - an arrow (->) followed by an expression\n\
                              - the cons operator (::) followed by more list elements\n").unwrap();
        }
//        RuntimeError::InternalErrorRecordAccess(_) => {}
//        RuntimeError::InternalErrorAdtCreation(_) => {}
//        RuntimeError::UnknownBuiltinFunction(_) => {}
//        RuntimeError::InvalidExpressionChain(_) => {},
        _ => {
            write!(&mut msg, "-- RUNTIME ERROR ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "{:?}", error).unwrap();
        }
    }
    msg
}

pub fn format_interop_error(error: &InteropError) -> String {
    let mut msg = String::new();
    write!(&mut msg, "-- interop ERROR ------------------------------------------------------------ elm\n\n").unwrap();
    write!(&mut msg, "{:?}", error).unwrap();
    msg
}

pub fn print_code_location(input: &str, start: u32, end: u32) -> String {
    if input.is_empty() {
        return String::from("Empty");
    }

    let byte_input: &[u8] = input.as_bytes();
    let marker_start = start as usize;
    let marker_end = end as usize;

    let mut line_start = marker_start.min(byte_input.len() - 1).max(0);
    let mut line_end = marker_end.min(byte_input.len() - 1).max(0);

    while line_start > 0 {
        if byte_input[line_start] == b'\n' {
            line_start += 1;
            break;
        }
        line_start -= 1;
    }

    while line_end < byte_input.len() {
        if byte_input[line_end] == b'\n' {
            break;
        }
        line_end += 1;
    }

    let mut line = String::new();
    let mut pointer = String::new();
    let mut trail = String::new();

    for index in line_start..line_end {
        if index == marker_start {
            trail.push('┘');
            pointer.push('\u{028C}');
        } else if index < marker_start {
            trail.push('─');
            pointer.push(' ');
        } else if index < marker_end {
            pointer.push('\u{028C}');
        }
        line.push(byte_input[index] as char);
    }

    let line_num = (&byte_input[0..marker_start]).iter().filter(|&i| *i == b'\n').count();
    let line_num_str = format!("{}", line_num + 1);
    let mut spaces = String::new();

    for _ in 0..line_num_str.len() {
        spaces.push(' ');
    }

    let mut output = String::new();
    write!(&mut output, "\n{} │ {}\n{} │ {}\n{} │ {}", line_num_str, line, spaces, pointer, spaces, trail).unwrap();

    output
}

impl Display for ErrorWrapper {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", format_error(self))
    }
}

impl Debug for ErrorWrapper {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", format_error(self))
    }
}
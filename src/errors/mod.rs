use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Write;
use std::sync::Arc;

use nom::ErrorKind;
use nom::Needed;

use analyzer::inter_mod_analyzer::ModulePath;
use analyzer::PatternMatchingError;
use ast::Pattern;
use ast::Span;
use interpreter::dynamic_env::DynamicEnv;
use loader::Declaration;
use loader::declaration_name;
use source::Location;
use source::SourceCode;
use tokenizer::Token;
use types::Value;
use util::expression_fold::ExprTreeError;
use util::format::print_vec;

#[derive(PartialEq, Clone)]
pub enum ElmError {
    Tokenizer { code: SourceCode, info: LexicalError },
    Parser { code: SourceCode, info: ParseError },
    Analyser { code: SourceCode, info: TypeError },
    Interpreter { info: RuntimeError },
    Interop { info: InteropError },
    Loader { info: LoaderError },
}

#[derive(PartialEq, Debug, Clone)]
pub enum LexicalError {
    ReachedEnd { pos: u32 },
    UnableToTokenize { span: Span },
    Incomplete(Needed),
    Error(Location, ErrorKind),
    Failure(Location, ErrorKind),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeError {
    List(Vec<TypeError>),
    MissingDefinition(Span, String),
    ListNotHomogeneous(Span, String),
    IfWithNonBoolCondition(Span, String),
    IfBranchesDoesntMatch(Span, String),
    ArgumentsDoNotMatch(Span, String),
    NotAFunction(Span, String),
    InvalidOperandChain(Span, String),
    RecordUpdateOnNonRecord(Span, String),
    RecordUpdateUnknownField(Span, String),
    CaseBranchDontMatchReturnType(Span, String),
    DefinitionTypeAndReturnTypeMismatch,
    InvalidPattern(Span, PatternMatchingError),
    ConstantEvaluationError(String),
    VariableAlreadyDeclared(String),
    UnableToCalculateFunctionType(String),
    VariableNameShadowed(String),
    UndeclaredTypeVariables(Vec<String>),
    UnusedTypeVariables(Vec<String>),
    InvalidPatternAmount(usize, usize),
    InternalError,
    CyclicStatementDependency(Vec<String>),
}


/// Enum with all possible parsing errors
#[derive(PartialEq, Debug, Clone)]
pub enum ParseError {
    //@formatter:off
    Expected                    { span: Span, expected: Token, found: Token },
    ExpectedInt                 { span: Span, found: Token },
    ExpectedId                  { span: Span, found: Token },
    ExpectedUpperId             { span: Span, found: Token },
    ExpectedBinaryOperator      { span: Span, found: Token },
    ExpectedIndentationLevel    { span: Span, expected: u32, found: u32 },
    ExpectedIndentation         { span: Span, found: Token },
    UnmatchedToken              { span: Span, found: Token, options: Vec<Token> },
    //@formatter:on
}

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeError {
    MissingModule(ModulePath),
    MissingDefinition(String, DynamicEnv),
    IncorrectDefType(TypeError),
    RecordUpdateOnNonRecord(String, Value),
    InvalidIfCondition(Value),
    InvalidExpressionChain(ExprTreeError),
    RecordFieldNotFound(String, Value),
    CaseExpressionNonExhaustive(Value, Vec<Pattern>),
    FunArgumentSizeMismatch(u32, u32),
    ExpectedRecord(Value),
    ExpectedFunction(Value),
    ExpectedAdt(Value),
    ExpectedTuple(Value),
    ExpectedList(Value),
    ExpectedFloat(Value),
    ExpectedInt(Value),
    ExpectedString(Value),
    ExpectedNumber(Value),
    ExpectedNonEmptyList(Value),
    UnknownOperatorPattern(String),
    InternalErrorRecordAccess(Value),
    InternalErrorAdtCreation(Value),
    UnknownBuiltinFunction(u32),
    BuiltinFunctionError,
    ImpossibleConversion,
    MissingSourceFile,
    CyclicModuleDependency(Vec<ModulePath>),
    MissingExposing(String, Vec<Declaration>),
    InternalError,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InteropError {
    FunctionArgMismatch,
    MismatchOutputType,
    FunctionNotFound(String),
    FunRegistrationUnknownTypeArg(usize),
    FunRegistrationUnknownTypeRet,
}

#[derive(Clone, Debug)]
pub enum LoaderError {
    IO { error: Arc<std::io::Error> },
    MissingDependencies { dependencies: Vec<String> },
    CyclicDependency { cycle: Vec<String> },
    MissingImport { name: String },
}

pub fn lexical_err<T>(code: &SourceCode, info: LexicalError) -> Result<T, ElmError> {
    Err(ElmError::Tokenizer { code: code.clone(), info })
}

pub fn parsing_err<T>(code: &SourceCode, info: ParseError) -> Result<T, ElmError> {
    Err(ElmError::Parser { code: code.clone(), info })
}

pub fn type_err<T>(code: &SourceCode, info: TypeError) -> Result<T, ElmError> {
    Err(ElmError::Analyser { code: code.clone(), info })
}

pub fn runtime_err(info: RuntimeError) -> ElmError {
    ElmError::Interpreter { info }
}

pub fn interop_err<T>(info: InteropError) -> Result<T, ElmError> {
    Err(ElmError::Interop { info })
}

pub fn loader_err<T>(info: LoaderError) -> Result<T, ElmError> {
    Err(ElmError::Loader { info })
}

pub fn format_error(error: &ElmError) -> String {
    match error {
        ElmError::Tokenizer { code, info } => { format_lexical_error(info) }
        ElmError::Parser { code, info } => { format_parse_error(code.as_str(), info) }
        ElmError::Analyser { code, info } => { format_type_error(code.as_str(), info) }
        ElmError::Interpreter { info } => { format_runtime_error(info) }
        ElmError::Interop { info } => { format_interop_error(info) }
        ElmError::Loader { info } => {
            // TODO
            String::from("TODO")
        }
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
            let loc = print_code_location(code, span);
            write!(&mut msg, "Expected token '{}', but found '{}': {}\n", expected, found, loc).unwrap()
        }
        ParseError::ExpectedInt { span, found } => {
            let loc = print_code_location(code, span);
            write!(&mut msg, "Expected integer, but found '{}': {}\n", found, loc).unwrap()
        }
        ParseError::ExpectedId { span, found } => {
            let loc = print_code_location(code, span);
            write!(&mut msg, "Expected identifier, but found '{}': {}\n", found, loc).unwrap()
        }
        ParseError::ExpectedUpperId { span, found } => {
            let loc = print_code_location(code, span);
            write!(&mut msg, "Expected capitalized identifier, but found '{}': {}\n", found, loc).unwrap()
        }
        ParseError::ExpectedBinaryOperator { span, found } => {
            let loc = print_code_location(code, span);
            write!(&mut msg, "Expected binary operator, but found '{}': {}\n", found, loc).unwrap()
        }
        ParseError::UnmatchedToken { span, found, .. } => {
            let loc = print_code_location(code, span);
            write!(&mut msg, "Found unexpected token '{}': {}\n", found, loc).unwrap()
        }
        ParseError::ExpectedIndentation { span, found } => {
            let loc = print_code_location(code, span);
            write!(&mut msg, "Expected indentation, but found '{}': {}\n", found, loc).unwrap()
        }
        ParseError::ExpectedIndentationLevel { span, expected, found } => {
            let loc = print_code_location(code, span);
            write!(&mut msg, "Expected indentation of {}, but found {}: {}\n", expected, found, loc).unwrap()
        }
    }
    msg
}

pub fn format_type_error(code: &str, error: &TypeError) -> String {
    let mut msg = String::new();
    match error {
        TypeError::List(errors) => {
            let len = errors.len();
            for e in errors {
                msg.push_str(&format_type_error(code, e));
                msg.push('\n');
            }

            write!(&mut msg, "\nFound {} errors\n", len).unwrap();
        }
        TypeError::MissingDefinition(span, name) => {
            write!(&mut msg, "-- NAMING ERROR ------------------------------------------------------------ elm\n\n").unwrap();
            let upper = name.chars().next().unwrap().is_ascii_uppercase();
            let ty = if upper { "constructor" } else { "variable" };
            write!(&mut msg, "I cannot find a `{}` {}:\n", name, ty).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code, span)).unwrap();
            write!(&mut msg, "Hint: Read <https://elm-lang.org/0.19.0/imports> to see how `import` declarations work in Elm.\n").unwrap();
        }
        TypeError::InvalidPattern(span, pe) => {
            write!(&mut msg, "-- PATTERN ERROR ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "{:?}\n", pe).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code, span)).unwrap();
        }
        TypeError::ArgumentsDoNotMatch(span, str) => {
            write!(&mut msg, "-- TYPE ERROR ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "{}\n", str).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code, span)).unwrap();
        }
        TypeError::NotAFunction(span, str) => {
            write!(&mut msg, "-- TYPE ERROR ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "{}\n", str).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code, span)).unwrap();
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
            // TODO
//            return format_type_error(e);
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
        RuntimeError::MissingExposing(name, decls) => {
            let names = decls.iter().map(|a| declaration_name(a)).collect::<Vec<&str>>();
            write!(&mut msg, "-- RUNTIME ERROR ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "Unable to expose '{}', available names: \n\n{:?}\n\n", name, names).unwrap();
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

pub fn print_code_location(input: &str, span: &Span) -> String {
    if input.is_empty() {
        return String::from("Empty");
    }

    let byte_input: &[u8] = input.as_bytes();
    let marker_start = span.0 as usize;
    let marker_end = span.1 as usize;

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

impl Display for ElmError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", format_error(self))
    }
}

impl Debug for ElmError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", format_error(self))
    }
}

macro_rules! iff {
    (let $p:pat = $e:expr) => {{
        if let $p = $e { true } else { false }
    }};
}

impl PartialEq for LoaderError {
    fn eq(&self, other: &LoaderError) -> bool {
        match self {
            LoaderError::IO { .. } => {
                // io:Error cannot be compared so we ignore this case
                true
            },
            LoaderError::MissingDependencies { dependencies: this, .. } => {
                if let LoaderError::MissingDependencies { dependencies: other } = other { this == other } else { false }
            },
            LoaderError::CyclicDependency { cycle: this, .. } => {
                if let LoaderError::CyclicDependency { cycle: other } = other { this == other } else { false }
            },
            LoaderError::MissingImport { name: this, .. } => {
                if let LoaderError::MissingImport { name: other } = other { this == other } else { false }
            },
        }
    }
}
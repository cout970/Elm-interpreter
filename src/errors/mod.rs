use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Write;
use std::sync::Arc;

use ast::Expr;
use ast::Pattern;
use ast::Span;
use ast::Type;
use loader::Declaration;
use loader::declaration_name;
use loader::SourceFile;
use source::SOURCE_CODE_PADDING;
use source::SourceCode;
use tokenizer::Token;
use typed_ast::{expr_type, TypedPattern};
use typed_ast::TypedExpr;
use types::{Function, Value};
use util::expression_fold::ExprTreeError;
use util::format::print_vec;

#[derive(PartialEq, Clone)]
pub enum ElmError {
    Tokenizer(SourceCode, LexicalError),
    Parser(SourceCode, ParseError),
    Analyser(SourceCode, TypeError),
    Interpreter(InterpreterError),
    Interop(InteropError),
    Loader(LoaderError),
    List(Vec<ElmError>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum LexicalError {
    ReachedEnd { pos: u32 },
    UnableToTokenize { span: Span },
}

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

// TODO remove unused error types
#[derive(Clone, Debug, PartialEq)]
pub enum TypeError {
    //@formatter:off
    PatternMatchingError                { span: Span, info: PatternMatchingError },
    MissingDefinition                   { span: Span, name: String },
    ListNotHomogeneous                  { span: Span, list_type: Type, item_type: Type, index: u32 },
    IfWithNonBoolCondition              { span: Span, expr: TypedExpr },
    IfBranchesDoesntMatch               { span: Span, true_branch: TypedExpr, false_branch: TypedExpr },
    ArgumentsDoNotMatch                 { span: Span, expected: Type, found: Type },
    NotAFunction                        { span: Span, function: Type, input: Expr, output: Expr },
    InvalidOperandChain                 { span: Span, msg: String },
    RecordUpdateOnNonRecord             { span: Span, expr: TypedExpr },
    RecordUpdateUnknownField            { span: Span, field: String, record_name: String, record: TypedExpr },
    CaseBranchDontMatchReturnType       { span: Span, expected: Type, found: Type },
    DefinitionTypeAndReturnTypeMismatch { span: Span, expected: Type, found: Type },
    VariableNameShadowed                { span: Span, name: String },
    UndeclaredTypeVariables             { name: String, values: Vec<String> },
    UnusedTypeVariables                 { name: String, values: Vec<String> },
    InvalidFunctionPatternAmount        { expected: usize, found: usize },
    CyclicStatementDependency           { cycle: Vec<String> },
    ExpectingRecordWithName             { record: TypedExpr, name: String },
    TypeMatchingError { span: Span, expected: Type, found: Type },
    RecursiveTypeDefinition { span: Span, var: String, ty: Type },
    UnknownType { span: Span, name: String },
    //@formatter:on
}

#[derive(Clone, Debug, PartialEq)]
pub enum PatternMatchingError {
    ListPatternsAreNotHomogeneous(Type, Type),
    UnknownOperatorPattern(String),
    UnknownAdtVariant(String),
    ExpectedListType(Type),
    ExpectedUnit(Type),
    ExpectedTuple(Pattern, Type),
    ExpectedRecord(Type),
    ExpectedAdt(String, Type),
    PatternNotExhaustive(Pattern),
    InvalidRecordEntryName(String),
    ExpectedLiteral(String, Type),
}

#[derive(Clone, Debug, PartialEq)]
pub enum InterpreterError {
    MissingModule(Vec<String>),
    MissingDefinition(String),
    IncorrectDefType(TypeError),
    RecordUpdateOnNonRecord(TypedExpr, Value),
    InvalidIfCondition(Value),
    InvalidExpressionChain(ExprTreeError),
    RecordFieldNotFound(String, Value),
    CaseExpressionNonExhaustive(Value, Vec<TypedPattern>),
    FunArgumentSizeMismatch(u32, u32, Arc<Function>),
    ExpectedRecord(Value),
    ExpectedFunction(Value),
    ExpectedAdt(Value),
    ExpectedTuple(Value),
    ExpectedList(Value),
    ExpectedFloat(Value),
    ExpectedInt(Value),
    ExpectedChar(Value),
    ExpectedString(Value),
    ExpectedBoolean(Value),
    ExpectedNumber(Value),
    ExpectedNonEmptyList(Value),
    UnknownOperatorPattern(String),
    InternalErrorRecordAccess(Value),
    InternalErrorAdtCreation(Value),
    UnknownBuiltinFunction(u32),
    BuiltinFunctionError,
    ImpossibleConversion,
    MissingSourceFile,
    CyclicModuleDependency(Vec<Vec<String>>),
    MissingExposing(String, Vec<Declaration>),
    FunctionTODO(String),
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
    IO { error: Arc<std::io::Error>, msg: String },
    MissingDependencies { dependencies: Vec<String>, src: SourceFile },
    CyclicDependency { cycle: Vec<String> },
    MissingModule { module: String },
    ModulePacking { msg: String, path: String },
}

pub fn format_error(error: &ElmError) -> String {
    match error {
        ElmError::Tokenizer(code, info) => format_lexical_error(code, info),
        ElmError::Parser(code, info) => format_parse_error(code, info),
        ElmError::Analyser(code, info) => format_type_error(code, info),
        ElmError::Interpreter(info) => format_runtime_error(info),
        ElmError::Interop(info) => format_interop_error(info),
        ElmError::Loader(info) => format_loader_error(info),
        ElmError::List(list) => {
            let mut msg = String::new();

            for error in list {
                msg.push_str(&format_error(error));
            }

            msg
        }
    }
}

pub fn format_lexical_error(code: &SourceCode, error: &LexicalError) -> String {
    let mut msg = String::new();
    msg.push_str("-- LEXICAL ERROR ------------------------------------------------------------- elm\n");

    match error {
        LexicalError::ReachedEnd { pos } => {
            let loc = print_code_location(code.as_str(), &(*pos, *pos + 1));
            write!(&mut msg, "Unable to read complete token: {}", loc).unwrap();
        },
        LexicalError::UnableToTokenize { span } => {
            let loc = print_code_location(code.as_str(), span);
            write!(&mut msg, "Unknown character sequence: {}", loc).unwrap();
        },
    }

    write!(&mut msg, "\n").unwrap();
    msg
}

pub fn format_parse_error(code: &SourceCode, error: &ParseError) -> String {
    let mut msg = String::new();
    msg.push_str("-- PARSE ERROR ------------------------------------------------------------- elm\n");

    match error {
        ParseError::Expected { span, expected, found } => {
            let loc = print_code_location(code.as_str(), span);
            write!(&mut msg, "Expected token '{}', but found '{}': {}", expected, found, loc).unwrap()
        }
        ParseError::ExpectedInt { span, found } => {
            let loc = print_code_location(code.as_str(), span);
            write!(&mut msg, "Expected integer, but found '{}': {}", found, loc).unwrap()
        }
        ParseError::ExpectedId { span, found } => {
            let loc = print_code_location(code.as_str(), span);
            write!(&mut msg, "Expected identifier, but found '{}': {}", found, loc).unwrap()
        }
        ParseError::ExpectedUpperId { span, found } => {
            let loc = print_code_location(code.as_str(), span);
            write!(&mut msg, "Expected capitalized identifier, but found '{}': {}", found, loc).unwrap()
        }
        ParseError::ExpectedBinaryOperator { span, found } => {
            let loc = print_code_location(code.as_str(), span);
            write!(&mut msg, "Expected binary operator, but found '{}': {}", found, loc).unwrap()
        }
        ParseError::UnmatchedToken { span, found, .. } => {
            let loc = print_code_location(code.as_str(), span);
            write!(&mut msg, "Found unexpected token '{}': {}", found, loc).unwrap()
        }
        ParseError::ExpectedIndentation { span, found } => {
            let loc = print_code_location(code.as_str(), span);
            write!(&mut msg, "Expected indentation, but found '{}': {}", found, loc).unwrap()
        }
        ParseError::ExpectedIndentationLevel { span, expected, found } => {
            let loc = print_code_location(code.as_str(), span);
            write!(&mut msg, "Expected indentation of {}, but found {}: {}", expected, found, loc).unwrap()
        }
    }

    write!(&mut msg, "\n").unwrap();
    msg
}

pub fn format_type_error(code: &SourceCode, error: &TypeError) -> String {
    let mut msg = String::new();
    write!(&mut msg, "-- TYPE ERROR ------------------------------------------------------------ elm\n").unwrap();

    match error {
        TypeError::PatternMatchingError { span, info } => {
            write!(&mut msg, "{:?}", info).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::MissingDefinition { span, name } => {
            write!(&mut msg, "I cannot find `{}`:\n", name).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::ListNotHomogeneous { span, list_type, item_type, index } => {
            write!(&mut msg, "Expected list of `{}`, but found item {} with type '{}'\n", list_type, index, item_type).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::IfWithNonBoolCondition { span, expr } => {
            write!(&mut msg, "If condition must have type Bool, but it was '{}'\n", expr_type(expr)).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::IfBranchesDoesntMatch { span, true_branch, false_branch } => {
            write!(&mut msg, "If branch types doesn't match: \n\ntrue branch: '{}', \n\nfalse branch: '{}'\n", expr_type(true_branch), expr_type(false_branch)).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::ArgumentsDoNotMatch { span, expected, found } => {
            write!(&mut msg, "Function call with incorrect argument types: \n\nexpected: '{}', \n\n   found: '{}'\n", expected, found).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::NotAFunction { span, function, input, .. } => {
            write!(&mut msg, "Attempt to call a non-function '{}' with input: \n'{:#?}'\n", function, input).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::InvalidOperandChain { span, msg: msg_ } => {
            write!(&mut msg, "{}\n", msg_).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::RecordUpdateOnNonRecord { span, expr } => {
            write!(&mut msg, "Trying to update record, but found '{}' instead\n", expr_type(expr)).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::RecordUpdateUnknownField { span, field, record_name, record } => {
            write!(&mut msg, "Cannot update non-existent field '{}' in record '{}' of type '{}'\n", field, record_name, expr_type(record)).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::CaseBranchDontMatchReturnType { span, expected, found } => {
            write!(&mut msg, "Case-of branch doesn't match expression type: \n\nexpected: '{}', \n\n   found: '{}'\n", expected, found).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::DefinitionTypeAndReturnTypeMismatch { span, expected, found } => {
            write!(&mut msg, "Type annotation and expresion type doesn't match: \n\nexpected: '{}', \n\n   found: '{}'\n", expected, found).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::VariableNameShadowed { span, name } => {
            write!(&mut msg, "Shadowed variable name '{}'\n", name).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::UndeclaredTypeVariables { name, values } => {
            write!(&mut msg, "Use of undeclared variable types on typealias '{}'\nvalues: {:?}", name, values).unwrap();
        },
        TypeError::UnusedTypeVariables { name, values } => {
            write!(&mut msg, "Definition of unused variable types on typealias '{}'\nvalues: {:?}", name, values).unwrap();
        },
        TypeError::InvalidFunctionPatternAmount { expected, found } => {
            write!(&mut msg, "Pattern amount mismatch, expected {} but found {}", expected, found).unwrap();
        },
        TypeError::CyclicStatementDependency { cycle } => {
            write!(&mut msg, "Found cyclic dependency between statements:\n\n   {}\n", cycle.join(" -> ")).unwrap();
        },
        TypeError::ExpectingRecordWithName { record, name } => {
            write!(&mut msg, "Expecting record with field '{}', but found '{}'", name, expr_type(record)).unwrap();
        },
        TypeError::TypeMatchingError { span, expected, found } => {
            write!(&mut msg, "Types doesn't match: \n\nexpected: '{}', \n   found: '{}'\n", expected, found).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::RecursiveTypeDefinition { span, var, ty } => {
            write!(&mut msg, "Found recursive type: \n\n var: '{}', \ntype: '{}'\n", var, ty).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
        TypeError::UnknownType { span, name } => {
            write!(&mut msg, "Found unknown type: {}\n", name).unwrap();
            write!(&mut msg, "{}\n\n", print_code_location(code.as_str(), span)).unwrap();
        },
    }

    write!(&mut msg, "\n").unwrap();
    msg
}

pub fn format_runtime_error(error: &InterpreterError) -> String {
    let mut msg = String::new();
    match error {
        InterpreterError::MissingDefinition(name) => {
            write!(&mut msg, "-- NAMING ERROR ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "I cannot find a `{}` variable:\n", name).unwrap();
            write!(&mut msg, "Hint: Read <https://elm-lang.org/0.19.0/imports> to see how `import` declarations work in Elm.").unwrap();
        }
        InterpreterError::IncorrectDefType(_) => {
            // TODO
//            return format_type_error(e);
        }
        InterpreterError::RecordUpdateOnNonRecord(field, value) => {
            write!(&mut msg, "-- TYPE MISMATCH ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a record with the field `{}` but found:\n\n{}\n\n", field, value).unwrap();
            write!(&mut msg, "Maybe you forgot some code?").unwrap();
        }
        InterpreterError::InvalidIfCondition(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "This `if` condition does not evaluate to a boolean value, True or False.\n\n").unwrap();
            write!(&mut msg, "It is a value of type:\n\n{}\n\nBut I need this `if` condition to be a Bool value.", value).unwrap();
            write!(&mut msg, "Hint: Elm does not have “truthiness” such that ints and strings and lists are \
                              automatically converted to booleans. Do that conversion explicitly!").unwrap();
        }
        InterpreterError::RecordFieldNotFound(field, value) => {
            write!(&mut msg, "-- TYPE MISMATCH ------------------------------------------------------------ elm\n\n").unwrap();
            write!(&mut msg, "This record does not have a `{}` field:\n\n{}\n\n", field, value).unwrap();
            write!(&mut msg, "This is usually a typo.").unwrap();
        }
        InterpreterError::CaseExpressionNonExhaustive(value, branches) => {
            write!(&mut msg, "-- MISSING PATTERNS -------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "This `case` does not have branches for all possibilities:\n\n{}\n\n", value).unwrap();
            write!(&mut msg, "Is not included in the existing branches:\n\n").unwrap();
            print_vec(&mut msg, &branches).unwrap();
            write!(&mut msg, "\n\nHint: If you want to write the code for each branch later, use `Debug.todo` as a \
                              placeholder. Read <https://elm-lang.org/0.19.0/missing-patterns> for more \
                              guidance on this workflow.").unwrap();
        }
        InterpreterError::ExpectedRecord(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a record but found:\n\n{}\n\n", value).unwrap();
        }
        InterpreterError::ExpectedFunction(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a function but found:\n\n{}\n\n", value).unwrap();
        }
        InterpreterError::ExpectedAdt(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a adt but found:\n\n{}\n\n", value).unwrap();
        }
        InterpreterError::ExpectedTuple(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a tuple but found:\n\n{}\n\n", value).unwrap();
        }
        InterpreterError::ExpectedList(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a list but found:\n\n{}\n\n", value).unwrap();
        }
        InterpreterError::ExpectedFloat(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a float but found:\n\n{}\n\n", value).unwrap();
        }
        InterpreterError::ExpectedInt(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a int but found:\n\n{}\n\n", value).unwrap();
        }
        InterpreterError::ExpectedNumber(value) => {
            write!(&mut msg, "-- TYPE MISMATCH ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a number but found:\n\n{}\n\n", value).unwrap();
        }
        InterpreterError::FunArgumentSizeMismatch(expected, found, func) => {
            write!(&mut msg, "-- TOO MANY ARGS ----------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "The `{}` function expects {} argument, but it got {} instead.\n{:?}\n", func.get_type(), expected, found, func).unwrap();
            write!(&mut msg, "Are there any missing commas? Or missing parentheses?").unwrap();
        }
        InterpreterError::ExpectedNonEmptyList(value) => {
            write!(&mut msg, "-- PATTERN MATCHING ERROR -------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I was expecting a non empty list, but found:\n\n{}\n\n", value).unwrap();
            write!(&mut msg, "Try adding a extra branch for []").unwrap();
        }
        InterpreterError::UnknownOperatorPattern(name) => {
            write!(&mut msg, "-- PARSE ERROR ------------------------------------------------------------- elm\n\n").unwrap();
            write!(&mut msg, "I cannot use the `{}` operator\n\n", name).unwrap();
            write!(&mut msg, "I was expecting:\n\n\
                              - the `as` keyword\n\
                              - an arrow (->) followed by an expression\n\
                              - the cons operator (::) followed by more list elements\n").unwrap();
        }
        InterpreterError::MissingExposing(name, decls) => {
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
    write!(&mut msg, "-- RUST INTEROP ERROR ------------------------------------------------------------ elm\n").unwrap();
    write!(&mut msg, "{:?}", error).unwrap();
    msg
}

pub fn format_loader_error(error: &LoaderError) -> String {
    let mut msg = String::new();
    write!(&mut msg, "-- MODULE LOADING ERROR ------------------------------------------------------------ elm\n").unwrap();

    match error {
        LoaderError::IO { error, msg: m } => {
            write!(&mut msg, "IO error: {} => {}", m, error).unwrap();
        },
        LoaderError::MissingDependencies { dependencies, src } => {
            write!(&mut msg, "Unsatisfied dependencies for module '{}' at '{}'. \nRequired dependencies: ", src.name, src.path).unwrap();

            for (i, item) in dependencies.iter().enumerate() {
                write!(&mut msg, "{}", item).unwrap();

                if i != dependencies.len() - 1 {
                    write!(&mut msg, ", ").unwrap();
                }
            }
        },
        LoaderError::CyclicDependency { cycle } => {
            write!(&mut msg, "Found cyclic dependencies: \n|\n|    ").unwrap();

            for item in cycle {
                write!(&mut msg, "{}", item).unwrap();
                write!(&mut msg, " -> ").unwrap();
            }

            write!(&mut msg, "{}\n|", &cycle[0]).unwrap();
        },
        LoaderError::MissingModule { module } => {
            write!(&mut msg, "Missing module '{}'", module).unwrap();
        },
        LoaderError::ModulePacking { msg: error_msg, path } => {
            write!(&mut msg, "Unable to unpack module at '{}': {}", path, error_msg).unwrap();
        },
    }

    write!(&mut msg, "\n").unwrap();
    msg
}

/// Prints a line of code and a pointer to a region in that line
/// For example: input = "test", span = (0, 3)
/// ``` output
/// 1 │ test
///   │ ʌʌʌ
///   │ ┘
/// ```
pub fn print_code_location(input: &str, span: &Span) -> String {
    if input.is_empty() || (input.len() == SOURCE_CODE_PADDING && input.as_bytes()[0] == b'\0') {
        return String::from("<No code available>");
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

impl PartialEq for LoaderError {
    fn eq(&self, other: &LoaderError) -> bool {
        match self {
            LoaderError::IO { .. } => {
                // io:Error cannot be compared so we ignore this case
                false
            },
            LoaderError::MissingDependencies { dependencies: this, src: name0, .. } => {
                if let LoaderError::MissingDependencies { dependencies: other, src: name1, .. } = other { this == other && name0 == name1 } else { false }
            },
            LoaderError::CyclicDependency { cycle: this, .. } => {
                if let LoaderError::CyclicDependency { cycle: other } = other { this == other } else { false }
            },
            LoaderError::MissingModule { module: this, .. } => {
                if let LoaderError::MissingModule { module: other } = other { this == other } else { false }
            },
            LoaderError::ModulePacking { msg: this, path: name0, .. } => {
                if let LoaderError::ModulePacking { msg: other, path: name1, .. } = other { this == other && name0 == name1 } else { false }
            },
        }
    }
}

pub trait Wrappable {
    type Wrapper;

    fn wrap(self) -> Self::Wrapper;
}

impl Wrappable for InterpreterError {
    type Wrapper = ElmError;

    fn wrap(self) -> ElmError {
        ElmError::Interpreter(self)
    }
}

impl Wrappable for InteropError {
    type Wrapper = ElmError;

    fn wrap(self) -> ElmError {
        ElmError::Interop(self)
    }
}

impl Wrappable for LoaderError {
    type Wrapper = ElmError;

    fn wrap(self) -> ElmError {
        ElmError::Loader(self)
    }
}


#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn print_loader_error() {
        eprintln!("{}", format_loader_error(&LoaderError::IO {
            error: Arc::new(fs::read_dir("non-existent-folder").unwrap_err()),
            msg: format!("read folder 'non-existent-folder'"),
        }));
        eprintln!("{}", format_loader_error(&LoaderError::MissingDependencies {
            dependencies: vec![format!("SubModule1"), format!("SubModule2")],
            src: SourceFile {
                name: "Main".to_string(),
                path: "root/Main.elm".to_string(),
                source: SourceCode::from_str("module Main"),
            },
        }));
        eprintln!("{}", format_loader_error(&LoaderError::CyclicDependency {
            cycle: vec![format!("Main"), format!("SubModule1"), format!("SubModule2")],
        }));
        eprintln!("{}", format_loader_error(&LoaderError::MissingModule {
            module: format!("Main"),
        }));
    }

    #[test]
    fn print_lexical_error() {
        eprintln!("{}", format_lexical_error(
            &SourceCode::from_str("test"),
            &LexicalError::ReachedEnd { pos: 2 },
        ));
        eprintln!("{}", format_lexical_error(
            &SourceCode::from_str("test"),
            &LexicalError::UnableToTokenize { span: (0, 3) },
        ));
    }

    #[test]
    fn print_parse_error() {
        eprintln!("{}", format_parse_error(
            &SourceCode::from_str("test"),
            &ParseError::Expected { span: (0, 4), expected: Token::If, found: Token::Id(format!("test")) },
        ));
        eprintln!("{}", format_parse_error(
            &SourceCode::from_str("test"),
            &ParseError::ExpectedInt { span: (0, 4), found: Token::Id(format!("test")) },
        ));
        eprintln!("{}", format_parse_error(
            &SourceCode::from_str("1"),
            &ParseError::ExpectedId { span: (0, 1), found: Token::LitInt(1) },
        ));
        eprintln!("{}", format_parse_error(
            &SourceCode::from_str("test"),
            &ParseError::ExpectedBinaryOperator { span: (0, 4), found: Token::Id(format!("test")) },
        ));
        eprintln!("{}", format_parse_error(
            &SourceCode::from_str(" test"),
            &ParseError::ExpectedIndentationLevel { span: (0, 1), expected: 4, found: 1 },
        ));
        eprintln!("{}", format_parse_error(
            &SourceCode::from_str("test"),
            &ParseError::ExpectedIndentation { span: (0, 4), found: Token::Id(format!("test")) },
        ));
        eprintln!("{}", format_parse_error(
            &SourceCode::from_str("test"),
            &ParseError::UnmatchedToken { span: (0, 4), found: Token::Id(format!("test")), options: vec![Token::If, Token::Let] },
        ));
    }
}
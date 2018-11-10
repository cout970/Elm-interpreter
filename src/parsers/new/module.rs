use ast::AdtExposing;
use ast::Exposing;
use ast::Import;
use ast::Module;
use ast::ModuleExposing;
use ast::ModuleHeader;
use parsers::new::Input;
use parsers::new::ParseError;
use parsers::new::statement::parse_statement;
use parsers::new::util::comma1;
use parsers::new::util::expect;
use parsers::new::util::expect_indent;
use parsers::new::util::expect_upper;
use parsers::new::util::many0;
use parsers::new::util::many1;
use tokenizer::Token;
use util::create_vec;

pub fn parse_module(input: Input) -> Result<(Module, Input), ParseError> {
    let i = skip_empty_lines(input)?;

    let (header, i) = match i.read() {
        Token::ModuleTk => {
            let (header, i) = parse_module_header(i)?;
            (Some(header), i)
        }
        _ => {
            (None, i)
        }
    };

    let mut imports = vec![];
    let mut i = i;

    loop {
        i = skip_empty_lines(i)?;
        match i.read() {
            Token::ImportTk => {
                let pair = parse_import(i)?;
                imports.push(pair.0);
                i = pair.1;
            }
            _ => {
                break;
            }
        }
    }

    let mut statements = vec![];
    let mut i = i;

    loop {
        i = skip_empty_lines(i)?;

        if let Token::Eof = i.read() {
            break;
        }

        match parse_statement(i.clone()) {
            Ok((stm, input)) => {
                statements.push(stm);
                i = input;
            }
            Err(_) => {
                // TODO collect all errors
                i = skip_to_the_next_block(i);
            }
        }
    }

    Ok((Module { header, imports, statements }, i))
}

fn skip_to_the_next_block(input: Input) -> Input {
    let mut i = input;
    loop {
        match i.read() {
            Token::Indent(0) => {
                break;
            }
            _ => {
                i = i.next();
            }
        }
    }

    i
}

fn skip_empty_lines(input: Input) -> Result<Input, ParseError> {
    let mut i = input;
    loop {
        match expect_indent(0, i.clone()) {
            Ok(_) => {
                i = i.next()
            }
            Err(_) => {
                break;
            }
        }
    }
    Ok(i)
}

fn parse_module_header(input: Input) -> Result<(ModuleHeader, Input), ParseError> {
    let i = expect(Token::ModuleTk, input)?;
    let (name, i) = expect_upper(i)?;
    let i = expect(Token::ExposingTk, i)?;
    let i = expect(Token::LeftParen, i)?;
    let (exposing, i) = match i.read() {
        Token::DoubleDot => {
            (ModuleExposing::All, i.next())
        }
        _ => {
            let (exposing, i) = many1(&parse_exposing, i)?;
            (ModuleExposing::Just(exposing), i)
        }
    };
    let i = expect(Token::RightParen, i)?;

    Ok((ModuleHeader { name, exposing }, i))
}


fn parse_import(input: Input) -> Result<(Import, Input), ParseError> {
    let i = expect(Token::ImportTk, input)?;
    let (first, i) = expect_upper(i)?;
    let (rest, i) = many0(&parse_dot_name, i)?;

    let (import, i) = match i.read() {
        Token::As => {
            let (alias, i) = expect_upper(i.next())?;

            (Import::Alias(create_vec(first, rest), alias), i)
        }
        Token::ExposingTk => {
            let i = expect(Token::LeftParen, i.next())?;
            let (exposing, i) = match i.read() {
                Token::DoubleDot => {
                    (ModuleExposing::All, i.next())
                }
                _ => {
                    let (exps, i) = comma1(&parse_exposing, i)?;
                    (ModuleExposing::Just(exps), i)
                }
            };
            let i = expect(Token::RightParen, i)?;

            (Import::Exposing(create_vec(first, rest), exposing), i)
        }
        _ => {
            (Import::Module(create_vec(first, rest)), i)
        }
    };

    Ok((import, i))
}

fn parse_dot_name(input: Input) -> Result<(String, Input), ParseError> {
    let i = expect(Token::Dot, input)?;
    let (name, i) = expect_upper(i)?;

    Ok((name, i))
}

fn parse_exposing(input: Input) -> Result<(Exposing, Input), ParseError> {
    match input.read() {
        Token::Id(def) => Ok((Exposing::Definition(def), input.next())),
        Token::UpperId(name) => {
            let i = input.next();
            match i.read() {
                Token::LeftParen => {
                    let i = i.next();
                    let (exp, i) = match i.read() {
                        Token::DoubleDot => {
                            (AdtExposing::All, i.next())
                        }
                        _ => {
                            let (j, i) = comma1(&expect_upper, i)?;
                            (AdtExposing::Variants(j), i)
                        }
                    };
                    let i = expect(Token::RightParen, i)?;

                    Ok((Exposing::Adt(name, exp), i))
                }
                _ => {
                    Ok((Exposing::Type(name), input.next()))
                }
            }
        }
        _ => {
            let found = input.read();
            return Err(ParseError::UnmatchedToken { input, found, options: vec![] });
        }
    }
}

#[cfg(test)]
mod tests {
    use parsers::new::util::test_parser;
    use parsers::new::util::test_parser_error;
    use super::*;

    #[test]
    fn import_test() {
        test_parser(parse_import, "import Util");
        test_parser(parse_import, "import Util as U");
        test_parser(parse_import, "import Util exposing (func)");
        test_parser(parse_import, "import Util exposing (A)");
        test_parser(parse_import, "import Util exposing (A, b, C, d)");
        test_parser(parse_import, "import Util exposing (A(..))");
        test_parser(parse_import, "import Util exposing (A(B))");
        test_parser(parse_import, "import Util exposing (A(B, C))");
        test_parser(parse_import, "import Defs exposing (Grid)");
    }

    #[test]
    fn import_error_test() {
        test_parser_error(parse_import, "import Util as U exposing (A)");
        test_parser_error(parse_import, "import Util exposing (A())");
        test_parser_error(parse_import, "import Util exposing ()");
    }

    #[test]
    fn module_header_test() {
        test_parser(parse_module_header, "module Main exposing (..)");
        test_parser(parse_module_header, "module Util exposing (func)");
        test_parser(parse_module_header, "module Util exposing (A)");
        test_parser(parse_module_header, "module Util exposing (A(..))");
        test_parser(parse_module_header, "module Util exposing (A(B))");
        test_parser(parse_module_header, "module Util exposing (A(B, C))");
    }

    #[test]
    fn basic_module_test() {
        test_parser(parse_module, "\nmodule Main exposing (..)\nimport Util\n
        \n\nx = 0\nfunc x = x\n");
    }

    #[test]
    fn complex_module_test() {
        test_parser(parse_module, include_str!("/Data/Dev/Elm/AI/src/AI.elm"));
        test_parser(parse_module, include_str!("/Data/Dev/Elm/AI/src/Defs.elm"));
        test_parser(parse_module, include_str!("/Data/Dev/Elm/AI/src/Element.elm"));
        test_parser(parse_module, include_str!("/Data/Dev/Elm/AI/src/Map.elm"));
        test_parser(parse_module, include_str!("/Data/Dev/Elm/AI/src/Util.elm"));
        test_parser(parse_module, include_str!("/Data/Dev/Elm/AI/src/Vec.elm"));
    }
}
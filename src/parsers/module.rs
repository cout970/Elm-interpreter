use ast::AdtExposing;
use ast::Exposing;
use ast::Import;
use ast::Module;
use ast::ModuleExposing;
use ast::ModuleHeader;
use parsers::input::Input;
use parsers::ParseError;
use parsers::statement::parse_statement;
use parsers::util::*;
use tokenizer::Token;
use util::create_vec;

pub fn parse_module(input: Input) -> Result<(Module, Input), ParseError> {
    let i = skip_empty_lines(input)?;

    let (header, i) = match i.read() {
        Token::ModuleTk | Token::EffectTk => {
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
            Err(e) => {
                // TODO collect all errors
                return Err(e);
//                i = skip_to_the_next_block(i);
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
    let (name, i) = match input.read() {
        Token::ModuleTk => {
            // module String exposing (..)
            let i = input.next();
            expect_upper_chain(i)?
        }
        Token::EffectTk => {
            // effect module Task where { command = MyCmd } exposing (..)
            let i = expect(Token::ModuleTk, input.next())?;
            let (name, i) = expect_upper_chain(i)?;
            let i = expect(Token::WhereTk, i)?;
            let i = expect(Token::LeftBrace, i)?;
            let (_, i) = expect_id(i)?;
            let i = expect(Token::Equals, i)?;
            let (_, i) = expect_upper(i)?;
            let i = expect(Token::RightBrace, i)?;

            (name, i)
        }
        _ => {
            let found = input.read();
            return Err(ParseError::UnmatchedToken { span: input.span(), found, options: vec![Token::ModuleTk, Token::EffectTk] });
        }
    };

    let i = expect(Token::ExposingTk, i)?;
    let i = expect(Token::LeftParen, i)?;
    let (exposing, i) = match i.read() {
        Token::DoubleDot => {
            (ModuleExposing::All, i.next())
        }
        _ => {
            let (exposing, i) = comma1(&parse_exposing, i)?;
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

    let path = create_vec(first, rest);

    let (alias, i) = match i.read() {
        Token::As => {
            let (alias, i) = expect_upper(i.next())?;

            (Some(alias), i)
        }
        _ => (None, i)
    };

    let (exposing, i) = match i.read() {
        Token::ExposingTk => {
            let i = expect(Token::LeftParen, i.next())?;
            let (exposing, i) = match i.read() {
                Token::DoubleDot => {
                    (ModuleExposing::All, i.next())
                }
                _ => {
                    let (exposings, i) = comma1(&parse_exposing, i)?;
                    (ModuleExposing::Just(exposings), i)
                }
            };
            let i = expect(Token::RightParen, i)?;

            (Some(exposing), i)
        }
        _ => (None, i)
    };

    Ok((Import { path, alias, exposing }, i))
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
        Token::LeftParen => {
            let (op, i) = expect_binop(input.next())?;
            let i = expect(Token::RightParen, i)?;
            Ok((Exposing::BinaryOperator(op), i))
        }
        _ => {
            let found = input.read();
            let options = vec![Token::Id("definition".to_owned()), Token::UpperId("type".to_owned()), Token::LeftParen];
            return Err(ParseError::UnmatchedToken { span: input.span(), found, options });
        }
    }
}

#[cfg(test)]
mod tests {
    use ast::Definition;
    use ast::Expr;
    use ast::Literal;
    use ast::Pattern;
    use ast::Statement;
    use parsers::util::test_utils::*;
    use util::StringConversion;

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
        test_parser(parse_import, "import Util as U exposing (A)");
        // TODO alias
//        test_parser(parse_import, "import Util exposing (A as B)");
    }

    #[test]
    fn import_error_test() {
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

//    #[test]
//    fn complex_module_test() {
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/AI/src/AI.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/AI/src/Defs.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/AI/src/Element.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/AI/src/Map.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/AI/src/Util.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/AI/src/Vec.elm"));
//    }
//
//    #[test]
//    fn elm_core_module_test() {
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Array.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Bitwise.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Debug.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/List.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Platform.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Result.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/String.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Tuple.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Basics.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Char.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Dict.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Maybe.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Process.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Set.elm"));
//        test_parser(parse_module, include_str!("/Data/Dev/Elm/core-master/src/Task.elm"));
//    }

    #[test]
    fn check_empty_module() {
        test_parser_result(parse_module, "module My_module exposing (..)", Module {
            header: Some(ModuleHeader {
                name: "My_module".s(),
                exposing: ModuleExposing::All,
            }),
            imports: vec![],
            statements: vec![],
        });
    }

    #[test]
    fn check_only_defs() {
        test_parser_result(parse_module, "func a = a + 1", Module {
            header: None,
            imports: vec![],
            statements: vec![
                Statement::Def(Definition {
                    header: None,
                    name: "func".s(),
                    patterns: vec![Pattern::Var((0, 0), "a".s())],
                    expr: Expr::OpChain(
                        (0, 0),
                        vec![Expr::Ref((0, 0), "a".s()), Expr::Literal((0, 0), Literal::Int(1))],
                        vec!["+".s()],
                    ),
                })
            ],
        });
    }

    #[test]
    fn check_module_exports() {
        test_parser_result(parse_module, "module MyMod exposing ( List, Maybe )", Module {
            header: Some(ModuleHeader {
                name: "MyMod".s(),
                exposing: ModuleExposing::Just(vec![
                    Exposing::Type("List".s()),
                    Exposing::Type("Maybe".s()),
                ]),
            }),
            imports: vec![],
            statements: vec![],
        });
    }

    #[test]
    fn check_module_imports() {
        test_parser_result(parse_module, "import Html exposing (..)", Module {
            header: None,
            imports: vec![
                Import { path: vec!["Html".s()], exposing: Some(ModuleExposing::All), alias: None }
            ],
            statements: vec![],
        });
    }

    #[test]
    fn check_module_imports_adv() {
        let code = "\
import Html exposing (..)\n\
import Util\n\
import Util as U\n\
import Util exposing (..)\n\
import Util exposing (Enum, map, Sides(..), UpDown(Up, Down))\n\
import Elm.JsArray as JsArray exposing (JsArray)\n\
";

        test_parser_result(parse_module, code, Module {
            header: None,
            imports: vec![
                Import { path: vec!["Html".s()], exposing: Some(ModuleExposing::All), alias: None },
                Import { path: vec!["Util".s()], exposing: None, alias: None },
                Import { path: vec!["Util".s()], exposing: None, alias: Some("U".s()) },
                Import { path: vec!["Util".s()], exposing: Some(ModuleExposing::All), alias: None },
                Import {
                    path: vec!["Util".s()],
                    exposing: Some(ModuleExposing::Just(
                        vec![
                            Exposing::Type("Enum".s()),
                            Exposing::Definition("map".s()),
                            Exposing::Adt("Sides".s(), AdtExposing::All),
                            Exposing::Adt("UpDown".s(), AdtExposing::Variants(
                                vec!["Up".s(), "Down".s()]
                            )),
                        ]
                    )),
                    alias: None,
                },
                Import {
                    path: vec!["Elm".s(), "JsArray".s()],
                    exposing: Some(ModuleExposing::Just(vec![Exposing::Type("JsArray".s())])),
                    alias: Some("JsArray".s()),
                },
            ],
            statements: vec![],
        });
    }
}

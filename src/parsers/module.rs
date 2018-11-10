use ast::*;
use nom::*;
use nom::verbose_errors::Context;
use parsers::statement::*;
use parsers::SyntaxError;
use parsers::Tk;
use tokenizer::Token::*;
use tokenizer::Token;
use tokenizer::TokenStream;

// Modules
// https://github.com/durkiewicz/elm-plugin/blob/master/src/main/java/org/elmlang/intellijplugin/Elm.bnf

pub fn read_module(i: Tk) -> IResult<Tk, Module, SyntaxError> {
//    do_parse!(i,
//        many0!(indent!()) >>
//        header: opt!(module_header) >>
//        imports: many0!(import) >>
//        statements: many0!(top_level_statement) >>
//        many0!(indent!()) >>
//        tk!(Eof) >>
//        (Module { header, imports, statements })
//    )

    // TODO remove
    // DEBUG
//    check_split_blocks(i.clone());

    let blocks = split_blocks(i.clone());
    let mut index = 0;
    let mut errors: Vec<(TokenStream, ErrorKind<SyntaxError>)> = vec![];

    let header: Option<ModuleHeader> = if let Some(block) = blocks.first() {
        match module_header(block.clone()) {
            Ok((rest, header)) => {
                if rest.len() <= 1 {
                    index += 1;
                    Some(header)
                } else {
                    errors.push((block.clone(), ErrorKind::Custom(
                        SyntaxError::UnableToConsumeAllInput(rest.read_info())
                    )));
                    None
                }
            }
            Err(e) => {
                errors.push((block.clone(), get_error_kind(e)));
                None
            }
        }
    } else {
        None
    };

    let mut imports: Vec<Import> = vec![];

    while index < blocks.len() {
        let block = &blocks[index];

        if let Token::ImportTk = block.read_tk() {
            match import(block.clone()) {
                Ok((rest, import)) => {
                    if rest.len() <= 1 {
                        imports.push(import);
                        index += 1;
                    } else {
                        errors.push((block.clone(), ErrorKind::Custom(
                            SyntaxError::UnableToConsumeAllInput(rest.read_info())
                        )));
                        break;
                    }
                }
                Err(e) => {
                    errors.push((block.clone(), get_error_kind(e)));
                    break;
                }
            }
        } else {
            break;
        }
    }

    let mut statements = vec![];

    while index < blocks.len() {
        let block = &blocks[index];

        // TODO function type and definition are in different blocks
        match read_statement(block.clone()) {
            Ok((rest, statement)) => {
                if rest.len() <= 1 {
                    statements.push(statement);
                    index += 1;
                } else {
                    errors.push((block.clone(), ErrorKind::Custom(
                        SyntaxError::UnableToConsumeAllInput(rest.read_info())
                    )));
                    break;
                }
            }
            Err(e) => {
                errors.push((block.clone(), get_error_kind(e)));
                break;
            }
        }
    }

    if index == blocks.len() {
        Ok((i.next(i.all.len() as u32), Module { header, imports, statements }))
    } else {
        Err(Err::Error(
            Context::List(errors)
        ))
    }
}


fn get_error_kind(e: Err<TokenStream, SyntaxError>) -> ErrorKind<SyntaxError> {
    match e {
        Err::Incomplete(i) => { ErrorKind::Custom(SyntaxError::IncompleteInput(i)) }
        Err::Error(ctx) => {
            match ctx {
                Context::Code(_, kind) => kind,
                Context::List(list) => list.first().unwrap().1.clone(),
            }
        }
        Err::Failure(ctx) => {
            match ctx {
                Context::Code(_, kind) => kind,
                Context::List(list) => list.first().unwrap().1.clone(),
            }
        }
    }
}

rule!(pub upper_ids<Vec<String>>, separated_nonempty_list!(tk!(Dot), upper_id!()));

rule!(pub read_ref<String>, alt!(
    id!() |
    do_parse!(
        tk!(LeftParen) >>
        op: binop!() >>
        tk!(RightParen) >>
        (op)
    )
));


// TODO add
//effect module Task where { command = MyCmd } exposing
rule!(module_header<ModuleHeader>, alt!(simple_module_header | effect_module_header));

rule!(effect_module_header<ModuleHeader>, do_parse!(
    tk!(EffectTk) >>
    tk!(ModuleTk) >>
    indentation >>
    mod_id: upper_id!() >>
    indentation >>
    tk!(WhereTk) >>
    tk!(LeftBracket) >>
    id!() >>
    tk!(Equals) >>
    upper_id!() >>
    tk!(RightBracket) >>
    e: exposing >>
    (ModuleHeader { name: mod_id, exposing: e })
));

rule!(simple_module_header<ModuleHeader>, do_parse!(
    tk!(ModuleTk) >>
    indentation >>
    mod_id: upper_id!() >>
    indentation >>
    e: exposing >>
    (ModuleHeader { name: mod_id, exposing: e })
));

rule!(exposing<ModuleExposing>, do_parse!(
    tk!(ExposingTk) >>
    indentation >>
    tk!(LeftParen) >>
    indentation >>
    e: exposing_int >>
    indentation >>
    tk!(RightParen) >>
    (e)
));

rule!(exposing_int<ModuleExposing>, alt!(exposing_all | exposing_list));

rule!(exposing_all<ModuleExposing>, do_parse!(
    indentation >>
    tk!(DoubleDot) >>
    (ModuleExposing::All)
));

rule!(exposing_list<ModuleExposing>, do_parse!(
    e: separated_nonempty_list!(comma_separator, exposing_item) >>
    (ModuleExposing::Just(e))
));

rule!(exposing_item<Exposing>, alt!(
    do_parse!(
        i: id!() >> (Exposing::Definition(i))
    )
    | do_parse!(
        i: upper_id!() >>
        e: adt_exposing >>
        (Exposing::Adt(i, e))
    )
    | do_parse!(
        i: upper_id!() >> (Exposing::Type(i))
    )
));

rule!(adt_exposing<AdtExposing>, alt!(
    do_parse!(
        tk!(LeftParen) >>
        tk!(DoubleDot) >>
        tk!(RightParen) >>
        (AdtExposing::All)
    )
    | do_parse!(
        tk!(LeftParen) >>
        b: separated_list!(comma_separator, upper_id!()) >>
        tk!(RightParen) >>
        (AdtExposing::Variants(b))
    )
));

rule!(import<Import>, alt!(
    do_parse!(
        path: import_pre >>
        exp: exposing >>
        (Import::Exposing(path, exp))
    )
    | do_parse!(
        path: import_pre >>
        tk!(As) >>
        alias: upper_id!() >>
        (Import::Alias(path, alias))
    )
    | do_parse!(
        path: import_pre >>
        (Import::Module(path))
    )
));

rule!(import_pre<Vec<String>>, do_parse!(
    many0!(indent!()) >>
    tk!(ImportTk) >>
    path: upper_ids >>
    (path)
));

rule!(comma_separator<()>, do_parse!(
    indentation >>
    tk!(Comma) >>
    indentation >>
    (())
));

rule!(indentation<()>, do_parse!(
    many0!(indent_except!(vec![0])) >>
    (())
));

// Breaks a list of tokens in blocks separated by line starts (Token::Indent(0)),
// each block can be a module header, a import, a type definition or
// a function definitions (type def and value def)
fn split_blocks(i: TokenStream) -> Vec<TokenStream> {
    let mut blocks: Vec<TokenStream> = vec![];

    let mut ptr = i.clone();
    let mut start = 0;
    let mut end = 0;
    loop {
        let tk = ptr.read_tk();

        match tk {
            Token::Indent(amount) => {
                if amount == 0 {
                    if start != end {
                        blocks.push(TokenStream {
                            all: i.all,
                            remaining: &i.remaining[start..(end + 1)],
                        });
                    }
                    start = end + 1;
                }
            }
            Token::Eof => {
                if start != end {
                    blocks.push(TokenStream {
                        all: i.all,
                        remaining: &i.remaining[start..(end + 1)],
                    });
                }
                break;
            }
            _ => {}
        }

        end += 1;
        ptr = ptr.next(1);
    }

    blocks
}

fn check_split_blocks(s: TokenStream) {
    let blocks = split_blocks(s);

    for block in blocks {
        print!("({:03}): ", block.remaining.len());
        for info in block.remaining {
            print!("{} ", info.token);
        }
        println!();
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use tokenizer::tokenize;
//    use tokenizer::TokenStream;
//    use util::StringConversion;
//
//    #[test]
//    fn check_empty_module() {
//        let tokens = tokenize(b"module My_module exposing (..)").unwrap();
//        let m = read_module(TokenStream::new(&tokens));
//        assert_ok!(m, Module {
//              header: Some(ModuleHeader{
//                    name: "My_module".s(),
//                    exposing: ModuleExposing::All,
//              }),
//              imports: vec![],
//              statements: vec![],
//        });
//    }
//
//    #[test]
//    fn check_only_defs() {
//        let tokens = tokenize(b"func a = a + 1").unwrap();
//        let m = read_module(TokenStream::new(&tokens));
//        assert_ok!(m, Module {
//            header: None,
//            imports: vec![],
//            statements: vec![
//                Statement::Def(Definition {
//                    header: None,
//                    name: "func".s(),
//                    patterns: vec![Pattern::Var("a".s())],
//                    expr: Expr::OpChain(
//                        vec![Expr::Ref("a".s()), Expr::Literal(Literal::Int(1))],
//                        vec!["+".s()]
//                    ),
//                })
//            ],
//        });
//    }
//
//    #[test]
//    fn check_module_exports() {
//        let tokens = tokenize(b"module MyMod exposing ( List, Maybe )").unwrap();
//        let m = read_module(TokenStream::new(&tokens));
//        assert_ok!(m, Module {
//              header: Some(ModuleHeader{
//                    name: "MyMod".s(),
//                    exposing: ModuleExposing::Just(vec![
//                        Exposing::Type("List".s()),
//                        Exposing::Type("Maybe".s()),
//                    ]),
//              }),
//              imports: vec![],
//              statements: vec![],
//        });
//    }
//
//    #[test]
//    fn check_module_imports() {
//        let tokens = tokenize(b"import Html exposing (..)").unwrap();
//
//        let m = read_module(TokenStream::new(&tokens));
//        assert_ok!(m, Module {
//            header: None,
//            imports: vec![
//                Import::Exposing(vec!["Html".s()], ModuleExposing::All)
//            ],
//            statements: vec![],
//        });
//    }
//
//    #[test]
//    fn check_module_imports_adv() {
//        let tokens = tokenize(b"\
//import Html exposing (..)\n\
//import Util\n\
//import Util as U\n\
//import Util exposing (..)\n\
//import Util exposing (Enum, map, Sides(..), UpDown(Up, Down))\n\
//").unwrap();
//
//        let m = read_module(TokenStream::new(&tokens));
//        assert_ok!(m, Module {
//            header: None,
//            imports: vec![
//                Import::Exposing(vec!["Html".s()], ModuleExposing::All),
//                Import::Module(vec!["Util".s()]),
//                Import::Alias(vec!["Util".s()], "U".s()),
//                Import::Exposing(vec!["Util".s()], ModuleExposing::All),
//                Import::Exposing(vec!["Util".s()], ModuleExposing::Just(vec![
//                    Exposing::Type("Enum".s()),
//                    Exposing::Definition("map".s()),
//                    Exposing::Adt("Sides".s(), AdtExposing::All),
//                    Exposing::Adt("UpDown".s(), AdtExposing::Variants(
//                        vec!["Up".s(), "Down".s()]
//                    )),
//                ])),
//            ],
//            statements: vec![],
//        });
//    }
//}

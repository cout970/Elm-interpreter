use nom::*;
use parsers::statement::*;
use parsers::Tk;
use tokenizer::Token::*;
use ast::*;

// Modules
// https://github.com/durkiewicz/elm-plugin/blob/master/src/main/java/org/elmlang/intellijplugin/Elm.bnf

named!(pub upper_ids<Tk, Vec<String>>, separated_nonempty_list!(tk!(Dot), upper_id!()));

named!(pub read_ref<Tk, String>, alt!(
    id!() |
    do_parse!(
        tk!(LeftParen) >>
        op: binop!() >>
        tk!(RightParen) >>
        (op)
    )
));

named!(pub read_module<Tk, Module>, do_parse!(
    opt!(indent!(0)) >>
    header: opt!(module_header) >>
    imports: many0!(import) >>
    statements: many0!(top_level_statement) >>
    (Module { header, imports, statements })
));

named!(module_header<Tk, ModuleHeader>, do_parse!(
    tk!(ModuleTk) >>
    mod_id: upper_id!() >>
    e: exposing >>
    (ModuleHeader { name: mod_id, exposing: e })
));

named!(exposing<Tk, ModuleExposing>, do_parse!(
    tk!(ExposingTk) >>
    tk!(LeftParen) >>
    e: exposing_int >>
    tk!(RightParen) >>
    (e)
));

named!(exposing_int<Tk, ModuleExposing>, alt!(exposing_all | exposing_list));

named!(exposing_all<Tk, ModuleExposing>, do_parse!(
    tk!(DoubleDot) >>
    (ModuleExposing::All)
));

named!(exposing_list<Tk, ModuleExposing>, do_parse!(
    e: separated_nonempty_list!(tk!(Comma), exposing_item) >>
    (ModuleExposing::Just(e))
));

named!(exposing_item<Tk, Exposing>, alt!(
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

named!(adt_exposing<Tk, AdtExposing>, alt!(
    do_parse!(
        tk!(LeftParen) >>
        tk!(DoubleDot) >>
        tk!(RightParen) >>
        (AdtExposing::All)
    )
    | do_parse!(
        tk!(LeftParen) >>
        b: separated_list!(tk!(Comma), upper_id!()) >>
        tk!(RightParen) >>
        (AdtExposing::Variants(b))
    )
));

named!(import<Tk, Import>, alt!(
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

named!(import_pre<Tk, Vec<String>>, do_parse!(
    opt!(indent!(0)) >>
    tk!(ImportTk) >>
    path: upper_ids >>
    (path)
));

#[cfg(test)]
mod tests {
    use super::*;
    use tokenizer::tokenize;
    use util::StringConversion;

    #[test]
    fn check_empty_module() {
        let stream = tokenize(b"module My_module exposing (..)").unwrap();
        let m = read_module(&stream);
        assert_ok!(m, Module {
              header: Some(ModuleHeader{
                    name: "My_module".s(),
                    exposing: ModuleExposing::All,
              }),
              imports: vec![],
              statements: vec![],
        });
    }

    #[test]
    fn check_only_defs() {
        let stream = tokenize(b"func a = a + 1").unwrap();
        let m = read_module(&stream);
        assert_ok!(m, Module {
            header: None,
            imports: vec![],
            statements: vec![
                Statement::Def(Definition {
                    header: None,
                    name: "func".s(),
                    patterns: vec![Pattern::Var("a".s())],
                    expr: Expr::OpChain(
                        vec![Expr::Ref("a".s()), Expr::Literal(Literal::Int(1))],
                        vec!["+".s()]
                    ),
                })
            ],
        });
    }

    #[test]
    fn check_module_exports() {
        let stream = tokenize(b"module MyMod exposing ( List, Maybe )").unwrap();
        let m = read_module(&stream);
        assert_ok!(m, Module {
              header: Some(ModuleHeader{
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
        let stream = tokenize(b"import Html exposing (..)").unwrap();

        let m = read_module(&stream);
        assert_ok!(m, Module {
            header: None,
            imports: vec![
                Import::Exposing(vec!["Html".s()], ModuleExposing::All)
            ],
            statements: vec![],
        });
    }

    #[test]
    fn check_module_imports_adv() {
        let stream = tokenize(b"\
import Html exposing (..)\n\
import Util\n\
import Util as U\n\
import Util exposing (..)\n\
import Util exposing (Enum, map, Sides(..), UpDown(Up, Down))\n\
").unwrap();

        let m = read_module(&stream);
        assert_ok!(m, Module {
            header: None,
            imports: vec![
                Import::Exposing(vec!["Html".s()], ModuleExposing::All),
                Import::Module(vec!["Util".s()]),
                Import::Alias(vec!["Util".s()], "U".s()),
                Import::Exposing(vec!["Util".s()], ModuleExposing::All),
                Import::Exposing(vec!["Util".s()], ModuleExposing::Just(vec![
                    Exposing::Type("Enum".s()),
                    Exposing::Definition("map".s()),
                    Exposing::Adt("Sides".s(), AdtExposing::All),
                    Exposing::Adt("UpDown".s(), AdtExposing::Variants(
                        vec!["Up".s(), "Down".s()]
                    )),
                ])),
            ],
            statements: vec![],
        });
    }
}

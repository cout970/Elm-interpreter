use *;
use nom::*;
use parsers::statement::*;
use tokenizer::Token;
use tokenizer::Token::*;
use types::Module;
use parsers::Tk;
use util::*;

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
    opt!(tk!(LineStart)) >>
    header: opt!(module_header) >>
    imports: many0!(import) >>
    statements: many0!(top_level_statement) >>
    (Module { header, imports, statements })
));

named!(adt_export<Tk, String>, alt!(id!() | map!(tk!(DoubleDot), |_c| "..".s())) );

named!(export<Tk, Export>, alt!(
    map!(read_ref, |c| Export::AdtRef(c)) |
    do_parse!(
        id: upper_id!() >>
        tk!(LeftParen) >>
        t: separated_nonempty_list!(tk!(Comma), adt_export) >>
        tk!(RightParen) >>
        (Export::Adt(id, t))
    ) |
    do_parse!(
        id: upper_id!() >>
        tk!(LeftParen) >>
        tk!(DoubleDot) >>
        tk!(RightParen) >>
        (Export::AdtAll(id))
    ) |
    map!(upper_id!(), |c| Export::AdtNone(c))
));

named!(exports<Tk, Vec<Export>>, map!(opt!(do_parse!(
    tk!(LeftParen) >>
    l: separated_list!(tk!(Comma), export) >>
    tk!(RightParen) >>
    (l)

)), |o| o.unwrap_or(Vec::new())));

named!(module_header<Tk, ModuleHeader>, do_parse!(
    tk!(ModuleTk) >>
    mod_id: upper_ids >>
    e: exports >>
    tk!(Where) >>
    (ModuleHeader { name: mod_id, exports: e })
));

named!(exposing<Tk, Vec<Export>>, do_parse!(
    tk!(Exposing) >>
    tk!(LeftParen) >>
    e: separated_list!(tk!(Comma), export) >>
    tk!(RightParen) >>
    (e)
));

named!(import<Tk, Import>, do_parse!(
    opt!(tk!(LineStart)) >>
    tk!(ImportTk) >>
    path: upper_ids >>
    alias: opt!(do_parse!(tk!(As) >> n: upper_id!() >> (n))) >>
    exp: opt!(exposing)  >>
    (Import{ path, alias, exposing: exp.unwrap_or(Vec::new()) })
));

#[cfg(test)]
mod tests {
    use nom::*;
    use super::*;
    use tokenizer::tokenize;

    #[test]
    fn check_empty_module() {
        let stream = tokenize(b"module My_module");
        let m = read_module(&stream);
        assert_ok!(m, Module::default());
    }

    #[test]
    fn check_module_name() {
        let stream = tokenize(b"module Com.My_module.My_sub_module");
        let m = read_module(&stream);
        assert_ok!(m, Module::default());
    }

    #[test]
    fn check_module_exports() {
        let stream = tokenize(b"module MyMod ( List, Maybe )");
        let m = read_module(&stream);
        assert_ok!(m, Module::default());
    }

    #[test]
    fn check_module_empty_exports() {
        let stream = tokenize(b"module MyMod ( )");
        let m = read_module(&stream);
        assert_ok!(m, Module::default());
    }

    #[test]
    fn check_module_imports() {
        let stream = tokenize(b"import Html exposing (Html, text)");
        let m = read_module(&stream);
        assert_ok!(m, Module {
            imports: vec![Import{
                path: vec!["Html".s()],
                alias: None,
                exposing: vec![Export::AdtNone("Html".s()), Export::AdtRef("text".s())]
            }],
            ..Module::default()
        });
    }
}

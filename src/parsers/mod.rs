use ast::{Expr, Module, Pattern, Statement, Type};
use errors::*;
use parsers::input::Input;
use parsers::util::complete;
use source::SourceCode;
use tokenizer::Tokenizer;

mod input;
mod util;
mod pattern;
mod expression;
mod types;
mod statement;
mod module;

pub struct Parser {
    code: SourceCode,
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Parser { code: tokenizer.source_code(), tokenizer }
    }

    /// Generates an abstract syntax tree from an elm expression
    pub fn parse_expression(&mut self) -> Result<Expr, ElmError> {
        let input = Input::new(self.code.clone(), self.tokenizer.tokenize()?);

        complete(&expression::parse_expr, input)
            .map_err(|e| ElmError::Parser(self.code.clone(), e))
    }

    /// Generates an abstract syntax tree from an elm statement
    pub fn parse_statement(&mut self) -> Result<Statement, ElmError> {
        let input = Input::new(self.code.clone(), self.tokenizer.tokenize()?);

        complete(&statement::parse_statement, input)
            .map_err(|e| ElmError::Parser(self.code.clone(), e))
    }

    /// Generates an abstract syntax tree from an elm module
    pub fn parse_module(&mut self) -> Result<Module, ElmError> {
        let input = Input::new(self.code.clone(), self.tokenizer.tokenize()?);

        complete(&module::parse_module, input)
            .map_err(|e| ElmError::Parser(self.code.clone(), e))
    }

    /// Generates an abstract syntax tree from an elm type definition
    pub fn parse_type(&mut self) -> Result<Type, ElmError> {
        let input = Input::new(self.code.clone(), self.tokenizer.tokenize()?);

        complete(&types::parse_type, input)
            .map_err(|e| ElmError::Parser(self.code.clone(), e))
    }

    /// Generates an abstract syntax tree from an elm pattern
    pub fn parse_pattern(&mut self) -> Result<Pattern, ElmError> {
        let input = Input::new(self.code.clone(), self.tokenizer.tokenize()?);

        complete(&pattern::parse_pattern, input)
            .map_err(|e| ElmError::Parser(self.code.clone(), e))
    }
}

#[cfg(test)]
mod tests {
    use test_utils::Test;

    #[test]
    fn test_bench_1() {
        Test::module(include_str!("../../resources/benches/tokenizer_1.elm"));
    }

    #[test]
    fn test_bench_2() {
        Test::module(include_str!("../../resources/benches/tokenizer_2.elm"));
    }

    #[test]
    fn test_edge_case() {
        let code = r#"sliceTree shift endIdx tree =
            let
                lastPos =
                    Bitwise.and bitMask <| Bitwise.shiftRightZfBy shift endIdx
            in
                case JsArray.unsafeGet lastPos tree of
                    SubTree sub ->
                        let
                            newSub =
                                sliceTree (shift - shiftStep) endIdx sub
                        in
                            if JsArray.length newSub == 0 then
                                -- The sub is empty, slice it away
                                JsArray.slice 0 lastPos tree
                            else
                                tree
                                    |> JsArray.slice 0 (lastPos + 1)
                                    |> JsArray.unsafeSet lastPos (SubTree newSub)

                     -- This is supposed to be the new tail. Fetched by `fetchNewTail`.
                     -- Slice up to, but not including, this point.
                    Leaf _ ->
                        JsArray.slice 0 lastPos tree"#;

        let tk = Test::tokens(code);

        for info in tk.iter() {
            println!("|> {}", info.token);
        }
        Test::module(code);
    }
}

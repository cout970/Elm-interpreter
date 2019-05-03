use analyzer::Analyzer;
use ast::{Definition, Expr};
use ast::Module;
use ast::Statement;
use parsers::Parser;
use source::SourceCode;
use tokenizer::TokenInfo;
use tokenizer::Tokenizer;
use typed_ast::TypedExpr;

#[cfg(test)]
pub struct Test;

#[cfg(test)]
impl Test {
    pub fn tokens(code: &str) -> Vec<TokenInfo> {
        Tokenizer::new(&SourceCode::from_str(code)).tokenize().unwrap()
    }

    #[cfg(test)]
    pub fn typed_expr(code: &str) -> TypedExpr {
        let src = SourceCode::from_str(code);
        let expr = Self::expr(code);
        let mut analyzer = Analyzer::new(src);
        let res = analyzer.analyze_expression(&expr);

        match res {
            Ok(res) => res,
            Err(error) => {
                println!("Error: {}\n", error);
                panic!();
            }
        }
    }

    #[cfg(test)]
    pub fn expr(code: &str) -> Expr {
        let mut parser = Parser::new(Tokenizer::new(&SourceCode::from_str(code)));
        let res = parser.parse_expression();

        match res {
            Ok(res) => res,
            Err(error) => {
                println!("Error: {}\n", error);
                panic!();
            }
        }
    }

    #[cfg(test)]
    pub fn expr_analyzer(code: &str) -> (Expr, Analyzer) {
        let src = SourceCode::from_str(code);
        let mut parser = Parser::new(Tokenizer::new(&src));
        let res = parser.parse_expression();

        match res {
            Ok(res) => (res, Analyzer::new(src)),
            Err(error) => {
                println!("Error: {}\n", error);
                panic!();
            }
        }
    }

    #[cfg(test)]
    pub fn statement(code: &str) -> Statement {
        let mut parser = Parser::new(Tokenizer::new(&SourceCode::from_str(code)));
        let res = parser.parse_statement();

        match res {
            Ok(res) => res,
            Err(error) => {
                println!("Error: {}\n", error);
                panic!();
            }
        }
    }
    #[cfg(test)]
    pub fn definition(code: &str) -> Definition {
        let mut parser = Parser::new(Tokenizer::new(&SourceCode::from_str(code)));
        let res = parser.parse_statement();

        match res {
            Ok(Statement::Def(res)) => res,
            Ok(other) => {
                println!("Statement: {:?}\n", other);
                panic!("Not a definition");
            },
            Err(error) => {
                println!("Error: {}\n", error);
                panic!();
            }
        }
    }

    #[cfg(test)]
    pub fn module(code: &str) -> Module {
        let mut parser = Parser::new(Tokenizer::new(&SourceCode::from_str(code)));
        let res = parser.parse_module();

        match res {
            Ok(res) => res,
            Err(error) => {
                println!("Error: {}\n", error);
                panic!();
            }
        }
    }
}
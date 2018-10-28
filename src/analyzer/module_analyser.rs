use types::Module;
use analyzer::TypeError;
use analyzer::static_env::StaticEnv;

fn analyze_module(module: &Module) -> Result<(), TypeError> {
    Ok(())
}



#[cfg(test)]
mod tests {
    use nom::*;
    use parsers::from_code_mod;
    use super::*;
    use types::Statement;


    #[test]
    fn check_constant() {
        let module = from_code_mod(b"module Main exposing (..)\n x = 0\n\n");


        assert_eq!(analyze_module(&module), Ok(()));
    }
}
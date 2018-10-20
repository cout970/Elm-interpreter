# Elm Interpreter
Small Elm interpreter made in Rust with the library [Nom](https://github.com/Geal/nom)

### Progress
 - [x] Tokenizer
 - [x] Parser
 - [x] Semantic Analyzer
 - [x] Tree walker interpreter
 - [ ] Bytecode generation
 - [ ] Bytecode interpreter
 
 
##### GTD
- Improve error reporting encapsulating the job of creating user-readable strings in a separated module
- Add modules and imports (loading and evaluation)
- Add better runtime exceptions
- Add tests for incorrect code
- Add code locations in tokenizer/parser/analyzer errors
- Add builtin functions and types from elm-core
- Add support for webassembly
- Reduce the use of clone on immutable values (using Rc to share the data)
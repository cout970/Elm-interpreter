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
- Fix type inference errors
- Integrate type alias and type declarations into the interpreter
- Add modules and imports
- Fix qualified references
- Add better runtime exceptions
- Improve environment struct
- Reorganize project files a bit
- Add large source code examples
- Add tests for incorrect code
- Add code locations in tokenizer/parser/analyzer errors
- Add builtin functions and types from elm-core

- Publish project on github
# Elm Interpreter
Small Elm interpreter made in Rust with the library [Nom](https://github.com/Geal/nom)

### Progress
 - Tokenizer: Complete
 - Parser: Complete
 - Semantic Analyzer: Mostly complete
 - Tree walker interpreter: Missing Let and Rust interop
 - Rust interop: Mostly done
 - FFI interop: Not started
 - Bytecode generation: Not started
 - Bytecode interpreter: Not started
 
 
##### GTD
- Improve error reporting encapsulating the job of creating user-readable strings in a separated module
- Add better runtime exceptions
- Add builtin functions and types from elm-core
- Benchmarking every step in the interpreter
- Create a language driver for Amethyst
# Elm Interpreter
Small Elm interpreter made in Rust with the library [Nom](https://github.com/Geal/nom)

### Progress
- Tokenizer: Complete
- Parser: Complete
- Semantic Analyzer: Almost complete
- Module Loader: Almost complete
- Tree walker interpreter: Missing Let and Rust interop
- Rust interop: Mostly done
- FFI interop: Not started
- Bytecode generation: Not started
- Bytecode interpreter: Not started


##### GTD
- Improve error reporting (redo module) encapsulating the job of creating user-readable strings in a separated module
- Add better runtime exceptions
- Add builtin functions and types from elm-core
- Benchmarking every step in the interpreter
- Create a language driver for Amethyst
- Make real world tests

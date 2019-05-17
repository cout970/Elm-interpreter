# Elm Interpreter
Small Elm interpreter made in Rust

### State of the project
I lost interest on this project, because its main purpose was to be used 
as an scripting engine in [Amethyst](https://amethyst.rs/), but Amethyst will 
take months before it's ready to support scripting in a useful way.

So if anyone has a reason to use this project for anything else just open an issue.

### Work needed
Right now the project is close to completion, but will has some things that need work, for example:
- Fix several TODOs in the code for edge cases
- Replace serde_json with other serde plugin that uses a more compact binary format
- Fix closures not correctly collecting values fom they environment
- Benchmarking and improving the performance of the library
- Add support for FFI
- Add let expressions in the interpreter
- Add test with large code examples, the project has a lot of tests but they only cover lexing, parsing and type resolution, ignoring execution.

#### Nice to have
- Better and more consistent error reporting for different types of errors
- Add more info when a runtime error occurs
- Benchmarking every step in the interpreter
- Add Array, Process, Task and Platform modules from elm core
- More functions for Rust interop, like casting rust values to elm values and vice versa.

### What works right now
- Lexing/Tokenizing work pretty well, except for weird unicode characters.
- Parsing seems to work ok and it complete, but I haven't tested with large codebases, just elm core.
- Static analysis and type inference work most of the time, if a function doesn't have a type definition it can only use other functions that have a type definition or that are defined before.
- Execution has issues with closures, and let expressions are not implemented, this is the less tested part of the project.
- Rust interop allow to register rust function with Runtime.register_fn(function_ptr) but only if they use simple types as arguments and return
- Rust interop also allow to register function of type `fn(&mut Interpreter, &[Value]) -> Result<Value, ElmError>` allowing to interact directly with elm values

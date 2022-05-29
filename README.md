## Stack-Based VM implementation of the lox language using Rust Programming language

rulox-lang is a stack based VM implementation (compiles to bytecode) of the lox language designed by [Bob Nystrom](https://github.com/munificent) in his book [Crafting Interpreters](http://www.craftinginterpreters.com/), using Rust programming language.

## Basic Usage

Run Ripple:

```Make
cargo run -p runner
```

Run with a specific sample file:

```Make
cargo run -p runner -- .\src\runner\samples\simple.lox
```

### Current Status

- [x] Expressions
- [x] Statements
- [x] Variables(Global&Local)
- [x] Control flow
- [ ] Functions
- [ ] Closures
- [ ] Classes
- [ ] Superclasses
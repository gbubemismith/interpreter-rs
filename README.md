# Interpreter-rs

A Rust implementation of the interpreter described in the book [Crafting Interpreters](https://craftinginterpreters.com/) by Bob Nystrom.

This project aims to follow the book's structure and philosophy, building a Lox interpreter in Rust. It is a learning project, focusing on clarity, idiomatic Rust, and testability.

## Features

- Lexical analysis (scanner) with support for single-line and block comments, string and number literals, identifiers, and keywords.
- Tokenization and error reporting.
- Modular code structure for easy extension as more chapters are implemented.
- Unit tests for core components.

## Usage

### Prerequisites

- Rust (latest stable recommended). Install from [rustup.rs](https://rustup.rs/).

### Running the Interpreter

Currently, the main focus is on the scanner (lexer). You can run the tests to see the scanner in action:

```sh
cd interpreter-rs
cargo test
```

To run the interpreter (when implemented):

```sh
cargo run -- path/to/source.smith
```

## Testing

Unit tests are included for the scanner and tokenization logic. Run all tests with:

```sh
cargo test
```

## Progress

- [x] Scanner (lexer)
- [ ] Parser
- [ ] Interpreter
- [ ] REPL
- [ ] Error handling improvements

## References

- [Crafting Interpreters](https://craftinginterpreters.com/) by Bob Nystrom

## Contributing

This project is primarily for educational purposes, but contributions and suggestions are welcome! Please open an issue or pull request.

---

Happy interpreting!

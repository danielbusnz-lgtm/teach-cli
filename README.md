# teach-cli

A command-line tool that displays random programming fundamentals topics to help with learning and review.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Get a random Python topic
teach-cli python

# Get a random Rust topic
teach-cli rust
```

## Topics Covered

**Python** (125 topics)
- Variables and types
- Control flow
- Functions and scope
- OOP (classes, inheritance, polymorphism)
- Async/await
- Exception handling
- Comprehensions and generators
- Dunder methods

**Rust** (140 topics)
- Ownership and borrowing
- Lifetimes
- Traits and generics
- Error handling
- Iterators
- Smart pointers
- Concurrency and async
- Modules and macros

## Building

```bash
cargo build --release
```

## License

MIT

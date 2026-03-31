# teach-cli

[![Rust](https://img.shields.io/badge/rust-1.91.1-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Topics](https://img.shields.io/badge/topics-265-green?style=flat-square)](#topic-coverage)
[![GitHub stars](https://img.shields.io/github/stars/danielbusnz-lgtm/teach-cli?style=flat-square)](https://github.com/danielbusnz-lgtm/teach-cli/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/danielbusnz-lgtm/teach-cli?style=flat-square)](https://github.com/danielbusnz-lgtm/teach-cli/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/danielbusnz-lgtm/teach-cli?style=flat-square)](https://github.com/danielbusnz-lgtm/teach-cli/commits/main)

A command-line learning tool that displays random programming fundamentals topics to reinforce knowledge through spaced repetition.

## Why This Exists

Developers learn new concepts constantly, but forgetting is inevitable. This tool solves the problem of passive knowledge decay by surfacing random topics on demand, creating active recall opportunities throughout your workflow. Run it before writing code, during breaks, or as a warm-up for technical interviews.

## Features

• **265 curated topics** across Python and Rust fundamentals
• **Instant random selection** for quick knowledge checks
• **Zero configuration** required
• **Offline-first** design, no network dependencies
• **Language-specific subcommands** for focused learning

## Quick Start

```bash
# Clone and install
git clone https://github.com/danielbusnz-lgtm/teach-cli.git
cd teach-cli
cargo install --path .

# Get a random Python topic
teach-cli python

# Get a random Rust topic
teach-cli rust
```

**Example output:**
```
$ teach-cli python
async with statement

$ teach-cli rust
Trait bounds (T: Trait)
```

## Topic Coverage

### Python (125 topics)

• **Type System:** Dynamic typing, primitives, collections, type hints, unions, generics
• **Control Flow:** if/elif/else, loops, match statements, break/continue
• **Functions:** Parameters, decorators, lambdas, *args/**kwargs
• **Scoping:** LEGB rule, global/nonlocal keywords
• **OOP:** Classes, inheritance, MRO, super(), polymorphism, duck typing
• **Encapsulation:** Public/protected/private, properties, descriptors
• **Advanced Types:** Dataclasses, enums, protocols, abstract base classes
• **Async/Await:** Coroutines, event loop, tasks, async context managers, async iterators
• **Error Handling:** try/except/else/finally, exception hierarchy, custom exceptions
• **Context Managers:** with statement, contextlib, ExitStack
• **Comprehensions:** List/dict/set comprehensions, generator expressions
• **Generators:** yield, yield from, lazy evaluation
• **Dunder Methods:** __str__, __repr__, __len__, __getitem__, __call__, operators
• **Iteration Protocol:** Iterable, Iterator, StopIteration, iter(), next()

### Rust (140 topics)

• **Type System:** let/mut, const/static, type inference, scalars, compounds, String/&str, Vec/HashMap
• **Ownership:** Move semantics, Copy/Clone traits, references, borrowing rules
• **Lifetimes:** Lifetime annotations, elision, 'static, dangling reference prevention
• **Control Flow:** if/else expressions, loops, match, if let/while let, labeled breaks
• **Functions:** fn syntax, closures, capture modes (Fn/FnMut/FnOnce), move closures
• **Structs:** Named/tuple/unit structs, impl blocks, methods, associated functions
• **Enums:** Variant definitions, Option/Result, pattern matching
• **Traits:** Definitions, implementations, default methods, derive macros, trait bounds
• **Advanced Traits:** Where clauses, associated types, supertraits, trait objects, object safety
• **Generics:** Generic functions/structs/enums, monomorphization, phantom data
• **Error Handling:** panic!, Result, ? operator, unwrap variants, custom errors, thiserror/anyhow
• **Iterators:** Iterator trait, map/filter/collect, fold/reduce, adapters vs consumers
• **Smart Pointers:** Box, Rc/Arc, RefCell, Mutex/RwLock, Weak, Deref trait
• **Concurrency:** Threads, JoinHandle, Send/Sync traits, channels, shared state
• **Async:** async fn/await, Future trait, tokio/async-std, select! macro
• **Modules:** mod/pub, use statements, visibility modifiers, crate structure
• **Macros:** macro_rules!, procedural/derive/attribute macros, common macros
• **Unsafe:** Raw pointers, unsafe blocks/functions/traits, FFI
• **Testing:** #[test], assertions, #[should_panic], integration tests

## Installation

### From Source

```bash
cargo install --path .
```

### Build Release Binary

```bash
cargo build --release
./target/release/teach-cli python
```

## Development

### Project Structure

```
teach-cli/
├── src/
│   ├── main.rs              # CLI entry point using clap
│   ├── lib.rs               # Library root (exports fundamentals)
│   ├── fundamentals.rs      # Module declarations
│   └── fundamentals/
│       ├── python.rs        # 125 Python topics
│       └── rust.rs          # 140 Rust topics
├── Cargo.toml
├── LICENSE
└── README.md
```

### Dependencies

• **clap 4.6.0** - Command-line argument parsing with derive macros
• **rand 0.10.0** - Random number generation for topic selection

### Build and Run

```bash
# Development build
cargo build
cargo run -- python

# Run with release optimizations
cargo run --release -- rust

# Check code quality
cargo clippy
cargo fmt
```

## Implementation Details

Topics are stored as `BTreeMap<u32, String>` for deterministic ordering and O(log n) lookup. Each subcommand generates a random ID in the valid range (1 to topic count), then retrieves the corresponding topic string.

The CLI is built with clap's derive API, mapping subcommand enum variants to the `python::rand_topic()` and `rust::rand_topic()` functions.

## Use Cases

• **Daily practice:** Add `teach-cli python` to your shell startup
• **Pre-commit ritual:** Review a topic before pushing code
• **Interview prep:** Generate random questions for self-quizzing
• **Mentoring:** Use as discussion prompts during pair programming
• **Documentation reference:** Identify gaps in your mental model

## Future Enhancements

• Add `--count` flag to display multiple topics at once
• Implement topic filtering by category (e.g., `--filter async`)
• Add quiz mode with user input validation
• Support custom topic files via TOML configuration
• Generate topic statistics (least/most reviewed)
• Add TypeScript, Go, and JavaScript topic sets

## License

MIT

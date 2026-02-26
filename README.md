# ⚙️ Rust the Forth

A robust, memory-safe **Forth-79 Standard** interpreter built from scratch in **Rust**. This project implements a stack-based execution engine, custom word definitions, and strict error handling without relying on external dependencies or unsafe code.

# 📍 Table of Contents
- [📝 Description](#-description)
  - [🧩 Key Features](#-key-features)
  - [🏗️ Design Constraints](#️-design-constraints)
- [🚀 Usage](#-usage)
  - [⌨️ Commands & Operations](#️-commands--operations)
- [🧪 Testing & Quality](#-testing--quality)
- [📁 Structure](#-structure)
- [🛠️ Technologies](#️-technologies)
- [📄 License](#-license)

# 📝 Description
Developed for **Taller de Programación I (FIUBA)**, this interpreter replicates the core mechanics of Forth. It utilizes a stack-based architecture where operations pop arguments and push results, supporting both standard arithmetic and complex conditional logic.

## 🧩 Key Features
- **Stack Manipulation:** Full support for `DUP`, `DROP`, `SWAP`, `OVER`, and `ROT`.
- **Word Definitions:** Dynamic dictionary expansion using `: <name> <body> ;`.
- **Control Flow:** Conditional execution with `IF ... ELSE ... THEN` blocks.
- **Persistence:** Automatic state preservation; the final stack is saved to `stack.fth` after execution.
- **Configurable Memory:** Custom stack size allocation via CLI arguments.

## 🏗️ Design Constraints
This project adheres to the strictest Rust development standards:
- **No Unsafe:** 100% safe Rust code.
- **No Panics:** Zero use of `.unwrap()`, `.expect()`, `panic!()`, or `exit()`.
- **Zero Dependencies:** Relies exclusively on the Rust Standard Library (`std`).
- **Standard Compliant:** Formatted with `cargo fmt` and verified with `clippy`.

# 🚀 Usage

### Build
```bash
cargo build --release
```

### Run
Execute a Forth script by providing its path:
```bash
cargo run -- path/to/script.fth
```

Optionally, define a custom stack size (in bytes):
```bash
cargo run -- path/to/script.fth stack-size=262144
```

## ⌨️ Commands & Operations
| Category   | Operations                           |
|------------|--------------------------------------|
| Arithmetic | `+`, `-`, `*`, `/`                   |
| Logic      | `=`, `<`, `>`, `AND`, `OR`, `NOT`    |
| Stack      | `DUP`, `DROP`, `SWAP`, `OVER`, `ROT` |
| I/O        | `.`, `EMIT`, `CR`, `." <message>"`   |

# 🧪 Testing & Quality
The project includes a comprehensive suite of unit and integration tests to ensure interpreter parity with the Forth-79 standard.
```bash
cargo test
```
* Clippy: No warnings allowed.
* Docs: Fully documented using cargo doc.

# 📁 Structure
```text
rust-the-forth/
├── src/
│   ├── main.rs         # Entry point & CLI parsing
│   ├── interpreter.rs  # Core execution logic
│   ├── stack.rs        # Stack implementation
│   └── dictionary.rs   # Word definitions & lookup
├── tests/              # Integration tests with .fth files
├── Enunciado.pdf       # Academic requirements
└── Cargo.toml          # Project metadata
```

## 🛠️ Technologies
* **Language**: Rust (v1.85+).
* **Environment**: Unix / Linux.
* **Standard Library**: `std::env`, `std::fs`, `std::io`

# 📄 License
This project is licensed under the MIT License - see the LICENSE file for details.

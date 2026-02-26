# Contributing to Rust the Forth

Thank you for your interest in improving this Forth interpreter! This project is a tribute to minimalist systems programming and Rust's safety guarantees. Whether you want to fix a stack bug or implement a new set of words, your help is welcome.

## How to Contribute

1. **Fork** the repository.
2. Create your branch: `git checkout -b feature/forth-extension`.
3. Make your changes and commit them: `git commit -m "Add support for DO...LOOP"`.
4. Push your branch: `git push origin feature/forth-extension`.
5. Open a **Pull Request**.

## 💡 Ideas for Contribution

- **Advanced Control Flow:** Implement loops like `DO ... LOOP` or `BEGIN ... UNTIL`.
- **Floating Point Support:** Add a separate floating-point stack and arithmetic words (e.g., `F+`, `F-`, `F*`).
- **Interactive REPL:** Create a mode where the user can type Forth commands line by line instead of just running files.
- **Extended Dictionary:** Add standard words like `PICK`, `ROLL`, or `DEPTH`.
- **Performance Benchmarking:** Create a suite to measure how many "words per second" the interpreter can process.
- **Visual Debugger:** Add a flag (`--debug`) that prints the state of the stack after every operation.



## 🛠️ Development Guidelines

- **The "Safe Rust" Rule:** We do not allow `unsafe` blocks, `.unwrap()`, `.expect()`, or `panic!()`. All errors must be propagated using the `Result` or `Option` types.
- **Memory Integrity:** If you modify the stack logic, ensure that **Stack Underflow** is checked *before* any operation to prevent crashes.
- **Code Hygiene:** Before submitting, your code must pass:
  - `cargo fmt` (Style check)
  - `cargo clippy` (Best practices)
  - `cargo test` (Logic verification)
- **Documentation:** New words or features must be documented in the README and accompanied by an integration test in the `tests/` folder.

## 🧱 Architectural Tips

- **The Dictionary:** Think of the dictionary as a mapping of names to execution tokens. Try to keep word definitions modular.
- **The Stack:** The stack is the heart of the interpreter. Keep it as lean as possible.

Thank you for helping us make **Rust the Forth** the safest interpreter out there! 🦀

# 🚀 Pull Request

## 📝 Description

Briefly describe your changes:
- What new Forth word or interpreter logic did you implement?
- Does this change how the stack is saved or how errors are handled?

### 📸 Media (Optional)
*If you added new output formatting or custom error messages, please attach a screenshot of the terminal here.*

---

## 🏗️ Type of Change
Please check the option that is relevant:
- [ ] 🐛 Bug fix (non-breaking change which fixes an issue like stack-underflow)
- [ ] ✨ New feature (new Forth-79 word or CLI argument)
- [ ] ♻️ Code refactor (improving interpreter efficiency without changing behavior)
- [ ] 🧪 Test suite update (adding new `.fth` integration tests)
- [ ] 📝 Documentation update

---

## ✅ Checklist

- [ ] My code complies with the project constraints: **No unwraps, no panics, no unsafe.**
- [ ] I have run `cargo clippy` and there are **zero warnings**.
- [ ] I have run `cargo fmt` to ensure consistent code style.
- [ ] I have tested edge cases like **Division by Zero** and **Stack Underflow**.
- [ ] All unit and integration tests (`cargo test`) are passing.
- [ ] The stack is still correctly exported to `stack.fth` after execution.

---

## 🔍 Specific Interpreter Checks
* **Memory:** Did you check that the `stack-size` argument is respected?
-   **Parsing:** Does the new logic handle multiple spaces or newlines between words?
-   **Dictionary:** If a word is redefined, does it follow the Forth-79 lookup rules?

---

Thanks for your contribution to **Rust the Forth**! ⚙️

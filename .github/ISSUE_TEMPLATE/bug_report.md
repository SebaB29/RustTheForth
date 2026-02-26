---
name: 🐛 Bug report
about: Create a report to help us improve Rust the Forth
title: '[BUG] '
labels: 'bug'
assignees: ''

---

## 📝 Description

Briefly describe the problem. Is the interpreter failing to parse a word, calculating the wrong result, or failing to save the stack state?

## 👣 How to Reproduce

Steps to reproduce the behavior:
1. Create a `.fth` file with the following code:
   ```forth
   [Insert the Forth code that causes the issue]
   ```
2. Run the command: `cargo run -- path/to/file.fth`
3. Perform the action: '...' (e.g., using a custom stack size)
4. See error: '...' (e.g., the output of `.` is incorrect, or a `division-by-zero` error is not caught)

## 🎯 Expected Behavior

A clear and concise description of what you expected to happen according to the Forth-79 standard or the project requirements.

## 📸 Screenshots (if applicable)

If the bug relates to terminal output or error formatting, please paste a screenshot or the raw text here.

## 💻 Environment

- **OS:** (e.g. Ubuntu 22.04, Fedora, macOS)
- **Rust Version:** (e.g. 1.85.0)
- **Terminal:** (e.g. Bash, Zsh, PowerShell)
- **Other relevant data:** (e.g., the specific `stack-size` value used or if you are running it in a container)

## 🔍 Additional Context

Add any other context about the problem here. Although the project avoids `panic!`, if you received an error message from the interpreter or a logic failure, please paste it here:
```text
[Paste your error log here]
```

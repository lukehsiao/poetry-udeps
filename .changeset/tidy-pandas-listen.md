---
"pyproject-udeps": patch
---

**fix**: upgrade `ruff_python_ast` and `ruff_python_parser` to 0.0.11, which raises the minimum supported Rust version to 1.96. Import extraction is unchanged: the 0.0.11 parser recovers from syntax errors the same way, so imports in the parseable parts of a broken file are still found.

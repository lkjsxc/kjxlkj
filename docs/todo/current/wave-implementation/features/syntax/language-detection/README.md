# Features: Syntax Language Detection (Iteration 36)

Back: [/docs/todo/current/wave-implementation/features/syntax/README.md](/docs/todo/current/wave-implementation/features/syntax/README.md)

## Scope

Implement deterministic language detection so syntax highlighting activates for common filetypes (including `.c`).

## Defining documents (direct, normative)

- Built-in languages and extension mapping:
  - [/docs/spec/features/syntax/syntax-files.md](/docs/spec/features/syntax/syntax-files.md)
- Syntax engine contract:
  - [/docs/spec/features/syntax/syntax.md](/docs/spec/features/syntax/syntax.md)

## Checklist

### A. Extension mapping

- [x] Implement the minimum built-in extension mapping table.
- [x] Ensure `.c` and `.h` map to C by default; ensure common C++ extensions map to C++.
- [x] Ensure unknown filetypes fall back to `plain` without crashing or panicking.

### B. Tests (required)

- [x] Add unit tests for extension → language id mapping.
- [x] Add integration tests that open representative fixtures and assert:
  - Extensions: rs, py, js, ts, go, java, cpp, zig, lua, unknown→Plain
  - Filenames: Dockerfile, Makefile, main.rs, config.yaml
  - Paths: /home/user/project/src/main.rs, scripts/build.sh
  - LSP IDs: rust, python, typescript, markdown, plaintext
  - Display: format!("{}", LanguageId::Rust) == "rust"
  - language id selection is correct
  - highlight spans are non-empty for fully supported languages

### C. Conformance and limitations updates

- [x] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)

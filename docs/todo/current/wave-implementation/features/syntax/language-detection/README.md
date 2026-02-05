# Features: Syntax Language Detection (Iteration 35)

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

- [ ] Implement the minimum built-in extension mapping table.
- [ ] Ensure `.c` and `.h` map to C by default; ensure common C++ extensions map to C++.
- [ ] Ensure unknown filetypes fall back to `plain` without crashing or panicking.

### B. Tests (required)

- [ ] Add unit tests for extension → language id mapping.
- [ ] Add integration tests that open representative fixtures and assert:
  - language id selection is correct
  - highlight spans are non-empty for fully supported languages

### C. Conformance and limitations updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)

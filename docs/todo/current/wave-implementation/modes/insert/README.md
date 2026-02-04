# Modes: Insert (Iteration 33)

Back: [/docs/todo/current/wave-implementation/modes/README.md](/docs/todo/current/wave-implementation/modes/README.md)

## Scope

Implement Insert mode editing behavior and Insert-mode subfeatures.

## Defining documents (direct, normative)

- Insert mode index:
  - [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)

## Coverage traversal

- Insert subtree:
  - [/docs/todo/doc-coverage/spec/modes/insert/README.md](/docs/todo/doc-coverage/spec/modes/insert/README.md)

## Checklist

### A. Placeholder scaffolding

- [ ] Define Insert-mode input handling and text insertion rules.
- [ ] Define integration points for completion and autopairs if specified.

### B. Minimal conformance slice

- [ ] Implement minimal insertion, deletion, and navigation behavior with deterministic tests.
  - Character insertion
  - Backspace (delete char before)
  - Enter (insert newline)
  - Arrow key navigation

### C. Full conformance

- [ ] Implement all Insert-mode features in the subtree (autopairs, completion, mappings, indentation, etc.).
  - [ ] Ctrl-w (delete word before)
  - [ ] Ctrl-u (delete to line start)
  - [ ] Ctrl-r {reg} (insert register contents)
  - [ ] Ctrl-o (execute one normal command) - future
  - [ ] Completion integration - future
  - [ ] Autopairs - future

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)


# Modes: Insert (Iteration 34)

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

- [x] Define Insert-mode input handling and text insertion rules.
- [x] Define integration points for completion and autopairs (only if/when specified as implemented).

### B. Minimal conformance slice

- [x] Ensure minimal insertion/deletion/navigation works in deterministic (headless) tests.
  - character insertion
  - backspace (delete char before)
  - newline insertion in core state
  - arrow-key navigation in core state
- [x] Ensure interactive TUI input mapping reliably delivers `Enter` as newline insertion.
  - [newline/README.md](newline/README.md)

### C. Full conformance

- [ ] Implement additional Insert-mode features from the subtree, gated by tests and conformance updates.
  - [x] Ctrl-w (delete word before)
  - [x] Ctrl-u (delete to line start)
  - [x] Ctrl-r {reg} (insert register contents)
  - [x] Ctrl-o (execute one normal command)
  - [x] Ctrl-k (digraph insert)
  - [x] Ctrl-v (literal character insert)
  - [ ] Completion integration
  - [x] Autopairs
  - [x] Indentation behaviors (Enter indent, `Ctrl-t`/`Ctrl-d`, etc.)

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

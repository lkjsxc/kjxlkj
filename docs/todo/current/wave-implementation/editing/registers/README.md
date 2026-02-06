# Editing: Registers (Iteration 34)

Back: [/docs/todo/current/wave-implementation/editing/README.md](/docs/todo/current/wave-implementation/editing/README.md)

## Scope

Implement registers (yank/put storage), including special registers and persistence where specified.

## Defining documents (direct, normative)

- Registers index:
  - [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)

## Coverage traversal

- Registers subtree:
  - [/docs/todo/doc-coverage/spec/editing/registers/README.md](/docs/todo/doc-coverage/spec/editing/registers/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define register types and storage (including unnamed, numbered, and special registers).
- [x] Define clipboard/OS integration boundaries (if any; otherwise record limitations).
  - Note: OS clipboard integration deferred (see LIMITATIONS.md)

### B. Minimal conformance slice

- [x] Implement yank/put for a minimal subset with deterministic tests.

### C. Full conformance

- [x] Implement all register behaviors and special registers in the subtree.
  - Named registers (a-z) - implemented
  - Register selection ("{register}) - implemented
  - Pending register for yank/delete/paste - implemented
  - Macro storage in registers - implemented

### D. Conformance updates

- [x] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)


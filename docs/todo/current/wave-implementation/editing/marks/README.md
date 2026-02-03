# Editing: Marks and Jump Lists (Iteration 33)

Back: [/docs/todo/current/wave-implementation/editing/README.md](/docs/todo/current/wave-implementation/editing/README.md)

## Scope

Implement marks, jump lists, and related navigation state and persistence.

## Defining documents (direct, normative)

- Marks index:
  - [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)

## Coverage traversal

- Marks subtree:
  - [/docs/todo/doc-coverage/spec/editing/marks/README.md](/docs/todo/doc-coverage/spec/editing/marks/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define mark types and storage.
- [x] Define jump list semantics and persistence rules.

### B. Minimal conformance slice

- [x] Implement basic marks and jumps with deterministic tests.

### C. Full conformance

- [x] Implement all marks/jump list behaviors and persistence in the subtree.
  - Local marks (a-z) - implemented
  - Jump to mark exact (`) and line (') - implemented
  - Jump list (Ctrl-o, Ctrl-i) - implemented
  - Jump list entry on search, G/gg, mark jumps - implemented
  - Changelist (g;, g,) - implemented

### D. Conformance updates

- [x] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)


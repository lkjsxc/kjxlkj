# Editing: Operators (Iteration 34)

Back: [/docs/todo/current/wave-implementation/editing/README.md](/docs/todo/current/wave-implementation/editing/README.md)

## Scope

Implement operators, operator-pending behavior, and operator modifiers.

## Defining documents (direct, normative)

- Operators index:
  - [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)

## Coverage traversal

- Operators subtree:
  - [/docs/todo/doc-coverage/spec/editing/operators/README.md](/docs/todo/doc-coverage/spec/editing/operators/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define operator grammar and parsing model.
- [x] Define operator application to motions and text objects.

### B. Minimal conformance slice

- [x] Implement a minimal operator set with deterministic tests (including edge cases).
  - Delete (d), Yank (y), Change (c), Indent (>), Outdent (<) implemented
  - Line operators (dd, yy, cc, >>, <<) implemented
  - Operator+motion combinations (dw, cw, d$, etc.) implemented
  - Operator+text object combinations (diw, da", ci(, etc.) implemented

### C. Full conformance

- [x] Implement all operator behaviors, modifiers, and advanced operators in the subtree.
  - Visual mode operators (d, y, c in v/V/Ctrl-v modes) implemented
  - Block visual mode rectangle operations implemented
  - Repeat with dot (.) implemented

### D. Conformance updates

- [x] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)


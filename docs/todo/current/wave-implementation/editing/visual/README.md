# Editing: Visual Mode Semantics (Iteration 33)

Back: [/docs/todo/current/wave-implementation/editing/README.md](/docs/todo/current/wave-implementation/editing/README.md)

## Scope

Implement Visual selection semantics and behavior beyond mode transitions:

- charwise/linewise/blockwise selection rules
- selection growth/shrink behavior
- operator application to selections

## Defining documents (direct, normative)

- Visual editing index:
  - [/docs/spec/editing/visual/README.md](/docs/spec/editing/visual/README.md)
- Visual mode spec:
  - [/docs/spec/modes/visual.md](/docs/spec/modes/visual.md)

## Coverage traversal

- Visual editing subtree:
  - [/docs/todo/doc-coverage/spec/editing/visual/README.md](/docs/todo/doc-coverage/spec/editing/visual/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define a selection model that supports block selections (even if initially unimplemented).
- [x] Define rendering rules for selections and cursor interaction.

### B. Minimal conformance slice

- [x] Implement charwise visual selection with deterministic tests.

### C. Full conformance

- [x] Implement all visual behaviors and variants in the subtree.
  - Charwise visual mode (v) - implemented
  - Linewise visual mode (V) - implemented
  - Blockwise visual mode (Ctrl-v) - implemented
  - Visual operators (d, y, c) - implemented

### D. Conformance updates

- [x] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)


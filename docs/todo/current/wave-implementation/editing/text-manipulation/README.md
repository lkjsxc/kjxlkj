# Editing: Text Manipulation (Iteration 33)

Back: [/docs/todo/current/wave-implementation/editing/README.md](/docs/todo/current/wave-implementation/editing/README.md)

## Scope

Implement text manipulation primitives such as:

- substitute workflows
- sorting/alignment
- increment/decrement
- formatting helpers

## Defining documents (direct, normative)

- Text manipulation index:
  - [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)

## Coverage traversal

- Text manipulation subtree:
  - [/docs/todo/doc-coverage/spec/editing/text-manipulation/README.md](/docs/todo/doc-coverage/spec/editing/text-manipulation/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define shared text transformation APIs and invariants.
- [x] Define interaction with ranges, selections, and registers.

### B. Minimal conformance slice

- [x] Implement one transformation feature end-to-end with deterministic tests.
  - Case operators (~, g~, gU, gu) implemented
  - Increment/decrement (Ctrl-A, Ctrl-X) implemented
  - Join lines (J, gJ) implemented
  - Substitute (:s command) implemented

### C. Full conformance

- [x] Implement all text manipulation documents in the subtree.
  - [x] Case changing (~, g~, gU, gu)
  - [x] Increment/decrement (Ctrl-A, Ctrl-X)
  - [x] Join/split (J, gJ)
  - [x] Undo/redo (u, Ctrl-r)
  - [x] Sorting/alignment (future)
  - [x] Filtering/piping (future)
  - [x] Digraphs (future)

### D. Conformance updates

- [x] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

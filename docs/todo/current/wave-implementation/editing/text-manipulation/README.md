# Editing: Text Manipulation (Iteration 36)

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

- [ ] Define shared text transformation APIs and invariants.
- [ ] Define interaction with ranges, selections, and registers.

### B. Minimal conformance slice

- [ ] Implement one transformation feature end-to-end with deterministic tests.
  - Case operators (~, g~, gU, gu) implemented
  - Increment/decrement (Ctrl-A, Ctrl-X) implemented
  - Join lines (J, gJ) implemented
  - Substitute (:s command) implemented

### C. Full conformance

- [ ] Implement all text manipulation documents in the subtree.
  - [ ] Case changing (~, g~, gU, gu)
  - [ ] Increment/decrement (Ctrl-A, Ctrl-X)
  - [ ] Join/split (J, gJ)
  - [ ] Undo/redo (u, Ctrl-r)
  - [ ] Sorting/alignment
  - [ ] Filtering/piping
  - [ ] Digraphs

If any sub-item is intentionally deferred, apply the deferral protocol from [/docs/todo/current/README.md](/docs/todo/current/README.md) and avoid leaving a terminal “future” section with unchecked boxes.

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

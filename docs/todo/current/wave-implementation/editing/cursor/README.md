# Editing: Cursor (Iteration 34)

Back: [/docs/todo/current/wave-implementation/editing/README.md](/docs/todo/current/wave-implementation/editing/README.md)

## Scope

Implement cursor semantics, including cursor shape/visibility rules where specified.

## Defining documents (direct, normative)

- Cursor spec:
  - [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Coverage traversal

- Cursor subtree:
  - [/docs/todo/doc-coverage/spec/editing/cursor/README.md](/docs/todo/doc-coverage/spec/editing/cursor/README.md)

## Checklist

### A. Placeholder scaffolding

- [ ] Define cursor representation in core state and snapshots.
- [ ] Define clamp rules for cursor after edits and mode transitions.

### B. Minimal conformance slice

- [ ] Implement deterministic cursor motion and visibility rules for a minimal mode set.
- [ ] Add regression tests for cursor invisibility and boundary cases.

### C. Full conformance

- [ ] Implement the full cursor subtree behavior, including interactions with overlays and highlights.

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)


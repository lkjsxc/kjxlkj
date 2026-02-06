# Editing: Macros (Iteration 34)

Back: [/docs/todo/current/wave-implementation/editing/README.md](/docs/todo/current/wave-implementation/editing/README.md)

## Scope

Implement macro recording and playback, including recursion and register integration.

## Defining documents (direct, normative)

- Macros index:
  - [/docs/spec/editing/macros/README.md](/docs/spec/editing/macros/README.md)

## Coverage traversal

- Macros subtree:
  - [/docs/todo/doc-coverage/spec/editing/macros/README.md](/docs/todo/doc-coverage/spec/editing/macros/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define macro representation and playback model.
- [x] Define recursion handling and safety limits.

### B. Minimal conformance slice

- [ ] Implement recording/playback for a minimal set with deterministic tests.

### C. Full conformance

- [ ] Implement all macro features in the subtree (advanced + recursive).
  - q{register} to start/stop recording - implemented
  - @{register} to playback - implemented
  - @@ to repeat last macro - implemented
  - Recursion prevention during playback - implemented

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)


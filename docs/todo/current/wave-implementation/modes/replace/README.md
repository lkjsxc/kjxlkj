# Modes: Replace (Iteration 33)

Back: [/docs/todo/current/wave-implementation/modes/README.md](/docs/todo/current/wave-implementation/modes/README.md)

## Scope

Implement Replace and Virtual Replace mode semantics.

## Defining documents (direct, normative)

- Replace mode index:
  - [/docs/spec/modes/replace/README.md](/docs/spec/modes/replace/README.md)

## Coverage traversal

- Replace subtree:
  - [/docs/todo/doc-coverage/spec/modes/replace/README.md](/docs/todo/doc-coverage/spec/modes/replace/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define overwrite vs insert behavior and how it interacts with virtual spaces.

### B. Minimal conformance slice

- [x] Implement Replace mode core behavior with deterministic tests.
  - R enters Replace mode
  - Typing overwrites existing characters
  - At end of line, characters are inserted
  - Backspace moves cursor left
  - Escape returns to Normal mode

### C. Full conformance

- [ ] Implement all Replace/Virtual Replace behaviors in the subtree.
  - [x] Basic replace mode
  - [ ] Virtual Replace mode (gR) - future
  - [x] Single character replace (r)

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)


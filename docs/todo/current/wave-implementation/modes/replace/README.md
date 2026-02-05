# Modes: Replace (Iteration 35)

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

- [ ] Define overwrite vs insert behavior and how it interacts with virtual spaces.

### B. Minimal conformance slice

- [ ] Implement Replace mode core behavior with deterministic tests.
  - R enters Replace mode
  - Typing overwrites existing characters
  - At end of line, characters are inserted
  - Backspace moves cursor left
  - Escape returns to Normal mode

### C. Full conformance

- [ ] Implement all Replace/Virtual Replace behaviors in the subtree.
  - [ ] Basic replace mode
  - [ ] Virtual Replace mode (gR)
  - [ ] Single character replace (r)

If Virtual Replace is intentionally deferred, apply the deferral protocol from [/docs/todo/current/README.md](/docs/todo/current/README.md) and avoid leaving a terminal “future” section with unchecked boxes.

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

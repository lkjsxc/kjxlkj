# Ex Commands: Syntax (Iteration 34)

Back: [/docs/todo/current/wave-implementation/commands/README.md](/docs/todo/current/wave-implementation/commands/README.md)

## Scope

Implement Ex-level syntax commands and their interaction with highlighting/syntax subsystems.

## Defining documents (direct, normative)

- Syntax commands:
  - [/docs/spec/commands/syntax.md](/docs/spec/commands/syntax.md)
- Syntax feature specs (for the underlying system):
  - [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)

## Coverage traversal

- Commands subtree:
  - [/docs/todo/doc-coverage/spec/commands/README.md](/docs/todo/doc-coverage/spec/commands/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define a syntax/highlight ownership model (who owns state, who renders).
- [x] Define how syntax commands mutate that state.

### B. Minimal conformance slice

- [x] Implement a minimal subset that is fully testable and user-visible.

### C. Full conformance

- [x] Implement all syntax command behavior exactly as specified.

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)


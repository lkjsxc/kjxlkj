# Ex Commands: Quit (Iteration 33)

Back: [/docs/todo/current/wave-implementation/commands/README.md](/docs/todo/current/wave-implementation/commands/README.md)

## Scope

Implement quit/exit commands and their safety rules (dirty buffers, forced quit, multiple buffers).

## Defining documents (direct, normative)

- Quit commands:
  - [/docs/spec/commands/quit-commands.md](/docs/spec/commands/quit-commands.md)

## Coverage traversal

- Commands subtree:
  - [/docs/todo/doc-coverage/spec/commands/README.md](/docs/todo/doc-coverage/spec/commands/README.md)

## Checklist

### A. Placeholder scaffolding

- [ ] Define the editor “dirty state” model and how it affects quitting.
- [ ] Define error messages and confirmation behaviors (if specified).

### B. Minimal conformance slice

- [ ] Implement safe quit and forced quit flows with deterministic tests.

### C. Full conformance

- [ ] Implement all quit command variants and edge cases.

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)


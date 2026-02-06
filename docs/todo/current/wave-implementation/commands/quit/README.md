# Ex Commands: Quit (Iteration 34)

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

- [x] Define the editor "dirty state" model and how it affects quitting.
- [x] Define error messages and confirmation behaviors (if specified).

### B. Minimal conformance slice

- [x] Implement safe quit and forced quit flows with deterministic tests.

### C. Full conformance

- [x] Implement all quit command variants and edge cases.

### D. Conformance updates

- [x] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)


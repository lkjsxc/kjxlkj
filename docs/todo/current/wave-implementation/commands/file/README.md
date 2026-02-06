# Ex Commands: File (Iteration 34)

Back: [/docs/todo/current/wave-implementation/commands/README.md](/docs/todo/current/wave-implementation/commands/README.md)

## Scope

Implement file-related Ex commands and path expansion behaviors.

## Defining documents (direct, normative)

- File commands index:
  - [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

## Coverage traversal

- File subtree:
  - [/docs/todo/doc-coverage/spec/commands/file/README.md](/docs/todo/doc-coverage/spec/commands/file/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define path expansion rules and their security constraints.
- [x] Define filesystem service interfaces required by file commands.

### B. Minimal conformance slice

- [x] Implement minimal `:edit` and `:write` behaviors with deterministic tests.
- [x] Implement safe error handling for invalid paths and permissions.

### C. Full conformance

- [x] Implement all file commands and behaviors in the subtree.
- [ ] Ensure behavior matches session/swap/undo persistence specs where applicable.

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)


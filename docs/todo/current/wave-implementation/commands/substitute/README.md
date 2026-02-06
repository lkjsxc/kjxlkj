# Ex Commands: Substitute and Global (Iteration 34)

Back: [/docs/todo/current/wave-implementation/commands/README.md](/docs/todo/current/wave-implementation/commands/README.md)

## Scope

Implement substitution and global command families, including flags and special cases.

## Defining documents (direct, normative)

- Substitute index:
  - [/docs/spec/commands/substitute/README.md](/docs/spec/commands/substitute/README.md)

## Coverage traversal

- Substitute subtree:
  - [/docs/todo/doc-coverage/spec/commands/substitute/README.md](/docs/todo/doc-coverage/spec/commands/substitute/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define the substitution expression model and parsing rules.
- [x] Define interaction with regex engine and ranges.

### B. Minimal conformance slice

- [x] Implement a minimal, fully tested `:substitute` subset that matches the spec.

### C. Full conformance

- [x] Implement all substitution/global/vglobal behaviors, flags, and specials.
  - :s/pattern/replacement/ - implemented
  - :s/pattern/replacement/g flag - implemented
  - :g/pattern/command - implemented
  - :v/pattern/command (inverted global) - implemented

### D. Conformance updates

- [x] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)


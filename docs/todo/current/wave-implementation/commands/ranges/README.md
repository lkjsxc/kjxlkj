# Ex Commands: Ranges and Addresses (Iteration 34)

Back: [/docs/todo/current/wave-implementation/commands/README.md](/docs/todo/current/wave-implementation/commands/README.md)

## Scope

Implement the range and address model used by Ex commands.

## Defining documents (direct, normative)

- Ranges index:
  - [/docs/spec/commands/ranges/README.md](/docs/spec/commands/ranges/README.md)
- Range/address behavior:
  - [/docs/spec/commands/ranges/ranges.md](/docs/spec/commands/ranges/ranges.md)
  - [/docs/spec/commands/ranges/range-specs.md](/docs/spec/commands/ranges/range-specs.md)
  - [/docs/spec/commands/ranges/address-patterns.md](/docs/spec/commands/ranges/address-patterns.md)

## Coverage traversal

- Ranges subtree:
  - [/docs/todo/doc-coverage/spec/commands/ranges/README.md](/docs/todo/doc-coverage/spec/commands/ranges/README.md)

## Checklist

### A. Placeholder scaffolding

- [ ] Define the address parser and representation.
- [ ] Define how ranges resolve against:
  - current buffer
  - marks (if specified)
  - search patterns (if specified)

### B. Minimal conformance slice

- [ ] Implement a minimal range set sufficient for:
  - `:write` (when applicable)
  - `:substitute` (when applicable)
- [ ] Add tests for common and boundary cases.

### C. Full conformance

- [ ] Implement all range/address behaviors and edge cases defined by the subtree.

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)


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

- [x] Define the address parser and representation.
- [x] Define how ranges resolve against:
  - current buffer
  - marks (if specified)
  - search patterns (if specified)

### B. Minimal conformance slice

- [x] Implement a minimal range set sufficient for:
  - `:write` (when applicable)
  - `:substitute` (when applicable)
- [x] Add tests for common and boundary cases.

### C. Full conformance

- [x] Implement all range/address behaviors and edge cases defined by the subtree. — done: `range_address.rs` with Address (CurrentLine/LastLine/LineNumber/Mark/ForwardSearch/BackwardSearch/Offset), Range (None/Single/FromTo/Entire), parse_range(), resolve_range()

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)


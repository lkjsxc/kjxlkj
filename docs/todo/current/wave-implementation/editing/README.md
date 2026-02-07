# Implementation: Editing (Iteration 34)

Back: [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)

## Scope

Implement the editing primitives that power modal behavior:

- cursor semantics and movement
- motions, operators, and text objects
- registers, macros, marks
- search and regex behavior
- visual selections and text manipulation

## Entry points (recursive)

| Subarea | Checklist |
|---|---|
| Cursor model | [cursor/README.md](cursor/README.md) |
| Motions | [motions/README.md](motions/README.md) |
| Operators | [operators/README.md](operators/README.md) |
| Text objects | [text-objects/README.md](text-objects/README.md) |
| Registers | [registers/README.md](registers/README.md) |
| Macros | [macros/README.md](macros/README.md) |
| Marks and jump lists | [marks/README.md](marks/README.md) |
| Search | [search/README.md](search/README.md) |
| Regex | [regex/README.md](regex/README.md) |
| Visual selections | [visual/README.md](visual/README.md) |
| Text manipulation | [text-manipulation/README.md](text-manipulation/README.md) |

## Read first (direct, normative)

- Editing index:
  - [/docs/spec/editing/README.md](/docs/spec/editing/README.md)
- Cursor and viewport-sensitive behavior:
  - [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
  - [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)
  - [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
  - [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)

## Coverage traversal

- Editing subtree:
  - [/docs/todo/doc-coverage/spec/editing/README.md](/docs/todo/doc-coverage/spec/editing/README.md)

## Placeholder scaffolding (sub-wave)

- [ ] Define a motion/operator engine with explicit:
  - grammar and parsing model
  - count application rules
  - inclusive/exclusive motion types
  - linewise/charwise/blockwise edits
- [ ] Define register types, storage, and persistence expectations.
- [ ] Define a mark model and its persistence strategy.

## Minimal conformance slice (sub-wave)

- [ ] Implement a minimal set of motions/operators/text objects that is:
  - fully specified by docs
  - test-backed
  - consistent across modes
- [ ] Implement undo/redo semantics for the minimal slice.

## Full conformance (sub-wave)

- [ ] Implement the full editing spec subtree, including:
  - search and regex features (/, ?, n, N, :s) - implemented
  - macros (including recursion rules) - fully implemented (q, @, @@)
  - marks and jump lists (m, `, ', Ctrl-o, Ctrl-i) - fully implemented
  - advanced operators and modifiers (visual mode d/y/c) - implemented
  - block visual mode (Ctrl-v) - implemented
  - global command (:g, :v) - implemented

## Tests (normative outputs)

- [ ] Add tests for:
  - boundary clamping (never panic)
  - repeatability and determinism
  - register correctness across operations
  - undo/redo invariants

## Conformance and limitations (required updates)

- [ ] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)

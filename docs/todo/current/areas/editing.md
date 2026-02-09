# TODO: Editing

Back: [/docs/todo/current/areas/README.md](/docs/todo/current/areas/README.md)

## Normative Sources

- [/docs/spec/editing/README.md](/docs/spec/editing/README.md)
- [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)
- [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
- [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
- [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)
- [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)
- [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)

## Inventory

- [x] Extract all editing requirements into requirement matrix.

## Implementation

- [x] Implement grapheme-correct cursor semantics, including CJK edge cases.
- [x] Implement motion/operator/text-object grammar and execution.
- [x] Implement registers, marks, macros, and search semantics.
- [x] Implement regex and text-manipulation behaviors from spec.

## Verification

- [x] Add/refresh deterministic boundary tests for CJK, wrap, marks, and operator grammar.
- [x] Record evidence in conformance and limitations ledgers.

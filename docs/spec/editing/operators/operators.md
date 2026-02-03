# Operators
Operators transform text over a range defined by a motion or text object.

## Requirements
- Operators are deterministic, core-owned operations.
- Operator + motion MUST resolve to an explicit range before mutation.
- Composite actions commit as a single transaction (one undo unit).
- Multi-cursor applies the same operator deterministically across cursors.

## Core operators (normative)

- `d` delete
- `y` yank
- `c` change
- `>` indent
- `<` outdent
- `=` format/indent
- `gu` lowercase
- `gU` uppercase
- `g~` toggle case
- `!` external filter (core-owned process invocation via terminal/service boundary)

## Special forms

- Character operators: `x`, `X`, `r{c}`, `s`, `S`
- Join: `J`, `gJ`

## Related

- Text objects: [docs/spec/editing/text-objects/text_objects.md](/docs/spec/editing/text-objects/text_objects.md)
- Registers: [docs/spec/editing/registers/registers.md](/docs/spec/editing/registers/registers.md)
- Undo: [docs/spec/editing/text-manipulation/undo.md](/docs/spec/editing/text-manipulation/undo.md)

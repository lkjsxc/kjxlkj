# Count with Operators

Back: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)

How counts interact with operators and motions.

## Overview

Counts can be placed before the operator, before the motion, or both. They multiply together.

## Count Positions

| Syntax | Meaning |
|---|---|
| `{count}d{motion}` | Repeat operator count times |
| `d{count}{motion}` | Apply motion count times |
| `{c1}d{c2}{motion}` | Multiply: c1 × c2 |

## Examples

| Command | Effective | Effect |
|---|---|---|
| `2dw` | Delete 2 words | Count before operator |
| `d2w` | Delete 2 words | Count before motion |
| `3d2w` | Delete 6 words | 3 × 2 |
| `5dd` | Delete 5 lines | Double operator with count |

## Count with Text Objects

`2diw` — not supported for most text objects (cannot go N levels out). Count is only meaningful for bracket-type text objects where it selects the Nth enclosing pair.

## Count with Line-wise Doubles

`3dd` — delete 3 lines starting from current line.

## Related

- Operators: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- Operator grammar: [/docs/spec/editing/operators/operator-grammar.md](/docs/spec/editing/operators/operator-grammar.md)

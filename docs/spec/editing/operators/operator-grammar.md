# Operator Grammar

Back: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)

The compositional grammar for combining operators, counts, motions, and text objects.

## Overview

The operator grammar is: `[count] operator [count] motion/text-object`. This compositional system allows a small number of operators and motions to produce a large number of editing commands.

## Grammar

| Component | Description |
|---|---|
| `[count]` | Optional repeat count |
| `operator` | The action to perform (d, c, y, etc.) |
| `motion` | Defines the range by movement |
| `text-object` | Defines the range by structure |

## Count Multiplication

When count appears before both operator and motion, they multiply:

`2d3w` = delete 6 words (2 × 3).

## Line-wise vs Character-wise

| Motion Type | Operator Behavior |
|---|---|
| Character-wise (`w`, `e`, `f`) | Operates on exact character range |
| Line-wise (`j`, `k`, `G`) | Operates on full lines |
| Block-wise (visual block) | Operates on rectangular block |

## Inclusive vs Exclusive

| Motion | Type | Operator includes last char? |
|---|---|---|
| `w` | Exclusive | No |
| `e` | Inclusive | Yes |
| `f{x}` | Inclusive | Yes |
| `t{x}` | Exclusive | No |

## Double Operator

Doubling an operator makes it line-wise, operating on the current line:

| Command | Equivalent |
|---|---|
| `dd` | Delete current line |
| `cc` | Change current line |
| `yy` | Yank current line |
| `>>` | Indent current line |
| `<<` | Unindent current line |

## Examples

| Command | Decomposition | Effect |
|---|---|---|
| `d2w` | d (delete) + 2w (2 words) | Delete 2 words |
| `ciw` | c (change) + iw (inner word) | Change inner word |
| `gUap` | gU (uppercase) + ap (a paragraph) | Uppercase paragraph |
| `>i{` | > (indent) + i{ (inner braces) | Indent inside braces |

## Related

- Operators: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- Operator-pending: [/docs/spec/editing/operators/operator-pending.md](/docs/spec/editing/operators/operator-pending.md)
- Double operators: [/docs/spec/editing/operators/double-operators.md](/docs/spec/editing/operators/double-operators.md)

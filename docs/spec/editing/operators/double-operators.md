# Double Operators

Back: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)

Operators applied to the current line by doubling the operator key.

## Overview

Typing an operator key twice applies the operator to the entire current line(s). With a count, it applies to N lines starting from the current line.

## Double Operator Commands

| Command | Effect |
|---|---|
| `dd` | Delete current line |
| `cc` | Change current line (delete + enter insert) |
| `yy` | Yank current line |
| `>>` | Indent current line |
| `<<` | Unindent current line |
| `==` | Re-indent current line |
| `gUU` | Uppercase current line |
| `guu` | Lowercase current line |
| `g~~` | Toggle case of current line |
| `gqq` | Format current line |

## With Count

`3dd` — delete 3 lines starting from the current line.

`5>>` — indent 5 lines.

## Behavior

The double-operator form is always line-wise. The entire line(s) including the newline character are affected.

## Register

`"a3dd` — delete 3 lines into register `a`.

## Undo

Each double-operator command is a single undo unit.

## Equivalence

| Double | Equivalent |
|---|---|
| `dd` | `d_` (delete to first non-blank, line-wise) |
| `yy` | `y_` or `Y` |
| `cc` | `c_` or `S` |

## Related

- Operators: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- Operator grammar: [/docs/spec/editing/operators/operator-grammar.md](/docs/spec/editing/operators/operator-grammar.md)
- Operator-pending: [/docs/spec/editing/operators/operator-pending.md](/docs/spec/editing/operators/operator-pending.md)

# Operator-Pending Mode

Back: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)

The transient mode entered after pressing an operator key, waiting for a motion or text object.

## Overview

After pressing an operator (e.g., `d`, `c`, `y`, `>`), the editor enters operator-pending mode. The next input must be a motion or text object that defines the range for the operator.

## Entry

Operator-pending mode is entered by pressing any operator key:

| Key | Operator |
|---|---|
| `d` | Delete |
| `c` | Change |
| `y` | Yank |
| `>` | Indent right |
| `<` | Indent left |
| `=` | Re-indent |
| `gU` | Uppercase |
| `gu` | Lowercase |
| `g~` | Toggle case |
| `gq` | Format |

## Motions

Any motion command completes the operator:

| Sequence | Range |
|---|---|
| `dw` | From cursor to start of next word |
| `d$` | From cursor to end of line |
| `dG` | From cursor line to end of file |
| `d/foo<CR>` | From cursor to next "foo" |

## Text Objects

Text objects provide structured ranges:

| Sequence | Range |
|---|---|
| `diw` | Inner word |
| `da(` | Around parentheses |
| `ci"` | Inner double-quoted string |
| `dit` | Inner tag |

## Exit

| Event | Result |
|---|---|
| Valid motion/text object | Operator executes, return to normal mode |
| `<Esc>` | Cancel, return to normal mode |
| Invalid key | Bell, stay in operator-pending |
| Timeout | Cancel if `timeoutlen` exceeded |

## Forced Motion Type

The motion type can be forced:

| Prefix | Forces |
|---|---|
| `v` | Character-wise |
| `V` | Line-wise |
| `<C-v>` | Block-wise |

Example: `dVj` deletes two full lines (forced line-wise).

## Related

- Operators: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- Operator grammar: [/docs/spec/editing/operators/operator-grammar.md](/docs/spec/editing/operators/operator-grammar.md)
- Motions: [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)

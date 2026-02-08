# g-Prefixed Operators

Back: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)

Operators triggered with the `g` prefix.

## Overview

Several operators require a `g` prefix to distinguish them from single-key commands.

## Operators

| Operator | Motion | Description |
|---|---|---|
| `gu` | `{motion}` | Lowercase text in range |
| `gU` | `{motion}` | Uppercase text in range |
| `g~` | `{motion}` | Toggle case of text in range |
| `gq` | `{motion}` | Format text (wrap to `textwidth`) |
| `gw` | `{motion}` | Format text, cursor stays |
| `g@` | `{motion}` | Call `operatorfunc` on range |

## Line-wise Doubles

| Command | Description |
|---|---|
| `guu` | Lowercase current line |
| `gUU` | Uppercase current line |
| `g~~` | Toggle case of current line |
| `gqq` | Format current line |

## g@ (Custom Operator)

`g@{motion}` calls the function set in `operatorfunc`. This allows defining custom operators.

## Examples

| Command | Effect |
|---|---|
| `gUiw` | Uppercase inner word |
| `guap` | Lowercase entire paragraph |
| `gqip` | Format paragraph to textwidth |
| `g~$` | Toggle case to end of line |

## Related

- Operators: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- Operator grammar: [/docs/spec/editing/operators/operator-grammar.md](/docs/spec/editing/operators/operator-grammar.md)

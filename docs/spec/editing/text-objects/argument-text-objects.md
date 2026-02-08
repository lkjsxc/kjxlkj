# Argument Text Objects

Back: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)

Text objects for function arguments and list elements.

## Overview

Argument text objects operate on comma-separated items within parentheses, brackets, or angle brackets.

## Text Objects

| Object | Description |
|---|---|
| `ia` | Inner argument (without surrounding commas/whitespace) |
| `aa` | Around argument (including trailing comma and whitespace) |

## Examples

For `func(one, two, three)` with cursor on `two`:

| Object | Selects |
|---|---|
| `ia` | `two` |
| `aa` | `, two` (or `two, ` if last argument) |

## Delimiter Awareness

Argument text objects respect nested brackets. In `func(a, vec![b, c], d)` with cursor on `b`, `ia` selects just `b` (the argument within the inner brackets).

## Operations

| Command | Effect |
|---|---|
| `cia` | Change argument |
| `daa` | Delete argument with comma |
| `yia` | Yank argument |

## Related

- Text objects: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
- Tree-sitter text objects: [/docs/spec/editing/text-objects/treesitter-text-objects.md](/docs/spec/editing/text-objects/treesitter-text-objects.md)

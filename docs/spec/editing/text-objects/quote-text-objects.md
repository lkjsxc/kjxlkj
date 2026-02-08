# Quote Text Objects

Back: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)

Text objects for quoted strings.

## Overview

Quote text objects select text within or including matching quote characters. They work with single quotes, double quotes, and backticks.

## Quote Types

| Text Object | Inner | Around | Delimiters |
|---|---|---|---|
| Single quote | `i'` | `a'` | `'...'` |
| Double quote | `i"` | `a"` | `"..."` |
| Backtick | `` i` `` | `` a` `` | `` `...` `` |

## Inner vs Around

| Object | Selected Text |
|---|---|
| `i"` | Text between the quotes (exclusive of quotes) |
| `a"` | Text including the quotes themselves |

## Search Behavior

When the cursor is:

1. **Inside quotes**: Selects the surrounding quote pair.
2. **On a quote character**: Selects the quoted span containing that quote.
3. **Outside quotes (on the line)**: Searches forward on the current line for the next quote pair.

Quote text objects do NOT cross line boundaries.

## Escape Handling

Escaped quotes (e.g., `\"` inside a double-quoted string) are not treated as quote boundaries.

| String | `ci"` selects |
|---|---|
| `"hello"` | `hello` |
| `"he\"llo"` | `he\"llo` |
| `"a""b"` | When cursor is on `a`: `a` |

## Nesting

Quotes of the same type do not nest. The first matching pair on the line is used. Different quote types do nest:

`"he said 'hello'"` — `i'` selects `hello`, `i"` selects `he said 'hello'`.

## Count

A count is NOT supported for quote text objects. `2i"` does not select the second-out pair.

## CJK Quotes

Full-width quotation marks (`「」`, `『』`, `""``) are NOT matched by `i"` / `a"`. These would need custom text objects.

## Operators

| Command | Effect |
|---|---|
| `ci"` | Change text inside double quotes |
| `da'` | Delete text including single quotes |
| `` yi` `` | Yank text inside backticks |

## Related

- Text objects: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
- Bracket text objects: [/docs/spec/editing/text-objects/bracket-text-objects.md](/docs/spec/editing/text-objects/bracket-text-objects.md)
- Inner text objects: [/docs/spec/editing/text-objects/inner-text-objects.md](/docs/spec/editing/text-objects/inner-text-objects.md)

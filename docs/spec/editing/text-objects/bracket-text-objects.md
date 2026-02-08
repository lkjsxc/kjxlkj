# Bracket Text Objects

Back: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)

Text objects for paired bracket characters.

## Overview

Bracket text objects select text within or including matched bracket pairs: `()`, `[]`, `{}`, `<>`.

## Text Objects

| Inner | Around | Delimiters |
|---|---|---|
| `i(` / `i)` / `ib` | `a(` / `a)` / `ab` | `(...)` |
| `i[` / `i]` | `a[` / `a]` | `[...]` |
| `i{` / `i}` / `iB` | `a{` / `a}` / `aB` | `{...}` |
| `i<` / `i>` | `a<` / `a>` | `<...>` |

## Inner vs Around

| Object | Includes |
|---|---|
| `i(` | Content between `(` and `)` (exclusive) |
| `a(` | Content including `(` and `)` themselves |

## Nesting

Brackets nest correctly. For `(a (b (c) d) e)` with cursor on `c`:

| Object | Selects |
|---|---|
| `i)` | `c` |
| `a)` | `(c)` |

To select at higher nesting levels, position cursor appropriately or use count.

## Multi-line

Bracket pairs can span multiple lines. `i{` in a function body selects all lines between the braces.

## Search Behavior

When the cursor is:

1. **Between matching brackets**: selects that pair.
2. **On an opening bracket**: selects the pair starting there.
3. **On a closing bracket**: selects the pair ending there.
4. **Outside any brackets**: searches forward on the line for an opening bracket.

## Count

`2i)` selects the contents of the second-level enclosing parentheses.

## Operators

| Command | Effect |
|---|---|
| `di(` | Delete inside parentheses |
| `ca{` | Change including curly braces |
| `yi[` | Yank inside square brackets |
| `da>` | Delete including angle brackets |

## Related

- Inner text objects: [/docs/spec/editing/text-objects/inner-text-objects.md](/docs/spec/editing/text-objects/inner-text-objects.md)
- Around text objects: [/docs/spec/editing/text-objects/around-text-objects.md](/docs/spec/editing/text-objects/around-text-objects.md)
- Quote text objects: [/docs/spec/editing/text-objects/quote-text-objects.md](/docs/spec/editing/text-objects/quote-text-objects.md)

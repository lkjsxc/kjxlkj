# Sorting and Alignment

Text organization utilities.

## Overview

The `:sort` command sorts lines within a range. Alignment commands format columns.

## Sort Command (normative)

`:[range]sort [flags] [/pattern/]`

Default range is `%` (all lines).

## Sort Flags (normative)

| Flag | Description |
|---|---|
| (none) | Alphabetical sort (lexicographic, case-sensitive) |
| `i` | Case-insensitive sort |
| `n` | Numeric sort (sort by first number on line) |
| `f` | Float sort (sort by first floating-point number) |
| `x` | Hexadecimal sort |
| `o` | Octal sort |
| `b` | Binary sort |
| `r` | Reverse order |
| `u` | Remove duplicate lines (unique) |

Flags can be combined: `:sort inu` sorts numerically, case-insensitive, removing duplicates.

## Pattern Sort

`:[range]sort /pattern/` sorts by the text AFTER the pattern match or by the match itself:

- `:[range]sort /pattern/` skips text matching the pattern and sorts by the remainder.
- `:[range]sort r /pattern/` sorts by the text that matches the pattern.

## External Sort

`:[range]!sort` pipes lines through the system `sort` command for Unix-compatible sorting.

## Alignment

Alignment is not a built-in Vim command but can be achieved with substitute:

- `:'<,'>s/\s*=\s*/  = /g` normalizes spacing around `=` signs.

For more structured alignment, plugins or macros are used. The editor provides `:Align {delimiter}` for aligning text on a specified character.

## Related

- Text manipulation: [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)
- Substitute: [/docs/spec/commands/substitute/README.md](/docs/spec/commands/substitute/README.md)

# Substitute Command

Back: [/docs/spec/commands/substitute/README.md](/docs/spec/commands/substitute/README.md)

Find and replace text using the `:substitute` command.

## Overview

`:substitute` (`:s`) finds occurrences of a pattern and replaces them with a string.

## Syntax

`:[range]s[ubstitute]/{pattern}/{replacement}/[flags]`

## Basic Examples

| Command | Effect |
|---|---|
| `:s/foo/bar/` | Replace first `foo` on current line with `bar` |
| `:s/foo/bar/g` | Replace all `foo` on current line |
| `:%s/foo/bar/g` | Replace all `foo` in entire buffer |
| `:'<,'>s/foo/bar/g` | Replace in visual selection |

## Flags

| Flag | Description |
|---|---|
| `g` | Replace all occurrences on the line (not just first) |
| `c` | Confirm each replacement |
| `i` | Case-insensitive (override `ignorecase`) |
| `I` | Case-sensitive |
| `n` | Count matches only (do not replace) |
| `e` | Suppress error when pattern not found |

## Confirmation

With the `c` flag, the editor prompts at each match:

| Response | Action |
|---|---|
| `y` | Replace this match |
| `n` | Skip this match |
| `a` | Replace all remaining |
| `q` | Quit substitution |
| `l` | Replace this match and quit |

## Replacement Special Characters

| Sequence | Replacement |
|---|---|
| `&` | Entire matched text |
| `\1` .. `\9` | Captured group N |
| `\r` | Newline |
| `\t` | Tab |
| `\u` | Next character uppercase |
| `\l` | Next character lowercase |
| `\U` | Following characters uppercase |
| `\L` | Following characters lowercase |
| `\e` / `\E` | End `\U` or `\L` |

## Delimiter

Any non-alphanumeric character can be used as delimiter:

`:s#foo#bar#g` — uses `#` as the delimiter.

## Repeat

| Command | Description |
|---|---|
| `:s` | Repeat last substitution (same range) |
| `&` | Repeat last `:s` on current line |
| `g&` | Repeat last `:s` on entire buffer |

## Ranges

| Range | Description |
|---|---|
| (none) | Current line |
| `%` | Entire buffer |
| `10,20` | Lines 10 to 20 |
| `'<,'>` | Visual selection |
| `.,$` | Current line to end |

## Related

- Global command: [/docs/spec/commands/substitute/global-command.md](/docs/spec/commands/substitute/global-command.md)
- Vglobal: [/docs/spec/commands/substitute/vglobal-command.md](/docs/spec/commands/substitute/vglobal-command.md)
- Regex: [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)

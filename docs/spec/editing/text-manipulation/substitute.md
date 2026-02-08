# Substitute Command

Search and replace operations.

## Overview

`:substitute` (`:s`) replaces text matching a pattern with a replacement string. It is one of the most frequently used Ex commands.

## Syntax (normative)

`:[range]s[ubstitute]/pattern/replacement/[flags]`

Default range is the current line.

## Flags (normative)

| Flag | Description |
|---|---|
| `g` | Replace all occurrences on the line (not just the first) |
| `c` | Confirm each replacement interactively |
| `i` | Case-insensitive pattern matching |
| `I` | Case-sensitive pattern matching |
| `n` | Count matches only (do not replace) |
| `e` | Suppress "pattern not found" error |
| `&` | Reuse flags from previous substitute |

## Confirmation Responses (normative)

When using the `c` flag, each match prompts:

| Key | Action |
|---|---|
| `y` | Replace this match |
| `n` | Skip this match |
| `a` | Replace all remaining matches |
| `q` | Quit substitution |
| `l` | Replace this match and quit (last) |
| `Ctrl-e` | Scroll up |
| `Ctrl-y` | Scroll down |

## Replacement Specials

| Sequence | Inserts |
|---|---|
| `&` / `\0` | Entire matched text |
| `\1`-`\9` | Captured group |
| `\r` | Newline (splits line) |
| `\t` | Tab |
| `\\` | Literal backslash |
| `\u`, `\U`, `\l`, `\L`, `\e` | Case modifiers |

## Repeat Last Substitute

| Command | Action |
|---|---|
| `:s` (no args) | Repeat last substitute on current line |
| `&` | Same as `:s` |
| `g&` | Repeat last substitute on all lines (`:%s//~/&`) |

## Related

- Substitute specials: [/docs/spec/commands/substitute/substitute-specials.md](/docs/spec/commands/substitute/substitute-specials.md)
- Substitute expressions: [/docs/spec/commands/substitute/substitute-expressions.md](/docs/spec/commands/substitute/substitute-expressions.md)
- Regex: [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)

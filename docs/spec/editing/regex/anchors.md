# Regex Anchors

Back: [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)

Position-matching atoms that do not consume text.

## Overview

Anchors assert positions in the text without matching characters. They are zero-width: they succeed or fail based on position, not content.

## Line anchors

| Anchor | Meaning |
|---|---|
| `^` | Match at the start of a line |
| `$` | Match at the end of a line |

In multiline mode (default for Vim-compatible regex), `^` matches after every newline and `$` matches before every newline.

## Word boundary anchors

| Anchor | Meaning |
|---|---|
| `\<` | Match at the start of a word (transition from non-word to word character) |
| `\>` | Match at the end of a word (transition from word to non-word character) |

Word characters are defined by `iskeyword` (default: `[a-zA-Z0-9_]`).

## Match boundary atoms

| Anchor | Meaning |
|---|---|
| `\zs` | Set the start of the match. Text before `\zs` is required but not part of the match result. |
| `\ze` | Set the end of the match. Text after `\ze` is required but not part of the match result. |

These are useful for substitutions where only part of the pattern should be replaced.

Example: `\(foo\)\zs bar\ze` matches ` bar` only when preceded by `foo`.

## File position anchors

| Anchor | Meaning |
|---|---|
| `\%^` | Match at the start of the file (before any character) |
| `\%$` | Match at the end of the file (after the last character) |

## Cursor position anchor

| Anchor | Meaning |
|---|---|
| `\%#` | Match at the current cursor position |

This is useful in patterns that should only match relative to the cursor.

## Mark position anchor

| Anchor | Meaning |
|---|---|
| `\%'m` | Match at the position of mark `m` |

## Line and column anchors

| Anchor | Meaning |
|---|---|
| `\%{n}l` | Match at line number `{n}` |
| `\%{n}c` | Match at column number `{n}` |
| `\%>{n}l` | Match after line `{n}` |
| `\%<{n}l` | Match before line `{n}` |
| `\%>{n}c` | Match after column `{n}` |
| `\%<{n}c` | Match before column `{n}` |

## Visual area anchor

| Anchor | Meaning |
|---|---|
| `\%V` | Match inside the last visual selection area |

This restricts matches to the region that was most recently selected in Visual mode.

## Related

- Regex atoms: [/docs/spec/editing/regex/regex-atoms.md](/docs/spec/editing/regex/regex-atoms.md)
- Quantifiers: [/docs/spec/editing/regex/quantifiers.md](/docs/spec/editing/regex/quantifiers.md)
- Lookaround: [/docs/spec/editing/regex/lookaround.md](/docs/spec/editing/regex/lookaround.md)

# Around Text Objects

Back: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)

Text objects that select content including enclosing delimiters or surrounding whitespace.

## Overview

Around text objects (`a{x}`) select text including enclosing characters and/or adjacent whitespace.

## Available Around Text Objects

| Text Object | Selects |
|---|---|
| `aw` | Word including surrounding whitespace |
| `aW` | WORD including surrounding whitespace |
| `as` | Sentence including surrounding whitespace |
| `ap` | Paragraph including surrounding blank lines |
| `a(` / `a)` / `ab` | Parenthesized block including `(` and `)` |
| `a[` / `a]` | Bracketed block including `[` and `]` |
| `a{` / `a}` / `aB` | Brace block including `{` and `}` |
| `a<` / `a>` | Angle bracket block including `<` and `>` |
| `a"` | Double-quoted string including quotes |
| `a'` | Single-quoted string including quotes |
| `` a` `` | Backtick string including backticks |
| `at` | Tag block including tags |

## Whitespace Handling

For `aw`, `as`, `ap`: trailing whitespace is included. If at end of line/paragraph, leading whitespace is included instead.

For delimiter types (`a(`, `a"`, etc.): the delimiters ARE included, but surrounding whitespace is NOT.

## Count

`2a)` selects the second-level enclosing parentheses pair.

## CJK Behavior

`aw` on a CJK character selects that character plus surrounding whitespace.

## Operators

| Command | Effect |
|---|---|
| `daw` | Delete word and trailing whitespace |
| `ca"` | Change quoted string including quotes |
| `yap` | Yank paragraph with surrounding blank lines |

## Related

- Inner text objects: [/docs/spec/editing/text-objects/inner-text-objects.md](/docs/spec/editing/text-objects/inner-text-objects.md)
- Text objects overview: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)

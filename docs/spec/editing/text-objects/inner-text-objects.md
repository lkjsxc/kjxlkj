# Inner Text Objects

Back: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)

Text objects that select content inside delimiters, excluding the delimiters themselves.

## Overview

Inner text objects (`i{x}`) select text between enclosing characters without including the enclosing characters.

## Available Inner Text Objects

| Text Object | Selects Inner Content Of |
|---|---|
| `iw` | Word (cursor word, no surrounding whitespace) |
| `iW` | WORD (cursor WORD, no surrounding whitespace) |
| `is` | Sentence (no surrounding whitespace) |
| `ip` | Paragraph (no surrounding blank lines) |
| `i(` / `i)` / `ib` | Parentheses |
| `i[` / `i]` | Square brackets |
| `i{` / `i}` / `iB` | Curly braces |
| `i<` / `i>` | Angle brackets |
| `i"` | Double quotes |
| `i'` | Single quotes |
| `` i` `` | Backticks |
| `it` | XML/HTML tag content |

## Word Object Difference

| Object | Text: `hello world` (cursor on `hello`) |
|---|---|
| `iw` | `hello` |
| `aw` | `hello ` (includes trailing space) |

## Delimiter Objects

For `i(`, `i[`, `i{`, `i<`, `i"`, `i'`, `` i` ``:

- Select everything between the nearest matching pair.
- Do NOT include the delimiters.
- Search outward from cursor for enclosing pair if cursor is not between them.

## Nesting

For bracket-type objects, nesting is handled correctly:

In `(a (b) c)` with cursor on `b`: `i)` selects `b` (inner parentheses). `2i)` is not supported — use `a)` to get the larger pair.

## Count

A count selects the Nth outer enclosing pair:

`d2i)` — delete inside the 2nd-level enclosing parentheses.

## Whitespace Handling

Inner objects for words, sentences, and paragraphs exclude surrounding whitespace. Inner objects for delimiters only exclude the delimiters, preserving any internal whitespace.

## CJK Behavior

`iw` operates on grapheme clusters. A CJK character is treated as a word by itself—`iw` on a CJK character selects that single character.

## Related

- Around text objects: [/docs/spec/editing/text-objects/around-text-objects.md](/docs/spec/editing/text-objects/around-text-objects.md)
- Text objects overview: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
- Bracket text objects: [/docs/spec/editing/text-objects/bracket-text-objects.md](/docs/spec/editing/text-objects/bracket-text-objects.md)

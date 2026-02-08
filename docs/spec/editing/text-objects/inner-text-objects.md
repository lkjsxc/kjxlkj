# Inner Text Objects

Select text content without delimiters.

## Overview

Inner text objects select content EXCLUDING delimiters
and surrounding whitespace. They select only the
"inside" of a construct.

## Word Objects

### Inner Word

`iw` selects the word under the cursor without
surrounding whitespace.

### Behavior

Selects a contiguous sequence of keyword characters
(letters, digits, underscore). If cursor is on
whitespace, selects the whitespace block instead.

### Example

On `the quick brown`, cursor on `quick`:
`diw` deletes `quick`, result: `the  brown`.
`ciw` changes `quick` and enters insert mode.

## WORD Objects

### Inner WORD

`iW` selects the WORD (non-whitespace sequence)
under the cursor without surrounding whitespace.

### Example

On `foo-bar baz`, cursor on `-`:
`diW` deletes `foo-bar`, result: ` baz`.

## Sentence Objects

### Inner Sentence

`is` selects the sentence content without trailing
whitespace.

## Paragraph Objects

### Inner Paragraph

`ip` selects the paragraph content without trailing
blank lines.

## Quote Objects

### Inner Double Quote

`i"` selects content between `"` delimiters,
excluding the quotes themselves.

### Inner Single Quote

`i'` selects content between `'` delimiters.

### Inner Backtick

`` i` `` selects content between backtick delimiters.

### Example

On `say "hello world" now`, cursor inside quotes:
`di"` deletes `hello world`, result: `say "" now`.
`ci"` changes `hello world` and enters insert.

## Bracket Objects

### Inner Parentheses

`i(` or `i)` or `ib` selects content inside `()`,
excluding the parentheses.

### Inner Brackets

`i[` or `i]` selects content inside `[]`.

### Inner Braces

`i{` or `i}` or `iB` selects content inside `{}`.

### Inner Angle Brackets

`i<` or `i>` selects content inside `<>`.

### Example

On `fn(a, b, c)`, cursor inside parens:
`di(` deletes `a, b, c`, result: `fn()`.
`ci)` changes inner content and enters insert.

## Tag Objects

### Inner Tag

`it` selects content between matching HTML/XML tags,
excluding the tags themselves.

### Example

On `<div>hello</div>`:
`dit` deletes `hello`, result: `<div></div>`.
`cit` changes `hello` and enters insert mode.

## Nested Delimiters

### Level Selection

`i(` selects the innermost `()` content around cursor.
`2i(` selects the next level out.

### Example

On `(a(b(c)))`, cursor on `c`:
`di(` deletes `c`, result: `(a(b()))`.
`2di(` deletes `b(c)`, result: `(a())`.

## Whitespace Handling

### No Extra Space

Unlike `a` (around) objects, `i` (inner) objects never
include adjacent whitespace. The selection is exactly
the content within the delimiters.

### Empty Content

On `()`, `di(` is a no-op (nothing to delete).
`ci(` enters insert mode between the delimiters.

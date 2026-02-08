# Around Text Objects

Selecting text with delimiters.

## Overview

Around text objects select content INCLUDING delimiters
or surrounding whitespace. Used with operators like
`d`, `c`, `y`.

## Word Objects

### Around Word

`aw` selects the word under the cursor plus adjacent
whitespace. With `d`, deletes the word and normalizes
spacing.

### Behavior

If the word has trailing whitespace, that whitespace is
included. If the word is at end of line (no trailing
space), leading whitespace is included instead.

### Example

On `the quick brown`, cursor on `quick`:
`daw` removes `quick ` leaving `the brown`.

## WORD Objects

### Around WORD

`aW` selects the WORD (whitespace-delimited) plus
adjacent whitespace. Same spacing rules as `aw` but
using WORD boundaries (non-whitespace sequences).

### Example

On `foo-bar baz`, cursor on `foo-bar`:
`daW` removes `foo-bar ` leaving `baz`.

## Sentence Objects

### Around Sentence

`as` selects the entire sentence including trailing
whitespace up to the start of the next sentence.

### Behavior

Sentence boundaries: `.`, `!`, `?` followed by space,
tab, or end-of-line. Must be followed by whitespace or
end of paragraph.

## Paragraph Objects

### Around Paragraph

`ap` selects the paragraph including trailing blank lines
up to the next paragraph.

### Behavior

Paragraphs are separated by blank lines. Adjacent blank
lines are included in the selection.

### Example

On a 3-line paragraph followed by a blank line:
`dap` deletes all 3 lines AND the blank line.

## Quote Objects

### Around Double Quote

`a"` selects the content including the surrounding
`"` delimiter pair.

### Around Single Quote

`a'` selects content including surrounding `'` pair.

### Around Backtick

`` a` `` selects content including surrounding `` ` `` pair.

### Example

On `say "hello world" now`, cursor inside quotes:
`da"` removes `"hello world"` leaving `say  now`.
`ca"` changes `"hello world"` and enters insert mode.

## Bracket Objects

### Around Parentheses

`a(` or `a)` or `ab` selects content including the
`(` and `)` delimiters.

### Around Brackets

`a[` or `a]` selects content including `[` and `]`.

### Around Braces

`a{` or `a}` or `aB` selects content including `{` and `}`.

### Around Angle Brackets

`a<` or `a>` selects content including `<` and `>`.

### Example

On `fn(a, b, c)`, cursor inside parens:
`da(` removes `(a, b, c)` leaving `fn`.
`ca)` changes `(a, b, c)` and enters insert mode.

## Tag Objects

### Around Tag

`at` selects from the opening `<tag>` through the closing
`</tag>`, including both tags.

### Example

On `<div>hello</div>`, `dat` removes entire element.
`cat` changes the entire element.

### Self-Closing

Self-closing tags like `<br/>` are treated as a single
unit.

## Nested Delimiters

### Behavior

With nested delimiters, the innermost pair containing
the cursor is selected. Count `{n}a(` selects the
{n}th level outward.

### Multiple Levels

On `(a(b(c)))`, cursor on `c`:
`da(` removes `(c)`, result: `(a(b))`.
`2da(` removes `(b(c))`, result: `(a)`.
`3da(` removes entire `(a(b(c)))`.

## Smart Space Handling

When deleting with `daw`, `das`, `dap`, the resulting
text has correct spacing: no double spaces or missing
separators. The algorithm:
1. If trailing whitespace exists, include it
2. Otherwise, include leading whitespace
3. At start/end of line, adjust accordingly

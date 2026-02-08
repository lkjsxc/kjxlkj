# Quote Text Objects

Text objects for quoted strings.

## Overview

Quote text objects select text delimited by quote
characters. Three quote types are supported:
double quotes, single quotes, and backticks.

## Double Quotes

### Inner

`i"` selects the text between the nearest enclosing
double-quote pair, excluding the quotes.

### Around

`a"` selects the text plus the quote characters.
If a space follows the closing quote, it is included.

### Example

With cursor on `hello` in `say("hello")`: `di"` deletes
`hello`, leaving `say("")`.

## Single Quotes

### Inner

`i'` selects text between single quotes.

### Around

`a'` includes the quote characters.

### Example

With cursor inside `'world'`: `ci'` changes `world`.

## Backticks

### Inner

`` i` `` selects text between backticks.

### Around

`` a` `` includes the backtick characters.

### Example

With cursor inside `` `code` ``: `` di` `` deletes `code`.

## Quote Detection

### Finding Pairs

The cursor does not need to be on a quote character.
If the cursor is not inside a quoted string, the editor
searches forward on the current line for the next
quote pair.

### Search Behavior

1. If cursor is on a quote char, determine if it is
   opening or closing by counting quotes before it
2. If cursor is between quotes, use those as the pair
3. If cursor is outside any quotes, search forward
   on the line for the next opening quote
4. Matching only considers the current line
   (quotes do not span lines by default)

## Nested Quotes

### Different Types

Different quote types nest independently.
In `"it's fine"`, `i"` selects `it's fine`
and `i'` would select `s fine` if cursor is past the `'`.

### Same Type (Escaped)

Escaped quotes (`\"`) inside a string are not treated
as delimiters. The matching algorithm skips any quote
preceded by an odd number of backslashes.

## Multiline Quotes

### Behavior

By default, quote text objects only work within
a single line. A quote on line 5 does not match
a quote on line 7.

### When Enabled

With tree-sitter active, multiline strings are
recognized by the syntax tree. `i"` correctly
selects content of multiline strings in languages
that support them (Python, Rust raw strings, etc.).

## String Continuation

### Escaped Newlines

Line continuations (`\` at end of line) do not
automatically join lines for quote matching.
Tree-sitter handles these correctly when available.

## Special Cases

### Empty Quotes

`i"` on `""` selects nothing (empty range).
Operators like `d` produce no change.
`a"` selects the entire `""`.

### Adjacent Quotes

In `"a""b"`, with cursor on `a`: `i"` selects `a`.
With cursor between the middle quotes: `i"` selects `b`.

## Language Context

### Python Triple Quotes

When tree-sitter is active, triple-quoted strings
(`"""..."""`  or `'''...'''`) are treated as a single
delimited pair. `i"` selects the full multiline content.

### Raw Strings

Language-specific string prefixes (Rust `r#"..."#`,
Python `r"..."``) are handled correctly when tree-sitter
is enabled. Without tree-sitter, raw-string prefixes
are not recognized.

## Quote on Line

### No Quotes Found

If no quote pair is found on the current line, the
operation is canceled. The cursor does not move
and a beep/bell is produced.


# Bracket Text Objects

Text objects for paired brackets.

## Overview

Bracket text objects select text delimited by paired
bracket characters. Each pair has an "inner" (content only)
and "around" (content plus delimiters) variant.

## Parentheses

### Inner

`i(` or `i)` or `ib` — selects everything between `(`
and `)`, excluding the parentheses themselves.

### Around

`a(` or `a)` or `ab` — selects everything between `(`
and `)`, including the parentheses.

### Example

With cursor on `bar` in `foo(bar, baz)`: `di(` deletes
`bar, baz`, leaving `foo()`. `da(` deletes `(bar, baz)`.

## Square Brackets

### Inner

`i[` or `i]` — selects content between `[` and `]`.

### Around

`a[` or `a]` — includes the brackets.

### Example

With cursor inside `arr[idx + 1]`: `ci[` changes `idx + 1`.

## Curly Braces

### Inner

`i{` or `i}` or `iB` — selects content between `{` and `}`.

### Around

`a{` or `a}` or `aB` — includes the braces.

### Example

With cursor inside a function body: `diB` deletes all lines
between the opening and closing brace.

## Angle Brackets

### Inner

`i<` or `i>` — selects content between `<` and `>`.

### Around

`a<` or `a>` — includes the angle brackets.

### Example

With cursor inside `<div class="x">`: `di<` deletes
`div class="x"`.

## Matching Algorithm

### Finding Pairs

1. From the cursor position, search backward for the opener
2. Track nesting depth: increment on opener, decrement on closer
3. When depth reaches zero backward, that is the matching opener
4. From the opener, search forward with depth tracking for the closer
5. Brackets inside string literals and comments are skipped
   when tree-sitter is available

### Cursor Position

If the cursor is on a bracket character, that bracket is
used as one end of the pair. The search proceeds in the
appropriate direction for the matching partner.

## Nested Brackets

### Same Type

Nesting is tracked by depth. In `((a)(b))` with cursor on `a`,
`i(` selects `a`, while `2i(` selects `(a)(b)` (outer level).

### Different Types

Different bracket types nest independently.
In `{ [a] }`, `i[` selects `a` and `i{` selects ` [a] `.

## Multiline Brackets

### Block Selection

Bracket text objects work across multiple lines.
With cursor inside a multiline block:
`diB` deletes all lines between the braces,
leaving only the opening and closing brace lines.

### Formatting

When the content is on separate lines from the brackets,
"inner" includes all lines between the bracket lines
(exclusive of the bracket lines themselves).

## Empty Brackets

### Behavior

`i(` on `()` selects nothing (empty range). Operators
like `d` have no effect. `a(` selects the entire `()`.

## Unmatched Brackets

### No Match Found

If no matching pair is found, the operation is canceled
and the cursor does not move. A beep/bell is produced.


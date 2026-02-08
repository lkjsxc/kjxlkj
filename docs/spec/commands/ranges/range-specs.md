# Range Specifications

Line range syntax for Ex commands.

## Overview

Many Ex commands accept a range specifying which lines
to operate on. Ranges appear before the command name.

## Basic Ranges

### Single Line

| Specifier | Meaning |
|-----------|---------|
| `.` | Current line |
| `$` | Last line |
| `{n}` | Absolute line number {n} |
| `%` | Entire buffer (shorthand for `1,$`) |

### Line Range

`{start},{end}` specifies a range from start to end.
`:10,20d` deletes lines 10 through 20.
`:.,$d` deletes from current line to end of file.

## Relative Addressing

### Offset

`{base}+{n}` adds {n} lines to the base address.
`{base}-{n}` subtracts {n} lines.
`.+3` means 3 lines below current line.
`$-5` means 5 lines before the last line.

### Examples

`:.,+5d` deletes current line and 5 lines below.
`:-3,.d` deletes from 3 lines above to current line.

## Pattern Ranges

### Forward Search

`/pattern/` finds the next line matching `pattern`.
`:/function/d` deletes the next line containing `function`.

### Backward Search

`?pattern?` finds the previous line matching `pattern`.

### Pattern Range

`:/start/,/end/d` deletes from the line matching
`start` to the line matching `end`.

## Mark Ranges

### Mark Address

`'a` refers to the line of mark `a`.
`:'a,'b d` deletes from mark `a` to mark `b`.

### Visual Range

`'<,'>` is set automatically when entering command mode
from visual mode. It refers to the visual selection.

## Special Symbols

### Last Substitute

`&` represents the last substitute pattern.

### Visual Start/End

| Symbol | Meaning |
|--------|---------|
| `'<` | Start of last visual selection |
| `'>` | End of last visual selection |

## Count

### With Count

`:d 5` deletes 5 lines starting from current line.
This is equivalent to `:.,.+4d`.

### Range + Count

`:10d 3` deletes 3 lines starting from line 10.

## Compound Ranges

### Multiple Separators

`;` can be used instead of `,`. With `;`, the current
line is set to the first address before evaluating the
second. `/pattern/;+3` finds pattern, then adds 3 from
that position.

## Validation

### Out of Range

Line numbers beyond the file are clamped to the last
line. Line 0 is treated as line 1 for most commands.

### Reversed Range

If start > end, the editor prompts:
`Backwards range given, OK to swap (y/n)?`

## Common Patterns

| Pattern | Meaning |
|---------|---------|
| `:%` | Entire file |
| `:.,$` | Current to end |
| `:1,.` | Beginning to current |
| `:'<,'>` | Visual selection |
| `:.,.+10` | Current plus 10 lines |
| `:/begin/,/end/` | Between patterns |

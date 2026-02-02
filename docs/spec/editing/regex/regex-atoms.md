# Regex Atoms

Fundamental pattern building blocks for search and substitute.

## Basic Atoms

| Atom | Meaning |
|------|---------|
| `.` | Any character except newline |
| `\_.` | Any character including newline |
| `\s` | Whitespace (space, tab) |
| `\S` | Non-whitespace |
| `\d` | Digit [0-9] |
| `\D` | Non-digit |
| `\w` | Word character [a-zA-Z0-9_] |
| `\W` | Non-word character |
| `\h` | Head of word [a-zA-Z_] |
| `\H` | Non-head of word |
| `\a` | Alphabetic [a-zA-Z] |
| `\A` | Non-alphabetic |
| `\l` | Lowercase [a-z] |
| `\L` | Non-lowercase |
| `\u` | Uppercase [A-Z] |
| `\U` | Non-uppercase |
| `\x` | Hexadecimal [0-9a-fA-F] |
| `\X` | Non-hexadecimal |
| `\o` | Octal [0-7] |
| `\O` | Non-octal |

## Special Characters

| Atom | Meaning |
|------|---------|
| `\n` | Newline |
| `\r` | Carriage return |
| `\t` | Tab |
| `\e` | Escape |
| `\b` | Backspace |
| `\\` | Literal backslash |

## Unicode Categories

| Atom | Meaning |
|------|---------|
| `\p{L}` | Unicode letter |
| `\p{N}` | Unicode number |
| `\p{P}` | Unicode punctuation |
| `\p{S}` | Unicode symbol |
| `\p{Z}` | Unicode separator |
| `\P{L}` | Not Unicode letter |

## Character Classes

### Defined Classes


### Special in Classes


### POSIX Classes


## Anchors

| Anchor | Meaning |
|--------|---------|
| `^` | Start of line |
| `$` | End of line |
| `\<` | Start of word |
| `\>` | End of word |
| `\zs` | Set match start |
| `\ze` | Set match end |

## Position Atoms

| Atom | Meaning |
|------|---------|
| `\%^` | Start of file |
| `\%$` | End of file |
| `\%#` | Cursor position |
| `\%'m` | Position of mark m |
| `\%23l` | Line 23 |
| `\%45c` | Column 45 |
| `\%>99l` | After line 99 |
| `\%<10c` | Before column 10 |
| `\%V` | In visual area |

## Line and Column


## Escaped Literals

| Sequence | Meaning |
|----------|---------|
| `\.` | Literal dot |
| `\*` | Literal asterisk |
| `\[` | Literal bracket |
| `\]` | Literal bracket |
| `\^` | Literal caret |
| `\$` | Literal dollar |
| `\/` | Literal slash |

## Byte Atoms

| Atom | Meaning |
|------|---------|
| `\%d97` | Decimal byte value |
| `\%x61` | Hex byte value |
| `\%o141` | Octal byte value |
| `\%u0041` | Unicode codepoint |

## Examples

### Match Identifiers


### Match Numbers


### Match at Position


## API Reference


## See Also

- [quantifiers.md](quantifiers.md) - Repetition patterns
- [anchors.md](anchors.md) - Position matching
- [char-classes.md](char-classes.md) - Character classes

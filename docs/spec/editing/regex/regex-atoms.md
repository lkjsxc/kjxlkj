# Regex Atoms

Back: [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)

Fundamental pattern building blocks for search and substitute operations.

## Basic atoms

| Atom | Meaning |
|---|---|
| `.` | Any character except newline |
| `\_.` | Any character including newline |
| `\s` | Whitespace (space, tab) |
| `\S` | Non-whitespace |
| `\d` | Digit `[0-9]` |
| `\D` | Non-digit |
| `\w` | Word character `[a-zA-Z0-9_]` |
| `\W` | Non-word character |
| `\h` | Head of word `[a-zA-Z_]` |
| `\H` | Non-head of word |
| `\a` | Alphabetic `[a-zA-Z]` |
| `\A` | Non-alphabetic |
| `\l` | Lowercase `[a-z]` |
| `\L` | Non-lowercase |
| `\u` | Uppercase `[A-Z]` |
| `\U` | Non-uppercase |
| `\x` | Hexadecimal `[0-9a-fA-F]` |
| `\X` | Non-hexadecimal |
| `\o` | Octal `[0-7]` |
| `\O` | Non-octal |

## Special characters

| Atom | Meaning |
|---|---|
| `\n` | Newline |
| `\r` | Carriage return |
| `\t` | Tab |
| `\e` | Escape (0x1B) |
| `\b` | Backspace (0x08) |
| `\\` | Literal backslash |

## Unicode categories

| Atom | Meaning |
|---|---|
| `\p{L}` | Unicode letter |
| `\p{N}` | Unicode number |
| `\p{P}` | Unicode punctuation |
| `\p{S}` | Unicode symbol |
| `\p{Z}` | Unicode separator |
| `\P{L}` | Not Unicode letter (negated) |

## Character classes

### Defined classes

Character classes use `[` and `]` brackets. Inside a class, most metacharacters lose their special meaning.

| Syntax | Meaning |
|---|---|
| `[abc]` | Match any of `a`, `b`, or `c` |
| `[a-z]` | Match any lowercase letter |
| `[^abc]` | Match any character except `a`, `b`, `c` |
| `[a-zA-Z0-9]` | Match any alphanumeric character |

### Special characters in classes

| Character | Meaning inside `[]` |
|---|---|
| `]` | Literal if first character (e.g., `[]abc]`) |
| `-` | Range if between characters; literal if first or last |
| `^` | Negation if first; literal otherwise |
| `\` | Escape for special sequences (`\d`, `\s`, etc.) |

### POSIX classes

| Class | Equivalent | Meaning |
|---|---|---|
| `[:alpha:]` | `[a-zA-Z]` | Alphabetic |
| `[:digit:]` | `[0-9]` | Decimal digit |
| `[:alnum:]` | `[a-zA-Z0-9]` | Alphanumeric |
| `[:space:]` | `[ \t\n\r\f\v]` | Whitespace |
| `[:upper:]` | `[A-Z]` | Uppercase |
| `[:lower:]` | `[a-z]` | Lowercase |
| `[:punct:]` | punctuation chars | Punctuation |
| `[:blank:]` | `[ \t]` | Space and tab |

## Anchors

| Anchor | Meaning |
|---|---|
| `^` | Start of line |
| `$` | End of line |
| `\<` | Start of word boundary |
| `\>` | End of word boundary |
| `\zs` | Set the start of the match (the match begins here) |
| `\ze` | Set the end of the match (the match ends here) |

## Position atoms

| Atom | Meaning |
|---|---|
| `\%^` | Start of file |
| `\%$` | End of file |
| `\%#` | Cursor position |
| `\%'m` | Position of mark `m` |
| `\%23l` | Matches on line 23 |
| `\%45c` | Matches at column 45 |
| `\%>99l` | Matches after line 99 |
| `\%<10c` | Matches before column 10 |
| `\%V` | Inside the visual area |

## Line and column atoms

Position atoms using `\%l` and `\%c` constrain matches to specific line numbers or column positions. This is useful for restricting substitutions to specific regions of the buffer.

| Atom | Meaning |
|---|---|
| `\%{n}l` | Match on line `{n}` |
| `\%>{n}l` | Match after line `{n}` |
| `\%<{n}l` | Match before line `{n}` |
| `\%{n}c` | Match at column `{n}` |
| `\%>{n}c` | Match after column `{n}` |
| `\%<{n}c` | Match before column `{n}` |

## Escaped literals

| Sequence | Meaning |
|---|---|
| `\.` | Literal dot |
| `\*` | Literal asterisk |
| `\[` | Literal open bracket |
| `\]` | Literal close bracket |
| `\^` | Literal caret |
| `\$` | Literal dollar sign |
| `\/` | Literal forward slash |

## Byte atoms

| Atom | Meaning |
|---|---|
| `\%d97` | Match character with decimal byte value 97 (letter `a`) |
| `\%x61` | Match character with hex byte value 0x61 |
| `\%o141` | Match character with octal byte value 141 |
| `\%u0041` | Match character with Unicode code point U+0041 |

## Examples

### Match identifiers

The pattern `\<\h\w*\>` matches whole words that start with a word-head character followed by zero or more word characters.

### Match numbers

The pattern `\<\d\+\(\.\d\+\)\?\>` matches integers and decimal numbers.

### Match at position

The pattern `\%3l\%10c.` matches any character at line 3, column 10.

## Related

- Quantifiers: [/docs/spec/editing/regex/quantifiers.md](/docs/spec/editing/regex/quantifiers.md)
- Anchors: [/docs/spec/editing/regex/anchors.md](/docs/spec/editing/regex/anchors.md)
- Character classes: [/docs/spec/editing/regex/char-classes.md](/docs/spec/editing/regex/char-classes.md)
- Magic modes: [/docs/spec/editing/regex/magic-modes.md](/docs/spec/editing/regex/magic-modes.md)

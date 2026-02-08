# Character Classes

Matching sets of characters in patterns.

## Basic Collection Syntax (normative)

`[abc]` matches any single character `a`, `b`, or `c`.

`[^abc]` matches any character except `a`, `b`, or `c` (negation).

## Range Patterns (normative)

| Pattern | Matches |
|---|---|
| `[a-z]` | Lowercase ASCII letters |
| `[A-Z]` | Uppercase ASCII letters |
| `[0-9]` | Digits |
| `[a-zA-Z]` | All ASCII letters |
| `[a-zA-Z0-9]` | Alphanumeric |
| `[a-fA-F0-9]` | Hex digits |

## Special Characters Inside Collections

| To match | Place it |
|---|---|
| `]` | First position: `[]abc]` |
| `^` | Not first: `[a^b]` |
| `-` | First or last: `[-abc]` or `[abc-]` |
| `\` | Escaped: `[\\]` |

## POSIX Character Classes

| Class | Equivalent | Description |
|---|---|---|
| `[[:alnum:]]` | `[a-zA-Z0-9]` | Alphanumeric |
| `[[:alpha:]]` | `[a-zA-Z]` | Alphabetic |
| `[[:blank:]]` | `[ \t]` | Space/tab |
| `[[:digit:]]` | `[0-9]` | Digits |
| `[[:lower:]]` | `[a-z]` | Lowercase |
| `[[:upper:]]` | `[A-Z]` | Uppercase |
| `[[:space:]]` | `[ \t\n\r\f\v]` | Whitespace |
| `[[:xdigit:]]` | `[0-9A-Fa-f]` | Hex digits |
| `[[:punct:]]` | Punctuation | Punctuation chars |
| `[[:print:]]` | `[ -~]` | Printable |
| `[[:graph:]]` | `[!-~]` | Visible (no space) |
| `[[:cntrl:]]` | Control chars | Control chars |

## Vim Shortcut Atoms (normative)

| Shortcut | Matches | Negation |
|---|---|---|
| `\d` | `[0-9]` | `\D` |
| `\w` | `[a-zA-Z0-9_]` | `\W` |
| `\s` | `[ \t]` | `\S` |
| `\a` | `[a-zA-Z]` | `\A` |
| `\l` | `[a-z]` | `\L` |
| `\u` | `[A-Z]` | `\U` |
| `\x` | `[0-9A-Fa-f]` | `\X` |

These shortcuts work outside of collections (standalone atoms).

## Collection Rules

1. `]` as the first character after `[` or `[^` is literal
2. `^` not in first position is literal
3. `-` in first or last position is literal
4. `\` always escapes the next character
5. All other characters are literal

## Common Patterns

| Pattern | Matches |
|---|---|
| `[_a-zA-Z][_a-zA-Z0-9]*` | C/Rust identifier |
| `[+-]\?[0-9]\+` | Optional-sign integer |
| `[0-9]\+\.[0-9]\+` | Decimal number |
| `[^\x00-\x7F]` | Non-ASCII (matches CJK, etc.) |

## Related

- Pattern atoms: [/docs/spec/editing/regex/pattern-atoms.md](/docs/spec/editing/regex/pattern-atoms.md)
- Quantifiers: [/docs/spec/editing/regex/quantifiers.md](/docs/spec/editing/regex/quantifiers.md)

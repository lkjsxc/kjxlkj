# Character Classes

Matching sets of characters in patterns.

## Basic Syntax


## Range Patterns

| Pattern | Description |
|---------|-------------|
| `[a-z]` | Lowercase letters |
| `[A-Z]` | Uppercase letters |
| `[0-9]` | Digits |
| `[a-zA-Z]` | All letters |
| `[a-zA-Z0-9]` | Alphanumeric |
| `[a-f0-9]` | Hex digits (lower) |

## Negation


## Special Characters Inside Classes

### Literal Special Characters

| To Match | Pattern |
|----------|---------|
| `]` | `[\]]` or `[]abc]` (first) |
| `^` | `[^^]` or `[a^b]` (not first) |
| `-` | `[-abc]` or `[abc-]` (first/last) |
| `\` | `[\\]` |

### Examples


## POSIX Character Classes

| Class | Equivalent | Description |
|-------|------------|-------------|
| `[[:alnum:]]` | `[a-zA-Z0-9]` | Alphanumeric |
| `[[:alpha:]]` | `[a-zA-Z]` | Alphabetic |
| `[[:blank:]]` | `[ \t]` | Space/tab |
| `[[:cntrl:]]` | `[\x00-\x1f\x7f]` | Control |
| `[[:digit:]]` | `[0-9]` | Digits |
| `[[:graph:]]` | `[!-~]` | Visible |
| `[[:lower:]]` | `[a-z]` | Lowercase |
| `[[:print:]]` | `[ -~]` | Printable |
| `[[:punct:]]` | Various | Punctuation |
| `[[:space:]]` | `[ \t\n\r\f\v]` | Whitespace |
| `[[:upper:]]` | `[A-Z]` | Uppercase |
| `[[:xdigit:]]` | `[0-9A-Fa-f]` | Hex digits |

### Usage


## Combining Classes


## Vim Character Class Shortcuts

| Shortcut | Equivalent | POSIX |
|----------|------------|-------|
| `\d` | `[0-9]` | `[[:digit:]]` |
| `\D` | `[^0-9]` | `[^[:digit:]]` |
| `\w` | `[a-zA-Z0-9_]` | N/A |
| `\W` | `[^a-zA-Z0-9_]` | N/A |
| `\s` | `[ \t]` | `[[:blank:]]` |
| `\S` | `[^ \t]` | `[^[:blank:]]` |
| `\a` | `[a-zA-Z]` | `[[:alpha:]]` |
| `\l` | `[a-z]` | `[[:lower:]]` |
| `\u` | `[A-Z]` | `[[:upper:]]` |
| `\x` | `[0-9A-Fa-f]` | `[[:xdigit:]]` |

## Unicode Classes


### Unicode Scripts


## Collection Operations


## Common Patterns

| Pattern | Matches |
|---------|---------|
| `[_a-zA-Z][_a-zA-Z0-9]*` | C identifier |
| `[+-]?[0-9]+` | Signed integer |
| `[0-9]+\.[0-9]+` | Decimal number |
| `[a-fA-F0-9]+` | Hex string |
| `[^\x00-\x7F]` | Non-ASCII |
| `[^\n\r]` | Not newline |

## Inside Collection Rules

1. `]` as first char is literal
2. `^` not first is literal
3. `-` first or last is literal
4. `\` always escapes next char
5. Other chars are literal

## Examples

### Match Filename


### Match Quoted String


### Match URL Characters


## API Reference


## See Also


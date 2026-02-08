# Substitute Special Characters

Special sequences in patterns and replacements.

## Pattern Metacharacters (normative)

| Character | Meaning |
|---|---|
| `.` | Any single character (except newline) |
| `*` | Zero or more of the preceding atom |
| `\+` | One or more of the preceding atom |
| `\?` or `\=` | Zero or one of the preceding atom |
| `^` | Start of line |
| `$` | End of line |
| `\|` | Alternation (OR) |
| `\(...\)` | Capturing group |
| `\%(…\)` | Non-capturing group |

## Character Classes (normative)

| Pattern | Matches |
|---|---|
| `[abc]` | Any of a, b, c |
| `[^abc]` | Any character NOT a, b, c |
| `[a-z]` | Lowercase letters |
| `\w` | Word character `[a-zA-Z0-9_]` |
| `\W` | Non-word character |
| `\s` | Whitespace (space, tab) |
| `\S` | Non-whitespace |
| `\d` | Digit `[0-9]` |
| `\D` | Non-digit |

## Replacement Specials (normative)

| Sequence | Inserts |
|---|---|
| `&` or `\0` | Entire matched text |
| `\1` through `\9` | Captured group N |
| `~` | Previous replacement string |
| `\r` | Newline (splits the line) |
| `\n` | NUL character (NOT newline in replacement) |
| `\t` | Tab character |
| `\\` | Literal backslash |

## Case Modifiers in Replacement (normative)

| Sequence | Effect |
|---|---|
| `\u` | Uppercase the next character |
| `\U` | Uppercase all following characters |
| `\l` | Lowercase the next character |
| `\L` | Lowercase all following characters |
| `\e` or `\E` | End case modification |

## Word Boundary Atoms

| Atom | Matches |
|---|---|
| `\<` | Start of word |
| `\>` | End of word |
| `\zs` | Set start of match (match from here) |
| `\ze` | Set end of match (match to here) |

## Newline Handling in Patterns

| Atom | Matches |
|---|---|
| `\n` | Newline character in search |
| `\_s` | Whitespace including newline |
| `\_.` | Any character including newline |

## Alternative Delimiters

Any non-alphanumeric character can replace `/` as the delimiter. Useful when the pattern contains `/`:

- `:%s#/usr/local#/opt#g` uses `#` as delimiter.

## Related

- Substitute command: [/docs/spec/commands/substitute/README.md](/docs/spec/commands/substitute/README.md)
- Substitute expressions: [/docs/spec/commands/substitute/substitute-expressions.md](/docs/spec/commands/substitute/substitute-expressions.md)
- Regex: [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)

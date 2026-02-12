# Regular Expressions

Back: [/docs/spec/editing/regex/README.md](/docs/spec/editing/regex/README.md)

Pattern matching syntax for search and substitution. The regex engine uses the Rust `regex` crate as the backend.

## Engine choice (normative)

The implementation MUST use the Rust `regex` crate (or `regex-automata`). This provides linear-time matching without backtracking, Unicode support, and deterministic performance.

## Default syntax mode

kjxlkj uses Vim-compatible "magic" mode by default for `/` and `?` search. See [/docs/spec/editing/regex/magic-modes.md](/docs/spec/editing/regex/magic-modes.md) for mode details.

## Supported patterns (normative)

All regex constructs recognized by the engine.

### Literals and metacharacters

| Pattern | Meaning |
|---|---|
| `.` | Any character except newline |
| `\` | Escape next character |
| `^` | Start of line |
| `$` | End of line |
| `\<` | Start of word boundary |
| `\>` | End of word boundary |

### Character classes

| Pattern | Meaning |
|---|---|
| `[abc]` | Any of a, b, c |
| `[^abc]` | Any except a, b, c |
| `[a-z]` | Range a through z |
| `\d` | Digit `[0-9]` |
| `\D` | Non-digit `[^0-9]` |
| `\w` | Word character `[a-zA-Z0-9_]` |
| `\W` | Non-word character |
| `\s` | Whitespace (space, tab, newline) |
| `\S` | Non-whitespace |

### Quantifiers

| Pattern | Meaning |
|---|---|
| `*` | Zero or more (greedy) |
| `\+` | One or more (greedy) |
| `\?` or `\=` | Zero or one |
| `\{n}` | Exactly n times |
| `\{n,m}` | Between n and m times |
| `\{n,}` | At least n times |
| `\{,m}` | At most m times |
| `\{-}` | Zero or more (non-greedy) |

### Grouping and alternation

| Pattern | Meaning |
|---|---|
| `\(` ... `\)` | Capture group |
| `\%(` ... `\)` | Non-capturing group |
| `\|` | Alternation (OR) |
| `\1` ... `\9` | Back-reference to capture group N |

### Special atoms

| Pattern | Meaning |
|---|---|
| `\n` | Newline |
| `\t` | Tab |
| `\r` | Carriage return |
| `\e` | Escape (0x1B) |

## Translation to Rust regex

The implementation MUST translate Vim-style magic patterns to Rust regex syntax before compilation:

| Vim magic | Rust regex |
|---|---|
| `\+` | `+` |
| `\?` | `?` |
| `\(` ... `\)` | `(` ... `)` |
| `\|` | pipe |
| `\<` | `\b` (word boundary) |
| `\>` | `\b` |
| `\{n,m}` | `{n,m}` |

## Case sensitivity

| Mode | Trigger | Behavior |
|---|---|---|
| Default | (none) | Follows `ignorecase` option |
| Smart case | `smartcase` option | Case-insensitive if pattern is all lowercase; case-sensitive if any uppercase |
| Force insensitive | `\c` in pattern | Always case-insensitive |
| Force sensitive | `\C` in pattern | Always case-sensitive |

## Related

- Magic modes: [/docs/spec/editing/regex/magic-modes.md](/docs/spec/editing/regex/magic-modes.md)
- Search: [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)
- Substitute: [/docs/spec/commands/substitute/README.md](/docs/spec/commands/substitute/README.md)

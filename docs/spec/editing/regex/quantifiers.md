# Quantifiers

Repetition matching in regex patterns.

## Quantifier Reference (normative)

| Magic Mode | Very Magic | Meaning | Greedy |
|---|---|---|---|
| `*` | `*` | Zero or more | Yes |
| `\+` | `+` | One or more | Yes |
| `\?` or `\=` | `?` | Zero or one | Yes |
| `\{n,m}` | `{n,m}` | Between n and m | Yes |
| `\{n}` | `{n}` | Exactly n | N/A |
| `\{n,}` | `{n,}` | At least n | Yes |
| `\{,m}` | `{,m}` | At most m | Yes |
| `\{-n,m}` | `{-n,m}` | Between n and m | No (lazy) |
| `\{-}` | `{-}` | Zero or more | No (lazy) |
| `\{-n,}` | `{-n,}` | At least n | No (lazy) |
| `\{-,m}` | `{-,m}` | At most m | No (lazy) |

## Greedy vs Lazy

**Greedy** quantifiers match as much as possible, then backtrack if needed.

**Lazy** quantifiers (prefixed with `-` in braces) match as little as possible.

Given text `<a>text</a>`:

- Greedy `<.*>` matches `<a>text</a>` (entire string)
- Lazy `<.\{-}>` matches `<a>` (first tag only)

## Common Usage

| Pattern | Meaning |
|---|---|
| `\d\+` | One or more digits |
| `\w*` | Zero or more word characters |
| `\s\?` | Optional whitespace |
| `.\{3,5}` | 3 to 5 of any character |
| `\_.\{-}` | Non-greedy across lines |

## Quantifier Precedence

Quantifiers bind to the immediately preceding atom. Use grouping to apply quantifiers to multi-character sequences: `\(foo\)\+` matches one or more `foo`.

## Related

- Pattern atoms: [/docs/spec/editing/regex/pattern-atoms.md](/docs/spec/editing/regex/pattern-atoms.md)
- Grouping: [/docs/spec/editing/regex/grouping-refs.md](/docs/spec/editing/regex/grouping-refs.md)
- Multi-line: [/docs/spec/editing/regex/multiline-patterns.md](/docs/spec/editing/regex/multiline-patterns.md)

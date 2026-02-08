# Multi-line Patterns

Searching and matching across line boundaries.

## Newline Atoms

| Pattern | Meaning |
|---|---|
| `\n` | Match newline character in search pattern |
| `\_s` | Whitespace including newline |
| `\_.` | Any character including newline |
| `\_^` | Start of line, anywhere in pattern |
| `\_$` | End of line, anywhere in pattern |

## Character Class Extensions

Prefix any character class with `\_` to include newline:

| Normal | With Newline | Matches |
|---|---|---|
| `.` | `\_.` | Any char including newline |
| `\s` | `\_s` | Space, tab, newline |
| `\w` | `\_w` | Word char or newline |
| `\a` | `\_a` | Alpha or newline |
| `[abc]` | `\_[abc]` | a, b, c, or newline |

## Common Multi-line Patterns

| Task | Pattern |
|---|---|
| Match text across lines | `start\_.\{-}end` |
| Consecutive lines | `line1\nline2` |
| C block comment | `/\*\_.\{-}\*/` |
| HTML tag with content | `<\w\+\_.\{-}<\/\w\+>` |
| Python triple string | `"""\_.\{-}"""` |
| Adjacent blank lines | `\n\n\n` |

## Performance Warning

`\_.*` is greedy and scans to end of buffer. Always prefer `\_.\{-}` (non-greedy) for multi-line matching to avoid extreme slowness on large files.

## Substitution Across Lines

Multi-line patterns work in `:s` with appropriate ranges. Use `\n` in the pattern and `\r` in the replacement to insert newlines.

## Related

- Pattern atoms: [/docs/spec/editing/regex/pattern-atoms.md](/docs/spec/editing/regex/pattern-atoms.md)
- Quantifiers: [/docs/spec/editing/regex/quantifiers.md](/docs/spec/editing/regex/quantifiers.md)

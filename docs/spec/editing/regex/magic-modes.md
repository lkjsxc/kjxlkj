# Magic Modes

Pattern interpretation modes controlling which characters are special.

## Four Magic Levels (normative)

| Mode | Prefix | Special without `\` | Need `\` to activate |
|---|---|---|---|
| Very Magic | `\v` | Most punctuation: `()`, `|`, `+`, `?`, `{}`, `<>` | Only `\` and delimiter |
| Magic | `\m` | `.`, `*`, `^`, `$`, `[` | `+`, `?`, `|`, `(`, `)`, `{`, `}` |
| No Magic | `\M` | Only `^`, `$` | Everything else |
| Very No Magic | `\V` | Nothing (only `\`) | Everything |

## Magic (Default)

The default mode. `.`, `*`, `^`, `$`, `[` are special. Groups require `\(...\)`, alternation requires `\|`, quantifiers require `\+`, `\?`, `\{n,m}`.

## Very Magic (`\v`)

After `\v`, most punctuation is special without backslash. Only `\`, the search delimiter, and alphanumerics/`_` are literal.

Comparison for grouping with alternation:

- Magic: `\(foo\|bar\)\+`
- Very Magic: `\v(foo|bar)+`

## No Magic (`\M`)

Only `^` and `$` are special. `.` matches literal dot. Use `\` to activate metacharacters: `\.` matches any char, `\*` matches zero or more.

## Very No Magic (`\V`)

Everything is literal except `\`. Useful for searching literal text with special characters (paths, URLs). Escape to activate: `\V/path/to/file` searches for literal `/path/to/file`.

## Mode Switching Mid-Pattern

Place `\v`, `\m`, `\M`, or `\V` anywhere in a pattern to switch modes from that point onward. Useful for mixing literal and regex sections.

## Recommended Usage

- **Very Magic** (`\v`) for complex patterns (groups, alternation, quantifiers)
- **Magic** (default) for simple patterns
- **Very No Magic** (`\V`) for literal text search containing special characters

## Related

- Very magic detail: [/docs/spec/editing/regex/very-magic.md](/docs/spec/editing/regex/very-magic.md)
- Pattern atoms: [/docs/spec/editing/regex/pattern-atoms.md](/docs/spec/editing/regex/pattern-atoms.md)

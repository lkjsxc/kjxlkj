# Address Patterns

Back: [/docs/spec/commands/ranges/README.md](/docs/spec/commands/ranges/README.md)

Search patterns used as line addresses in ex commands.

## Forward search address (normative)

`/{pattern}/` specifies the next line matching `{pattern}` below the cursor.

| Syntax | Meaning | Example |
|---|---|---|
| `/{pattern}/` | Next line matching pattern | `/error/d` deletes next line containing "error" |
| `/{pattern}/+{n}` | N lines after the match | `/^func/+1` is the line after next function header |
| `/{pattern}/-{n}` | N lines before the match | `/^end/-1` is the line before next "end" |

## Backward search address (normative)

`?{pattern}?` specifies the previous line matching `{pattern}` above the cursor.

| Syntax | Meaning |
|---|---|
| `?{pattern}?` | Previous line matching pattern |
| `?{pattern}?+{n}` | N lines after the previous match |
| `?{pattern}?-{n}` | N lines before the previous match |

## Pattern ranges (normative)

Two address patterns form a range:

| Syntax | Meaning | Example |
|---|---|---|
| `/start/,/end/` | From first match of "start" to first match of "end" | `/^begin/,/^end/d` |
| `/pat/,{line}` | From pattern match to absolute line | `/error/,$d` |
| `{line},/pat/` | From absolute line to pattern match | `1,/^---/y` |
| `/pat1/;/pat2/` | Semicolon: search for pat2 starts from pat1's line | `/func/;/end/d` |

The semicolon `;` differs from comma `,` in range evaluation: with `;`, the cursor moves to the first address before evaluating the second.

## Empty pattern (normative)

An empty pattern reuses the last search pattern:

| Syntax | Equivalent |
|---|---|
| `//` | `/{last_search}/` |
| `??` | `?{last_search}?` |
| `//,//` | Range using last search pattern for both endpoints |

## Delimiter alternatives (normative)

When the pattern contains `/`, an alternative delimiter may be used:

| Form | Syntax |
|---|---|
| Forward with backslash | `\/{pattern}\/` (standard) |
| Substitute with different delimiter | `:s#pattern#replacement#` |

## Offset arithmetic (normative)

Offsets are applied after the pattern match:

| Syntax | Result |
|---|---|
| `/pat/+0` | The matched line itself |
| `/pat/+3` | Three lines below the match |
| `/pat/-2` | Two lines above the match |
| `/pat/+3-1` | Offsets are cumulative: net +2 |

## Global command integration

| Command | Meaning |
|---|---|
| `:g/pattern/cmd` | Execute `cmd` on every line matching `pattern` |
| `:v/pattern/cmd` | Execute `cmd` on every line NOT matching `pattern` |
| `:g/pat1/,/pat2/cmd` | Execute `cmd` on ranges delimited by patterns |

## Related

- Range specs: [/docs/spec/commands/ranges/range-specs.md](/docs/spec/commands/ranges/range-specs.md)
- Ranges: [/docs/spec/commands/ranges/ranges.md](/docs/spec/commands/ranges/ranges.md)
- Regex: [/docs/spec/editing/regex/regex.md](/docs/spec/editing/regex/regex.md)


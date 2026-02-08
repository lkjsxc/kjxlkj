# Ranges
Ranges select line spans for commands like substitute and filter.

## Requirements
- Range evaluation is deterministic and based on buffer snapshots.
- Pattern-based ranges use core search state (not service results).

## Common range forms (normative)

| Range | Meaning |
|---|---|
| (none) | Current line |
| `{n}` | Line number `{n}` (1-indexed) |
| `.` | Current line |
| `$` | Last line |
| `%` | All lines (`1,$`) |
| `'<,'>` | Visual selection (start to end) |
| `'{mark}` | Line of mark `{mark}` |
| `{n},{m}` | From line `{n}` to line `{m}` |
| `.+{n}` | Current line plus `{n}` offset |
| `.-{n}` | Current line minus `{n}` offset |
| `/pattern/` | Next line matching `pattern` |
| `?pattern?` | Previous line matching `pattern` |
| `\/` | Next occurrence of last search pattern |
| `\?` | Previous occurrence of last search pattern |

## Range arithmetic (normative)

Ranges support addition and subtraction offsets:

| Example | Meaning |
|---|---|
| `/pattern/+3` | 3 lines after the next match of `pattern` |
| `?pattern?-1` | 1 line before the previous match of `pattern` |
| `$-5,$` | Last 6 lines of the buffer |
| `'a,'b` | From mark `a` to mark `b` |

## Validation (normative)

| Condition | Behavior |
|---|---|
| Start > End | Error: "Backwards range given" (unless the command supports it, e.g., with confirmation) |
| Line out of bounds | Clamp to valid range (1 to last line) |
| Pattern not found | Error: "Pattern not found: {pattern}" |
| Invalid mark | Error: "Mark not set" |

## Commands that accept ranges (normative)

| Command | Default range | Description |
|---|---|---|
| `:d[elete]` | current line | Delete lines |
| `:y[ank]` | current line | Yank lines |
| `:s[ubstitute]` | current line | Substitute pattern |
| `:g[lobal]` | `%` (all lines) | Execute command on matching lines |
| `:v[global]` | `%` (all lines) | Execute command on non-matching lines |
| `:sort` | `%` (all lines) | Sort lines |
| `:!` | current line | Filter through external command |
| `:w[rite]` | `%` (all lines) | Write lines to file |
| `:r[ead]` | current line | Read file after line |
| `:copy` / `:t` | current line | Copy lines to destination |
| `:move` / `:m` | current line | Move lines to destination |
| `:normal` | current line | Execute Normal-mode commands on lines |

## Related

- Command syntax: [/docs/spec/commands/syntax.md](/docs/spec/commands/syntax.md)
- Substitute: [/docs/spec/commands/substitute/README.md](/docs/spec/commands/substitute/README.md)
- Marks: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)

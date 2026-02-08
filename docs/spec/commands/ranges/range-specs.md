# Range Specifications

Back: [/docs/spec/commands/ranges/README.md](/docs/spec/commands/ranges/README.md)

How ranges are specified for ex commands.

## Overview

Many ex commands accept a range prefix that specifies which lines the command operates on.

## Range Types

| Range | Description |
|---|---|
| (none) | Current line |
| `.` | Current line |
| `$` | Last line |
| `%` | Entire buffer (`1,$`) |
| `{N}` | Line number N |
| `'a` | Line of mark `a` |
| `'{` | Line of mark `{` (auto-mark) |
| `/pattern/` | Next line matching pattern |
| `?pattern?` | Previous line matching pattern |
| `'<,'>` | Visual selection range |

## Range Arithmetic

| Expression | Meaning |
|---|---|
| `.+3` | Current line + 3 |
| `$-5` | 5 lines before end |
| `/foo/+1` | Line after next "foo" |

## Combined Ranges

A range is `{start},{end}`:

| Range | Lines |
|---|---|
| `10,20` | Lines 10 through 20 |
| `.,+5` | Current line through 5 lines below |
| `'a,'b` | From mark `a` to mark `b` |
| `%` | All lines (shorthand for `1,$`) |

## Visual Range

After making a visual selection and pressing `:`, the command line shows `:'<,'>`. This applies the command to the selected lines.

## Global Range

`:g/pattern/cmd` applies `cmd` to all lines matching pattern. This is a different mechanism from line ranges.

## Examples

| Command | Effect |
|---|---|
| `:10d` | Delete line 10 |
| `:10,20d` | Delete lines 10-20 |
| `:%s/foo/bar/g` | Substitute in entire buffer |
| `:.,$d` | Delete from current line to end |
| `:'<,'>normal! I// ` | Comment visual selection |

## Related

- Commands overview: [/docs/spec/commands/README.md](/docs/spec/commands/README.md)
- Substitute: [/docs/spec/commands/substitute/substitute-command.md](/docs/spec/commands/substitute/substitute-command.md)
- Global command: [/docs/spec/commands/substitute/global-command.md](/docs/spec/commands/substitute/global-command.md)

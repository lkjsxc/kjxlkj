# Range Commands

Line range specifications used in text-manipulation context.

## Overview

This file covers range-based text manipulation. For the full range syntax specification, see [/docs/spec/commands/ranges/ranges.md](/docs/spec/commands/ranges/ranges.md).

## Range Specifiers (normative)

| Specifier | Meaning |
|---|---|
| `n` | Absolute line number n |
| `.` | Current line |
| `$` | Last line in buffer |
| `%` | All lines (alias for `1,$`) |
| `+n` | n lines below current |
| `-n` | n lines above current |
| `.+3` | Current line plus 3 |
| `'a` | Line of mark `a` |
| `'<,'>` | Lines of last visual selection |
| `/pattern/` | Next line matching pattern |
| `?pattern?` | Previous line matching pattern |

## Range + Offset (normative)

Offsets can be appended to any range address:

| Example | Meaning |
|---|---|
| `/pattern/+3` | 3 lines after the match |
| `?pattern?-1` | 1 line before the match |
| `'a+2` | 2 lines below mark `a` |
| `$-5,$` | Last 6 lines of buffer |

## Global / Inverse-Global

| Command | Effect |
|---|---|
| `:[range]g/pattern/cmd` | Execute `cmd` on every line matching `pattern` |
| `:[range]v/pattern/cmd` | Execute `cmd` on every line NOT matching `pattern` |

Default range for `:g` and `:v` is `%` (all lines).

## Common Text-Manipulation Commands with Ranges

| Command | Action |
|---|---|
| `:[range]d [reg]` | Delete lines (optionally into register) |
| `:[range]y [reg]` | Yank lines |
| `:[range]s/pat/rep/[flags]` | Substitute |
| `:[range]copy {addr}` (`:t`) | Copy lines to address |
| `:[range]move {addr}` (`:m`) | Move lines to address |
| `:[range]join` | Join lines |
| `:[range]!{cmd}` | Filter lines through external command |
| `:[range]sort [flags]` | Sort lines |
| `:[range]normal {cmds}` | Execute Normal-mode commands on each line |
| `:[range]print` (`:p`) | Display lines |

## Visual Ranges

After exiting visual mode, `'<,'>` automatically refers to the selected line range. This is pre-filled when typing `:` from visual mode.

## Related

- Full range syntax: [/docs/spec/commands/ranges/ranges.md](/docs/spec/commands/ranges/ranges.md)
- Substitute: [/docs/spec/commands/substitute/README.md](/docs/spec/commands/substitute/README.md)

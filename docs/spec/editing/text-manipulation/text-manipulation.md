# Text Manipulation Commands

Core text manipulation operations.

## Overview

Text manipulation covers the fundamental editing primitives: delete, change, yank, put, join, indent, case conversion, and formatting.

## Delete (normative)

| Command | Action |
|---|---|
| `d{motion}` | Delete text covered by motion |
| `dd` | Delete current line |
| `D` | Delete from cursor to end of line |
| `x` | Delete character under cursor |
| `X` | Delete character before cursor |
| `:[range]d [reg]` | Delete lines in range |

Deleted text is stored in the unnamed register `"` and numbered registers `1`-`9`.

## Change (normative)

| Command | Action |
|---|---|
| `c{motion}` | Delete motion, enter Insert mode |
| `cc` | Change entire line |
| `C` | Change to end of line |
| `s` | Substitute character (delete + insert) |
| `S` | Substitute line (same as `cc`) |

## Yank (normative)

| Command | Action |
|---|---|
| `y{motion}` | Yank (copy) text covered by motion |
| `yy` / `Y` | Yank current line |

Yanked text goes to register `0` (yank register) and `"` (unnamed register).

## Put (normative)

| Command | Action |
|---|---|
| `p` | Put after cursor (characterwise) or below (linewise) |
| `P` | Put before cursor (characterwise) or above (linewise) |
| `gp` | Put after, cursor moves to end of pasted text |
| `gP` | Put before, cursor moves to end of pasted text |
| `]p` | Put with adjusted indentation |

## Indentation (normative)

| Command | Action |
|---|---|
| `>>` | Indent current line by `shiftwidth` |
| `<<` | Unindent current line by `shiftwidth` |
| `>{motion}` | Indent lines covered by motion |
| `={motion}` | Auto-indent lines covered by motion |
| `==` | Auto-indent current line |

## Case Operations (normative)

| Command | Action |
|---|---|
| `~` | Toggle case of character under cursor, advance |
| `g~{motion}` | Toggle case of motion |
| `gu{motion}` | Lowercase motion |
| `gU{motion}` | Uppercase motion |
| `g~~` | Toggle case of line |
| `guu` | Lowercase line |
| `gUU` | Uppercase line |

## Number Increment/Decrement (normative)

| Command | Action |
|---|---|
| `Ctrl-a` | Increment number under/after cursor |
| `Ctrl-x` | Decrement number under/after cursor |
| `{count}Ctrl-a` | Add count to number |

Supported formats: decimal, hex (`0x`), octal (`0`), binary (`0b`). Controlled by `nrformats` option.

## Related

- Operators: [/docs/spec/editing/operators/README.md](/docs/spec/editing/operators/README.md)
- Join/Split: [/docs/spec/editing/text-manipulation/join-split.md](/docs/spec/editing/text-manipulation/join-split.md)
- Registers: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)

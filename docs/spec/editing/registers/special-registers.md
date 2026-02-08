# Special Registers

Back: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)

Read-only and special-purpose registers.

## Overview

Special registers provide access to system state, clipboard, and other dynamic values. Most are read-only.

## Read-Only Registers

| Register | Content |
|---|---|
| `%` | Current file name |
| `#` | Alternate file name |
| `.` | Last inserted text |
| `:` | Last ex command |
| `/` | Last search pattern |

## Clipboard Registers

| Register | Maps to |
|---|---|
| `+` | System clipboard |
| `*` | Primary selection (X11) or clipboard |

## Black Hole Register

`_` — writing to this register discards the text. Useful for deleting without affecting other registers.

## Small Delete Register

`-` — stores the last delete that was less than one line.

## Accessing

| Usage | Context |
|---|---|
| `"{reg}y{motion}` | Yank into register |
| `"{reg}p` | Put from register |
| `<C-r>{reg}` | Insert register content (insert/cmdline mode) |
| `:registers` | List all register contents |

## Related

- Register specification: [/docs/spec/editing/registers/register-specification.md](/docs/spec/editing/registers/register-specification.md)
- Registers overview: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)

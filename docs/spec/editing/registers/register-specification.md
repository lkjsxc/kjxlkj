# Register Specification

Back: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)

Complete specification of all register types and their behavior.

## Overview

Registers store text for yank, delete, put, and macro operations. Each register has a name, type, and content.

## Register Types

| Type | Names | Description |
|---|---|---|
| Unnamed | `"` | Default register for yank/delete |
| Named | `a`-`z` | User-named storage |
| Named (append) | `A`-`Z` | Append to corresponding lowercase register |
| Numbered | `0`-`9` | Auto-filled by yank/delete |
| Small delete | `-` | Deletes less than one line |
| Read-only | `%`, `#`, `.`, `:` | Current file, alternate file, last insert, last command |
| Expression | `=` | Evaluate expression |
| Selection | `*`, `+` | System clipboard |
| Black hole | `_` | Discard (no storage) |
| Last search | `/` | Last search pattern |

## Register Content

Each register stores:

| Field | Description |
|---|---|
| `text` | The text content (string or array of lines) |
| `type` | `char`, `line`, or `block` |

## Named Registers

`"ayy` — yank current line into register `a`.
`"Ayy` — append current line to register `a`.

## System Clipboard

| Register | Maps to |
|---|---|
| `+` | System clipboard |
| `*` | Primary selection (X11) or clipboard |

## Read-only Registers

| Register | Content | Writable |
|---|---|---|
| `%` | Current file name | No |
| `#` | Alternate file name | No |
| `.` | Last inserted text | No |
| `:` | Last ex command | No |

## Related

- Registers overview: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
- Numbered registers: [/docs/spec/editing/registers/numbered-registers.md](/docs/spec/editing/registers/numbered-registers.md)
- Expression register: [/docs/spec/editing/registers/expression-register.md](/docs/spec/editing/registers/expression-register.md)
- Black hole register: [/docs/spec/editing/registers/blackhole-register.md](/docs/spec/editing/registers/blackhole-register.md)

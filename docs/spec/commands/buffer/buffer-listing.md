# Buffer Listing

Displaying and filtering buffers.

## Overview

The `:ls` (`:buffers`, `:files`) command lists open buffers with their status flags.

## Commands (normative)

| Command | Shows |
|---|---|
| `:ls` | All listed buffers |
| `:ls!` | All buffers including unlisted |
| `:filter /pattern/ ls` | Buffers matching pattern |

## Output Format

Each line: `{bufnr} {flags} "{name}" line {lnum}`

Example output:

- `  1 %a   "src/main.rs"          line 42`
- `  2 #h   "src/lib.rs"           line 1`
- `  3  h+  "README.md"            line 15`

## Buffer Flags (normative)

| Flag | Position | Meaning |
|---|---|---|
| `%` | 1 | Current buffer |
| `#` | 1 | Alternate buffer |
| ` ` | 1 | Neither current nor alternate |
| `a` | 2 | Active (loaded and displayed in a window) |
| `h` | 2 | Hidden (loaded but not displayed) |
| ` ` | 2 | Not loaded |
| `+` | 3 | Modified |
| `-` | 3 | Read-only (modifiable off) |
| `=` | 3 | Read-only (file permission) |
| `R` | 4 | Running terminal |
| `F` | 4 | Finished terminal |
| `?` | 4 | Not yet read |
| `u` | special | Unlisted (only shown with `:ls!`) |

## Buffer Types

| `buftype` value | Purpose | Listed by default |
|---|---|---|
| (empty) | Normal file buffer | Yes |
| `nofile` | Scratch / temporary content | No |
| `nowrite` | Cannot be saved | No |
| `help` | Help documentation | No |
| `quickfix` | Quickfix/location list | No |
| `terminal` | Terminal emulator | No |
| `prompt` | Input prompt | No |

## Buffer States

| State | Description |
|---|---|
| Active | Content in memory AND displayed in a window |
| Hidden | Content in memory but NO window shows it |
| Unloaded | In buffer list but content not in memory |
| Wiped | Completely removed from buffer list |

## Navigation from Listing

| Command | Action |
|---|---|
| `:b {N}` | Switch to buffer number N |
| `:b {name}` | Switch by partial name match |
| `:bnext` / `:bn` | Next buffer |
| `:bprev` / `:bp` | Previous buffer |
| `:bfirst` / `:bf` | First buffer |
| `:blast` / `:bl` | Last buffer |

## Related

- Buffer management: [/docs/spec/editor/buffers.md](/docs/spec/editor/buffers.md)
- Advanced buffers: [/docs/spec/features/buffer/buffer-advanced.md](/docs/spec/features/buffer/buffer-advanced.md)

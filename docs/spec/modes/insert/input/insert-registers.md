# Insert Register Access

Inserting register content in insert mode.

## Overview

`Ctrl-r` followed by a register name inserts that register's content at the cursor position without leaving insert mode.

## Key Sequences (normative)

| Key sequence | Behavior |
|---|---|
| `Ctrl-r {reg}` | Insert register contents; interprets mappings, triggers abbreviations, applies autoindent |
| `Ctrl-r Ctrl-r {reg}` | Insert literally; no mapping interpretation, no abbreviation trigger, applies autoindent |
| `Ctrl-r Ctrl-o {reg}` | Insert literally; no autoindent, preserves original formatting |
| `Ctrl-r Ctrl-p {reg}` | Insert literally and fix indent |

## Register Names

| Register | Content |
|---|---|
| `"` | Unnamed (last delete or yank) |
| `0` | Last yank |
| `1`-`9` | Last deletes (1 = most recent) |
| `a`-`z` | Named registers |
| `+` | System clipboard |
| `*` | Primary selection (X11) |
| `/` | Last search pattern |
| `:` | Last command-line command |
| `.` | Last inserted text |
| `%` | Current file name |
| `#` | Alternate file name |
| `-` | Small delete (less than one line) |
| `=` | Expression register (prompts for expression) |

## Expression Register (=)

`Ctrl-r =` opens a prompt in the command line. The user types an expression, presses Enter, and the result is inserted. This enables computed insertions (e.g., arithmetic, string manipulation).

## Multi-line Content

- Linewise register: text is inserted as new lines above/below the cursor line depending on context.
- Blockwise register: text is inserted starting at the cursor column, with each block line on a successive screen line.
- Characterwise register: text is inserted inline at the cursor position.

## Related

- Registers: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)

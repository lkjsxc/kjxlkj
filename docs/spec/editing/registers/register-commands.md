# Register Commands

Back: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)

Commands for interacting with registers.

## Overview

Registers can be viewed, pasted from, yanked into, and executed as macros.

## Viewing Registers

| Command | Description |
|---|---|
| `:registers` | Show all register contents |
| `:registers {names}` | Show specific registers |
| `:display` | Alias for `:registers` |

## Pasting Registers

| Command | Mode | Description |
|---|---|---|
| `"{reg}p` | Normal | Paste after cursor from register |
| `"{reg}P` | Normal | Paste before cursor from register |
| `<C-r>{reg}` | Insert/Cmdline | Insert register contents |

## Setting Registers

| Method | Description |
|---|---|
| `"{reg}y{motion}` | Yank into register |
| `"{reg}d{motion}` | Delete into register |
| `:let @{reg} = "text"` | Set register content directly |

## Executing Registers

| Command | Description |
|---|---|
| `@{reg}` | Execute register as macro |
| `@@` | Re-execute last macro register |

## Related

- Registers: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
- Register specification: [/docs/spec/editing/registers/register-specification.md](/docs/spec/editing/registers/register-specification.md)

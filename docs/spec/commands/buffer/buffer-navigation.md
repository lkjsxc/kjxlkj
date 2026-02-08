# Buffer Navigation

Back: [/docs/spec/commands/buffer/README.md](/docs/spec/commands/buffer/README.md)

Commands for switching between buffers.

## Overview

Buffer navigation commands switch the current window to display a different buffer.

## Commands

| Command | Key | Description |
|---|---|---|
| `:bnext` | `]b` | Switch to next buffer |
| `:bprev` | `[b` | Switch to previous buffer |
| `:bfirst` | - | Switch to first buffer |
| `:blast` | - | Switch to last buffer |
| `:buffer {N}` | - | Switch to buffer number N |
| `:buffer {name}` | - | Switch to buffer matching name |
| `<C-^>` / `<C-6>` | - | Toggle alternate file |

## Buffer Picker

| Key | Command | Description |
|---|---|---|
| `<leader>fb` | `:Buffers` | Open buffer picker |

## Modified Buffer Warning

If the current buffer is modified, `:bnext` etc. will warn. Use `!` to force, or set `hidden` to allow background modified buffers.

| Setting | Default | Description |
|---|---|---|
| `hidden` | `true` | Allow switching away from modified buffers |

## Related

- Buffer management: [/docs/spec/commands/buffer/README.md](/docs/spec/commands/buffer/README.md)
- Alternate file: [/docs/spec/features/buffer/alternate-file.md](/docs/spec/features/buffer/alternate-file.md)
- Buffer list: [/docs/spec/features/buffer/buffer-list.md](/docs/spec/features/buffer/buffer-list.md)

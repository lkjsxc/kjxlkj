# Buffer Groups

Back: [/docs/spec/features/buffer/README.md](/docs/spec/features/buffer/README.md)

Organize buffers into logical groups for easier navigation.

## Automatic groups (normative)

The editor MAY automatically group buffers by:

| Strategy | Grouping key |
|---|---|
| Directory | Parent directory of the file path |
| Filetype | Detected filetype (e.g., rust, typescript, markdown) |
| Project root | Nearest directory containing a project marker (`.git`, `Cargo.toml`, `package.json`) |

Automatic grouping is enabled with `buffer.auto_group` option (default: `"directory"`).

## Manual groups

| Command | Action |
|---|---|
| `:BufferGroup {name}` | Add the current buffer to group `{name}` |
| `:BufferUngroup {name}` | Remove the current buffer from group `{name}` |
| `:BufferGroups` | List all groups and their buffers |
| `:BufferGroupClose {name}` | Close all buffers in group `{name}` |

## Group navigation

| Command / Key | Action |
|---|---|
| `:bnext {group}` | Next buffer within the specified group |
| `:bprev {group}` | Previous buffer within the specified group |
| `<leader>bg` | Open group picker (fuzzy finder filtered by group) |

## Bufferline integration

When the bufferline is visible, groups are rendered as separate sections with the group name as a separator label. The active group may be highlighted.

## Related

- Buffer switching: [/docs/spec/features/buffer/buffer-switching.md](/docs/spec/features/buffer/buffer-switching.md)
- Bufferline: [/docs/spec/features/buffer/bufferline.md](/docs/spec/features/buffer/bufferline.md)


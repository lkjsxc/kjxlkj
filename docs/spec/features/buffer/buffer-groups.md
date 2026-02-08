# Buffer Groups

Back: [docs/spec/features/buffer/README.md](/docs/spec/features/buffer/README.md)

Organize buffers into logical groups for navigation.

## Overview

Buffer groups allow organizing open buffers into
named sets. Groups can be created automatically
based on file properties or manually by the user.

## Automatic Grouping

### Strategies

| Strategy | Grouping Key |
|----------|-------------|
| `directory` | Parent directory of file path |
| `filetype` | Detected file type |
| `project` | Nearest project root marker |
| `none` | Disable automatic grouping |

### Configuration

Set `buffer.auto_group` to choose the strategy.
Default: `"directory"`.

### Project Root Detection

Project roots are detected by scanning upward for:
- `.git` directory
- `Cargo.toml`
- `package.json`
- `pyproject.toml`
- `.project-root` marker file

## Manual Groups

### Commands

| Command | Action |
|---------|--------|
| `:BufferGroup {name}` | Add current buffer to group |
| `:BufferUngroup {name}` | Remove from group |
| `:BufferGroups` | List all groups and members |
| `:BufferGroupClose {name}` | Close all in group |
| `:BufferGroupRename {old} {new}` | Rename a group |

### Multiple Membership

A buffer can belong to multiple groups. When using
both automatic and manual grouping, the buffer
appears in its auto-group and any manual groups.

## Group Navigation

### Within-Group Movement

| Command | Action |
|---------|--------|
| `:bnext {group}` | Next buffer in group |
| `:bprev {group}` | Previous buffer in group |

### Group Picker

`<leader>bg` opens the finder filtered by group.
The picker shows group names; selecting a group
shows its buffers as a second-level list.

### Group Cycling

| Key | Action |
|-----|--------|
| `<leader>bn` | Next group |
| `<leader>bp` | Previous group |

## Bufferline Integration

### Visual Separation

Groups appear as labeled sections in the bufferline.
Each section has a header showing the group name.
The active group section is highlighted.

### Tab Order

Within each group, tabs follow the configured sort
order (insertion, name, etc.). Groups themselves
are sorted alphabetically by name.

## Session Persistence

### Saving Groups

Manual group assignments are saved in the session
JSON file under the `buffer_groups` key.

### Restoring Groups

On session restore, manual groups are re-applied
to buffers that match by file path. Automatic
groups are regenerated from file properties.

## Data Model

### Group Structure

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Group identifier |
| `strategy` | enum | auto/manual origin |
| `buffers` | Vec<BufferId> | Member buffer IDs |

## Related

- Bufferline: [docs/spec/features/buffer/bufferline.md](/docs/spec/features/buffer/bufferline.md)
- Buffer switching: [docs/spec/features/buffer/buffer-switching.md](/docs/spec/features/buffer/buffer-switching.md)
- Session: [docs/spec/features/session/session-management.md](/docs/spec/features/session/sessions.md)

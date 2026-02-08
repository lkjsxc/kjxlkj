# Buffer Groups

Back: [/docs/spec/features/buffer/README.md](/docs/spec/features/buffer/README.md)

Organize buffers into named groups.

## Overview

Buffer groups allow categorizing buffers for focused navigation. Only buffers in the active group are shown in buffer lists and tab lines.

## Commands

| Command | Description |
|---|---|
| `:BufferGroup {name}` | Switch to buffer group |
| `:BufferGroupAdd {name}` | Add current buffer to group |
| `:BufferGroupRemove {name}` | Remove current buffer from group |
| `:BufferGroupCreate {name}` | Create a new group |
| `:BufferGroupDelete {name}` | Delete a group |

## Default Group

All buffers belong to the default group. Custom groups overlay this.

## Navigation

When a group is active, `:bnext` / `:bprev` cycle only within the group.

## Configuration

| Setting | Default | Description |
|---|---|---|
| `buffergroups.tabline` | `true` | Show group selector in tabline |

## Related

- Buffer list: [/docs/spec/commands/buffer/buffer-listing.md](/docs/spec/commands/buffer/buffer-listing.md)
- Buffer navigation: [/docs/spec/commands/buffer/buffer-navigation.md](/docs/spec/commands/buffer/buffer-navigation.md)

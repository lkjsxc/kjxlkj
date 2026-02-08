# Jump List

Back: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)

Navigate through a history of cursor jump positions.

## Overview

The jump list records cursor positions before large jumps. It allows navigating backward and forward through these positions.

## Navigation

| Key | Command | Description |
|---|---|---|
| `<C-o>` | `:jumps` | Jump to older position |
| `<C-i>` | - | Jump to newer position |
| `:jumps` | - | Display the jump list |

## What Creates a Jump Entry

| Action | Creates Jump |
|---|---|
| `/` search | Yes |
| `?` search | Yes |
| `n` / `N` | Yes |
| `*` / `#` | Yes |
| `G` / `gg` | Yes |
| `%` | Yes |
| `(` / `)` | Yes |
| `{` / `}` | Yes |
| `H` / `M` / `L` | Yes |
| `:number` (go to line) | Yes |
| `` `{mark} `` | Yes |
| Regular motions (h/j/k/l/w/e/b) | No |

## Cross-file Jumps

Jump list entries include the file path. Jumping back with `<C-o>` can switch buffers.

## List Size

| Setting | Default | Description |
|---|---|---|
| `jumplist_size` | `100` | Maximum jump list entries |

When the list is full, the oldest entry is removed.

## Session Persistence

The jump list is saved in the session file.

## Related

- Marks: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)
- Change list: [/docs/spec/editing/marks/changelist.md](/docs/spec/editing/marks/changelist.md)

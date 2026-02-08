# Git Signs

Back: [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)

Sign column indicators for Git changes.

## Overview

Git signs display in the sign column to show which lines have been added, modified, or deleted compared to the Git index (staged state) or HEAD.

## Sign Types

| Sign | Highlight | Meaning |
|---|---|---|
| `│` | `GitSignsAdd` | Added lines |
| `│` | `GitSignsChange` | Changed lines |
| `_` | `GitSignsDelete` | Deleted lines (below the sign) |
| `‾` | `GitSignsTopDelete` | Deleted lines (above the sign) |
| `~` | `GitSignsChangedelete` | Changed + deleted |

## Navigation

| Key | Command | Description |
|---|---|---|
| `]h` | `:Gitsigns next_hunk` | Jump to next hunk |
| `[h` | `:Gitsigns prev_hunk` | Jump to previous hunk |

## Hunk Operations

| Key | Command | Description |
|---|---|---|
| `<leader>hs` | Stage hunk | Stage the hunk under cursor |
| `<leader>hr` | Reset hunk | Revert hunk to index state |
| `<leader>hp` | Preview hunk | Show hunk diff in popup |
| `<leader>hS` | Stage buffer | Stage all changes in buffer |

## Configuration

| Setting | Default | Description |
|---|---|---|
| `gitsigns.enabled` | `true` | Enable git signs |
| `gitsigns.base` | `"index"` | Compare against: `index` or `HEAD` |

## Related

- Git integration: [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)
- Diff mode: [/docs/spec/features/git/diff-mode.md](/docs/spec/features/git/diff-mode.md)

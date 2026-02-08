# Command-Line History

Command and search history management.

## Overview

Each command-line type maintains its own history. History entries are navigable via arrow keys and persist across sessions.

## History Types (normative)

| Type | Prompt | Content |
|---|---|---|
| Command | `:` | Ex commands |
| Search | `/` or `?` | Search patterns |
| Expression | `=` | Expression register inputs |
| Input | `@` | Generic input prompts |

## Navigation (normative)

| Key | Action |
|---|---|
| `Up` | Previous entry (older) |
| `Down` | Next entry (newer) |
| `Ctrl-p` | Previous entry |
| `Ctrl-n` | Next entry |

### Prefix Filtering

If text has been typed before pressing Up/Down, only history entries starting with that prefix are shown. This enables efficient recall of similar commands.

## History Size

The `history` option controls how many entries are stored per type. Default: 1000.

## Persistence (normative)

History is saved to a file on normal exit and loaded on startup:

| Setting | Description |
|---|---|
| Location | `~/.local/share/kjxlkj/history` |
| Format | One entry per line, grouped by type |
| Load | On startup, history file is read |
| Save | On normal exit, history file is written |

## Command-Line Window (normative)

The command-line window provides a full buffer-editing view of history:

| Key | Opens |
|---|---|
| `q:` | Command history window |
| `q/` | Forward search history window |
| `q?` | Backward search history window |
| `Ctrl-f` (from cmdline) | Switch current cmdline into window |

Features of the command-line window:

- Full Normal-mode editing of history entries.
- Navigate with `j`/`k`, standard motions.
- Press `Enter` on a line to execute that entry.
- Press `Ctrl-c` or `:q` to cancel and close.
- New entries can be typed at the bottom of the buffer.
- Edits within the window do NOT modify the actual history until executed.

## Re-Execute

| Command | Action |
|---|---|
| `@:` | Re-execute last Ex command |
| `n` / `N` | Repeat last search (not from history, from search state) |

## Related

- Command-line entry: [/docs/spec/commands/cmdline/cmdline-entry.md](/docs/spec/commands/cmdline/cmdline-entry.md)
- Command-line editing: [/docs/spec/commands/cmdline/cmdline-editing.md](/docs/spec/commands/cmdline/cmdline-editing.md)

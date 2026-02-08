# Recent Files

Back: [/docs/spec/commands/session/README.md](/docs/spec/commands/session/README.md)

Track and navigate to recently opened files.

## Overview

The editor maintains a list of recently opened files for quick access.

## Commands

| Key | Command | Description |
|---|---|---|
| `<leader>fr` | `:RecentFiles` | Open recent files picker |

## Storage

Recent files are stored in the session data directory. The list persists across editor restarts.

| Setting | Default | Description |
|---|---|---|
| `recent_files.max` | `100` | Maximum entries |

## Finder Integration

The recent files picker uses the same fuzzy finder interface as file finding.

## Related

- Session: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)
- Finder: [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)

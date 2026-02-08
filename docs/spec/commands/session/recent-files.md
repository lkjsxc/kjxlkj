# Recent Files

Back: [/docs/spec/commands/session/README.md](/docs/spec/commands/session/README.md)

Track and access recently opened files.

## Storage (normative)

Recent files are persisted to `~/.local/share/kjxlkj/recent_files.json`. The file contains an ordered array of absolute file paths, most-recent first.

| Setting | Default | Description |
|---|---|---|
| `recent.max_entries` | `100` | Maximum number of entries |
| `recent.exclude_patterns` | `[]` | Glob patterns to exclude (e.g., `"/tmp/*"`) |

## Commands (normative)

| Command | Action |
|---|---|
| `:RecentFiles` | Open the recent files picker |
| `:browse oldfiles` | List recent files (Vim-compatible) |

## Lifecycle (normative)

| Event | Action |
|---|---|
| File opened (`:e`, `:o`) | Prepend to recent list (move to front if already present) |
| On startup | Validate entries: remove files that no longer exist |
| On exit | Persist the list to disk |

## Oldfiles register

The `v:oldfiles` variable contains the list of recent file paths. It is read-only.

## Related

- Finder: [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
- Session management: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)

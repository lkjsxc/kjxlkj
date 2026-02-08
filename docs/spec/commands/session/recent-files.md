# Recent Files

Back: [docs/spec/commands/session/README.md](docs/spec/commands/session/README.md)

Track and navigate recently opened files.

## Overview

The editor maintains a list of recently opened files
across sessions. This enables quick re-opening of
previously edited files through a picker interface
or ex commands.

## Storage

### File Location

Recent files are stored in the data directory at
`~/.local/share/kjxlkj/recent_files.json`.

### Data Format

Each entry stores:

| Field | Type | Description |
|-------|------|-------------|
| `path` | String | Absolute file path |
| `timestamp` | u64 | Unix epoch of last open |
| `line` | u32 | Last cursor line position |
| `col` | u32 | Last cursor column position |

### Capacity

Maximum entries: 1000 (configurable via
`recent_files.max_entries`). When full, the oldest
entry is evicted.

## Commands

### Browse Recent Files

`:RecentFiles` opens the finder with the recent
files list, sorted by most recently opened.

### Open by Number

`:recent {n}` opens the nth most recent file.
`:recent 1` opens the most recently closed file.

### Clear History

`:RecentFilesClear` removes all recent file entries.

## Finder Integration

### Display

Each entry in the finder shows:
- Relative path from workspace root (or absolute)
- Last modification timestamp
- File icon based on extension

### Filtering

The finder supports fuzzy matching on file paths.
Type partial path components to narrow results.

### Preview

Selected entry shows file content preview in the
right pane (if preview is enabled in finder config).

## Automatic Tracking

### When Files Are Added

Files are added to the history when:
- A buffer is opened with `:edit`
- A file is opened via the finder
- A file is opened via `gf` or similar navigation
- The argument list adds a file

### When Entries Update

Existing entries update their timestamp and cursor
position when the file is re-opened or saved.

### Exclusions

Files matching these patterns are excluded:
- Temporary files (`/tmp/*`)
- Swap and backup files
- Files matching `recent_files.exclude` patterns
- Buffers that were never written (unnamed)

## Session Integration

### Startup Behavior

On startup, recent file positions are used to
restore cursor position when a file is re-opened.

### Cross-Session

The recent files list persists across editor
sessions. Multiple concurrent editor instances
merge their entries on write using last-write-wins.

## Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `recent_files.max_entries` | int | 1000 | Max items |
| `recent_files.exclude` | list | [] | Glob patterns |
| `recent_files.restore_cursor` | bool | true | Restore position |

## Related

- Session management: [docs/spec/features/session/README.md](docs/spec/features/session/README.md)
- Finder: [docs/spec/features/navigation/finder.md](docs/spec/features/navigation/finder.md)

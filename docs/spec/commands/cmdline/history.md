# Command History

Back: [docs/spec/commands/cmdline/README.md](/docs/spec/commands/cmdline/README.md)

Persistent history for ex commands and searches.

## Overview

The editor maintains separate histories for ex
commands, search patterns, and expressions. History
is navigable, persistent across sessions, and
searchable.

## History Types

| Type | Content | Access |
|------|---------|--------|
| Command | Ex commands (`:...`) | `:` then Up/Down |
| Search | Search patterns (`/...`) | `/` then Up/Down |
| Expression | Expression register | Ctrl-r `=` |
| Input | User input prompts | Automatic |

## Navigation

Browsing through history entries.

### Arrow Keys

| Key | Action |
|-----|--------|
| `Up` | Previous history entry |
| `Down` | Next history entry |
| `Ctrl-p` | Previous (same as Up) |
| `Ctrl-n` | Next (same as Down) |

### Prefix Filtering

When text has been typed, Up/Down navigate only
entries matching the typed prefix. For example:
typing `:set` then pressing Up shows only previous
`:set...` commands.

## History Window

A full-screen buffer for browsing and editing history.

### Opening

`q:` opens the command history window showing all
previous commands in a buffer. Normal editing
commands work in this window.

`q/` opens the search history window.

### Editing in History Window

Users can edit history entries before executing.
Press `Enter` on a line to execute it as a command.
Press `Ctrl-c` to close without executing.

## Storage

How history is persisted across sessions.

### File Location

History is persisted to
`~/.local/share/kjxlkj/history.json`.

### Capacity

| History Type | Default Max Entries |
|-------------|-------------------|
| Command | 1000 |
| Search | 1000 |
| Expression | 100 |
| Input | 100 |

Configurable via `history` option (applies globally).

### Deduplication

Duplicate entries are removed. When a command is
entered that already exists in history, the old
entry is removed and the new one is added at the
top (most recent position).

### Session Persistence

History is written to disk on:
- Normal exit (`:quit`, `:wq`)
- Auto-save interval (every 60 seconds)
- Explicit `:wshada` command

History is loaded on startup from the file.

## History Commands

| Command | Action |
|---------|--------|
| `:history` | Show command history |
| `:history search` | Show search history |
| `:history expr` | Show expression history |
| `:history all` | Show all histories |
| `:history {n}` | Show last n entries |

## Concurrent Instances

Multiple editor instances share the history file.
On write, the editor merges its in-memory history
with the file using timestamp-based deduplication.

## Privacy

Excluding sensitive commands from history.

### Sensitive Commands

Commands containing sensitive information can be
excluded from history by prefixing with a space
(when `histignore` option includes the space flag).

## Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `history` | int | 1000 | Max entries per type |
| `histignore` | string | "" | Patterns to exclude |

## Related

- Command-line mode: [docs/spec/modes/cmdline/README.md](/docs/spec/modes/command.md)
- Completion: [docs/spec/commands/cmdline/completion.md](/docs/spec/commands/cmdline/completion.md)
- Session: [docs/spec/features/session/README.md](/docs/spec/features/session/README.md)

# Command history
Command history is core-owned state exposed via the command line and list UI.

## Requirements
- History updates are transactional.
- History browsing never blocks on disk.
- Optional persistence is versioned.

## History types (normative)

| Type | Content | Max entries (default) |
|---|---|---|
| Command | Ex commands entered with `:` | 1000 |
| Search | Search patterns entered with `/` and `?` | 1000 |
| Expression | Expressions entered with `=` | 1000 |
| Input | Responses to `:input` prompts | 1000 |

## Navigation keys (normative)

| Key | Action | Context |
|---|---|---|
| `Up` | Previous history entry | Command-line mode |
| `Down` | Next history entry | Command-line mode |
| `Ctrl-p` | Previous history entry (alternative) | Command-line mode |
| `Ctrl-n` | Next history entry (alternative) | Command-line mode |

When navigating with a partial command already typed, only entries starting with that prefix are shown.

## History windows (normative)

| Key | Opens | Content |
|---|---|---|
| `q:` | Command history window | All ex-command history in a buffer |
| `q/` | Search history window | Forward search history in a buffer |
| `q?` | Search history window | Backward search history in a buffer |

The history window is a regular buffer window. The user can navigate, search, and edit entries. Pressing `Enter` on a line executes that command or search. Pressing `Esc` or `:q` closes the history window.

## Deduplication (normative)

When a command is executed that already exists in history, the old entry is removed and the command is added at the most recent position. History MUST NOT contain consecutive duplicates.

## Persistence (normative)

| Aspect | Specification |
|---|---|
| File location | `~/.local/share/kjxlkj/history` |
| Format | Line-oriented text; one entry per line, prefixed with type tag |
| Save trigger | On graceful shutdown and periodically (every 60 seconds if dirty) |
| Load | On startup, merged with any existing in-memory entries |
| Permissions | File MUST be created with `0600` permissions |

## Related

- Command-line completion: [/docs/spec/commands/cmdline/completion.md](/docs/spec/commands/cmdline/completion.md)
- Session persistence: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)

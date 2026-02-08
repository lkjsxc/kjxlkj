# Command-Line Entry

Back: [/docs/spec/commands/cmdline/README.md](/docs/spec/commands/cmdline/README.md)

Specification for entering and using the command line (ex mode, search prompt).

## Entry triggers

| Key | Mode | Command-line type | Prompt character |
|---|---|---|---|
| `:` | Normal | Ex command | `:` |
| `/` | Normal | Forward search | `/` |
| `?` | Normal | Backward search | `?` |

## Command-line state model

| Field | Type | Description |
|---|---|---|
| `prompt` | char | The prompt character (`:`, `/`, or `?`) |
| `content` | String | The command text entered so far |
| `cursor_col` | usize | Cursor position within `content` (0-based) |
| `completions` | list | Active completion candidates |
| `selected` | Option index | Currently highlighted completion |
| `history_index` | Option index | Current position in history navigation |

## Key handling in command-line mode (normative)

| Key | Action |
|---|---|
| printable chars | Insert at cursor position, advance cursor |
| `Enter` | Execute command/search and return to Normal |
| `Esc` | Cancel and return to Normal |
| `Ctrl-c` | Cancel and return to Normal |
| `Backspace` | Delete character before cursor; if empty, cancel |
| `Delete` | Delete character at cursor |
| `Left` | Move cursor left |
| `Right` | Move cursor right |
| `Home` / `Ctrl-b` | Move cursor to start of line |
| `End` / `Ctrl-e` | Move cursor to end of line |
| `Ctrl-w` | Delete word before cursor |
| `Ctrl-u` | Delete from cursor to start of line |
| `Up` | Previous history entry |
| `Down` | Next history entry |
| `Tab` | Trigger completion |
| `Shift-Tab` | Previous completion |
| `Ctrl-r {reg}` | Insert register contents |

## Incremental search

When the command-line type is search (`/` or `?`):

| Behavior | Requirement |
|---|---|
| `incsearch` | If `incsearch` option is true, the buffer view MUST highlight and scroll to the first match as the pattern is typed. |
| Pattern update | On each character typed, recompile the regex and update highlights. |
| Invalid pattern | If the pattern is invalid regex, show no highlights (do not show an error until Enter). |
| Empty pattern | Reuse the previous search pattern. |

## Execution

| Type | Action on Enter |
|---|---|
| Ex (`:`) | Parse the command text per [/docs/spec/commands/syntax.md](/docs/spec/commands/syntax.md) and dispatch |
| Search (`/`) | Set search pattern, jump to next match forward |
| Search (`?`) | Set search pattern, jump to next match backward |

## Error display

When a command fails:

- The error message MUST be displayed in the command-line area.
- The message MUST persist until the next user action.
- Error format: `E{number}: {message}` (Vim-compatible error codes where applicable).

## Related

- Command-line editing: [/docs/spec/commands/cmdline/cmdline-editing.md](/docs/spec/commands/cmdline/cmdline-editing.md)
- History: [/docs/spec/commands/cmdline/cmdline-history.md](/docs/spec/commands/cmdline/cmdline-history.md)
- Completion: [/docs/spec/commands/cmdline/completion.md](/docs/spec/commands/cmdline/completion.md)
- Command syntax: [/docs/spec/commands/syntax.md](/docs/spec/commands/syntax.md)

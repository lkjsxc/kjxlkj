# Command-Line Window

Editing commands in a full buffer.

## Overview

The command-line window is a special buffer that displays command or search history, allowing full Normal-mode editing before execution.

## Opening (normative)

| Key | From | Window content |
|---|---|---|
| `q:` | Normal mode | Command history |
| `q/` | Normal mode | Forward search history |
| `q?` | Normal mode | Backward search history |
| `Ctrl-f` | Command-line mode | Converts current cmdline to cmdline window |

## Window Properties

| Property | Value |
|---|---|
| Position | Bottom of screen |
| Height | Configurable via `cmdwinheight` option (default 7) |
| Buffer type | `nofile` |
| Modifiable | Yes (editable like a normal buffer) |
| Saved | Never (content is lost on close) |
| Listed | No (unlisted buffer) |

## Editing

The command-line window supports full Normal-mode editing:

- Navigate with `j`, `k`, motions, search, etc.
- Edit any line (modify existing history entries or type new commands).
- Standard insert mode works for adding/modifying text.

## Execution (normative)

| Key | Action |
|---|---|
| `Enter` (Normal mode) | Execute the command on the current line and close the window |
| `Ctrl-c` | Close the window without executing |
| `:q` | Close the window without executing |

## History Integration

- The window displays history entries in chronological order (oldest at top).
- The last line is empty, ready for new input.
- Editing a line in the window does NOT modify the actual history until that line is executed.
- After execution, the executed command IS added to history.

## Restrictions

While the command-line window is open:

- Cannot open another command-line window.
- Cannot switch to other windows (some implementations allow it; kjxlkj does not).
- Buffer-switching commands are restricted.

## Related

- Command-line history: [/docs/spec/commands/cmdline/cmdline-history.md](/docs/spec/commands/cmdline/cmdline-history.md)
- Command-line entry: [/docs/spec/commands/cmdline/cmdline-entry.md](/docs/spec/commands/cmdline/cmdline-entry.md)

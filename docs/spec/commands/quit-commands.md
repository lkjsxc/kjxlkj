# Quit Commands

Exiting the editor.

## Overview

Quit commands close buffers, windows, and the editor.
They enforce save-guards to prevent data loss.

## Basic Commands

### Quit Current

`:q` closes the current window. If it is the last window
showing a buffer, the buffer is unloaded. If it is the
last window in the editor, the editor exits.

### Force Quit

`:q!` closes the current window discarding any unsaved
changes in the buffer. If it is the last window, exits
without saving.

## Normal Mode

### Quick Quit

`ZZ` saves the current buffer (if modified) and closes
the window. Equivalent to `:x`.
`ZQ` closes without saving. Equivalent to `:q!`.

## Quit All

### All Windows

`:qa` closes all windows and exits the editor.
Fails if any buffer has unsaved changes.

### Force All

`:qa!` closes all windows, discards all unsaved changes,
and exits immediately.

## Write and Quit

### Combined

`:wq` writes the current buffer and closes the window.
Always writes even if the buffer was not modified.

`:x` writes only if the buffer was modified, then closes.
`ZZ` is equivalent to `:x`.

| Command | Writes | Condition |
|---------|--------|-----------|
| `:wq` | Always | Unconditional |
| `:x` | If modified | Only when buffer changed |
| `ZZ` | If modified | Same as `:x` |

### All Buffers

`:wqa` or `:xa` writes all modified buffers and exits.

## Window Close

### Close Window

`:close` or `<C-w>c` closes the current window.
Fails if it is the last window (use `:q` instead).

### Close Others

`:only` or `<C-w>o` closes all windows except the current.
Fails if any other window has unsaved changes.
`:only!` forces close of others.

## Buffer Commands

### Unload Buffer

`:bunload` unloads the buffer from memory but keeps
it in the buffer list. Fails if buffer has unsaved changes.

### Delete Buffer

`:bdelete` or `:bd` removes the buffer from the buffer list
and unloads it. The window switches to the alternate buffer
or the next buffer. Fails if unsaved.

### Wipe Buffer

`:bwipeout` or `:bw` completely removes the buffer, its marks,
undo history, and options. More thorough than `:bdelete`.

## Confirmation

### Unsaved Changes

When a quit command encounters unsaved changes, it displays:
"No write since last change (add ! to override)". The user must
either save first (`:w`) or force (`:q!`).

Options:
- `:w` then `:q` - Save first
- `:q!` - Discard changes
- `:wq` - Save and quit

### All Unsaved

`:qa` with multiple unsaved buffers lists all unsaved buffer
names in the error message.

## Hidden Buffers

### With Changes

When hidden buffers have unsaved changes, `:qa` fails even
though they are not visible in any window. The error lists
the hidden buffer names.

### Force Close

`:qa!` closes all hidden buffers regardless. `:bdelete!`
forces a single hidden buffer closed.

## Exit Code

### Normal Exit

`:q`, `:qa`, `:wq`, `:x` all exit with code `0`.

### Error Exit

`:cq` or `:cquit` exits with a non-zero exit code (default `1`).
`:cq {code}` exits with the specified code.
Useful for aborting git commit messages or similar workflows.

## Auto Commands

### On Quit

The `QuitPre` event fires before a quit command processes.
Autocommands can inspect or modify state before exit.

### Confirm

Plugins can use `QuitPre` to show a confirmation dialog
or perform cleanup before the editor exits.

## Conditional Quit

### Check Modified

`:confirm q` shows a confirmation dialog if the buffer is
modified, offering Save / Discard / Cancel options.

### All Saved

`:confirm qa` shows confirmation for each unsaved buffer.

## Session Save

### Before Quit

When `auto_save_session = true`, the session file is
automatically updated before the editor exits via `:qa` or `:wqa`.
`:qa!` also saves the session (session data is separate from
buffer content).

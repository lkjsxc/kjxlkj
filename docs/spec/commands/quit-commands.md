# Quit Commands

Back: [/docs/spec/commands/README.md](/docs/spec/commands/README.md)

Commands for closing windows, tabs, and exiting the editor.

## Window close

| Command | Description |
|---|---|
| `:q[uit]` | Close current window. If last window, exit editor. Errors if buffer has unsaved changes. |
| `:q[uit]!` | Force close current window, discarding unsaved changes |
| `:clo[se]` | Close current window. Does not exit if it is the last window. |
| `:hid[e]` | Close current window, keep buffer loaded |

## Close all

| Command | Description |
|---|---|
| `:qa[ll]` | Close all windows and exit. Errors if any buffer has unsaved changes. |
| `:qa[ll]!` | Force close all windows and exit, discarding all unsaved changes |
| `:wqa[ll]` / `:xa[ll]` | Write all modified buffers and exit |

## Write and quit

| Command | Description |
|---|---|
| `:wq` | Write current buffer and close window |
| `:wq!` | Force write and close |
| `:x[it]` | Write if modified, then close window |

## Error exit

| Command | Description |
|---|---|
| `:cq[uit]` | Quit with non-zero exit code. Used to abort `git rebase -i` and similar external processes. |

## Close behavior

When closing the last window showing a buffer:

| Scenario | Behavior |
|---|---|
| Buffer is modified, no `!` | Error: "No write since last change" |
| Buffer is modified, with `!` | Buffer changes are discarded |
| Buffer is unmodified | Buffer is unloaded (default) or kept in background |
| Last window in editor | Exit the editor process |

## Modified buffer guard

Setting `hidden = true` allows closing windows without saving, keeping modified buffers in the background buffer list. This is the default behavior.

| Setting | Default | Description |
|---|---|---|
| `hidden` | `true` | Allow hiding modified buffers |

## Exit sequence

When the editor exits:

1. Fire `EditorExit` autocommand.
2. Save session if `session.auto_save = true`.
3. Save persistent undo for all buffers.
4. Close all terminal PTY processes.
5. Restore terminal state.
6. Exit process.

## Related

- Write commands: [/docs/spec/commands/file/write-commands.md](/docs/spec/commands/file/write-commands.md)
- Session: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
- Essential commands: [/docs/spec/commands/essential.md](/docs/spec/commands/essential.md)

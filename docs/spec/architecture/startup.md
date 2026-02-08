# Startup Sequence

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

This document specifies the exact initialization order when the `kjxlkj` binary starts. An implementor MUST follow this sequence precisely.

## Entry point

The `main` function in the `kjxlkj` binary crate performs these steps in order:

| Step | Action | Failure behavior |
|---|---|---|
| 1 | Parse command-line arguments (file paths, flags) | Print usage and exit 1 |
| 2 | Initialize Tokio runtime (multi-thread, default thread count) | Panic (unrecoverable) |
| 3 | Enter the async `run()` function on the Tokio runtime | Propagate error to exit code |

## Async initialization (`run`)

Inside the async entry, these steps execute sequentially:

| Step | Action | Detail |
|---|---|---|
| 1 | Detect terminal capabilities | Query `$COLORTERM`, `$TERM`, terminfo for color depth (true color, 256, 16). Detect kitty keyboard protocol support. |
| 2 | Load configuration | Read `~/.config/kjxlkj/config.toml` per [/docs/spec/features/session/project-config.md](/docs/spec/features/session/project-config.md). Merge project-local `.kjxlkj.toml` if present. Apply system defaults for missing keys. |
| 3 | Load theme | Resolve `colorscheme` option to a theme file under `~/.config/kjxlkj/themes/`. Fall back to built-in default theme if not found. |
| 4 | Initialize editor state | Create `EditorState` with empty buffer list, default window layout (single leaf window), mode set to `Normal`, and config applied. |
| 5 | Process file arguments | For each file path in CLI args, open the file into a buffer. If no files given, create one empty scratch buffer. The first buffer is displayed in the initial window. |
| 6 | Create channels | Allocate bounded mpsc channels: `input_tx/rx` (256), `service_request_tx/rx` (128), `service_response_tx/rx` (128). Create `watch` channel for snapshots. |
| 7 | Enter raw mode | Call `crossterm::terminal::enable_raw_mode()`. Register cleanup hook for panic and normal exit. |
| 8 | Set up terminal | Write to stdout: enable alternate screen (`CSI ?1049h`), enable bracketed paste (`CSI ?2004h`), hide cursor initially, enable focus reporting (`CSI ?1004h`), enable kitty keyboard protocol if supported. |
| 9 | Spawn input task | Start the input reader Tokio task that reads `crossterm::event::EventStream` and sends to `input_tx`. |
| 10 | Spawn service tasks | Start service tasks: file system watcher, git status (if `.git` exists), LSP servers (per filetype config). Each service receives `service_request_rx` clone and sends to `service_response_tx`. |
| 11 | Spawn render task | Start the render task that watches the snapshot `watch::Receiver` and writes frames to stdout. Perform initial full-screen render. |
| 12 | Auto-restore session | If `session.auto_restore` is true and an auto-session file exists for the cwd, load it per [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md). This replaces the default single-window layout. |
| 13 | Source init file | Execute `~/.config/kjxlkj/init.kjxlkj` if it exists (ex commands, mappings, autocommands). |
| 14 | Enter core event loop | Enter the main select loop per [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md). |

## Shutdown sequence

When the editor exits (`:q`, `:qa`, or signal):

| Step | Action |
|---|---|
| 1 | Auto-save session if `session.auto_save` is true |
| 2 | Send shutdown signal to all service tasks |
| 3 | Wait for service tasks to complete (bounded timeout: 2 seconds) |
| 4 | Cancel input and render tasks |
| 5 | Restore terminal: disable kitty keyboard protocol, disable focus reporting, disable bracketed paste, show cursor, leave alternate screen |
| 6 | Call `crossterm::terminal::disable_raw_mode()` |
| 7 | Exit process with code 0 (normal) or 1 (error) |

## Signal handling (normative)

| Signal | Behavior |
|---|---|
| `SIGWINCH` | Emit `Action::Resize(cols, rows)` into the input channel |
| `SIGTERM` | Initiate graceful shutdown (same as `:qa!`) |
| `SIGINT` | Ignored in raw mode (Ctrl-C is handled as a key event) |
| `SIGHUP` | Initiate graceful shutdown with auto-session save |
| `SIGTSTP` | Suspend: restore terminal, stop process. On `SIGCONT`: re-enter raw mode, redraw. |
| `SIGCONT` | Re-enter raw mode, set up terminal, full redraw |

## Panic handling

On panic, the binary MUST:

1. Restore terminal state (raw mode off, alternate screen off, cursor visible).
2. Print the panic message to stderr.
3. Write a crash report to `~/.local/share/kjxlkj/crash/` per [/docs/technical/crash-reporting.md](/docs/technical/crash-reporting.md).
4. Exit with code 101.

This is achieved with a custom panic hook set before any other initialization.

## Command-line arguments (normative)

| Argument | Description |
|---|---|
| `[file...]` | Files to open |
| `+{line}` | Start at specified line in first file |
| `+/{pattern}` | Search for pattern in first file |
| `-c {command}` | Execute ex command after startup |
| `--clean` | Skip config loading (no init file, no session restore) |
| `--version` | Print version and exit |
| `--help` | Print usage and exit |

## Related

- Runtime event loop: [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- Configuration: [/docs/spec/features/session/project-config.md](/docs/spec/features/session/project-config.md)
- Session restore: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
- Crash reporting: [/docs/technical/crash-reporting.md](/docs/technical/crash-reporting.md)
- Input decoding: [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)

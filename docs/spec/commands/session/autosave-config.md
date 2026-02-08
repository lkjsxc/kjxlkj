# Auto-Save Configuration

Automatic file saving.

## Overview

Automatically save files to prevent data loss.
Auto-save is event-driven and debounced to avoid
excessive disk writes.

## Enable Auto-Save

### Configuration

Set in `~/.config/kjxlkj/config.toml` under `[editor]`:
`auto_save = true` (default: `false`).

### Commands

`:set autosave` enables auto-save for the session.
`:set noautosave` disables it.

## Trigger Events

### When to Save

| Event | Saved | Notes |
|-------|-------|-------|
| Focus lost | Yes | Terminal loses focus (requires `focus_events`) |
| Idle timeout | Yes | No keystrokes for `auto_save_delay` ms |
| Buffer switch | Yes | Switching to another buffer |
| Window switch | Yes | Switching to another window |
| Mode change | No | Changing modes does not trigger |
| Command execution | No | Running ex commands does not trigger |

### Common Configurations

- Focus-only: set `auto_save_delay = 0` (save only on focus lost)
- Aggressive: set `auto_save_delay = 1000` (1 second idle)
- Conservative: set `auto_save_delay = 5000` (5 seconds idle)

## Delay

### Wait Before Save

`auto_save_delay` (integer, milliseconds) controls how long
the editor waits after the last keystroke before saving.
Default: `3000` (3 seconds).

### Purpose

Avoids saving on every keystroke. Many file watchers
(LSP, build tools) react to each write, so frequent
saves cause unnecessary recompilation.

### Debounce

Each new edit resets the timer. The save only fires
when no edits occur for the full delay duration.
Implementation uses a `tokio::time::sleep` that is
canceled and restarted on each buffer modification.

## Conditions

### Skip When

Auto-save is skipped for:
- Buffers with no file name (unnamed buffers)
- Read-only buffers
- Buffers with `buftype` set (help, terminal, quickfix)
- Buffers where the file no longer exists on disk
- Buffers with active format-on-save that returns an error

### File Patterns

`auto_save_exclude` (array of globs) lists patterns to
exclude: e.g. `["*.log", "/tmp/*"]`.

## Write Options

### Silent Save

Auto-save uses silent write: no "written" message appears
in the command line. Errors are shown as warnings in the
status line rather than blocking prompts.

### Update vs Write

Auto-save uses `:update` semantics: only writes if the
buffer has been modified since the last write. This
avoids touching the file timestamp unnecessarily.

## Buffer Types

### Regular Files Only

Auto-save only applies to normal file buffers.
Terminal buffers, help buffers, quickfix lists, and
scratch buffers are never auto-saved.

## Per-Buffer Override

### Disable for Buffer

`:setlocal noautosave` disables auto-save for the
current buffer only.

### Enable for Buffer

`:setlocal autosave` enables for current buffer even if
globally disabled (requires `auto_save = true` globally).

## Session Integration

### Save Session Too

When `auto_save_session = true`, the session file is
also updated on each auto-save trigger. Default: `false`.

## Format on Save

### Combined

When both `auto_save` and `format_on_save` are true,
formatting runs before each auto-save write.
If the formatter fails, the raw buffer is saved.

### Formatter

Uses the configured formatter (LSP `textDocument/formatting`
or external command from `[languages.{lang}.formatter]`).

## Backup Before Save

### Safety Net

When `backup = true`, a backup file is created before
each auto-save write. The backup is removed after
successful write. See write-commands.md for backup details.

## Undo Integration

### Undo Still Works

Auto-save does not affect undo history.
`u` undoes changes even after auto-save.

### Undo Persistence

When `undofile = true`, undo history persists across
sessions. Auto-saved files retain their full undo tree.

## Status Indication

### Show Status

The statusline shows an indicator when auto-save is active.
After a successful auto-save, a brief flash or icon
appears for 2 seconds.

### Indicators

| Indicator | Meaning |
|-----------|---------|
| `[AS]` | Auto-save enabled for buffer |
| `[saved]` | Auto-save just completed |
| (none) | Auto-save disabled |

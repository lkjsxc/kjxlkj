# Write Commands

Saving files and buffers.

## Overview

Write commands save buffer contents to files.
All write operations are atomic: content is written
to a temporary file first, then renamed into place.

## Basic Commands

### Write Current

`:w` writes the current buffer to its associated file.
Only writes if the buffer has been modified.
`:w!` forces write even to readonly files.

### Write As

`:w {path}` writes the buffer to a new file without
changing the buffer association. `:saveas {path}`
writes and changes the association.

## Write and Quit

### Combined

| Command | Behavior |
|---------|----------|
| `:wq` | Always writes, then closes window |
| `:x` | Writes only if changed, then closes |
| `ZZ` | Same as `:x` |

### Difference

`:wq` updates the file timestamp even if nothing changed.
`:x` preserves the timestamp when the buffer is unmodified.

## Write All

### All Buffers

`:wa` or `:wall` writes all modified buffers.
Skips unnamed buffers and special buffer types.

### Write All and Quit

`:wqa` or `:xa` writes all modified buffers and exits.

## Force Write

### Override Readonly

`:w!` writes to a file even if the buffer or file is
marked readonly. Does not override filesystem permissions.

### Sudo Write

`:SudoWrite` (built-in command) writes using elevated
privileges via `pkexec` or `sudo tee`. Prompts for
password if needed.

## Write Range

### Specific Lines

`:{range}w {file}` writes the specified line range.
`:10,20w chunk.txt` writes lines 10-20.

### Append to File

`:{range}w >> {file}` appends the range to the file.
`:w >> log.txt` appends the entire buffer.

## Write Part

### Selection

In visual mode, `:'<,'>w {file}` writes the selection.

### Pattern Range

`:g/pattern/w >> matches.txt` appends matching lines.

## Backup Options

### Backup Before Write

When `backup = true`, a backup file (`{name}~`) is
created before each write and persists after write.

### Writebackup

When `writebackup = true` (default), a backup is created
before write and removed after successful write.
This protects against write failures.

## File Format

### Line Endings

| Format | Line Ending | Platform |
|--------|-------------|----------|
| `unix` | LF (`\n`) | Linux, macOS |
| `dos` | CRLF (`\r\n`) | Windows |
| `mac` | CR (`\r`) | Classic Mac |

### On Write

The buffer is written using the `fileformat` setting.
`:set fileformat=unix` before `:w` converts line endings.

## Encoding

### Set Encoding

`:set fileencoding=utf-8` sets the encoding for the
current buffer.

### Write with Encoding

`:w ++enc=utf-8` writes with a specific encoding for
this write only, without changing the buffer setting.

## Write to Command

### Pipe to Command

`:w !cmd` pipes the buffer content to a shell command.
`:w !sort > sorted.txt` sorts and writes to a file.
`:w !pbcopy` copies buffer to system clipboard (macOS).

### No File Change

`:w !cmd` does not mark the buffer as saved.
The buffer remains modified.

## Confirm Overwrite

### Existing File

`:w {path}` to an existing file fails with "File exists".
`:w! {path}` overwrites. `:confirm w {path}` prompts.

## Readonly Files

### Attempt Write

Writing to a readonly buffer produces:
"'readonly' option is set (add ! to override)".

### Check Status

`:set readonly?` shows readonly status.
`:set noreadonly` clears the flag.

## BOM Handling

### Byte Order Mark

When `bomb = true`, a UTF-8 BOM (EF BB BF) is written
at the start of the file. Default: `false`.
`:set bomb` enables, `:set nobomb` disables.

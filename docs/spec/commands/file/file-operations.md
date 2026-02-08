# File Operations

Saving, renaming, and managing files.

## Overview

Commands for file operations beyond basic read/write.
Includes file identity changes, renaming, copying,
deleting, encoding management, and file metadata queries.

## Save As

### New Name

`:saveas {path}` or `:sav {path}` writes the buffer to
a new file and switches the buffer to that file.

### Behavior

- Writes buffer content to the new path
- Changes the current buffer name to the new path
- The original file is unchanged on disk
- Marks the buffer as unmodified

## File Command

### Change Name

`:file {name}` changes the buffer name without writing.
The buffer is marked as modified (since the name changed
but no write occurred to the new name).

### No Write

Only changes the in-memory file association.
Use `:w` to save to the new name.

### Clear Name

`:file` with no argument displays the current file info.
`:0file` clears the file name, making the buffer unnamed.

## Move/Rename

### Using Shell

`:!mv {old} {new}` renames the file on disk.
Then `:e {new}` to reopen. Or use the built-in:

### Rename Function

`:Rename {newname}` (built-in command) renames the file
on disk, updates the buffer name, and updates any
open references. Accepts a bare filename (stays in same
directory) or a full path.

### Script Method

Alternative: `:saveas {new}` then `:!rm {old}`.

## Copy File

### To New Location

`:w {path}` writes current buffer to a new file.
The buffer continues to be associated with the original file.

### Range Copy

`:{range}w {path}` writes only the specified line range.

## Delete File

### Shell Command

`:!rm %` deletes the current file. The buffer remains
open but the file no longer exists on disk.

### With Confirmation

`:Delete` (built-in command) deletes the current file
after confirmation prompt, then closes the buffer.

## File Information

### Current File

`:f` or `<C-g>` displays file information in the command line.

### Output

Shows: filename, modified flag, line count, percentage
through file, cursor position.

### Detailed Output

`2<C-g>` shows the full absolute path.
`g<C-g>` shows word count, character count, byte count.

## Check Modified

### Status

`:set modified?` shows whether the buffer is modified.

### Indicator

The statusline shows `[+]` for modified buffers.
The bufferline shows a dot or `[+]` indicator.

## File Exists

### Check

The expression `filereadable("{path}")` returns 1 if the
file exists and is readable, 0 otherwise.

## File Paths

### Current File

`%` in commands expands to the current file path.
`%:p` expands to the full absolute path.

### Expand

Path modifiers:

| Modifier | Expansion |
|----------|-----------|
| `:p` | Full path |
| `:h` | Head (directory) |
| `:t` | Tail (filename) |
| `:r` | Root (remove extension) |
| `:e` | Extension only |
| `:~` | Relative to home |
| `:.` | Relative to cwd |

## Create Backup

### Manual

`:w %.bak` writes a backup copy.

### Auto Backup

Set `backup = true` in config TOML to create backup files
automatically before each write. Backup files are named
`{file}~` and stored in the same directory (or in
`backupdir` if configured).

## File Encoding

### Check

`:set fileencoding?` shows the encoding of the current buffer.

### Change

`:set fileencoding=utf-8` changes the encoding.
The file is re-encoded on next write.
Supported: `utf-8`, `latin1`, `utf-16le`, `utf-16be`,
`sjis`, `euc-jp`, `gbk`.

## File Format

### Check

`:set fileformat?` shows the line ending format:
`unix` (LF), `dos` (CRLF), or `mac` (CR).

### Change

`:set fileformat=unix` converts line endings on next write.

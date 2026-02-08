# Undo File Persistence

Persistent undo across sessions.

## Overview

Save undo history to disk so edits can be undone
even after closing and reopening a file.

## Enable

### Configuration

`undofile = true` enables persistent undo globally.
Default: `false`.

### Command

`:set undofile` enables for the current session.
`:set noundofile` disables for current session.

## Undo Directory

### Location

`undodir = "~/.local/state/kjxlkj/undo"` specifies
where undo files are stored.

### Auto Create

The directory is created automatically on first use
with permissions `0o700` (owner only).

## Undo File Names

### Format

The undo file name is derived from the absolute path
of the source file. Path separators are replaced with
`%` to create a flat file structure.

### Example

`/home/user/project/src/main.rs` becomes
`%home%user%project%src%main.rs` in the undo directory.

## How It Works

### On Save

When a buffer is written (`:w`), the undo tree is
serialized and written to the undo file. The format
is a binary encoding of the undo tree structure.

### On Open

When a file is opened, the matching undo file is
loaded if it exists. The undo tree is deserialized
and attached to the buffer.

### Seamless

After loading, `u` and `<C-r>` navigate the full
history including edits from previous sessions.

## Undo Levels

### Limit History

`undolevels = 1000` limits maximum undo steps.
Default: 1000.

### Unlimited

`undolevels = -1` disables undo entirely.
Very high values (e.g., 100000) effectively unlimited.

## File Size

### Maximum

`undoreload = 10000` sets the line count threshold.
Files with more lines than this skip undo file loading
on buffer reload.

### Over Limit

For very large undo files (configurable, default 10MB),
the undo file is not loaded to avoid memory issues.

## Commands

### Check Status

`:set undofile?` shows whether persistent undo is on.
`:echo &undodir` shows the undo directory.

### Clear Undo

`:edit!` reloads the buffer and clears in-memory undo.
Delete the undo file from disk to clear persistent undo.

## Undo Tree

### Visual

`:UndoTree` opens a visual undo tree browser showing
all branches of the undo history with timestamps.

### Navigation

In the undo tree viewer:
- `j`/`k` navigate between undo states
- `<CR>` restores the selected state
- `p` previews the diff for a state
- `q` closes the viewer

## Related Commands

### Undo/Redo

| Key | Command | Description |
|-----|---------|-------------|
| `u` | `:undo` | Undo last change |
| `<C-r>` | `:redo` | Redo last undo |
| `U` | Line undo | Undo all changes on current line |

### Jump in History

`:earlier {time}` goes back in time (e.g., `:earlier 10m`
for 10 minutes ago).
`:later {time}` goes forward.
`:earlier {count}f` goes back {count} file saves.

## Privacy

### Sensitive Files

Undo files contain the full change history including
deleted text. This is a privacy concern for sensitive
files.

### Skip Patterns

Configure patterns to skip undo persistence:
`undofile_exclude = ["*.gpg", "*.secret", "/tmp/*"]`
Files matching these patterns will not have undo files.

## Security

### Permissions

Undo files are created with mode `0o600` (owner
read/write only).

### Encryption

Undo files are not encrypted. For sensitive content,
exclude files from undo persistence or use encrypted
filesystem.

## Cleanup

### Manual

Delete files in the undo directory:
`rm ~/.local/state/kjxlkj/undo/*`

### Age-Based

Configure `undofile_max_age = 90` (days) to automatically
clean up undo files older than the specified age.

### Script

A cleanup can be triggered on startup if configured:
files older than `undofile_max_age` are deleted.

## Troubleshooting

### Undo Not Working

1. Check `:set undofile?` — must show `undofile`
2. Check directory permissions for undodir
3. Verify file is not in exclude patterns
4. Check disk space
5. Look for errors in `:messages`

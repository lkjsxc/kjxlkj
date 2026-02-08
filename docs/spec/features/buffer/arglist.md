# Argument List

Command-line file arguments and batch editing workflows.

## Overview

The arglist is an ordered subset of the buffer list, initially populated from command-line arguments. It supports batch operations across a set of files.

## Navigation Commands

| Command | Action |
|---|---|
| `:next` / `:n` | Open next file in arglist |
| `:prev` / `:N` | Open previous file |
| `:first` | Open first file |
| `:last` | Open last file |
| `:argument {n}` | Open file at position n (1-based) |
| `:next {count}` | Skip forward count files |

All navigation commands accept `!` to discard unsaved changes.

## Viewing the Arglist

`:args` prints the arglist. The current file is enclosed in `[]`.

`:arglocal` shows the window-local arglist (if set).

## Modifying the Arglist

| Command | Action |
|---|---|
| `:argadd {file}...` | Append files to arglist |
| `:argdelete {pattern}` | Remove matching files |
| `:args {file}...` | Replace entire arglist |
| `:argdelete *` | Clear arglist |

Files may use glob patterns: `*`, `**` (recursive), `?`, `[abc]`.

## Window-Local Arglist

`:arglocal` creates a separate arglist for the current window. `:argglobal` reverts to the shared global arglist.

## Batch Editing

| Command | Action |
|---|---|
| `:argdo {cmd}` | Execute cmd on every arglist file |
| `:argdo update` | Save each modified file |

`:argdo` opens each arglist buffer in sequence, executes the command, then moves to the next. Use with `:update` or `:write` to save changes.

## Arglist vs Buffer List

| Aspect | Arglist | Buffer List |
|---|---|---|
| Source | Explicit (CLI or `:args`) | Any opened file |
| Order | Stable, user-controlled | By buffer number |
| Subset | Yes | Contains all buffers |
| Batch ops | `:argdo` | `:bufdo` |

## Statusline Integration

The current arglist position is available for statusline display as `[n/total]`.

## Related

- Buffers: [/docs/spec/editor/buffers.md](/docs/spec/editor/buffers.md)
- Buffer listing: [/docs/spec/features/buffer/buffer-listing.md](/docs/spec/features/buffer/buffer-listing.md)

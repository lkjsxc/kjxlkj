# Diff Mode

Compare and merge files.

## Overview

Diff mode shows differences between files for comparison
and merging. It uses a side-by-side or inline display with
synchronized scrolling.

## Enter Diff Mode

### Two Files

Launch from command line: `kjxlkj -d file1 file2`.
This opens both files in vertical splits with diff enabled.

### From Editor

`:diffsplit {file}` opens a file in a new split and enables
diff mode between the current buffer and the new split.

## Diff Split

### Vertical Diff

`:vert diffsplit {file}` opens the diff in a vertical split
(side by side). This is the default for `:diffsplit`.

### Horizontal Diff

`:diffsplit {file}` without `:vert` uses horizontal split.

### New File

`:diffthis` enables diff mode on the current buffer.
Run in two windows to diff them.

## Navigation

### Jump to Changes

| Key | Action |
|-----|--------|
| `]c` | Jump to next change |
| `[c` | Jump to previous change |

### Commands

`:diffnext` and `:diffprev` are command equivalents.

## Merging

### Get Changes

`:diffget` (or `do`) obtains the change from the other buffer
into the current buffer at the cursor position.

### Put Changes

`:diffput` (or `dp`) puts the current buffer's change into
the other buffer.

### With Buffer Number

`:diffget {bufnr}` specifies which buffer to get from
when more than two buffers are in diff mode.

## Three-Way Merge

### Setup

`:diffsplit {base}` then `:diffsplit {theirs}` opens a
three-way diff. Or use `kjxlkj -d file1 file2 file3`.

### Layout

Three vertical splits: LOCAL (left), BASE (center),
REMOTE (right). The merged result is edited in any
of the three.

### Merge Commands

`do` with a count specifies which buffer:
`1do` gets from buffer 1, `3do` from buffer 3.

## Diff Options

### Configuration

Under `[diff]` in config TOML:
- `algorithm` (string): diff algorithm
- `context` (integer): lines of context around changes

### Algorithms

| Algorithm | Best For |
|-----------|----------|
| `myers` | Default, fast |
| `minimal` | Smallest diff |
| `patience` | Code changes, better readability |
| `histogram` | Large files, similar to patience |

## Ignore Options

### Whitespace

| Option | Effect |
|--------|--------|
| `iwhite` | Ignore whitespace amount changes |
| `iwhiteall` | Ignore all whitespace differences |
| `iwhiteeol` | Ignore trailing whitespace |
| `iblank` | Ignore blank line additions/removals |
| `icase` | Ignore case differences |

### Commands

`:set diffopt+=iwhite` adds whitespace ignoring.
`:set diffopt-=iwhite` removes it.

## Display Options

### Filler Lines

Filler lines (`---`) are shown in the buffer that has
fewer lines to keep the two sides aligned.

### Fold Unchanged

Unchanged regions are folded by default to show only
the differences. The `context` option controls how many
unchanged lines are shown around each change (default: 6).

### Highlight

| Group | Purpose |
|-------|---------|
| `DiffAdd` | Added lines (green background) |
| `DiffChange` | Changed lines (blue background) |
| `DiffDelete` | Deleted lines (red background) |
| `DiffText` | Changed text within a changed line |

## Scroll Binding

### Synchronized Scroll

Diff mode enables `scrollbind` and `cursorbind` automatically.
Scrolling one buffer scrolls the other. Cursor movement
in one window moves the cursor in the other.

### Disable

`:set noscrollbind nocursorbind` disables sync.
`:diffoff` disables all diff settings on the current buffer.
`:diffoff!` disables on all windows.

## Update Diff

### Refresh

`:diffupdate` recalculates the diff after making edits.
Diff is also auto-updated on each edit when the diff
algorithm is fast enough (controlled by `redrawtime`).

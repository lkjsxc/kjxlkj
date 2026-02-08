# Vimdiff Mode

Back: [docs/spec/features/git/README.md](/docs/spec/features/git/README.md)

Side-by-side file comparison with merge support.

## Overview

Vimdiff mode shows two or three files side by side
with change highlighting. It supports merge conflict
resolution and general file comparison.

## Activation

### Command Line

`kjxlkj -d file1 file2` opens in diff mode with
two files in a vertical split.

`kjxlkj -d file1 file2 file3` opens three-way diff
for merge conflict resolution.

### Ex Commands

| Command | Action |
|---------|--------|
| `:diffthis` | Mark current window for diff |
| `:diffoff` | Remove diff mode from window |
| `:diffoff!` | Remove diff from all windows |
| `:diffsplit {file}` | Open file in diff split |

## Display

### Change Highlighting

| Highlight | Meaning |
|-----------|---------|
| Added line | Green background |
| Deleted line | Red background |
| Changed line | Blue background |
| Changed text | Bold within changed line |

### Scroll Binding

Windows in diff mode scroll together. Scrolling
one window scrolls all diff-linked windows by the
same amount. This is `scrollbind` integration.

### Fold Unchanged

Unchanged regions are automatically folded to show
only changed areas. The fold context (lines above
and below changes) is controlled by `diffopt`.

## Navigation

### Hunk Navigation

| Key | Action |
|-----|--------|
| `]c` | Jump to next change hunk |
| `[c` | Jump to previous change hunk |

### Hunk Operations

| Command | Action |
|---------|--------|
| `do` / `:diffget` | Get change from other buffer |
| `dp` / `:diffput` | Put change to other buffer |

### With Buffer Argument

`:diffget {bufnr}` gets changes from a specific
buffer (for three-way diffs where multiple source
buffers exist).

## Diff Algorithm

### Myers Algorithm

The default diff algorithm is Myers (same as git
default). It produces minimal edit distance diffs.

### Patience Algorithm

`diffopt` can select patience algorithm which
produces more readable diffs for code changes by
matching low-frequency unique lines first.

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `diffopt` | "filler" | Diff display options |
| `diffopt+=algorithm:patience` | -- | Use patience alg |
| `diffopt+=context:{n}` | 6 | Fold context lines |
| `diffopt+=iwhite` | -- | Ignore whitespace changes |
| `diffopt+=icase` | -- | Ignore case changes |
| `diffopt+=indent-heuristic` | -- | Better hunk splits |

## Three-Way Merge

### Layout

Three-way diffs show:
- Left: LOCAL (current branch)
- Center: BASE (common ancestor) or merged result
- Right: REMOTE (incoming branch)

### Merge Resolution

1. Navigate to conflict hunk with `]c`
2. Use `:diffget LOCAL` or `:diffget REMOTE`
3. Edit the result manually if needed
4. Mark resolved with `:diffupdate`

## Buffer Update

### Manual Refresh

`:diffupdate` recalculates the diff. Use after
making manual edits that change the diff state.

### Auto Update

The diff is automatically recalculated after any
text change in a diff-linked buffer.

## Related

- Git integration: [docs/spec/features/git/git.md](/docs/spec/features/git/git.md)
- Diff mode: [docs/spec/features/git/diff-mode.md](/docs/spec/features/git/diff-mode.md)
- Merge conflicts: [docs/spec/features/git/merge-conflicts.md](/docs/spec/features/git/merge-conflicts.md)

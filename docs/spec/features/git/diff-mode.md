# Diff Mode

Back: [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)

Side-by-side and inline diff viewing for Git changes.

## Overview

Diff mode shows differences between two versions of a file. The editor supports both inline (gutter signs) and split (side-by-side) diff views.

## Inline Diff (Gutter Signs)

Git changes are shown in the sign column:

| Sign | Color | Meaning |
|---|---|---|
| `+` / `│` | Green | Added lines |
| `~` / `│` | Yellow | Changed lines |
| `-` / `_` | Red | Deleted lines (shown at deletion point) |

## Navigation

| Key | Command | Description |
|---|---|---|
| `]c` | `:DiffNext` | Jump to next change hunk |
| `[c` | `:DiffPrev` | Jump to previous change hunk |

## Hunk Operations

| Key | Command | Description |
|---|---|---|
| `<leader>hs` | `:DiffStage` | Stage current hunk |
| `<leader>hr` | `:DiffReset` | Revert current hunk to HEAD |
| `<leader>hp` | `:DiffPreview` | Preview hunk diff in popup |

## Split Diff

| Command | Description |
|---|---|
| `:DiffSplit` | Open side-by-side diff of current file vs HEAD |
| `:DiffSplit {ref}` | Diff current file vs `{ref}` (commit, branch) |

In split diff mode, both windows scroll together (scroll-bind), and matching lines are aligned.

## Diff Algorithm

| Setting | Default | Description |
|---|---|---|
| `diff.algorithm` | `patience` | Diff algorithm: `myers`, `patience`, `histogram` |
| `diff.context` | `3` | Lines of context around changes |

## Highlight Groups

| Group | Usage |
|---|---|
| `DiffAdd` | Added text |
| `DiffChange` | Changed line |
| `DiffDelete` | Deleted text |
| `DiffText` | Changed text within a changed line |

## Blame

| Command | Description |
|---|---|
| `:GitBlame` | Show blame annotations in virtual text |
| `:GitBlameToggle` | Toggle blame display |

## Related

- Git integration: [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)
- Merge conflicts: [/docs/spec/features/git/merge-conflicts.md](/docs/spec/features/git/merge-conflicts.md)

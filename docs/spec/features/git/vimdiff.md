# Vimdiff Mode

Back: [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)

Side-by-side diff editing mode.

## Overview

Vimdiff opens two (or three) files side by side with differences highlighted and scroll-bound.

## Opening

| Command | Description |
|---|---|
| `:diffsplit {file}` | Split and diff current file with `{file}` |
| CLI: `kjxlkj -d {file1} {file2}` | Open two files in diff mode |

## Navigation

| Key | Description |
|---|---|
| `]c` | Jump to next change |
| `[c` | Jump to previous change |

## Merge Operations

| Command | Description |
|---|---|
| `:diffget` / `do` | Obtain change from other buffer |
| `:diffput` / `dp` | Put change into other buffer |

## Scroll Binding

Both windows scroll together. This is controlled by `scrollbind` which is automatically set in diff mode.

## Update

`:diffupdate` re-computes the diff after manual edits.

## Related

- Diff mode: [/docs/spec/features/git/diff-mode.md](/docs/spec/features/git/diff-mode.md)
- Git integration: [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)

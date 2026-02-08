# Quickfix List

Back: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)

A global list of locations for navigating errors, search results, and other entries.

## Overview

The quickfix list holds a collection of file positions (file, line, column, text). It is populated by commands like `:make`, `:grep`, `:vimgrep`, and LSP diagnostics.

## Navigation

| Command | Key | Description |
|---|---|---|
| `:cnext` | `]q` | Go to next quickfix entry |
| `:cprev` | `[q` | Go to previous quickfix entry |
| `:cfirst` | - | Go to first entry |
| `:clast` | - | Go to last entry |
| `:cc {N}` | - | Go to entry N |

## Open/Close

| Command | Description |
|---|---|
| `:copen` | Open quickfix window |
| `:cclose` | Close quickfix window |
| `:cwindow` | Open quickfix window if there are entries |

## Populate

| Command | Source |
|---|---|
| `:make` | Build errors |
| `:grep {pattern}` | Grep results |
| `:vimgrep {pattern} {files}` | Internal grep |
| LSP | Diagnostics, references |

## Location List

The location list is a per-window variant of the quickfix list:

| Quickfix | Location list |
|---|---|
| `:cnext` | `:lnext` |
| `:cprev` | `:lprev` |
| `:copen` | `:lopen` |
| `:cclose` | `:lclose` |

## Quickfix Window

The quickfix window is a special buffer that displays entries. Press `<CR>` on an entry to jump to that location.

## History

The editor remembers the last 10 quickfix lists. `:colder` and `:cnewer` navigate between them.

| Command | Description |
|---|---|
| `:colder` | Switch to older quickfix list |
| `:cnewer` | Switch to newer quickfix list |

## Related

- Navigation: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)
- Live grep: [/docs/spec/editing/search/live-grep.md](/docs/spec/editing/search/live-grep.md)
- Diagnostics: [/docs/spec/features/lsp/diagnostics.md](/docs/spec/features/lsp/diagnostics.md)

# Finder (Fuzzy Picker)

Back: [/docs/spec/features/navigation/README.md](/docs/spec/features/navigation/README.md)

Fuzzy-search picker for files, buffers, symbols, and other lists.

## Overview

The finder provides a fuzzy-matching interface for navigating to items. It supports file finding, buffer switching, symbol search, command palette, and more.

## Activation

| Key | Command | Description |
|---|---|---|
| `<leader>ff` | `:FindFile` | Find files |
| `<leader>fb` | `:Buffers` | Find buffers |
| `<leader>fg` | `:LiveGrep` | Grep project |
| `<leader>fs` | `:DocumentSymbols` | Document symbols |
| `<leader>fS` | `:WorkspaceSymbols` | Workspace symbols |
| `<leader>fc` | `:Commands` | Command palette |

## Fuzzy Matching

The finder uses a fuzzy matching algorithm that:

1. Scores based on character positions (consecutive characters score higher).
2. Prioritizes matches at word boundaries.
3. Is case-insensitive by default; uppercase letters force case-sensitive matching.

## Navigation

| Key | Action |
|---|---|
| `<C-j>` / `<Down>` | Next item |
| `<C-k>` / `<Up>` | Previous item |
| `<CR>` | Open selected item |
| `<C-v>` | Open in vertical split |
| `<C-s>` | Open in horizontal split |
| `<Esc>` | Close finder |

## Configuration

| Setting | Default | Description |
|---|---|---|
| `finder.max_results` | `200` | Maximum displayed results |
| `finder.respect_gitignore` | `true` | Exclude gitignored files |
| `finder.hidden` | `false` | Include hidden files |

## Related

- Live grep: [/docs/spec/editing/search/live-grep.md](/docs/spec/editing/search/live-grep.md)
- Quickfix: [/docs/spec/features/navigation/quickfix.md](/docs/spec/features/navigation/quickfix.md)
- Command palette: [/docs/spec/features/navigation/command-palette.md](/docs/spec/features/navigation/command-palette.md)

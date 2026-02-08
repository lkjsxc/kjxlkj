# Workspace Symbols

Back: [/docs/spec/features/lsp/navigation/README.md](/docs/spec/features/lsp/navigation/README.md)

Search for symbols across all files in the workspace via LSP `workspace/symbol`.

## Overview

Workspace symbols allow searching for functions, classes, variables, and other named entities across the entire project. Unlike document symbols which are scoped to one file, workspace symbols search all indexed files.

## Usage

| Key | Action |
|---|---|
| `<leader>ws` | Open workspace symbols picker |

| Command | Description |
|---|---|
| `:WorkspaceSymbols [query]` | Search workspace symbols with optional initial query |

## Display

The picker shows matching symbols with their kind icon, name, container name, and file path. Results are filtered as the user types.

| Column | Content |
|---|---|
| Icon | Symbol kind (function, class, etc.) |
| Name | Symbol name |
| Container | Parent scope (e.g., class name for methods) |
| File | Relative file path and line number |

## Search behavior

The query is sent to the LSP server, which performs the search and returns results. The search is re-triggered on each keystroke with debouncing.

| Setting | Type | Default | Description |
|---|---|---|---|
| `workspace_symbols.debounce_ms` | integer | `150` | Debounce interval for search requests |
| `workspace_symbols.max_results` | integer | `100` | Maximum results to display |

## Navigation

| Key | Action |
|---|---|
| `<CR>` | Jump to symbol definition |
| `<C-v>` | Open in vertical split |
| `<C-s>` | Open in horizontal split |
| `<Esc>` | Close picker |

## LSP requirements

The LSP server must support `workspace/symbol`. Most language servers support this capability.

## Related

- Document symbols: [/docs/spec/features/lsp/navigation/document-symbols.md](/docs/spec/features/lsp/navigation/document-symbols.md)
- Finder: [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)

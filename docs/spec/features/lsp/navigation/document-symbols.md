# Document Symbols

Back: [/docs/spec/features/lsp/navigation/README.md](/docs/spec/features/lsp/navigation/README.md)

Navigate the structural outline of the current file via LSP `textDocument/documentSymbol`.

## Overview

Document symbols provide a hierarchical view of named entities (functions, classes, variables, etc.) in the current buffer. The editor requests symbols from the LSP server and presents them in a picker or sidebar.

## Opening

### Keybinding

| Key | Action |
|---|---|
| `<leader>ds` | Open document symbols picker |
| `go` | Toggle symbol outline sidebar |

### Command

| Command | Description |
|---|---|
| `:DocumentSymbols` | Open document symbols picker |
| `:Outline` | Toggle outline sidebar |

## Display

### List view

The picker displays symbols as a flat or hierarchical list. Each entry shows the symbol icon, name, kind, and detail string. The list is sorted by position in the file (top to bottom).

### Tree view

In the outline sidebar, symbols are displayed as a tree reflecting the nesting structure returned by the LSP server (e.g., methods inside classes, fields inside structs).

## Symbol types

| Icon | Kind | SymbolKind value |
|---|---|---|
| ◇ | Class / Struct | 5 / 23 |
| ƒ | Function | 12 |
| ○ | Method | 6 |
| ⬡ | Interface | 11 |
| ▢ | Enum | 10 |
| • | Property / Field | 7 / 8 |
| △ | Variable | 13 |
| ☆ | Constant | 14 |

## Navigation

### Fuzzy search

While the picker is open, typing filters symbols by fuzzy match against the symbol name. Matching is case-insensitive by default.

### Jump

| Key | Action |
|---|---|
| `<CR>` | Jump to symbol location and close picker |
| `o` | Jump to symbol and close |
| `<Esc>` | Close picker without jumping |
| `<C-v>` | Open symbol in vertical split |
| `<C-s>` | Open symbol in horizontal split |

## Outline sidebar

### Toggle

`go` toggles the outline sidebar. When visible, it occupies a vertical split on the right side. The sidebar updates automatically when the cursor moves to a different buffer.

### Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `outline.enabled` | boolean | `true` | Allow outline sidebar |
| `outline.width` | integer | `30` | Sidebar width in columns |
| `outline.position` | string | `right` | `left` or `right` |
| `outline.auto_close` | boolean | `false` | Close after jumping |

## Breadcrumbs

### Statusline

The statusline can display a breadcrumb trail showing the path to the symbol at the cursor position (e.g., `MyClass > my_method > inner_block`).

### Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `breadcrumbs.enabled` | boolean | `false` | Show symbol breadcrumbs in statusline |

## LSP integration

### Requirements

The LSP server must support `textDocument/documentSymbol`. Servers that return `DocumentSymbol[]` (hierarchical) are preferred over `SymbolInformation[]` (flat).

### Hierarchical symbols

When the server returns hierarchical `DocumentSymbol` responses, the tree structure is preserved. When the server returns flat `SymbolInformation`, symbols are displayed as a flat list.

## Keybindings

### Navigation between symbols

| Key | Action |
|---|---|
| `]s` | Jump to next symbol boundary |
| `[s` | Jump to previous symbol boundary |

### Within picker

| Key | Action |
|---|---|
| `j` / `<Down>` | Move selection down |
| `k` / `<Up>` | Move selection up |
| `<C-d>` | Page down |
| `<C-u>` | Page up |

## Commands

| Command | Description |
|---|---|
| `:DocumentSymbols` | Open document symbols picker |
| `:Outline` | Toggle outline sidebar |
| `:OutlineClose` | Close outline sidebar |

## Related

- Workspace symbols: [/docs/spec/features/lsp/navigation/workspace-symbols.md](/docs/spec/features/lsp/navigation/workspace-symbols.md)
- Finder: [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
- LSP: [/docs/spec/features/lsp/lsp.md](/docs/spec/features/lsp/lsp.md)

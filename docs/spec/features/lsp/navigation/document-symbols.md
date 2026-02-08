# Document Symbols

Back: [/docs/spec/features/lsp/navigation/README.md](/docs/spec/features/lsp/navigation/README.md)

Browse symbols within the current document.

## Overview

Document symbols list all named entities (functions, classes, variables, etc.) in the current buffer, provided by the LSP server.

## Opening

| Key | Command | Description |
|---|---|---|
| `<leader>fs` | `:DocumentSymbols` | Open symbol picker |
| `<leader>o` | `:SymbolOutline` | Toggle outline sidebar |

## Display

Symbols are shown with their kind icon, name, and line number. Nested symbols (methods inside classes) are shown hierarchically.

## Navigation

Select a symbol in the picker and press `<CR>` to jump to its definition.

## Outline Sidebar

A persistent side panel showing the document symbol tree. The current symbol is highlighted as the cursor moves.

| Setting | Default | Description |
|---|---|---|
| `outline.position` | `right` | Side panel position |
| `outline.width` | `30` | Panel width |

## LSP Integration

Uses `textDocument/documentSymbol` LSP request. Falls back to flat symbol list if the server does not support hierarchy.

## Related

- LSP: [/docs/spec/features/lsp/README.md](/docs/spec/features/lsp/README.md)
- Workspace symbols: [/docs/spec/features/lsp/navigation/workspace-symbols.md](/docs/spec/features/lsp/navigation/workspace-symbols.md)

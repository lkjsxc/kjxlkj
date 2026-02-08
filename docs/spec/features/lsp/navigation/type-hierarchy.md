# Type Hierarchy

Back: [/docs/spec/features/lsp/navigation/README.md](/docs/spec/features/lsp/navigation/README.md)

View supertypes and subtypes of a type via LSP `typeHierarchy/supertypes` and `typeHierarchy/subtypes`.

## Overview

Type hierarchy shows the inheritance chain of the type under the cursor. Supertypes show parent classes/interfaces. Subtypes show derived classes/implementations.

## Usage

| Key | Action |
|---|---|
| `<leader>ts` | Show supertypes |
| `<leader>tt` | Show subtypes |

| Command | Description |
|---|---|
| `:Supertypes` | Show supertypes hierarchy |
| `:Subtypes` | Show subtypes hierarchy |

## Display

Results are shown in an expandable tree view. Each node shows the type name, kind, and file location.

## Navigation

| Key | Action |
|---|---|
| `j` / `k` | Move up/down |
| `l` | Expand node |
| `h` | Collapse node |
| `<CR>` | Jump to type definition |
| `<Esc>` | Close tree |

## Lazy loading

Subtree nodes are fetched on demand when expanded. A loading indicator appears during the LSP request.

## LSP requirements

Requires `textDocument/prepareTypeHierarchy`, `typeHierarchy/supertypes`, and `typeHierarchy/subtypes`. Not all language servers support this.

| Server | Type hierarchy |
|---|---|
| rust-analyzer | Yes |
| clangd | Yes |
| typescript-language-server | Yes |
| gopls | Partial |

## Related

- Call hierarchy: [/docs/spec/features/lsp/navigation/call-hierarchy.md](/docs/spec/features/lsp/navigation/call-hierarchy.md)
- References: [/docs/spec/features/lsp/navigation/references.md](/docs/spec/features/lsp/navigation/references.md)

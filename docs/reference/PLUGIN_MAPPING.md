# Plugin Equivalent Mapping (Target)

Back: [/docs/reference/README.md](/docs/reference/README.md)
Mapping from common Neovim plugins to kjxlkj built-in features.

This document is **target-oriented**. Many items are not implemented yet.

For the currently supported surface (or initial reconstruction target), see:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## File management

| Neovim plugin examples | kjxlkj target | Current status |
|------------------------|--------------|----------------|
| nvim-tree.lua, neo-tree.nvim, oil.nvim | File Explorer | Not implemented |

## Fuzzy finding

| Neovim plugin examples | kjxlkj target | Current status |
|------------------------|--------------|----------------|
| telescope.nvim, fzf.vim, fzf-lua | Finder / Index UI | Not implemented (target service: `kjxlkj-service-index`) |

## LSP and completion

| Neovim plugin examples | kjxlkj target | Current status |
|------------------------|--------------|----------------|
| nvim-lspconfig, mason.nvim | Built-in LSP client | Not implemented (target service: `kjxlkj-service-lsp`) |
| nvim-cmp, coq_nvim | Completion UI | Not implemented |

## Syntax and diagnostics

| Neovim plugin examples | kjxlkj target | Current status |
|------------------------|--------------|----------------|
| nvim-treesitter | Syntax highlighting | Not implemented |
| trouble.nvim | Diagnostics UI | Not implemented |

## Git

| Neovim plugin examples | kjxlkj target | Current status |
|------------------------|--------------|----------------|
| gitsigns.nvim, fugitive.vim | Git integration | Not implemented (target service: `kjxlkj-service-git`) |

## Editing helpers

| Neovim plugin examples | kjxlkj target | Current status |
|------------------------|--------------|----------------|
| undotree | Undo UX | Core undo exists; UI not implemented |
| nvim-surround | Surround editing | Not implemented |

## Terminal

| Neovim plugin examples | kjxlkj target | Current status |
|------------------------|--------------|----------------|
| toggleterm.nvim, floaterm.vim | Integrated terminal panes | Not implemented (only `:! {cmd}` exists) |

## Related

- Built-in features spec: [/docs/spec/features/README.md](/docs/spec/features/README.md)

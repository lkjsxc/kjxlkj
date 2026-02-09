# Plugin Equivalent Mapping (Target)

Back: [/docs/reference/README.md](/docs/reference/README.md)
Mapping from common Neovim plugins to kjxlkj built-in features.

This document is target-oriented and intentionally non-authoritative for current runtime status.

For verified current behavior, always check:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## File management

| Neovim plugin examples | kjxlkj target | Current status source |
|------------------------|--------------|-----------------------|
| nvim-tree.lua, neo-tree.nvim, oil.nvim | File Explorer | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` |

## Fuzzy finding

| Neovim plugin examples | kjxlkj target | Current status source |
|------------------------|--------------|-----------------------|
| telescope.nvim, fzf.vim, fzf-lua | Finder / Index UI | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` |

## LSP and completion

| Neovim plugin examples | kjxlkj target | Current status source |
|------------------------|--------------|-----------------------|
| nvim-lspconfig, mason.nvim | Built-in LSP client | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` |
| nvim-cmp, coq_nvim | Completion UI | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` |

## Syntax and diagnostics

| Neovim plugin examples | kjxlkj target | Current status source |
|------------------------|--------------|-----------------------|
| nvim-treesitter | Syntax highlighting | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` |
| trouble.nvim | Diagnostics UI | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` |

## Git

| Neovim plugin examples | kjxlkj target | Current status source |
|------------------------|--------------|-----------------------|
| gitsigns.nvim, fugitive.vim | Git integration | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` |

## Editing helpers

| Neovim plugin examples | kjxlkj target | Current status source |
|------------------------|--------------|-----------------------|
| undotree | Undo UX | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` |
| nvim-surround | Surround editing | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` |

## Terminal

| Neovim plugin examples | kjxlkj target | Current status source |
|------------------------|--------------|-----------------------|
| toggleterm.nvim, floaterm.vim | Integrated terminal panes | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` |

## Related

- Built-in features spec: [/docs/spec/features/README.md](/docs/spec/features/README.md)

# Plugin Equivalent Mapping

Neovim plugins and their kjxlkj built-in equivalents.

## File Management

| Neovim Plugin | kjxlkj Feature |
|---------------|----------------|
| nvim-tree.lua | File Explorer |
| neo-tree.nvim | File Explorer |
| oil.nvim | File Explorer |
| telescope-file-browser | Finder |

## Fuzzy Finding

| Neovim Plugin | kjxlkj Feature |
|---------------|----------------|
| telescope.nvim | Finder |
| fzf.vim | Finder |
| fzf-lua | Finder |

## LSP

| Neovim Plugin | kjxlkj Feature |
|---------------|----------------|
| nvim-lspconfig | LSP Client |
| mason.nvim | Manual install |
| null-ls.nvim | Formatters/Linters |
| lsp-zero.nvim | LSP Client |

## Completion

| Neovim Plugin | kjxlkj Feature |
|---------------|----------------|
| nvim-cmp | Completion |
| coq_nvim | Completion |
| ddc.vim | Completion |

## Syntax

| Neovim Plugin | kjxlkj Feature |
|---------------|----------------|
| nvim-treesitter | Syntax Engine |
| nvim-treesitter-textobjects | Text Objects |
| nvim-treesitter-context | Context Bar |

## Git

| Neovim Plugin | kjxlkj Feature |
|---------------|----------------|
| gitsigns.nvim | Git Signs |
| fugitive.vim | Git Commands |
| lazygit.nvim | Terminal + lazygit |
| diffview.nvim | Diff View |
| neogit | Git Integration |

## UI Enhancement

| Neovim Plugin | kjxlkj Feature |
|---------------|----------------|
| bufferline.nvim | Bufferline |
| lualine.nvim | Statusline |
| which-key.nvim | Which Key |
| indent-blankline | Indent Guides |
| nvim-colorizer | Color Preview |
| dressing.nvim | Native UI |

## Editing

| Neovim Plugin | kjxlkj Feature |
|---------------|----------------|
| nvim-surround | Surround |
| Comment.nvim | Comments |
| nvim-autopairs | Auto Pairs |
| undotree | Undo Tree |
| flash.nvim | Flash Navigation |
| leap.nvim | Flash Navigation |

## Navigation

| Neovim Plugin | kjxlkj Feature |
|---------------|----------------|
| harpoon | Marks |
| marks.nvim | Marks |
| hop.nvim | Flash Navigation |

## Terminal

| Neovim Plugin | kjxlkj Feature |
|---------------|----------------|
| toggleterm.nvim | Terminal |
| floaterm.vim | Terminal |

## Miscellaneous

| Neovim Plugin | kjxlkj Feature |
|---------------|----------------|
| auto-save.nvim | Auto Save |
| persistence.nvim | Sessions |
| project.nvim | Workspaces |
| trouble.nvim | Quickfix |

## Not Needed

These plugins have no equivalent because
the feature is built into the core:

- impatient.nvim (already fast)
- packer.nvim (no plugins)
- lazy.nvim (no plugins)
- plenary.nvim (internal utils)
- nvim-web-devicons (built-in icons)

## Configuration Mapping

See [docs/guides/MIGRATION.md](/docs/guides/MIGRATION.md) for config
translation examples.

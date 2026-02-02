# Neovim Migration Guide

Guide for Neovim users transitioning to kjxlkj.

## Philosophy Differences

| Aspect | Neovim | kjxlkj |
|--------|--------|--------|
| Config | Lua | TOML |
| Plugins | External (lazy.nvim) | Built-in |
| LSP | lspconfig | Native |
| Treesitter | nvim-treesitter | Native |

## Keybinding Equivalents

Most vim keybindings work identically.

### File Explorer

| Neovim (nvim-tree) | kjxlkj |
|--------------------|--------|
| `:NvimTreeToggle` | `<leader>e` |
| Same navigation | Same navigation |

### Fuzzy Finder

| Neovim (Telescope) | kjxlkj |
|--------------------|--------|
| `:Telescope find_files` | `<leader>ff` |
| `:Telescope live_grep` | `<leader>fg` |
| `:Telescope buffers` | `<leader>fb` |

### Terminal

| Neovim (toggleterm) | kjxlkj |
|---------------------|--------|
| `<C-\><C-n>` exit | Same |
| `:ToggleTerm` | `<leader>t` |

### LSP

| Neovim | kjxlkj |
|--------|--------|
| `gd` | `gd` |
| `K` | `K` |
| `<leader>rn` | `<leader>rn` |

## Config Migration

### Neovim (Lua)


### kjxlkj (TOML)


## Keybinding Migration

### Neovim


### kjxlkj


## Plugin Replacements

| Neovim Plugin | kjxlkj Built-in |
|---------------|-----------------|
| nvim-tree.lua | File Explorer |
| telescope.nvim | Finder |
| toggleterm.nvim | Terminal |
| nvim-lspconfig | LSP Client |
| nvim-cmp | Completion |
| nvim-treesitter | Syntax |
| gitsigns.nvim | Git Signs |
| which-key.nvim | Which Key |
| bufferline.nvim | Bufferline |
| lualine.nvim | Statusline |
| Comment.nvim | Comments |
| nvim-surround | Surround |
| flash.nvim | Flash |
| undotree | Undo Tree |

## What's Different

### No Plugin Management

Everything built-in. No lazy.nvim, packer, etc.

### No Lua Scripting

Configuration only. No arbitrary code execution.

### Faster Startup

No plugin loading phase.

## What's Similar

- Modal editing
- Vim keybindings
- Splits and tabs
- Registers and marks
- Macros
- Search and replace

## Tips

1. Start with default config, customize gradually
2. Read the keybinding docs for differences
3. Use `:help` for built-in documentation
4. Check FAQ for common issues

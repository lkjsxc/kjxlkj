# Built-in Feature System

Back: [docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

All features are built-in; no external plugin system.

## Overview

kjxlkj does not support external plugins. All
functionality that would traditionally be a plugin
is built into the editor as a first-class feature.
This ensures consistent UX, unified configuration,
and no runtime dependencies.

## Design Rationale

### Why No Plugins

| Concern | Resolution |
|---------|------------|
| Security | No arbitrary code execution |
| Stability | All features tested together |
| Performance | No runtime loading overhead |
| UX consistency | Shared list UIs, keymaps, themes |
| Single binary | No plugin installation step |

### Trade-offs

The trade-off is that new features require editor
source changes. This is acceptable because the
target feature set (Neovim-equivalent editing) is
well-defined and finite.

## Built-in Feature Categories

### Editing Helpers (replacing plugins)

| Feature | Replaces |
|---------|----------|
| Auto-pairs | auto-pairs.nvim |
| Surround | vim-surround |
| Comment toggle | vim-commentary |
| Multi-cursor | vim-visual-multi |
| Flash motions | flash.nvim |

### UI Features (replacing plugins)

| Feature | Replaces |
|---------|----------|
| Bufferline | bufferline.nvim |
| Statusline | lualine.nvim |
| File explorer | neo-tree.nvim |
| Fuzzy finder | telescope.nvim |
| Which-key hints | which-key.nvim |
| Indent guides | indent-blankline.nvim |

### Integration Features (replacing plugins)

| Feature | Replaces |
|---------|----------|
| LSP client | nvim-lspconfig |
| Completion | nvim-cmp |
| Git signs | gitsigns.nvim |
| Git diff | diffview.nvim |
| Diagnostics | trouble.nvim |
| Snippet engine | LuaSnip |
| Tree-sitter | nvim-treesitter |

## Feature Configuration

### Unified Config

All features are configured through the single
TOML configuration file. Each feature has its
own configuration section.

### Feature Toggling

Individual features can be enabled or disabled:

| Toggle | Effect |
|--------|--------|
| `autopairs.enabled = false` | Disable auto-pairs |
| `bufferline.enabled = false` | Hide bufferline |
| `indent_guides.enabled = false` | Hide guides |
| `gitsigns.enabled = false` | Hide git signs |

### Filetype-Specific

Features can be configured per filetype via TOML sections:

| TOML key | Value |
|---|---|
| `filetype.rust.autopairs.enabled` | `true` |
| `filetype.rust.format_on_save` | `true` |

## Extensibility

### User Commands

Users can define custom commands using `:command`
that compose existing ex commands into new workflows.

### Key Mappings

Custom key mappings can trigger any built-in command
or sequence of commands.

### External Tools

Integration with external tools is via:
- `:!{cmd}` for shell commands
- `:{range}!{cmd}` for text filtering
- LSP server for language features
- Tree-sitter grammars for syntax

## After Directory

### Purpose

The `~/.config/kjxlkj/after/` directory contains
filetype-specific configuration that is sourced
after the main config, allowing per-filetype
overrides.

### Structure

`after/ftplugin/{filetype}.toml` is sourced when
a buffer of that filetype is opened.

## Related

- Architecture: [docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- Configuration: [docs/spec/features/config/README.md](/docs/spec/features/config/README.md)
- Crates: [docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)

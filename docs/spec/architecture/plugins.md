# Plugin/Extension Architecture

kjxlkj provides built-in features rather than a plugin system.

## Design Philosophy

Unlike Neovim which relies heavily on plugins (Lua ecosystem), kjxlkj:
- Bundles essential features out-of-the-box
- Eliminates plugin management complexity
- Ensures consistent, tested integration
- Provides fast startup (no runtime loading)

## Built-in Feature Equivalents

| Neovim Plugin | kjxlkj Built-in |
|---------------|-----------------|
| nvim-tree.lua | File Explorer |
| toggleterm.nvim | Integrated Terminal |
| telescope.nvim | Fuzzy Finder |
| nvim-lspconfig | LSP Client |
| nvim-cmp | Completions |
| auto-save.nvim | Auto Save |
| winresizer | Window Resize Mode |
| gitsigns.nvim | Git Integration |
| nvim-treesitter | Syntax Highlighting |

## Configuration-Based Customization

Instead of Lua plugins, customization via TOML:


## Keybinding Customization

Custom keybindings without scripting:


## Event Hooks

Declarative event handling:


## Future: Extension Points

Potential extension mechanisms (not implemented):
- Wasm plugins for sandboxed extensions
- External process integration via JSON-RPC
- Command palette extensions

## Rationale

Benefits of built-in approach:
1. **Performance** - No plugin loading overhead
2. **Reliability** - Features are tested together
3. **Simplicity** - No package manager needed
4. **Security** - No third-party code execution
5. **Consistency** - Uniform UX across features

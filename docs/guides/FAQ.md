# Frequently Asked Questions

## General

### What is kjxlkj?

kjxlkj is a Neovim-inspired terminal text editor with all features
built-in natively. No plugins required.

### Why "kjxlkj"?

The name is a project codename. Pronounce it however you like!

### Is it compatible with Neovim config?

No. kjxlkj uses TOML configuration, not Lua. However, the keybindings
and editing model are familiar to vim/neovim users.

### Can I use my vim plugins?

No. kjxlkj has built-in features that replace common plugins.
See the feature comparison table.

## Installation

### What platforms are supported?

- Linux (x86_64, aarch64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64, aarch64)

### How do I update?

Download the latest release or run:

### Where is the config file?

- Linux/macOS: `~/.config/kjxlkj/config.toml`
- Windows: `%APPDATA%\kjxlkj\config.toml`

## Usage

### How do I exit?

Press `:q` and Enter. Or `ZZ` to save and quit.

### How do I save?

Press `:w` and Enter. Or `:wq` to save and quit.

### How do I open the file explorer?

Press `<Space>e` (with default leader key).

### How do I search?

Press `/` to search forward, `?` to search backward.

### How do I use multiple cursors?

Press `<C-n>` on a word to select it, then `<C-n>` again
to select the next occurrence.

## Features

### Does it have LSP support?

Yes, built-in. Configure language servers in config.toml.

### Does it have syntax highlighting?

Yes, via tree-sitter. Most languages supported.

### Does it support splits?

Yes. `<C-w>v` for vertical, `<C-w>s` for horizontal.

### Does it have tabs?

Yes. `:tabnew`, `gt`/`gT` to switch.

## Troubleshooting

### Colors look wrong

Ensure your terminal supports true color:

### Keys not working

Check your terminal's key reporting. Some terminals
require configuration for modifier keys.

### Slow startup

Check for large config files or many buffers in session.

### High memory usage

Large files or many buffers. See memory optimization docs.

## Contributing

### How can I contribute?

See CONTRIBUTING.md. PRs welcome!

### Where do I report bugs?

GitHub Issues: https://github.com/kjxlkj/kjxlkj/issues

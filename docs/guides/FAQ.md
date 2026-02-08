# Frequently Asked Questions

Back: [/docs/guides/README.md](/docs/guides/README.md)

## What is kjxlkj?

A Neovim-inspired TUI text editor written in Rust. It provides modal editing, LSP support, Tree-sitter syntax highlighting, an embedded terminal, and Git integration.

## How is it different from Neovim?

kjxlkj is a from-scratch implementation. It does not use Neovim's codebase. Key differences:

| Aspect | kjxlkj | Neovim |
|---|---|---|
| Language | Rust | C + Lua |
| Plugin system | None (all native) | Lua plugins |
| Configuration | TOML | Lua / Vimscript |
| Architecture | Async-first (Tokio) | Event loop |

## What platforms are supported?

Linux (primary), macOS, and Windows (via WSL or native).

## Does it support plugins?

No. All functionality is built-in natively. This ensures consistent behavior and performance.

## How do I configure it?

Edit `~/.config/kjxlkj/config.toml`. See the quickstart guide.

## How do I install it?

See [/docs/guides/INSTALL_WINDOWS.md](/docs/guides/INSTALL_WINDOWS.md) for Windows, or build from source with `cargo build --release`.

## Related

- Quickstart: [/docs/guides/QUICKSTART.md](/docs/guides/QUICKSTART.md)
- Migration: [/docs/guides/MIGRATION.md](/docs/guides/MIGRATION.md)

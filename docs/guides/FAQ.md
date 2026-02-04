# Frequently Asked Questions

Back: [/docs/guides/README.md](/docs/guides/README.md)

## General

### What is kjxlkj?

kjxlkj is a Neovim-inspired terminal (TUI) text editor written in Rust.

This repository follows an “All in Docs” approach: `/docs/` is the source of truth and the system can be reconstructed from docs alone.

### Why "kjxlkj"?

The name is a project codename. Pronounce it however you like!

### Is it compatible with Neovim config?

No. kjxlkj does not execute Neovim’s Lua configuration or plugin ecosystem.

The long-term target includes configuration and keybinding remapping, but the currently shipped surface is tracked in:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

### Can I use my vim plugins?

No. kjxlkj has no plugin system by design.

Some “plugin-like” capabilities are planned as built-in features, but many are not implemented yet. See:

- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Installation

### What platforms are supported?

The codebase aims to be cross-platform via Rust and Crossterm, but platform support is not fully validated yet.

### How do I update?

There are no tagged releases in this repository yet. Rebuild from source.

### Where is the config file?

Persistent configuration is not implemented yet.

## Usage

### How do I exit?

Press `:q` and Enter. Or `ZZ` to save and quit.

### How do I save?

Use `:w {file}` to write to a path, or `:wq` / `:x` to write then quit.

### How do I search?

Press `/` to search forward, `?` to search backward.

## Features

### Does it have LSP support?

Not yet. The service crate exists as a placeholder, but LSP features are not implemented.

### Does it have syntax highlighting?

Not yet.

### Does it support splits?

Not yet.

### Does it have tabs?

Not yet.

## Troubleshooting

### Colors look wrong

Ensure your terminal supports at least 256 colors.

### Keys not working

Some terminals do not report all modifier combinations consistently. Try a different terminal emulator.

### Slow startup

Performance has not been benchmarked yet.

### High memory usage

Large files have not been benchmarked yet.

## Contributing

### How can I contribute?

This repo is currently optimized for doc-driven iteration and LLM-based implementation. Improvements should start with docs and conformance.

### Where do I report bugs?

Use the repository issue tracker if one exists, or record issues in `/docs/reference/IMPLEMENTATION_HISTORY.md` with reproduction steps and spec references.

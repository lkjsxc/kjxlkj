# kjxlkj Quick Start Guide

Back: [/docs/guides/README.md](/docs/guides/README.md)
Getting started with the editor surface described by the conformance ledger.

For the authoritative “what exists right now” ledger, see:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

If this repository is in a docs-only baseline (no `src/`/Cargo workspace), reconstruct the implementation first:

- [/docs/todo/RECONSTRUCTION_PROMPT.md](/docs/todo/RECONSTRUCTION_PROMPT.md)

## Installation

How to obtain and build the editor.

### Pre-built binaries

This repository does not currently publish tagged releases. Build from source.

### Build from source

1. Install the Rust toolchain (stable). If `rust-toolchain.toml` exists, use it to install the pinned toolchain + components for reproducibility.
2. Build with `cargo build`.
3. Run with `cargo run`.

Docker-based usage is a target derived artifact; if `Dockerfile` exists, see [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md).

## First steps

Getting started after installation.

### Opening a file

Run the editor with an optional file path argument (the file content is loaded into the buffer).

You can also open a file inside the editor with `:e {file}`.

### Basic navigation

| Key | Action |
|-----|--------|
| `h j k l` | Move cursor left/down/up/right |
| `w b e` | Word forward/backward/end |
| `0 $` | Line start/end |
| `gg G` | File start/end |
| `Ctrl-d` / `Ctrl-u` | Scroll half page down/up |

### Editing

| Key | Action |
|-----|--------|
| `i` | Insert before cursor |
| `a` | Insert after cursor |
| `o` | Open line below (enter Insert) |
| `O` | Open line above (enter Insert) |
| `x` | Delete character under cursor |
| `dd` | Delete line |
| `yy` | Yank line |
| `p` | Paste after cursor |
| `u` | Undo |
| `Ctrl-r` | Redo |

### Mode switching

| Key | Action |
|-----|--------|
| `Esc` | Return to Normal mode |
| `i, a, o` | Enter Insert mode |
| `v` | Enter Visual mode |
| `V` | Enter Visual Line mode |
| `Ctrl-v` | Enter Visual Block mode |
| `R` | Enter Replace mode |
| `:` | Enter Command mode |

## Saving and quitting

- Quit: `:q` (or `:q!` to force).
- Write to a path: `:w {file}`.
- Write then quit: `:wq` or `:x`.
- Convenience: `ZZ` writes then quits; `ZQ` quits without saving.

## What is not implemented yet

Many “modern editor” features (LSP, git integration, syntax highlighting, explorer/finder UI, configuration) are specified as targets but are not implemented yet.

See:

- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Getting help

- Documentation index: [/docs/README.md](/docs/README.md)
- Target keybindings spec: [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)

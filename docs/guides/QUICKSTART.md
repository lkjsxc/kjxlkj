# kjxlkj Quick Start Guide

Welcome to kjxlkj! This guide gets you started quickly.

## Installation

### Pre-built Binaries

Download from GitHub Releases for your platform:
- Linux (x86_64, aarch64): `.tar.gz` or AppImage
- macOS (Intel, Apple Silicon): `.tar.gz`
- Windows (x86_64, aarch64): `.zip`

### Build from Source


## First Steps

### Opening Files


### Basic Navigation

| Key | Action |
|-----|--------|
| `h j k l` | Move cursor left/down/up/right |
| `w b e` | Word forward/backward/end |
| `0 $` | Line start/end |
| `gg G` | File start/end |
| `Ctrl-d/u` | Page down/up |

### Editing

| Key | Action |
|-----|--------|
| `i` | Insert before cursor |
| `a` | Insert after cursor |
| `o` | New line below |
| `O` | New line above |
| `x` | Delete character |
| `dd` | Delete line |
| `yy` | Copy line |
| `p` | Paste |
| `u` | Undo |
| `Ctrl-r` | Redo |

### Mode Switching

| Key | Action |
|-----|--------|
| `i, a, o` | → Insert mode |
| `v` | → Visual mode |
| `V` | → Visual Line mode |
| `Ctrl-v` | → Visual Block mode |
| `:` | → Command mode |
| `Esc` | → Normal mode |

## Built-in Features

### File Explorer (Space + e)
- Navigate: `j/k`
- Open: `Enter` or `l`
- Parent: `h`
- Create: `a` (file) / `A` (dir)
- Delete: `d`

### Terminal (Space + t)
- Toggle: `Ctrl-\`
- Exit: type `exit` or close window

### Fuzzy Finder (Space + f)
- Files: `<leader>ff`
- Grep: `<leader>fg`
- Buffers: `<leader>fb`

## Configuration

Config file: `~/.config/kjxlkj/config.toml`

See `examples/config.toml` for options.

## Getting Help

- In-editor: `:help` or `:h topic`
- Documentation: [docs/spec/](/docs/spec/)
- Issues: GitHub Issues

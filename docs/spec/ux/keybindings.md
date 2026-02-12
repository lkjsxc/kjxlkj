# Keybindings Reference

Target keybinding reference for kjxlkj.

## Overview

This document describes the target keybinding surface.

For the currently supported subset (when a reconstructed implementation exists), see:

- [docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Leader Key

Default leader: `Space`

Normative:

- In Normal mode, `Space` MUST act as `<leader>` and MUST NOT perform cursor motion or editing by itself.
- `<leader>` alone MUST NOT mutate buffer content; it MAY trigger a which-key style hint view if that feature is enabled.

## Documentation Index

| Document | Content |
|----------|---------|
| [mode-entry.md](keybindings/mode-entry.md) | Mode entry and exit keys |
| [navigation.md](keybindings/navigation.md) | Movement and scrolling |
| [editing.md](keybindings/editing.md) | Delete, yank, change, undo |
| [text-objects.md](keybindings/text-objects.md) | Text object selections |
| [visual.md](keybindings/visual.md) | Visual mode operations |
| [command.md](keybindings/command.md) | Command-line mode |
| [windows-tabs.md](keybindings/windows-tabs.md) | Window and tab management |
| [features.md](keybindings/features.md) | Built-in features (explorer, terminal) |
| [lsp.md](keybindings/lsp.md) | LSP and code intelligence |
| [macros-registers.md](keybindings/macros-registers.md) | Macros, registers, marks |
| [advanced.md](keybindings/advanced.md) | Expert-level operations |

## Key Notation

| Notation | Meaning |
|----------|---------|
| `<leader>` | Leader key (Space) |
| `Ctrl-x` | Control + x |
| `<C-x>` | Control + x (alternative) |
| `<A-x>` | Alt + x |
| `<M-x>` | Meta + x (same as Alt) |
| `<S-x>` | Shift + x |
| `<Esc>` | Escape key |
| `<CR>` | Enter/Return |
| `<Tab>` | Tab key |
| `<BS>` | Backspace |
| `<Space>` | Space bar |

## Mode Indicators

| Mode | Cursor | Description |
|------|--------|-------------|
| Normal | Block (█) | Default command mode |
| Insert | Bar (│) | Text entry mode |
| Visual | Hollow (▯) | Selection mode |
| Replace | Underline (_) | Overwrite mode |
| Command | N/A | Ex command entry |

## Quick Reference

Common keybindings at a glance.

### Essential Keys

| Key | Action |
|-----|--------|
| `Esc` | Return to Normal mode |
| `i` | Insert before cursor |
| `a` | Append after cursor |
| `A` | Append at end of line |
| `o` | Open line below |
| `v` | Visual mode |
| `V` | Visual line mode |
| `dd` | Delete line |
| `yy` | Yank line |
| `p` | Paste after |
| `u` | Undo |
| `Ctrl-r` | Redo |
| `.` | Repeat last change |
| `/` | Search forward |
| `n` | Next match |
| `:w` | Save |
| `:q` | Quit |

### Window Keys

| Key | Action |
|-----|--------|
| `Ctrl-w s` | Split horizontal |
| `Ctrl-w v` | Split vertical |
| `Ctrl-w h/j/k/l` | Navigate windows |
| `Ctrl-w c` | Close window |

### Feature Keys

| Key | Action |
|-----|--------|
| `<leader>e` | File explorer |
| `<leader>f` | Find files |
| `<leader>g` | Live grep |
| `<leader>t` | Terminal |
| `<leader>u` | Undo tree |

# Terminal Features

Back: [/docs/spec/features/README.md](/docs/spec/features/README.md)

Integrated terminal emulator and external tool features. Terminals are first-class windows in the editor's window tree.

## Core principle

Terminal panes are windows. They share the same window tree, layout engine, split/resize/navigation system, and session persistence as editor buffer windows. The terminal is not a separate UI panel; it is a window whose content comes from a PTY process instead of a text buffer.

## Documents

| Document | Content |
|----------|---------|
| [terminal.md](terminal.md) | Full-scratch terminal emulator spec (PTY, screen buffer, modes) |
| [escape-parser.md](escape-parser.md) | VT100/xterm escape sequence parser state machine and dispatch tables |
| [tmux.md](tmux.md) | Terminal multiplexer integration contract |
| [wm-integration.md](wm-integration.md) | Window manager integration |
| [remote.md](remote.md) | Remote editing over SSH |
| [dap.md](dap.md) | Debug adapter protocol |

## Terminal overview

| Feature | Description |
|---------|-------------|
| Full VT100/xterm emulation | State-machine escape sequence parser with SGR color, cursor movement, scroll regions |
| PTY process management | Async PTY spawn, read, write, resize (SIGWINCH), cleanup (SIGHUP) |
| Terminal as window | Terminals are windows in the editor window tree with `WindowId` |
| Scrollback navigation | Vi-like motions in terminal Normal mode over scrollback buffer |
| Send to terminal | Execute current line or visual selection in a terminal |
| Multiple terminals | Named terminal instances, concurrent PTY processes |

## Keybindings

| Key | Action |
|-----|--------|
| `<leader>t` | Toggle terminal |
| `<leader>tf` | Floating terminal |
| `<leader>th` | Horizontal split terminal |
| `<leader>tv` | Vertical split terminal |
| `Ctrl-\ Ctrl-n` | Terminal to Normal mode |

## Related

- Features overview: [/docs/spec/features/README.md](/docs/spec/features/README.md)
- Window management: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
- Editor windows: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Multiplexer contract: [/docs/spec/features/terminal/tmux.md](/docs/spec/features/terminal/tmux.md)

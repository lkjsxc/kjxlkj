# Terminal Features

Back: [/docs/spec/features/README.md](/docs/spec/features/README.md)

Integrated terminal emulator features. Terminals are first-class windows.

## Core Principle

Terminal panes share the same window tree, split engine, focus movement, and session persistence model as buffer and explorer panes.

## Documents

| Document | Content |
|---|---|
| [terminal.md](terminal.md) | terminal window, PTY lifecycle, routing, and reliability rules |
| [escape-parser.md](escape-parser.md) | VT parser state machine and escape handling |
| [tmux.md](tmux.md) | multiplexer integration contract |
| [wm-integration.md](wm-integration.md) | window-manager integration |
| [remote.md](remote.md) | remote workflow rules |
| [dap.md](dap.md) | debug-adapter terminal integration |

## Keybindings

| Key | Action |
|---|---|
| `<leader>t` | toggle terminal |
| `<leader>th` | horizontal split terminal |
| `<leader>tv` | vertical split terminal |
| `<leader>tf` | floating terminal |
| `Ctrl-\\ Ctrl-n` | terminal insert -> terminal normal |

## Related

- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Split behavior: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- E2E matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)

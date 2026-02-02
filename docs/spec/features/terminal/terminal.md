# Integrated Terminal (toggleterm Built-in)

Native terminal emulator replacing toggleterm.nvim plugin.

## User Intent

Run shells and commands alongside editing without leaving the editor.

## Activation

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>t` | Toggle terminal | Toggle default terminal |
| `<leader>tf` | Floating | Floating terminal window |
| `<leader>th` | Horizontal | Horizontal split terminal |
| `<leader>tv` | Vertical | Vertical split terminal |
| `<leader>tn` | New | Create new terminal |
| `Ctrl-\` | Quick toggle | Quick toggle terminal |

## Terminal Mode Keys

| Key | Action |
|-----|--------|
| `Ctrl-\ Ctrl-n` | Exit to Normal mode |
| `Esc Esc` | Exit to Normal mode |
| `Ctrl-w h/j/k/l` | Navigate windows |
| `Ctrl-w c` | Close terminal |

## Terminal Types

| Type | Description |
|------|-------------|
| Float | Centered overlay window |
| Horizontal | Bottom split |
| Vertical | Right split |
| Tab | Full-screen tab |

## Session Persistence

- Terminals survive window layout changes
- Terminal state persists across toggles
- Multiple named terminals supported
- Terminal history preserved

## Sending Text

| Key | Action |
|-----|--------|
| `<leader>ts` | Send line | Send current line to terminal |
| `<leader>tS` | Send selection | Send visual selection |
| `<leader>tc` | Send command | Send specific command |

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `terminal.shell` | System default | Shell to use |
| `terminal.size` | `15` | Default height/width |
| `terminal.float_size` | `0.8` | Float window ratio |
| `terminal.start_insert` | `true` | Enter Insert on open |
| `terminal.persist` | `true` | Persist sessions |

## Async Model

Terminal integration runs as a supervised service:

- Owns pty processes
- Streams output events to core
- Accepts input events from core
- Supports multiple concurrent terminals

## Acceptance Criteria

- Noisy terminal MUST NOT freeze editing
- Panes survive UI redraws
- Crashed jobs produce visible diagnostics
- Output streaming is non-blocking

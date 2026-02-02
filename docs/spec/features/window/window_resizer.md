# Window Resizer (winresizer Built-in)

Native window resizing replacing winresizer plugin.

## Overview

Built-in window resizing with intuitive keyboard controls.

## Resize Mode

Enter resize mode for continuous window resizing:

| Key | Action | Description |
|-----|--------|-------------|
| `<leader>wr` | Enter resize mode | Start window resizing |
| `Ctrl-w r` | Enter resize mode | Alternative entry |

## Resize Mode Keys

Once in resize mode:

| Key | Action |
|-----|--------|
| `h` | Decrease width |
| `l` | Increase width |
| `j` | Decrease height |
| `k` | Increase height |
| `H` | Decrease width (large step) |
| `L` | Increase width (large step) |
| `J` | Decrease height (large step) |
| `K` | Increase height (large step) |
| `=` | Equalize all windows |
| `_` | Maximize height |
| `\|` | Maximize width |
| `Enter` | Confirm and exit |
| `Esc` | Cancel and revert |
| `q` | Exit resize mode |

## Standard Resize Keys (Normal Mode)

Always available without entering resize mode:

| Key | Action |
|-----|--------|
| `Ctrl-w +` | Increase height by 1 |
| `Ctrl-w -` | Decrease height by 1 |
| `Ctrl-w >` | Increase width by 1 |
| `Ctrl-w <` | Decrease width by 1 |
| `Ctrl-w =` | Equalize all windows |
| `Ctrl-w _` | Maximize height |
| `Ctrl-w \|` | Maximize width |
| `<N>Ctrl-w +` | Increase height by N |
| `<N>Ctrl-w -` | Decrease height by N |
| `<N>Ctrl-w >` | Increase width by N |
| `<N>Ctrl-w <` | Decrease width by N |

## Resize Step Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `window.resize_step` | `2` | Normal resize step |
| `window.resize_step_large` | `5` | Large resize step (Shift) |

## Visual Feedback

During resize mode:

- Current window highlighted
- Size displayed in status line
- Visual guides for dimensions

## Acceptance Criteria

- Resizing is immediate and responsive
- Resize mode clearly indicated
- Cancel reverts all changes
- Works with any window layout


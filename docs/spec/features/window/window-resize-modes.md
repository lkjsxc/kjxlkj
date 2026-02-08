# Window Resize Modes

Interactive window resizing.

## Overview

Enter resize mode for precise window dimension adjustments.
Resize mode is a sub-mode of normal mode that remaps
movement keys to resize operations.

## Resize Mode

### Enter Mode

`<C-w>r` enters resize mode. The statusline shows `-- RESIZE --`.
Alternatively, configure a custom key in TOML.

### Exit Mode

| Key | Effect |
|-----|--------|
| `<Esc>` | Confirm current sizes, exit resize mode |
| `<CR>` | Confirm current sizes, exit resize mode |
| `q` | Revert to original sizes, exit resize mode |

## Mode Keybindings

### In Resize Mode

| Key | Action |
|-----|--------|
| `h` | Decrease width by `resize_step` columns |
| `l` | Increase width by `resize_step` columns |
| `j` | Increase height by `resize_step` rows |
| `k` | Decrease height by `resize_step` rows |
| `H` | Decrease width by `resize_step_fast` columns |
| `L` | Increase width by `resize_step_fast` columns |
| `J` | Increase height by `resize_step_fast` rows |
| `K` | Decrease height by `resize_step_fast` rows |
| `=` | Equalize all window sizes |

## Step Sizes

### Normal Step

`resize_step = 2` (default). Each `h`/`l`/`j`/`k` press
changes the dimension by this many cells.

### Fast Step

`resize_step_fast = 5` (default). Shift+key uses this
larger step for quick rough adjustments.

## Visual Feedback

### Highlight

During resize mode, the borders of the active window
are highlighted with the `WinResize` highlight group
(default: bright yellow border).

### Status Display

The statusline shows current window dimensions as
`[cols x rows]` during resize mode, updating live.

## Presets

### Quick Sizes

| Key (in resize mode) | Effect |
|-----------------------|--------|
| `1` | Set width to 25% of terminal |
| `2` | Set width to 33% of terminal |
| `3` | Set width to 50% of terminal |
| `4` | Set width to 67% of terminal |
| `5` | Set width to 75% of terminal |

### Ratios

All ratios are computed relative to the parent split
container, not the full terminal.

## Constraints

### Minimum Size

Windows have a minimum size of `window_min_width` columns
(default: 1) and `window_min_height` rows (default: 1).
The statusline always occupies 1 row.

### Fixed Windows

Windows with `winfixwidth = true` or `winfixheight = true`
are not resized by equalize or auto-balance operations.
They can still be resized manually.

## Mouse

Mouse input is ignored; resizing is keyboard-only
when in resize mode. Standard mouse drag on borders
works outside resize mode (if mouse is enabled).

## Commands

### Set Size

`:resize {n}` sets the height of the current window to `n` rows.
`:vertical resize {n}` sets the width to `n` columns.

### Relative Size

`:resize +{n}` increases height by `n`.
`:resize -{n}` decreases height by `n`.
`:vertical resize +{n}` / `-{n}` for width.

## Scripted Resize

### Absolute

`:resize 20` sets height to 20 rows.
`:vertical resize 80` sets width to 80 columns.

### Proportional

No built-in percentage command. Calculate from
terminal dimensions: `:resize` with computed value.

## Smart Resize

### Auto-Balance

When `equalalways = true` (default), windows are
automatically rebalanced after any window is
created, closed, or resized.

### Focus Grow

When `focus_grow = true`, the focused window automatically
expands to `focus_grow_width` / `focus_grow_height`
and other windows shrink proportionally.

## Golden Ratio

### Enable

`golden_ratio = true` in config TOML makes the focused
window take approximately 61.8% of the available space.
Other windows share the remainder equally.

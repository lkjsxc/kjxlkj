# Window Resize Modes

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

Interactive and command-based window resizing.

## Overview

Windows can be resized using commands, key sequences, and an interactive resize mode.

## Resize Keys

| Key | Action |
|---|---|
| `<C-w>+` | Increase height by 1 |
| `<C-w>-` | Decrease height by 1 |
| `<C-w>>` | Increase width by 1 |
| `<C-w><` | Decrease width by 1 |
| `{N}<C-w>+` | Increase height by N |
| `<C-w>=` | Equalize all window sizes |

## Absolute sizing

| Key | Action |
|---|---|
| `<C-w>_` | Set window height to maximum (or count) |
| `<C-w>\|` | Set window width to maximum (or count) |
| `{N}<C-w>_` | Set window height to N rows |
| `{N}<C-w>\|` | Set window width to N columns |

## Resize Commands

| Command | Description |
|---|---|
| `:resize {N}` | Set height to N |
| `:resize +{N}` | Increase height by N |
| `:resize -{N}` | Decrease height by N |
| `:vertical resize {N}` | Set width to N |
| `:vertical resize +{N}` | Increase width by N |
| `:vertical resize -{N}` | Decrease width by N |

## Interactive Resize Mode

`:ResizeMode` enters an interactive mode where arrow keys or `hjkl` resize the current window. Press `<Esc>` or `<CR>` to exit.

| Key (in resize mode) | Action |
|---|---|
| `h` / `<Left>` | Decrease width |
| `l` / `<Right>` | Increase width |
| `j` / `<Down>` | Increase height |
| `k` / `<Up>` | Decrease height |

## Minimum Size

| Setting | Default | Description |
|---|---|---|
| `window.min_height` | `1` | Minimum window height in rows |
| `window.min_width` | `1` | Minimum window width in columns |

Windows cannot be resized below these minimums.

## Equalization

`<C-w>=` distributes available space equally among all siblings in the layout tree. When the terminal is resized, equalization is performed automatically if `equalalways` is set.

| Setting | Default | Description |
|---|---|---|
| `equalalways` | `true` | Auto-equalize on window create/close |

## Related

- Window management: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
- Window layout: [/docs/spec/features/window/window-layouts.md](/docs/spec/features/window/window-layouts.md)
- Window zoom: [/docs/spec/features/window/window-zoom.md](/docs/spec/features/window/window-zoom.md)

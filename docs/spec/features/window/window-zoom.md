# Window Zoom

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

Temporarily maximize a window and restore the previous layout.

## Overview

Zoom temporarily makes one window fill the entire editor area. The original layout is saved and restored when zoom is toggled off.

## Zoom Toggle

| Key | Command | Description |
|---|---|---|
| `<C-w>z` | `:ZoomToggle` | Toggle zoom for current window |

When zoomed, the window fills the full editor area. All other windows become hidden but retain their buffers and state.

## Maximize Commands

| Command | Key | Description |
|---|---|---|
| `:ZoomHeight` | - | Maximize window height only |
| `:ZoomWidth` | - | Maximize window width only |
| `:ZoomToggle` | `<C-w>z` | Full maximize/restore |

## Zoom vs Only

| Feature | Zoom | Only (`<C-w>o`) |
|---|---|---|
| Other windows | Hidden, preserved | Closed |
| Restore | Toggle restores layout | Cannot restore |
| Buffer state | All preserved | Other buffers stay in list |

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `zoom.statusline_indicator` | string | `"[Z]"` | Text shown in statusline when zoomed |
| `zoom.restore_on_switch` | boolean | `false` | Restore layout when switching windows |

## Statusline Integration

When zoomed, the statusline displays `[Z]` (configurable). This uses the `zoom_state` variable in the statusline DSL.

## Keybindings

| Key | Action |
|---|---|
| `<C-w>z` | Toggle zoom |
| `<C-w>_` | Maximize height (not zoom) |
| `<C-w>\|` | Maximize width (not zoom) |
| `<C-w>=` | Equalize all window sizes |

## Layout Preservation

When zoom is toggled on, the current layout tree is serialized. When toggled off, the layout tree is restored. If windows were closed while zoomed, those are removed from the saved layout.

## Related

- Window management: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
- Window layout: [/docs/spec/features/window/window-layouts.md](/docs/spec/features/window/window-layouts.md)
- Session: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)

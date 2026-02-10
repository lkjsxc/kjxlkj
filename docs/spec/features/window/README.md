# Window Features

Back: [/docs/spec/features/README.md](/docs/spec/features/README.md)

Window and split management features.

## Core Principle

Buffer, explorer, and terminal panes are all windows in one shared layout tree.

## Keybindings

| Key | Action |
|---|---|
| `Ctrl-w s` | split horizontal |
| `Ctrl-w v` | split vertical |
| `Ctrl-w h/j/k/l` | directional focus |
| `Ctrl-w w/W/p` | next/previous/last focus |
| `Ctrl-w =` | equalize sizes |
| `Ctrl-w _` / `Ctrl-w |` | maximize height / width |
| `Ctrl-w q` | close window |

## Documents

| Document | Content |
|---|---|
| [splits-windows.md](splits-windows.md) | normative split/focus/resize/close behavior |
| [splits-advanced.md](splits-advanced.md) | advanced split operations and options |
| [window-layouts.md](window-layouts.md) | layout persistence and restore rules |
| [window-presets.md](window-presets.md) | preset layout patterns |
| [window-resize-modes.md](window-resize-modes.md) | resize modes and intent |
| [window_resizer.md](window_resizer.md) | resizer tool behavior |
| [window-zoom.md](window-zoom.md) | zoom/maximize behavior |
| [wincmd.md](wincmd.md) | command catalog for `Ctrl-w` family |
| [floating-windows.md](floating-windows.md) | floating window rules |
| [tabs.md](tabs.md) | tab-page behavior |

## Related

- Editor window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Explorer window behavior: [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- Terminal window behavior: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)

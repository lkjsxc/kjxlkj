# Terminal Window Manager Integration

Back: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)

How terminal windows integrate with the editor's window management system.

## Overview

Terminal buffers are displayed in regular editor windows. They participate in the window layout tree and can be split, moved, and resized like any buffer window.

## Terminal as Window

A terminal instance is associated with a buffer. That buffer is displayed in a window. The window can be:

- Split horizontally or vertically
- Moved within the layout
- Zoomed
- Closed (which may or may not kill the terminal process)

## Opening Terminals

| Command | Description |
|---|---|
| `:terminal` | Open terminal in current window |
| `:split \| terminal` | Open terminal in horizontal split |
| `:vsplit \| terminal` | Open terminal in vertical split |
| `:tabnew \| terminal` | Open terminal in new tab |

## Window Focus

When a terminal window is focused, the editor enters terminal mode. Terminal mode passes keystrokes directly to the terminal process except for the escape sequence (`<C-\><C-n>`).

## Window Layout

Terminal windows participate in the layout tree identically to file windows:

| Feature | Terminal Window |
|---|---|
| Splitting | Supported |
| Moving | Supported |
| Resizing | Supported |
| Zoom | Supported |
| Tab switching | Supported |

## Process Lifecycle

| Event | Behavior |
|---|---|
| Window closed | Terminal process receives SIGHUP (configurable) |
| Process exits | Window shows exit status, remains open until dismissed |
| Editor exits | All terminal processes receive SIGHUP |

| Setting | Default | Description |
|---|---|---|
| `terminal.close_on_exit` | `true` | Auto-close window when process exits with 0 |
| `terminal.kill_on_close` | `true` | Kill process when window is closed |

## Scroll behavior

When the terminal process produces output and the user has scrolled up in the scrollback, the terminal does NOT auto-scroll to the bottom. A marker indicates new output is available. Pressing `G` in terminal-normal mode jumps to the latest output.

## Related

- Terminal: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Window management: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
- Window layout: [/docs/spec/features/window/window-layout.md](/docs/spec/features/window/window-layout.md)

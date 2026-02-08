# Window Commands (wincmd)

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

Comprehensive reference for `Ctrl-w` window commands.

## Navigation

| Key | Command | Description |
|---|---|---|
| `Ctrl-w h` | `:wincmd h` | Move focus to window left |
| `Ctrl-w j` | `:wincmd j` | Move focus to window below |
| `Ctrl-w k` | `:wincmd k` | Move focus to window above |
| `Ctrl-w l` | `:wincmd l` | Move focus to window right |
| `Ctrl-w w` | `:wincmd w` | Cycle focus to next window |
| `Ctrl-w W` | `:wincmd W` | Cycle focus to previous window |
| `Ctrl-w t` | `:wincmd t` | Move focus to top-left window |
| `Ctrl-w b` | `:wincmd b` | Move focus to bottom-right window |
| `Ctrl-w p` | `:wincmd p` | Move focus to previous (last accessed) window |

## Splitting

| Key | Command | Description |
|---|---|---|
| `Ctrl-w s` | `:split` | Horizontal split current window |
| `Ctrl-w v` | `:vsplit` | Vertical split current window |
| `Ctrl-w n` | `:new` | New horizontal split with empty buffer |

## Closing

| Key | Command | Description |
|---|---|---|
| `Ctrl-w c` | `:close` | Close current window |
| `Ctrl-w q` | `:quit` | Quit current window |
| `Ctrl-w o` | `:only` | Close all other windows |

## Moving windows

| Key | Description |
|---|---|
| `Ctrl-w H` | Move current window to far left (becomes full-height vertical split) |
| `Ctrl-w J` | Move current window to very bottom (becomes full-width horizontal split) |
| `Ctrl-w K` | Move current window to very top (becomes full-width horizontal split) |
| `Ctrl-w L` | Move current window to far right (becomes full-height vertical split) |
| `Ctrl-w r` | Rotate windows downward/rightward |
| `Ctrl-w R` | Rotate windows upward/leftward |
| `Ctrl-w x` | Exchange current window with next |
| `Ctrl-w T` | Move current window to a new tab |

## Resizing

| Key | Description |
|---|---|
| `Ctrl-w +` | Increase height by 1 (or count) |
| `Ctrl-w -` | Decrease height by 1 (or count) |
| `Ctrl-w >` | Increase width by 1 (or count) |
| `Ctrl-w <` | Decrease width by 1 (or count) |
| `Ctrl-w =` | Equalize all window sizes |
| `Ctrl-w _` | Maximize height of current window |
| `Ctrl-w \|` | Maximize width of current window |

## Count prefix

All wincmd commands accept a count prefix. For navigation, `3 Ctrl-w j` moves 3 windows down. For resize, `5 Ctrl-w +` increases height by 5.

## Terminal window interaction

All wincmd navigation works identically for terminal and buffer windows. Terminal windows participate in the same window tree and layout.

## Related

- Window management: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Splits: [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- Window layouts: [/docs/spec/features/window/window-layouts.md](/docs/spec/features/window/window-layouts.md)

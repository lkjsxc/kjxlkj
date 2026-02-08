# Split / Window Management

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

The split system divides the editor area into tiled panes. Each pane holds exactly one buffer view. Splits are organized as a binary tree of horizontal and vertical containers within a tab page.

## Creating splits

| Command / Key | Required behavior |
|---|---|
| `:split` / `Ctrl-W s` | MUST create a horizontal split, placing the new window below the current one. Both windows display the same buffer. |
| `:vsplit` / `Ctrl-W v` | MUST create a vertical split, placing the new window to the right. Both windows display the same buffer. |
| `:split {path}` | MUST create a horizontal split and open `{path}` in the new window. If `{path}` does not exist, an empty buffer with that name MUST be created. |
| `:vsplit {path}` | MUST create a vertical split and open `{path}` in the new window. |
| `:new` | MUST create a horizontal split with a new unnamed buffer. |
| `:vnew` | MUST create a vertical split with a new unnamed buffer. |

A count prefix (e.g. `5 Ctrl-W s`) SHOULD set the initial height or width of the new window in rows or columns.

## Navigation

Moving focus between windows.

### Directional movement

| Key | Required behavior |
|---|---|
| `Ctrl-W h` | Focus the window directly to the left. If none exists, the cursor MUST stay in the current window. |
| `Ctrl-W j` | Focus the window directly below. |
| `Ctrl-W k` | Focus the window directly above. |
| `Ctrl-W l` | Focus the window directly to the right. |

### Cycling

| Key | Required behavior |
|---|---|
| `Ctrl-W w` | MUST cycle focus to the next window in tree traversal order, wrapping from last to first. |
| `Ctrl-W W` | MUST cycle focus to the previous window, wrapping from first to last. |
| `Ctrl-W p` | MUST return focus to the previously focused window. If no previous window exists, MUST report an error. |
| `Ctrl-W t` | MUST focus the top-left window. |
| `Ctrl-W b` | MUST focus the bottom-right window. |

## Resizing

| Key / Command | Required behavior |
|---|---|
| `Ctrl-W +` | Increase current window height by one row (count-aware). |
| `Ctrl-W -` | Decrease current window height by one row (count-aware). |
| `Ctrl-W >` | Increase current window width by one column (count-aware). |
| `Ctrl-W <` | Decrease current window width by one column (count-aware). |
| `{n} Ctrl-W _` | Set window height to exactly `n` rows. Without count, MUST maximize height. |
| `{n} Ctrl-W \|` | Set window width to exactly `n` columns. Without count, MUST maximize width. |
| `Ctrl-W =` | MUST equalize all sibling windows so they share the parent container evenly. |
| `:resize {n}` | Set window height to `n`. A `+` or `-` prefix MUST be treated as relative adjustment. |
| `:vertical resize {n}` | Set window width to `n`. Same relative prefix semantics. |

Windows MUST NOT be resized below their minimum dimension (see Configuration below).

## Closing

| Key / Command | Required behavior |
|---|---|
| `:close` / `Ctrl-W c` | MUST close the current window. If the buffer has unsaved changes and no other window displays it, MUST refuse and show error unless `!` is appended. |
| `:only` / `Ctrl-W o` | MUST close all windows in the current tab except the current one. Unsaved-change safeguards apply to each closed window. |
| `:quit` / `Ctrl-W q` | MUST close the current window. If it is the last window in the last tab, MUST initiate the quit flow. |
| `:hide` | MUST close the current window without writing. The buffer remains in the buffer list. |

When closing yields zero windows in a tab, the tab MUST be closed. When all tabs close, the editor MUST exit.

## Moving windows

| Key | Required behavior |
|---|---|
| `Ctrl-W r` | Rotate windows downward / rightward within the current container. |
| `Ctrl-W R` | Rotate windows upward / leftward within the current container. |
| `Ctrl-W x` | Exchange current window with the next sibling. With count, exchange with the `{n}`th sibling. |
| `Ctrl-W H` | Move current window to the far left, converting it to a full-height vertical split. |
| `Ctrl-W J` | Move current window to the bottom, converting it to a full-width horizontal split. |
| `Ctrl-W K` | Move current window to the top, converting it to a full-width horizontal split. |
| `Ctrl-W L` | Move current window to the far right, converting it to a full-height vertical split. |
| `Ctrl-W T` | Move current window into a new tab page. |

## Layout presets

| Preset | Required behavior |
|---|---|
| Horizontal stack | `:wincmd =` after all horizontal splits MUST distribute height equally. |
| Vertical stack | `:wincmd =` after all vertical splits MUST distribute width equally. |
| Grid | When four or more windows exist, `:GridLayout` SHOULD arrange them in the most square-like grid possible (e.g. 2x2, 2x3). |

Preset application MUST be reversible through undo or session restore.

## Configuration

| Setting | Default | Description |
|---|---|---|
| `window.split_direction` | `"right"` | Where `:vsplit` opens: `"left"` or `"right"`. |
| `window.split_below` | `true` | When `true`, `:split` opens the new window below; when `false`, above. |
| `window.min_height` | `1` | Minimum window height in rows. MUST be at least 1. |
| `window.min_width` | `1` | Minimum window width in columns. MUST be at least 1. |
| `window.focus_follows_mouse` | `false` | When `true`, moving the mouse into a window SHOULD focus it. |
| `window.equalalways` | `false` | When `true`, splits and closes MUST automatically equalize sibling sizes. |

## Decorations

| Element | Requirement |
|---|---|
| Separator lines | The editor MUST render a visible separator between adjacent windows. The separator character SHOULD default to `\|` for vertical and `---` for horizontal but MUST be configurable. |
| Active highlight | The active window's separator or border MUST use a distinct highlight group so the user can identify which window has focus. |
| Title bar | Each window SHOULD optionally display a title bar showing the buffer name and modified indicator. The title bar MUST be togglable via `window.show_title`. |
| Line numbers | Gutter elements (line numbers, signs) MUST be rendered per-window, not globally. |

## Linked scrolling

When `scrollbind` is enabled on two or more windows, scrolling one MUST scroll all bound windows by the same line delta. The `cursorbind` option SHOULD additionally synchronize cursor line position across bound windows.

## Test requirements

| Test category | Minimum checks |
|---|---|
| Unit | split create, close, resize arithmetic, minimum-size enforcement, equalize |
| Integration | directional navigation across complex trees, move-to-edge re-parenting |
| PTY E2E | create splits, navigate, resize, close, verify layout matches expectation |

## Related

- Tab pages: [/docs/spec/features/window/tabs.md](/docs/spec/features/window/tabs.md)
- Floating windows: [/docs/spec/features/window/floating-windows.md](/docs/spec/features/window/floating-windows.md)
- Layout persistence: [/docs/spec/features/window/window-layouts.md](/docs/spec/features/window/window-layouts.md)
- Resize modes: [/docs/spec/features/window/window-resize-modes.md](/docs/spec/features/window/window-resize-modes.md)
- Wincmd reference: [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md)

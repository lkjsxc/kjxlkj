# Windows

Back: [/docs/spec/editor/README.md](/docs/spec/editor/README.md)

Windows are viewports over content sources (buffers or terminal PTY screens). They are core-owned layout state.

## Requirements

- Core owns the window tree / split layout.
- Rendering uses window snapshots; renderer never mutates layout.
- Cursor and viewport semantics are deterministic and mode-aware.

## Window types

| Type | Content source | Description |
|---|---|---|
| Buffer window | `BufferId` | Displays a text buffer; standard editing window. |
| Terminal window | `TerminalId` | Displays a terminal emulator screen backed by a PTY process. |

Both window types share the same `WindowId`, layout tree, navigation, resize, and split semantics. The renderer selects the appropriate rendering path based on the content source type.

## Window model

A window is defined by:

| Field | Type | Description |
|---|---|---|
| `window_id` | `WindowId` | Stable unique identifier. |
| `content` | enum: `Buffer(BufferId)` or `Terminal(TerminalId)` | What this window displays. |
| `cursor` | cursor state | Per-window cursor position. For buffer windows, this is a grapheme-based position. For terminal windows, this is the terminal cursor. |
| `viewport` | viewport state | Top line, left column, text area dimensions. |
| `options` | per-window options | wrap, line numbers, scrolloff, sidescrolloff. |

## Layout tree

The layout is a recursive tree of splits and leaf windows:

| Node type | Description |
|---|---|
| Leaf | A single window (`WindowId`). |
| Horizontal split | Children arranged top-to-bottom, each with a height weight. |
| Vertical split | Children arranged left-to-right, each with a width weight. |

The layout tree MUST support:

- Arbitrary nesting depth of splits.
- Integer cell rounding from floating-point weights for terminal output.
- Minimum window size enforcement (at least 1 row and 1 column for text area).

## Window navigation

| Key | Action |
|---|---|
| `Ctrl-w h` | Focus window to the left |
| `Ctrl-w j` | Focus window below |
| `Ctrl-w k` | Focus window above |
| `Ctrl-w l` | Focus window to the right |
| `Ctrl-w w` | Cycle to next window |
| `Ctrl-w W` | Cycle to previous window |
| `Ctrl-w p` | Focus previous (last active) window |

Navigation works identically for buffer and terminal windows.

## Window operations

| Operation | Description |
|---|---|
| Split | Create a new split, dividing the current window's space. |
| Close | Remove window from tree; rebalance layout. For terminal windows, send `SIGHUP` to PTY. |
| Resize | Change weight allocation; enforce minimums. For terminal windows, send `SIGWINCH`. |
| Move | Relocate window to a different edge of the layout (`Ctrl-w H/J/K/L`). |
| Zoom | Temporarily maximize a window to fill the entire editor grid. |

## Invariants

| Invariant | Requirement |
|---|---|
| No overlap | Windows MUST NOT overlap in the rendered grid (except floating windows). |
| Full coverage | The editor grid MUST be fully covered by windows plus separators. |
| Focus uniqueness | Exactly one window is focused at any time. |
| Stable IDs | `WindowId` values MUST remain stable across layout changes. |

## Related

- UI views/components: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)
- Layout UX: [/docs/spec/ux/layout.md](/docs/spec/ux/layout.md)
- Viewport behavior: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Terminal as window: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Window features: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
- Session persistence: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)

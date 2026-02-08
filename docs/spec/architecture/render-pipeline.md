# Render Pipeline Detail

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Detailed specification for converting editor state snapshots into terminal frames. This document extends [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md).

## Pipeline stages (normative)

The render task operates in five stages per frame cycle:

| Stage | Input | Output |
|---|---|---|
| 1. Receive snapshot | `watch` channel | `EditorSnapshot` (latest-wins) |
| 2. Build cell grid | snapshot + window geometry | 2D `CellGrid` per window |
| 3. Apply decorations | cell grid + diagnostics, search, cursor | decorated cell grid |
| 4. Diff against previous | current grid, previous grid | list of changed cells |
| 5. Flush | diff commands | raw bytes to stdout |

## EditorSnapshot contents (normative)

The snapshot MUST contain all data needed to render without querying core or services.

| Field | Type | Description |
|---|---|---|
| `sequence` | u64 | Monotonic sequence number for stale detection |
| `tabs` | array of Tab | Tab pages with layout trees |
| `active_tab` | index | Currently visible tab |
| `buffers` | map of BufferId to BufferSnapshot | Text content and metadata |
| `terminals` | map of TerminalId to TerminalSnapshot | Terminal screen buffers |
| `mode` | Mode | Current editing mode |
| `cmdline` | CmdlineState | Command-line content and cursor |
| `notifications` | array of Notification | Active notification messages |
| `diagnostics` | map of BufferId to DiagList | LSP diagnostics per buffer |
| `search` | SearchState | Active search pattern and matches |
| `theme` | Theme | Active color theme |
| `terminal_size` | (cols, rows) | Total terminal dimensions |

## Cell grid construction (normative)

For each visible window in the layout tree:

### Buffer windows

1. Compute the window's text area rectangle from the layout tree.
2. Subtract gutter width (line number digits + sign column + fold column) from `text_cols`.
3. For each visible buffer line (from `top_line`):
   a. Decompose the line into grapheme clusters using `unicode-segmentation`.
   b. Compute display width of each grapheme using `unicode-width` (UAX #11).
   c. If `wrap = true`, split into display rows using the wrapping algorithm from [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md).
   d. If `wrap = false`, extract the visible slice starting at `left_col`.
   e. For each grapheme in the visible slice, write to the cell grid: set `grapheme`, `width`, `fg`, `bg`, `attrs`.
   f. For width-2 graphemes, the next cell MUST be a continuation cell (`width = 0`, `grapheme = ""`, `is_wide_continuation = true`).
   g. For the cursor cell, apply cursor highlight. If the cursor is on a width-2 grapheme, both cells carry the highlight.

### Terminal windows

1. Copy the terminal's internal screen buffer cells directly to the window's cell grid area.
2. Apply the terminal cursor position if the terminal window is focused.
3. Terminal cells already have fg/bg/attrs set by the escape sequence parser.

### Gutter rendering

1. For each visible buffer line, render the line number in the gutter area.
2. Line number format: absolute, relative, or hybrid (configurable via `number` and `relativenumber` options).
3. The sign column renders diagnostic severity icons and git change markers.
4. The fold column renders fold indicators (`+` for collapsed, `|` for open).

### Statusline rendering

1. The statusline occupies one row at the bottom of each window.
2. Segments: mode indicator, file name, modification flag, cursor position (`line:col`), encoding, file type.
3. Segments MUST be configured via the statusline DSL from [/docs/spec/features/ui/statusline/statusline-dsl.md](/docs/spec/features/ui/statusline/statusline-dsl.md).

### Command line rendering

1. When in Command mode, the bottom row of the terminal displays the command-line content.
2. The prefix character (`:`, `/`, `?`) MUST be shown.
3. The command-line cursor position MUST be rendered.

## Diff rendering (normative)

To minimize terminal writes, the render task MUST compare the current cell grid against the previous frame's cell grid.

| Rule | Requirement |
|---|---|
| Cell comparison | Two cells are equal if all fields match (grapheme, width, fg, bg, attrs). |
| Dirty tracking | Only cells that differ from the previous frame produce terminal output. |
| Cursor movement | Use absolute cursor positioning (`CSI row;col H`) to skip unchanged regions. |
| Batch writes | Accumulate all escape sequences into a single buffer, then flush with one `write_all`. |
| Full redraw | On terminal resize, the entire grid MUST be redrawn (previous frame is invalidated). |

## Color rendering

| Terminal capability | Rendering strategy |
|---|---|
| True color (24-bit) | Emit `CSI 38;2;R;G;B m` for fg, `CSI 48;2;R;G;B m` for bg |
| 256-color | Map theme colors to nearest 256-color index, emit `CSI 38;5;N m` |
| 16-color | Map theme colors to ANSI indices, emit `CSI 3N m` |
| Detection | Use `$COLORTERM` and terminfo as specified in [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md) |

## Window separator rendering

Window separators MUST be rendered between adjacent windows.

| Separator | Character | Attrs |
|---|---|---|
| Vertical | `│` (U+2502) or `|` | Use `WinSeparator` highlight group |
| Horizontal | `─` (U+2500) or `-` | Use `WinSeparator` highlight group |

## Tab line rendering

When multiple tabs exist, a tab line MUST be rendered at the top of the terminal.

| Element | Description |
|---|---|
| Tab labels | Display the tab number and the active buffer name |
| Active tab | Highlighted with `TabLineSel` group |
| Inactive tabs | Use `TabLine` group |
| Fill | Remaining space uses `TabLineFill` group |

## Performance requirements

| Requirement | Detail |
|---|---|
| Incremental lines | Only visible lines are processed; off-screen lines MUST NOT be measured or rendered. |
| Width cache | Display width computations SHOULD be cached per line version to avoid re-computation. |
| Snapshot coalescing | Multiple rapid snapshots MUST be coalesced; only the latest is rendered. |
| Batched IO | All terminal output for one frame MUST be flushed in a single write syscall. |

## Related

- Runtime model: [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- Viewport management: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Theme system: [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md)
- Statusline: [/docs/spec/features/ui/statusline/README.md](/docs/spec/features/ui/statusline/README.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

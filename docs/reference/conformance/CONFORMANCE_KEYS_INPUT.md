# Conformance: Input Modes and Core Systems

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

Insert/Replace mode keybindings and core editing systems in the conformance ledger.

## Implementation status

| Area | Status | Evidence |
|------|--------|----------|
| Insert mode | `implemented` | dispatch_tests.rs, integration_tests.rs |
| Replace mode | `implemented` | boundary_tests (BD-12) |
| IME composition | `implemented` | ime module tests, REG-06 |
| Registers | `implemented` | register module tests |

## Insert mode

| Key | Action |
|---|---|
| `Esc` | Return to Normal mode |
| (any char) | Insert character at cursor |
| `Backspace` | Delete character before cursor |
| `Ctrl-h` | Delete character before cursor (same as Backspace) |
| `Enter` | Insert newline |
| `Ctrl-j` | Insert newline (same as Enter) |
| `Ctrl-m` | Insert newline (same as Enter) |
| Arrow keys | Move cursor |
| `Home` | Move to line start |
| `End` | Move to line end |
| `Delete` | Delete character at cursor |
| `Tab` | Insert tab character |
| `Ctrl-w` | Delete word before cursor |
| `Ctrl-u` | Delete to start of line |
| `Ctrl-t` | Indent current line |
| `Ctrl-d` | Outdent current line |
| `Ctrl-o` | Execute one Normal mode command then return to Insert |
| `Ctrl-r {reg}` | Insert contents of register |
| `Ctrl-v` | Insert literal character |
| `Ctrl-k {c1} {c2}` | Insert digraph character |

## Replace mode

| Key | Action |
|---|---|
| `Esc` | Return to Normal mode |
| (any char) | Replace character at cursor and advance |
| `Backspace` | Move cursor left |

At end of line, typed characters are inserted rather than replacing.

### Replace mode extended

| Feature | Behavior |
|---|---|
| `ReplaceState` | Tracks continuous vs single-char replace with original chars |
| `replace_char_at()` | Overwrites character at position, records original for undo |
| `undo_replace_at()` | Restores original character at position |
| `apply_single_replace()` | Single-character replace (`r`) without entering Replace mode |
| `is_valid_replacement()` | Validates replacement char (rejects control chars except tab/newline) |

### Insert mode extended

| Feature | Behavior |
|---|---|
| `delete_word_back()` | Ctrl-W: deletes word before cursor in insert mode |
| `delete_to_line_start()` | Ctrl-U: deletes from cursor to start of line |
| `indent_line()` | Ctrl-T: adds one level of indentation |
| `dedent_line()` | Ctrl-D: removes one level of indentation |
| `collect_completions()` | Gathers word completions from buffer matching prefix |

## Window splits

| Feature | Behavior |
|---|---|
| `LayoutNode` | Tree of Leaf (single window) and Split (h/v with children) |
| `compute_rects()` | Computes pixel rectangles for all windows from layout |
| `split_window()` | Splits a leaf into two windows (horizontal or vertical) |
| `remove_window()` | Removes a window, simplifying single-child splits |
| `WinRect` | x, y, w, h rectangle for each window |

## Latency tracking

| Feature | Behavior |
|---|---|
| `LatencyProbe` | Records samples, computes min/max/avg/p95 statistics |
| `LatencyBudgets` | KEYSTROKE=16ms, FRAME=16ms, RESIZE=50ms, SCROLL=8ms |
| `TimingGuard` | RAII guard that records elapsed time on drop |
| `is_idle_busy_loop()` | Detects >120fps redraw rate as busy-loop |
| `exceeds_budget()` | Checks if duration exceeds a latency budget |

## Buffer lifecycle

| Feature | Behavior |
|---|---|
| `LifecycleStage` | Created/Loading/Loaded/Saving/Closing/Closed state machine |
| `BufferLifecycle` | Tracks stage, path, dirty flag, swap state |
| `ModificationInfo` | Change counting with mark_changed/saved, changes_since_save |
| `SwapState` | Swap file path management (swap_path from file path) |
| `AutoSavePolicy` | Disabled/Interval(secs)/OnFocusLost auto-save policies |
| `buffers_needing_save()` | Filters collection for dirty buffers needing persistence |

## Viewport follow

| Feature | Behavior |
|---|---|
| `ViewportState` | top_line/left_col/visible_lines/visible_cols/scrolloff/sidescrolloff |
| `follow_cursor_v()` | Adjusts top_line to keep cursor visible with scrolloff |
| `follow_cursor_h()` | Adjusts left_col to keep cursor visible with sidescrolloff |
| `center_on_line()` | Centers viewport on a given line (zz) |
| `cursor_to_top()` / `cursor_to_bottom()` | Moves viewport so cursor is at top/bottom (zt/zb) |
| `scroll()` | Scrolls by delta lines with clamping |
| `scroll_percent()` | Reports percentage through file |

## Insert newline

| Feature | Behavior |
|---|---|
| `insert_newline()` | Splits line at cursor, applies auto-indent, returns NewlineResult |
| `AutoIndentMode` | None / CopyIndent / SmartIndent (extra indent after {/:/(/) |
| `open_line()` | Opens new line above (O) or below (o) with indent |
| `should_increase_indent()` | Detects trailing `{`, `(`, `[`, `:` for indent increase |
| `should_decrease_indent()` | Detects leading `}`, `)`, `]` for indent decrease |

## LSP requests

| Feature | Behavior |
|---|---|
| `LspMethod` | 15 methods: Initialize, Completion, Hover, Definition, etc. |
| `PendingRequests` | Send/complete lifecycle with auto-incrementing IDs |
| `DiagnosticStore` | Per-file diagnostics with severity filtering and error_count |
| `ServerCapabilities` | Feature flags (completion, hover, definition, rename, etc.) |
| `DiagnosticSeverity` | Error/Warning/Information/Hint (ordered) |

## Long line handling

| Feature | Behavior |
|---|---|
| `segment_line()` | Splits long lines into segments of max_width display columns |
| `safe_slice()` | Viewport-bounded string slicing by display column range |
| `display_width()` | Computes display width (tabs=8, wide chars=2) |
| `char_to_col()` / `col_to_char()` | Bidirectional char-index to display-column conversion |
| `MAX_RENDER_COLS` | 10,000 column rendering threshold |

## User functions

| Feature | Behavior |
|---|---|
| `UserFunction` | Name, params, body, varargs, script-local, range, abort |
| `FunctionRegistry` | define/get/remove/list with uppercase/s: enforcement |
| `parse_function()` | Parses `:function` definition from lines |
| `parse_call()` | Extracts function name and arguments from call expression |
| `is_script_local()` | Checks s: or `<SID>` prefix |

## Input timing

| Feature | Behavior |
|---|---|
| `Debouncer` | Signal/check_pending with configurable delay |
| `ResizeCoalescer` | Coalesces rapid resize events with settle timeout |
| `IdleDetector` | Fires once after no input for specified duration |
| `InputRateTracker` | Tracks event rate with sliding window, detects burst |

## Mappings engine

| Feature | Behavior |
|---|---|
| `MapMode` | Normal/Insert/Visual/Command/OperatorPending/Terminal |
| `MappingStore` | add/remove/lookup/has_prefix/list/clear |
| `parse_map_command()` | Parses :map/:nmap/:imap/:nnoremap etc. |
| `resolve_mapping()` | Resolves through recursive mappings with depth limit |
| Noremap | Non-recursive mappings stop after first resolution |

## Related

- Modes and normal keys: [/docs/reference/conformance/CONFORMANCE_MODES.md](/docs/reference/conformance/CONFORMANCE_MODES.md)
- Key systems: [/docs/reference/conformance/CONFORMANCE_KEYS_SYSTEMS.md](/docs/reference/conformance/CONFORMANCE_KEYS_SYSTEMS.md)

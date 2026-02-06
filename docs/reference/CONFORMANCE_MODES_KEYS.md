# Conformance: Modes and Keybindings

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
Mode set and keybindings in the conformance ledger.

In a docs-only baseline, treat this as the intended initial reconstruction target (update it after regenerating the implementation).

## Modes

| Mode | Entry | Exit | Notes |
|---|---|---|---|
| Normal | startup | N/A | Command/navigation mode |
| Insert | `i`, `a`, `A`, `o` | `Esc` | Text insertion |
| Command | `:` | `Esc`, `Enter` | Ex command entry |
| Visual | `v` | `Esc` | Charwise selection with operators |
| Visual Line | `V` | `Esc` | Linewise selection with operators |
| Visual Block | `Ctrl-v` | `Esc` | Block (rectangular) selection |
| Replace | `R` | `Esc` | Overwrites existing characters |
| Terminal | `<leader>t`, `:terminal` | `Esc` (to Normal) | Pass-through to PTY (scaffolded) |

## Normal-mode keys (subset)

| Key | Action |
|---|---|
| `h`/`j`/`k`/`l` | Cursor move left/down/up/right |
| Arrow keys | Cursor move |
| `Space` | Leader prefix (default leader); feature chords: `Space e` (explorer), `Space t` (terminal), `Space f` (find), `Space g` (livegrep), `Space b` (buffers), `Space u` (undotree) |
| `Backspace` | Move left (same as `h`) |
| `0` | Move to start of line (column 0) |
| `^` | Move to first non-blank character |
| `_` | Move to first non-blank (with count offset) |
| `g_` | Move to last non-blank character |
| `gm` | Move to middle of line |
| `$` | Move to end of line |
| `\|` | Go to column N (with count) |
| `w` | Move to next word start |
| `W` | Move to next WORD start (same as `w` currently) |
| `b` | Move to previous word start |
| `B` | Move to previous WORD start (same as `b` currently) |
| `e` | Move to word end |
| `E` | Move to WORD end (same as `e` currently) |
| `ge` | Move to previous word end |
| `gE` | Move to previous WORD end (same as `ge` currently) |
| `+` | Move to first non-blank of next line |
| `Enter` | Move to first non-blank of next line (same as `+`) |
| `-` | Move to first non-blank of previous line |
| `gg` | Move to file start |
| `G` | Move to file end |
| `{count}gg` | Go to line N |
| `{count}G` | Go to line N |
| `{count}%` | Go to N% of file |
| `H` | Move to top of visible screen |
| `M` | Move to middle of visible screen |
| `L` | Move to bottom of visible screen |
| `zz` | Scroll cursor to center of screen |
| `zt` | Scroll cursor to top of screen |
| `zb` | Scroll cursor to bottom of screen |
| `z<CR>` | Scroll cursor to top, move to first non-blank |
| `z.` | Scroll cursor to center, move to first non-blank |
| `z-` | Scroll cursor to bottom, move to first non-blank |
| `i` | Enter Insert mode |
| `I` | Enter Insert mode (first non-blank of line) |
| `a` | Enter Insert mode (after cursor) |
| `A` | Enter Insert mode (end of line) |
| `o` | Open line below and enter Insert mode |
| `O` | Open line above and enter Insert mode |
| `v` | Enter Visual mode |
| `V` | Enter Visual line mode |
| `R` | Enter Replace mode |
| `r{char}` | Replace character under cursor with {char} |
| `x` | Delete character under cursor |
| `X` | Delete character before cursor |
| `D` | Delete from cursor to end of line |
| `C` | Change from cursor to end of line |
| `s` | Substitute character under cursor (delete and enter Insert) |
| `S` | Substitute entire line (delete content and enter Insert) |
| `Y` | Yank current line (like `yy`) |
| `p` | Paste after cursor |
| `P` | Paste before cursor |
| `gp` | Paste after cursor, cursor at end of pasted text |
| `gP` | Paste before cursor, cursor at end of pasted text |
| `u` | Undo |
| `Ctrl-r` | Redo |
| `:` | Enter Command mode |
| `.` | Repeat last change |
| `/` | Search forward |
| `?` | Search backward |
| `n` | Repeat last search (same direction) |
| `N` | Repeat last search (opposite direction) |
| `*` | Search forward for word under cursor |
| `#` | Search backward for word under cursor |
| `g*` | Search forward for partial word under cursor |
| `g#` | Search backward for partial word under cursor |
| `m{a-z}` | Set local mark |
| `` ` ``{a-z} | Jump to mark (exact position) |
| `'{a-z}` | Jump to mark (line, first non-blank) |
| `"{a-z}` | Select register for next yank/delete/paste |
| `q{a-z}` | Start/stop macro recording |
| `@{a-z}` | Play macro from register |
| `@@` | Repeat last macro |
| `Ctrl-o` | Jump to older position in jump list |
| `Ctrl-i` | Jump to newer position in jump list |
| `g;` | Jump to older position in change list |
| `g,` | Jump to newer position in change list |
| `(` | Move to previous sentence |
| `)` | Move to next sentence |
| `{` | Move to previous paragraph |
| `}` | Move to next paragraph |
| `%` | Move to matching bracket |
| `[(` | Move to previous unmatched `(` |
| `])` | Move to next unmatched `)` |
| `[{` | Move to previous unmatched `{` |
| `]}` | Move to next unmatched `}` |
| `ZZ` | Write and quit |
| `ZQ` | Quit without saving |
| `J` | Join current line with next (adds space) |
| `gJ` | Join current line with next (no space) |
| `~` | Toggle case of character under cursor |
| `g~{motion}` | Toggle case over motion |
| `g~~` | Toggle case of entire line |
| `gU{motion}` | Uppercase over motion |
| `gUU` | Uppercase entire line |
| `gu{motion}` | Lowercase over motion |
| `guu` | Lowercase entire line |
| `Ctrl-a` | Increment number under cursor |
| `Ctrl-x` | Decrement number under cursor |
| `Ctrl-6` / `Ctrl-^` | Switch to alternate/previous buffer |
| `Ctrl-g` | Display file info (same as `:file`) |
| `gv` | Reselect last visual selection |
| `Ctrl-d` | Scroll half page down |
| `Ctrl-u` | Scroll half page up |
| `Ctrl-f` | Scroll full page down |
| `Ctrl-b` | Scroll full page up |
| `Ctrl-e` | Scroll one line down (cursor stays) |
| `Ctrl-y` | Scroll one line up (cursor stays) |

## Visual mode

| Key | Action |
|---|---|
| `h`/`j`/`k`/`l` | Extend selection (cursor movement) |
| `w`/`b`/`e` | Extend selection by word |
| `0`/`^`/`$` | Extend selection to line boundaries |
| `gg`/`G` | Extend selection to file boundaries |
| `d`/`x` | Delete selection |
| `y` | Yank selection |
| `c`/`s` | Change selection (delete and enter Insert mode) |
| `o` | Swap cursor to other end of selection |
| `>` | Indent selection |
| `<` | Outdent selection |
| `Esc` | Cancel selection, return to Normal mode |

Visual Line mode (`V`) operates on entire lines for all operators.

Visual Block mode (`Ctrl-v`) operates on rectangular regions:

| Key | Action |
|---|---|
| `h`/`j`/`k`/`l` | Extend block selection |
| `d`/`x` | Delete block (rectangular region from each line) |
| `y` | Yank block |
| `c` | Change block (delete and enter Insert mode) |
| `Esc` | Cancel selection, return to Normal mode |

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

### UI Views and Tabs (`ui_views.rs` — core-edit)

| Aspect | Status |
| --- | --- |
| `ViewKind` | Buffer / Terminal / Explorer / Help / Preview / QuickFix / LocationList / Empty |
| `View` | Typed view with id, kind, active flag + buffer/terminal/explorer constructors |
| `TabPage` | Ordered view collection with add/remove |
| `ViewManager` | Multi-tab view management with create_view/close_view/new_tab/active_view |
| `tab_line_label()` | Format tab label with view count indicator |

### Terminal Pane Management (`terminal_full.rs` — service-terminal)

| Aspect | Status |
| --- | --- |
| `TerminalPane` | Pane with resize and scroll region support |
| `PaneManager` | Create/close/get/set_active/list panes |
| `TmuxState` / `TmuxAction` | Tmux session state and action dispatch (8 variants) |
| `map_tmux_key()` | Map key names to tmux key sequences |
| `scrollback_capacity()` | Compute scrollback buffer size with 10K cap |

### Contracts System (`contracts.rs` — core-types)

| Aspect | Status |
| --- | --- |
| `ContractChecker` | Collect violations (non-strict) or panic (strict mode) |
| `require()` / `ensure()` / `invariant()` | Precondition / postcondition / invariant enforcement |
| `Violation` | Structured record with level, module, message, Display impl |
| `in_range()` / `non_empty()` / `valid_buffer_id()` / `within_limit()` | Contract helper predicates |

## Related

- Editing semantics: [/docs/reference/CONFORMANCE_EDITING.md](/docs/reference/CONFORMANCE_EDITING.md)

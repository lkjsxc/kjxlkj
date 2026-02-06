# Conformance: Ex Commands and Testing

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
Command-line (Ex) command subset and headless/E2E surface in the conformance ledger.

In a docs-only baseline, treat this as the intended initial reconstruction target (update it after regenerating the implementation).

## Command-line (Ex) commands (subset)

| Command | Behavior |
|---|---|
| `:q` / `:q!` | Quit (forced with `!`). Refuses if buffer is modified unless forced. |
| `:qa` / `:qa!` | Quit all (forced with `!`) |
| `:w` | Write to current buffer path (if set) |
| `:w {file}` | Write to `{file}` |
| `:wa` / `:wall` | Write all buffers |
| `:wq` / `:x` / `:exit` | Write then quit |
| `:wq {file}` | Write to `{file}` then quit |
| `:e {file}` / `:e! {file}` | Edit file (forced with `!`) |
| `:ls` / `:buffers` | List open buffers |
| `:bn` / `:bnext` | Go to next buffer |
| `:bp` / `:bprev` / `:bprevious` | Go to previous buffer |
| `:bd` / `:bdelete` | Delete current buffer |
| `:bd!` / `:bdelete!` | Force delete current buffer |
| `:sp` / `:split` | Split window horizontally (stub) |
| `:vsp` / `:vsplit` | Split window vertically (stub) |
| `:new` | Open new empty buffer in split (stub) |
| `:vnew` | Open new empty buffer in vertical split (stub) |
| `:only` | Close all other windows (stub) |
| `:set {option}` | Set editor option (number, nonumber, wrap, nowrap, etc. — 22 boolean + 4 numeric) |
| `:{number}` | Go to line number |
| `:! {cmd}` | Run `{cmd}` via terminal service and display first output line as status |
| `:{range}!{cmd}` | Filter lines through external command |
| `:s/pattern/replacement/` | Substitute on current line |
| `:s/pattern/replacement/g` | Substitute all occurrences on current line |
| `:g/pattern/d` | Delete all lines matching pattern |
| `:g/pattern/command` | Execute command on matching lines |
| `:v/pattern/d` | Delete all lines NOT matching pattern (inverted global) |
| `:enew` | Open new empty buffer in current window |
| `:saveas {file}` | Save buffer to new file path |
| `:b {N}` / `:buffer {N}` | Switch to buffer by number |
| `:b#` | Switch to alternate buffer |
| `:scratch` | Create a scratch (unlisted) buffer |
| `:marks` | Display marks |
| `:reg` / `:registers` | Display registers |
| `:jumps` | Display jump list |
| `:changes` | Display change list |
| `:digraphs` / `:dig` | Display digraph table |
| `:file` / `:f` | Display current file info |
| `:sort` | Sort lines in buffer |
| `:noh` / `:nohlsearch` | Clear search highlighting |
| `:cnext` / `:cn` | Jump to next quickfix entry |
| `:cprev` / `:cp` | Jump to previous quickfix entry |
| `:copen` / `:clist` | Show quickfix list |
| `:messages` / `:mes` | Display messages |
| `:source {file}` | Source a configuration file |
| `:execute {expr}` | Execute expression as Ex command |
| `:normal {keys}` | Execute normal-mode keys on current/range lines |
| `:syntax on\|off` | Toggle syntax highlighting |
| `:highlight {group}` | Display highlight group info |
| `:map` / `:nmap` / `:imap` / ... | Create key mappings |
| `:unmap` / `:nunmap` / ... | Remove key mappings |
| `:mapclear` | Clear all mappings for a mode |
| `:autocmd` / `:au` | Create autocommand |
| `:d` / `:delete` | Delete lines (with range) |
| `:y` / `:yank` | Yank lines (with range) |
| `:t` / `:copy` | Copy lines (with range) |
| `:m` / `:move` | Move lines (with range) |
| `:r` / `:read` | Read file into buffer |
| `:put` | Put register contents |
| `:filetype` / `:ft` | Set/display filetype |
| `:cd {dir}` | Change working directory |
| `:pwd` | Display working directory |
| `:mksession` / `:mks` | Save session |
| `:oldfiles` / `:ol` | Display recent files |

## Ranges and addresses

| Syntax | Meaning |
|---|---|
| `%` | Entire file (all lines) |
| `{N}` | Line number N (1-indexed in command, 0-indexed internally) |
| `.` | Current line |
| `$` | Last line |
| `'{a-z}` | Line of mark (basic support) |
| `{addr}+{N}` / `{addr}-{N}` | Offset from address |
| `{addr1},{addr2}` | Range from addr1 to addr2 |

## Scripting types

| Component | Behavior |
|---|---|
| `CompletionRegistry` | Register/unregister completion providers by name+kind |
| `UserCommandRegistry` | Define/remove user-defined Ex commands with `:command` semantics |
| `UserFunctionRegistry` | Define/remove user-defined functions with `:function` semantics |
| `Scheduler` | Timer/debounce management for deferred command execution |

## UI types

| Component | Behavior |
|---|---|
| `CursorState` | Per-snapshot cursor position, shape, blink, visibility |
| `CursorHint` | Show/Hide hint for TUI layer from viewport coordinates |
| `CursorConfig` | Per-mode cursor shape and blink configuration |
| `LineNumberStyle` | None/Absolute/Relative/Hybrid line number display |
| `ModeIndicatorFormat` | Uppercase/Short/Char/Hidden mode indicator format |
| `ComponentId/Kind` | Typed component identifiers for layout tree (BufferView, StatusLine, etc.) |
| `Rect` | Screen rectangle with split operations for layout composition |
| `LayoutNode` | Component layout node with id, kind, rect, visibility |
| `standard_layout()` | Compute standard editor layout (tab line, buffer, status, command) |

## Runtime types

| Component | Behavior |
|---|---|
| `RuntimePhase` | Init → CoreStarting → ServicesStarting → Running → ShuttingDown → Terminated |
| `RestartPolicy` | Never / Limited(N) / Always restart on service failure |
| `ServiceLifecycle` | Tracks restart count, status, errors, backoff for supervised services |
| `BusCapacity` | Bounded message bus with send/receive/utilization tracking |

## Explorer bridge

| Component | Behavior |
|---|---|
| `dispatch_explorer_action()` | Wires ExplorerAction::OpenFile to `:e {path}` command dispatch |

## Theme types

| Component | Behavior |
|---|---|
| `ThemePalette` | 17 semantic colors (fg, bg, cursor, selection, keyword, etc.) with per-group style resolution |
| `Theme` | Named theme with palette + highlight group overrides |
| `ThemeRegistry` | Registry of named themes with active theme switching |
| `theme_dark/light/gruvbox()` | Three built-in theme definitions |

## Floating window types

| Component | Behavior |
|---|---|
| `FloatBorder` | None/Single/Double/Rounded/Solid/Shadow border styles |
| `FloatConfig` | Anchor, size (fixed/percent), border, row/col offset, zindex |
| `ZoomState` | Normal/Zoomed toggle with restore dimensions |
| `LayoutPreset` | Single/EqualH/EqualV/MainLeft/MainTop/Grid layout computation |
| `ResizeOp` | IncrementH/W, SetH/W, Maximize, Equalize with clamping |
| `WinCmd` | Split/Close/Focus/Rotate/Exchange/Resize/Zoom commands |

## Search highlight types

| Component | Behavior |
|---|---|
| `SearchMatch` | Start/end position + is_current flag for each match |
| `SearchHighlights` | Match collection with next/prev cycling, hlsearch toggle, visible filtering |

## Undo branching types

| Component | Behavior |
|---|---|
| `BranchingUndoTree` | Tree-structured undo with parent/children, branch selection, path traversal |
| `NodeId` / `ChangeEntry` | Typed node IDs and forward/reverse patch entries |

## DAP debugging types

| Component | Behavior |
|---|---|
| `DapState` | Debug session state with breakpoints, stack frames, variables |
| `Breakpoint` | Line/conditional/logpoint/function/data breakpoints with toggle |
| `StackFrame` / `Variable` | Call stack traversal and variable inspection |

## Extended marks types

| Component | Behavior |
|---|---|
| `MarkScope` | Local (a-z, special) vs Global (A-Z) mark classification |
| `Mark` / `MarkRegistry` | Per-buffer local marks, cross-buffer global marks, special marks (`[`, `]`, `<`, `>`) |

## Substitute flags types

| Component | Behavior |
|---|---|
| `SubstituteFlags` | Parse g/c/i/I/n/& flags from `:s///flags` |
| `ConfirmState` | Interactive `:s///c` confirmation with Yes/No/All/Quit responses |
| `parse_substitute_cmd()` | Parse full `:s/pattern/replacement/flags` command string |

## Extended completion types

| Component | Behavior |
|---|---|
| `CompletionItemKind` | 25 LSP completion kinds with icon() and from_lsp() mapping |
| `CompletionItemEx` | Rich completion item with filter_text, sort_text, preselect, deprecated |
| `CompletionList` | Filterable/selectable completion list with prefix matching |

## Buffer list types

| Component | Behavior |
|---|---|
| `BufferFilter` | All/Listed/Unlisted/Modified/Active buffer filtering |
| `BufferListEntry` | Buffer metadata with flags() for `:ls` display formatting |
| `build_buffer_list()` | Build filtered buffer list from EditorState buffers |

## Visual block types

| Component | Behavior |
|---|---|
| `BlockSelection` | Two-corner block selection with line/col range, height/width |
| `BlockOp` / `BlockEdit` | Insert/Append/Change/Delete block operations with per-line edits |
| `extend_to_eol()` | Extend block selection to end-of-line per row (like `$` in visual block) |

## Command-line completion types

| Component | Behavior |
|---|---|
| `complete_command()` | Prefix-match against 55 built-in command names with scoring |
| `complete_option()` | Prefix-match against 21 `:set` option names |
| `complete_buffer()` | Match buffer names by prefix or substring |
| `detect_completion_kind()` | Infer completion type from command-line context (set→option, buffer→buffer, edit→path) |

## Keybinding DSL types

| Component | Behavior |
|---|---|
| `KeyChord` | Parsed key with ctrl/alt/shift modifiers and display() round-trip |
| `parse_key_sequence()` | Parse `<C-x>`, `<M-a>`, `<leader>`, `<CR>`, combined modifiers, plain chars |
| `validate_key_sequence()` | Check for unclosed `<` brackets and syntax errors |
| `resolve_special()` | Map CR/Esc/BS/Tab/Space/arrows/Del/Home/End/PageUp/PageDown to canonical names |

## View tree types

| Component | Behavior |
|---|---|
| `FocusTarget` | Editor/CommandLine/Explorer/Popup(id)/Notification focus targets |
| `ViewNode` | Tree node with id, kind, rect, focusable flag, and children |
| `ViewTree` | Root view tree with focus stack (push/pop), from_splits() layout builder |

## Popup menu types

| Component | Behavior |
|---|---|
| `PopupMenu` | Item list with selection cycling, scroll window, anchor-based positioning |
| `HoverTooltip` | Positioned text tooltip with dismiss |
| `PopupAnchor` | AboveCursor/BelowCursor/ScreenCenter/AtPosition anchor modes |
| `compute_rect()` | Calculate popup rectangle constrained to screen bounds |

## Status line layout types

| Component | Behavior |
|---|---|
| `StatusSection` | Left/Center/Right aligned section with priority |
| `StatusLineLayout` | Compose sections into fixed-width rendered line |
| `vim_default()` | Standard Vim-like status format: mode, filename, modified, line:col, percent |

## Contract checker types

| Component | Behavior |
|---|---|
| `ContractChecker` | Accumulate pass/fail contract results with summary |
| `check_viewport_bounded()` | Verify snapshot doesn't clone entire buffer |
| `check_input_ordering()` | Verify monotonic input sequence numbers |
| `check_bus_utilization()` | Warn when message bus >90% capacity |
| `check_no_plugin_loading()` | Assert no dynamic plugin loading (built-in only) |
| `check_restart_limit()` | Verify service restart count within policy |

## User command execution types

| Component | Behavior |
|---|---|
| `ExecResult` | Ok / Error(message) / NoSuchCommand result from user command dispatch |
| `validate_nargs()` | Validate argument count against NArgs spec (0/1/*/+/?) |
| `substitute_args()` | Replace `<args>`, `<bang>`, `<line1>`, `<line2>`, `<count>`, `<q-args>` in command template |
| `execute_user_command()` | Look up registered command + validate nargs + substitute args |
| `dispatch_user_command()` | Full dispatch: find command, validate, substitute, return expanded command body |

## User function execution types

| Component | Behavior |
|---|---|
| `FuncResult` | Value(String) / Void / Error(String) result from function execution |
| `FuncContext` | Function execution context with args, locals, return value tracking |
| `execute_function()` | Interpret function body: let assignments, return, concat expressions |
| `parse_let()` | Parse `let var = expr` assignments within function body |
| `resolve_expression()` | Resolve variable references (`a:`, `l:` scope) and string concatenation |

## Debounce manager types

| Component | Behavior |
|---|---|
| `FakeClock` | Deterministic fake clock for testing with advance() |
| `PendingAction` | Scheduled action with deadline and coalesced_count |
| `DebounceManager` | Schedule/cancel/tick debounce actions with coalescing |
| `fired_actions()` | Report which actions fired after tick based on fake clock time |

## Mapping expansion types

| Component | Behavior |
|---|---|
| `MappingEntry` | Mode-scoped mapping from trigger keys to replacement keys |
| `ExpansionResult` | Expanded(keys) / NoMapping / RecursionLimit result |
| `expand_mapping()` | Longest-prefix match for one-level mapping expansion |
| `expand_recursive()` | Recursive expansion with MAX_DEPTH=100 guard |
| `has_prefix_match()` | Check if partial input has a potential mapping prefix |
| `list_mappings()` | List all mappings for a given mode |

## Accessibility types

| Component | Behavior |
|---|---|
| `ContrastRatio` | WCAG 2.1 relative luminance and contrast ratio computation |
| `luminance()` | Compute relative luminance from sRGB (0-255) tuple |
| `contrast_ratio()` | Compute L1/L2 contrast ratio between two colors |
| `FocusIndicator` | Underline/Reverse/Bold/HighContrast focus indicator styles |
| `A11yCheck` | pass/fail named accessibility check result |
| `check_color_scheme()` | Verify foreground/background meet WCAG AA 4.5:1 minimum |
| `check_focus_visible()` | Verify focus indicator is not None |
| `AriaHint` | Status/Editor/Menu ARIA role hints for screen readers |

## Profiling types

| Component | Behavior |
|---|---|
| `ProfilingSpan` | Named span with start/finish timestamps and duration_us |
| `Counter` | Named event counter |
| `Profiler` | Enable/disable profiling, begin_span/end_span/count/report/reset |
| `report()` | Aggregate span durations and counter values into summary |

## Event automation types

| Component | Behavior |
|---|---|
| `EventFired` | Result of firing an event: event type, filename, collected commands |
| `fire_event()` | Collect matching autocommand handlers for event + filename pattern |
| `fire_event_chain()` | Fire a sequence of events and collect all resulting commands |
| `has_handler_for()` | Check if any handler is registered for event + filename |
| `parse_spec_event()` | Map spec event names (buffer_new, insert_enter, etc.) to AutoEvent variants |

## Script loader types

| Component | Behavior |
|---|---|
| `ScriptLine` | Parsed line: Blank / Comment / SetOption / Mapping / AutoCmd / ExCommand |
| `parse_script_line()` | Parse a single script line into ScriptLine variant |
| `parse_script()` | Parse entire script file into Vec of ScriptLine |
| `execute_script_lines()` | Execute parsed script, counting commands and collecting errors |

## Keyboard layout types

| Component | Behavior |
|---|---|
| `KeyboardLayout` | Qwerty / Dvorak / Colemak / Workman / Custom layout enum |
| `LayoutRemapper` | Layout-aware key remapper with preserve_hjkl option |
| `parse_layout()` | Parse layout name string to KeyboardLayout enum |
| `dvorak_remaps()` | QWERTY-position hjkl remapping for Dvorak (d→h, h→j, t→k, n→l) |

## Viewport wrap types

| Component | Behavior |
|---|---|
| `DisplayRow` | Display row referencing buffer line, wrap offset, and column count |
| `DisplayMap` | Collection of display rows with lookup by buffer position |
| `compute_display_rows()` | Compute display rows from line widths and viewport width |
| `effective_scrolloff()` | Clamp scrolloff to half-viewport per spec invariants |
| `follow_cursor_wrap()` | Cursor-follow algorithm for wrap model — adjust top_line |

## Supervisor types

| Component | Behavior |
|---|---|
| `HealthStatus` | Healthy / Degraded / Failed / Stopped service health |
| `RestartDecision` | Restart / GiveUp / Backoff(ms) decision from failure |
| `SupervisorConfig` | max_restarts, backoff_base_ms, backoff_max_ms configuration |
| `ServiceState` | Tracked service with name, status, restart_count, uptime |
| `compute_backoff()` | Exponential backoff with cap for restart delay |

## Extended text object types

| Component | Behavior |
|---|---|
| `find_sentence()` | Sentence text object with `.`/`!`/`?` boundary detection |
| `find_paragraph_ext()` | Paragraph text object with blank-line boundary detection |
| `find_argument()` | Function argument text object between `,` and `(`/`)` boundaries |

## Headless test runner

This conformance target includes a deterministic headless mode intended for E2E tests and CI-like environments:

- `--headless` runs without a terminal UI.
- `--script {path}` runs an event script (if provided).
- An optional positional `{file}` argument loads a file into the initial buffer before the script runs.

### Script format

The headless script file is UTF-8 JSON in one of two accepted shapes:

| Shape | Description |
|---|---|
| Steps array | A JSON array of `ScriptStep` objects. Each element is an object with a `kind` discriminator. |
| Keys array | A JSON array of `ScriptKey` objects. Each element is a key description without `kind`. |

#### `ScriptKey` schema (used by both shapes)

| Field | Type | Required | Meaning |
|---|---:|:---:|---|
| `code` | string | yes | Key identity (either a single character, or a named special key). |
| `ctrl` | boolean | no | Whether Ctrl is held. Defaults to `false`. |
| `alt` | boolean | no | Whether Alt/Meta is held. Defaults to `false`. |
| `shift` | boolean | no | Whether Shift is held. Defaults to `false`. |

Named `code` values supported by this conformance target:

| `code` | Meaning |
|---|---|
| `Escape`, `Esc` | Escape key |
| `Enter`, `Return` | Enter/Return key |
| `Backspace` | Backspace key |
| `Tab` | Tab key |
| `Left`, `Right`, `Up`, `Down` | Arrow keys |

If `code` is not a named value above, it MUST be a single-character string.

#### `ScriptStep` kinds

| `kind` | Additional fields | Effect |
|---|---|---|
| `key` | `code`, optional `ctrl`/`alt`/`shift` | Inject one key event. |
| `keys` | `keys` (string) | Inject literal characters, one per Unicode scalar value. |
| `assert_mode` | `mode` (string) | Assert the current editor mode. |
| `assert_cursor` | `line` (integer), `col` (integer) | Assert the cursor position (0-based). |
| `assert_line` | `line` (integer), `content` (string) | Assert an exact line string match (0-based). |

Mode strings accepted by this conformance target include `normal`, `insert`, `visual`, `visual_line`, `visual_block`, `command`, `replace`, `terminal` (case-insensitive, with a small set of aliases).

## E2E test coverage

Once an implementation claims conformance to this document, it MUST include automated tests for at least the following E2E scenarios:

| Test | Behavior verified |
|---|---|
| headless_mode_starts | Editor starts, quits on `:q` |
| insert_mode | `i` enters Insert mode, text entry works |
| cursor_movement | `h/j/k/l` move cursor correctly |
| command_mode | `:` enters Command mode |
| visual_mode | `v` enters Visual mode |
| visual_line_mode | `V` enters Visual-Line mode |
| replace_mode | `R` enters Replace mode |
| append_mode | `a` appends after cursor |
| append_at_eol | When cursor is on last character, `a` appends at true end-of-line (no off-by-one) |
| open_line_below | `o` opens line below |
| open_line_above | `O` opens line above |
| text_insert_delete | Insert mode text entry and backspace |
| word_motions | `w/b` word movement |
| line_motions | `0/$` line start/end |
| file_motions | `gg/G` file start/end |
| typing_burst | 50 chars typed rapidly in insert mode |
| scroll_burst | 20 lines scrolled rapidly with `j` |
| mode_switch_burst | 10 rapid Normal/Insert mode switches |
| input_ordering | Verify input sequence order preserved |

These scenarios are primarily **headless** (no terminal UI). The project also requires **interactive PTY-driven E2E** tests for bugs that can hide in terminal decoding, key-chord parsing, focus/routing, or render-loop behavior. See [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md).

## Command history

| Feature | Behavior |
|---|---|
| `CommandHistory.push()` | Adds entry, deduplicates consecutive identical entries |
| `CommandHistory.prev()` | Navigate to previous history entry |
| `CommandHistory.next()` | Navigate to next history entry |
| `search_prefix()` | Find previous entry matching prefix (backward search) |
| `search_prefix_forward()` | Find next entry matching prefix (forward search) |
| `search_substring()` | Find entries containing substring |

## Git status and diff

| Feature | Behavior |
|---|---|
| `FileStatus` | Unmodified, Modified, Added, Deleted, Renamed, Untracked, Ignored, Conflicted |
| `StatusEntry` | Path + status + staged flag |
| `parse_diff_hunks()` | Parses unified diff text into `DiffHunk` with lines |
| `compute_gutter_signs()` | Produces `(line, GutterSign)` from hunks |
| `parse_blame_output()` | Parses blame lines into `BlameLine` entries |

## Terminal emulator

| Feature | Behavior |
|---|---|
| `TerminalGrid` | Cell grid with cursor position and current style |
| `put_char()` | Write character at cursor, advance with wrapping |
| `clear()` / `clear_to_eol()` | Clear screen or to end of line |
| `scroll_up()` | Scroll grid up by one line |
| `parse_ansi_simple()` | Parse subset of ANSI escapes (cursor move, clear, SGR) |

## Large buffer support

| Feature | Behavior |
|---|---|
| `LoadStrategy` | Full (≤1MB), Chunked (≤100MB), Streamed (>100MB) |
| `build_line_index()` | Byte offsets of each line start |
| `compute_chunks()` | Split file into fixed-size chunks |
| `annotate_chunks_with_lines()` | Map chunks to line ranges |
| `extract_line_range()` | Extract text for line range using index |

## Range and address parsing

| Feature | Behavior |
|---|---|
| `Address` | CurrentLine (`.`), LastLine (`$`), LineNumber, Mark (`'a`), ForwardSearch (`/pat/`), BackwardSearch (`?pat?`), Offset |
| `Range` | None, Single, FromTo (`,` separated), Entire (`%`) |
| `parse_range()` | Parses range string into Range enum |
| `parse_address()` | Parses single address with offset support |
| `resolve_range()` | Resolves range to (start, end) line numbers |

## Plugin prevention

| Feature | Behavior |
|---|---|
| `audit_source()` | Scans source for forbidden patterns (dlopen, libloading, PluginManager, etc.) |
| `audit_files()` | Batch audit across multiple files |
| `check_dependencies()` | Verifies no forbidden crate dependencies |
| `verify_architecture_rule()` | Validates architecture description matches no-plugin rule |

## Session commands

| Feature | Behavior |
|---|---|
| `SessionData` | Working dir, open files, cursor positions, window layout |
| `serialize_session()` | Saves session to script format |
| `parse_session()` | Restores session from script |
| `WindowLayout` | Single, Horizontal, Vertical split trees |

## Buffer metadata

| Feature | Behavior |
|---|---|
| `BufferInfo` | id, name, modified, readonly, listed, loaded, line_count |
| `AlternateFile` | Tracks current/alternate buffer for Ctrl-^ switching |
| `BufferVariables` | Buffer-local variable store (b:var equivalent) |
| `format_buffer_info()` | Formats buffer for `:ls` display |

## Syntax commands

| Feature | Behavior |
|---|---|
| `parse_syntax_command()` | Parses `:syntax on/off/manual/enable/disable` |
| `detect_language()` | Maps file extension to language name (21 languages) |
| `parse_filetype_command()` | Parses `:setfiletype` argument |
| `format_syntax_info()` | Displays `syntax=on/off filetype=...` status |

## Command-line parser

| Feature | Behavior |
|---|---|
| `CmdlineState` | Tracks prefix, content, cursor position, history index |
| `CmdlineAction` | 16 editing actions (insert, delete, move, history, etc.) |
| `map_cmdline_key()` | Maps keys/ctrl/special to command-line actions |
| Editing | InsertChar, DeleteBack, DeleteWord, DeleteToStart, Move* |
| History | HistoryPrev, HistoryNext via Up/Down/Ctrl-p/Ctrl-n |

## File explorer

| Feature | Behavior |
|---|---|
| `ExplorerTree` | Tree model with NodeId, expand/collapse, hidden files |
| `TreeNode` | id, name, path, kind (File/Directory/Symlink), depth |
| `visible_nodes()` | Depth-first traversal respecting expanded/hidden/filter |
| `format_node()` | Renders with indentation and directional icons (▶/▼) |
| `GitBadge` | Modified/Added/Deleted/Untracked/Ignored/Conflict/Clean |

## File I/O commands

| Feature | Behavior |
|---|---|
| `FileCommand` | Write/Edit/SaveAs/WriteQuit/WriteAll/Reload variants |
| `parse_file_command()` | Parses `:w`, `:e`, `:saveas`, `:wq`, `:wa`, `:e!` from input |
| `validate_write()` | Checks path exists, permissions, directory validity |
| `expand_tilde()` | Expands `~/` to home directory in file paths |
| `buffer_title()` | Derives display title from path or `[No Name]` |
| `display_path()` | Shortens home-relative paths with `~/` prefix |

## Completion engine

| Feature | Behavior |
|---|---|
| `CompletionSource` | Command/Path/Option/Buffer/Help/ColorScheme/Custom |
| `CompletionState` | Tracks candidates, index; next/prev/current/reset cycling |
| `detect_source()` | Infers completion source from cmdline prefix context |
| `complete_commands()` | Filters built-in command names by prefix |
| `complete_paths()` | Filesystem path completion with directory awareness |
| `common_prefix()` | Computes longest common prefix for menu narrowing |

## Config options

| Feature | Behavior |
|---|---|
| `OptionScope` | Global/Buffer/Window scope hierarchy |
| `ConfigStore` | Define/get/set/resolve options with scope precedence |
| `parse_set_arg()` | Parses `:set` arguments into SetAction variants |
| `SetAction` | ShowAll/Query/SetBool/SetInt/SetStr/Invalid |
| `build_defaults()` | 10 built-in options (number, wrap, tabstop, etc.) |

## Session persistence

| Feature | Behavior |
|---|---|
| `SessionState` | Marks, jumps, registers, history, buffer positions |
| `add_mark()` / `add_jump()` | Capped collections (100 marks, 100 jumps) |
| `add_register()` | Deduplicates by name, stores linewise flag |
| `add_history()` | Capped at 1000 entries, supports Command/Search/Input/Debug kinds |
| `serialize_session()` | JSON summary serialization |
| `filter_history()` | Filters history entries by HistoryKind |

## Event automation

| Feature | Behavior |
|---|---|
| `AutoEvent` | 17 events: BufEnter/Leave/Read/Write, Insert*, Cursor*, Win*, Vim*, FileType, etc. |
| `AutoPattern` | All / Glob (*.ext) / FileType matching |
| `AutoCmdRegistry` | Add/match/clear_group/remove_once_fired autocommands |
| `fire_event()` | Collects matching commands for an event firing |
| `once` flag | Single-fire autocommands removed after execution |

## Script files

| Feature | Behavior |
|---|---|
| `ScriptFile` | Parsed script with path and command lines |
| `ScriptLine` | ExCommand / Comment / Blank / Conditional variants |
| `parse_script()` | Parses file content into script lines |
| `executable_commands()` | Extracts only executable (non-comment/blank) lines |
| `SourceTracker` | Tracks sourced files, prevents double-sourcing |
| `resolve_source_path()` | Searches directories for script files with .vim fallback |

## User commands

| Feature | Behavior |
|---|---|
| `UserCommandDef` | Name, replacement, nargs, range/bang/bar/complete flags |
| `NArgs` | Zero/One/Any/AtLeastOne/ZeroOrOne with validation |
| `UserCommandRegistry` | define/get/remove/list/expand with uppercase enforcement |
| `expand()` | Substitutes `<args>`, `<q-args>`, `<bang>` in replacement |
| `parse_command_def()` | Parses `:command` arguments into definition |

### Notification Dispatch (`notification_dispatch.rs` — services)

| Aspect | Status |
| --- | --- |
| `Dispatcher` | Route notifications with severity filtering, auto-dismiss, max-visible limit |
| `Severity` | Debug / Info / Warning / Error with Ord ordering |
| `NotifySource` | Editor / Lsp / Plugin / Git / System source classification |
| `dismiss()` / `dismiss_source()` | Dismiss individual or by source |
| `gc()` | Garbage-collect old notifications past auto_dismiss_ms |
| `format_notification()` | Formats notification with severity prefix [D]/[I]/[W]/[E] |

### Git Full Integration (`git_full.rs` — service-git)

| Aspect | Status |
| --- | --- |
| `parse_diff()` | Parse unified diff format into DiffHunks with DiffLines |
| `parse_hunk_header()` | Extract line ranges from @@ hunk headers |
| `parse_log()` | Parse git log output into LogEntry records |
| `BlameEntry` / `BranchInfo` | Structured blame and branch metadata |
| `compute_signs()` | Map diff hunks to gutter GitSign indicators |
| `count_changes()` | Aggregate added/removed line counts |

## Related

- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

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

## Related

- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

# Conformance: Ex Commands and Ranges

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

Command-line (Ex) command subset and core types in the conformance ledger.

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
| `:set {option}` | Set editor option (number, nonumber, wrap, nowrap, etc. -- 22 boolean + 4 numeric) |
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
| `RuntimePhase` | Init -> CoreStarting -> ServicesStarting -> Running -> ShuttingDown -> Terminated |
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

## Related

- Command types: [/docs/reference/CONFORMANCE_COMMANDS_TYPES.md](/docs/reference/CONFORMANCE_COMMANDS_TYPES.md)
- Testing: [/docs/reference/CONFORMANCE_TESTING.md](/docs/reference/CONFORMANCE_TESTING.md)

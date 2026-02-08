# Proposal: Deep Wiring Checklist

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

The anti-MVP measures in [/docs/log/proposals/anti-mvp-measures.md](/docs/log/proposals/anti-mvp-measures.md) established minimum code volume targets and wiring points, but previous reconstruction still produced only ~10% of expected code. The root cause is that top-level TODO items like "implement motions" allow an agent to create a motion dispatch function that handles `h`/`l`/`j`/`k` and skip the other 30+ motions.

## Solution: Explicit per-feature wiring assertions

Every TODO checklist item that references a spec file MUST be implemented as a pass-through from real user input to real behavior change. The following wiring assertions MUST hold:

### Input-to-handler wiring

For every keybinding defined in the spec, the key dispatch path MUST include:

1. The keybinding is registered in the `KeymapTrie` for the correct mode.
2. The key event resolves to a specific `Action` variant.
3. The `Action` is handled in the core task's `select!` loop.
4. The handler produces a state mutation or an observable side effect.

### Command-to-handler wiring

For every ex command defined in the spec:

1. The command parser recognizes the command name and valid argument syntax.
2. The command dispatches to a handler function (not a no-op).
3. The handler performs the documented behavior.
4. Error cases produce the documented error messages.

### Feature verification pattern

Each feature MUST be verifiable by this pattern:

| Step | Assertion |
|---|---|
| User input | Trigger the feature via documented keybinding or command. |
| State change | Buffer, mode, viewport, or external state changes as documented. |
| Observable output | The rendered frame reflects the state change correctly. |
| Persist | If applicable, the state change survives `:w`, session save/load, or `:wq`. |
| Undo | If applicable, `u` reverses the state change correctly. |

## Crate-level wiring inventory

Per-crate integration checklist.

### kjxlkj-core-text (min 400 lines)

This crate MUST contain non-trivial implementations:

| Module | Responsibility | Key functions |
|---|---|---|
| `rope.rs` | Ropey wrapper | `insert`, `delete`, `line_count`, `line_at`, `byte_to_line`, `line_to_byte` |
| `grapheme.rs` | UAX #29 segmentation | `graphemes(line) -> Vec<GraphemeCluster>`, `grapheme_count(line)`, `grapheme_at(line, idx)` |
| `display_width.rs` | UAX #11 width | `display_width(grapheme) -> u8`, `line_display_width(line) -> usize`, `display_col_to_grapheme(line, col)`, `grapheme_to_display_col(line, idx)` |
| `line_ops.rs` | Line manipulation | `insert_line`, `delete_line`, `split_line`, `join_lines`, `indent_line` |

### kjxlkj-core-edit (min 600 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `operators.rs` | d/c/y/>/</= | `apply_operator(op, range, state)` with correct linewise/charwise behavior |
| `motions.rs` | All motion dispatch | `resolve_motion(motion, count, state) -> (start, end, type)` |
| `word.rs` | w/W/b/B/e/E | Word boundary detection with Unicode category awareness |
| `text_objects.rs` | iw/aw/is/as/i(/a(/i"/a"/it/at | Boundary resolution per text object type |
| `registers.rs` | Register storage | Named, numbered, clipboard, black hole, expression, read-only registers |
| `dot_repeat.rs` | `.` command | Record last change and replay |
| `undo.rs` | Undo tree | `push_change`, `undo`, `redo`, `undo_group` |

### kjxlkj-core-mode (min 500 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `mode.rs` | Mode enum and transitions | `transition(current, trigger) -> next` with all edges |
| `normal.rs` | Normal mode dispatch | `handle_key(key) -> Action` for all normal-mode keys |
| `insert.rs` | Insert mode dispatch | `handle_key(key) -> Action` with auto-indent, completion, etc. |
| `visual.rs` | Visual mode dispatch | `handle_key(key) -> Action` with selection tracking |
| `command.rs` | Command-line mode | `handle_key(key) -> Action` with editing, history, completion |
| `replace.rs` | Replace mode dispatch | `handle_key(key) -> Action` with overstrike behavior |
| `cursor_clamp.rs` | Mode-transition clamping | `clamp(cursor, line_len, mode) -> cursor` |

### kjxlkj-core-state (min 500 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `state.rs` | EditorState aggregation | Buffer list, window tree, mode, option management |
| `viewport.rs` | Viewport follow | `follow_cursor(viewport, cursor, opts) -> viewport` for both wrap and no-wrap |
| `snapshot.rs` | Snapshot production | `produce_snapshot(state) -> EditorSnapshot` |
| `command_dispatch.rs` | Ex command routing | `dispatch(cmd) -> Result<Action>` for all commands |
| `window_tree.rs` | Layout tree ops | `split`, `close`, `navigate`, `resize`, `zoom`, `rotate` |
| `session.rs` | Session JSON I/O | `save(state) -> JSON`, `load(json) -> State` |

### kjxlkj-render (min 500 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `cell.rs` | Cell and CellGrid types | `Cell { grapheme, width, fg, bg, attrs }` |
| `grid.rs` | Grid construction | `build_grid(snapshot, window) -> CellGrid` |
| `wrap.rs` | Line wrapping | `wrap_line(graphemes, cols) -> Vec<DisplayRow>` with CJK padding |
| `gutter.rs` | Gutter rendering | Line numbers, sign column, fold column |
| `diff.rs` | Frame diffing | `diff(prev, curr) -> Vec<CellChange>` |
| `flush.rs` | Terminal output | `flush(changes, stdout)` with batched escape sequences |
| `statusline.rs` | Statusline rendering | `render_statusline(snapshot, width) -> Vec<Cell>` |
| `decorations.rs` | Overlay rendering | Cursor, diagnostics, search highlights, visual selection |

### kjxlkj-input (min 300 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `keymap_trie.rs` | Keybinding tree | `insert(keys, action)`, `lookup(keys) -> Match` |
| `key_parse.rs` | Key event parsing | `parse(bytes) -> Key` |
| `mapping.rs` | Mapping expansion | `expand(keys, mode) -> Action` with timeout |
| `leader.rs` | Leader handling | `expand_leader(keys, config) -> keys` |
| `count.rs` | Count accumulation | `accumulate_count(digit) -> Option<count>` |

### kjxlkj-service-terminal (min 400 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `parser.rs` | VT100 state machine | `feed(byte)` with Ground/Escape/CSI/OSC states |
| `screen.rs` | Screen buffer | `write_char`, `erase`, `scroll`, `resize`, cursor movement |
| `sgr.rs` | SGR parsing | `parse_sgr(params) -> CellAttrs` |
| `pty.rs` | PTY management | `spawn`, `read_async`, `write`, `resize`, `cleanup` |
| `csi.rs` | CSI dispatch | `dispatch_csi(params, action, private)` |

## Related

- Part 2 (remaining crates): [/docs/log/proposals/deep-wiring-checklist-2.md](/docs/log/proposals/deep-wiring-checklist-2.md)
- Anti-MVP measures: [/docs/log/proposals/anti-mvp-measures.md](/docs/log/proposals/anti-mvp-measures.md)
- Reconstruction prompt: [/docs/todo/RECONSTRUCTION_PROMPT.md](/docs/todo/RECONSTRUCTION_PROMPT.md)
- Crate topology: [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)

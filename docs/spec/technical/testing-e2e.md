# Integration, E2E, and Boundary Test Scenarios

Back: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

This document specifies multi-module integration scenarios, headless and PTY-based end-to-end tests, and mandatory boundary/edge-case tests. An implementor MUST write and maintain every test listed here.

## Integration test scenarios

Integration tests validate cross-crate interactions. Each scenario specifies which crates participate and the acceptance criterion.

| Test ID | Crates involved | Scenario | Acceptance criterion |
|---|---|---|---|
| INT-01 | core-text, core-edit, core-state | Insert text, undo, redo | Type `"hello"` in Insert mode, `Esc`, `u` undoes all typed text, `Ctrl-r` re-applies it. Buffer matches `"hello"`. |
| INT-02 | core-mode, core-edit, core-state | Operator across mode boundary | In Normal mode, `d` then `v` enters Visual, extend selection, complete `d`. Correct text is deleted. |
| INT-03 | input, core-mode, core-state | Key mapping to mode transition | Configure `jk -> Esc`. In Insert mode, type `j`, `k` within timeout. Mode transitions to Normal. Buffer contains no literal `j` or `k`. |
| INT-04 | core-state, render | Snapshot to frame pipeline | After inserting one character, a new snapshot is produced. Rendering the snapshot yields a frame where the inserted character is visible at the correct cell. |
| INT-05 | core-state, core-text, render | CJK viewport follow through render | Load a buffer with 100 CJK-only lines in a 40-column window. Navigate to line 80. The rendered frame shows line 80 with correct wrapping and cursor position. |
| INT-06 | input, core-mode, core-edit | Count-prefixed operator | Input sequence `3`, `d`, `w` processed through input decoding, mode interpretation, and edit application deletes exactly 3 words. |
| INT-07 | core-state, service-terminal | Terminal window in editor state | Open a terminal window via `:terminal`. Editor state contains two windows. Sending `ls\n` to the PTY produces output in the terminal screen buffer. |
| INT-08 | core-edit, core-state | Multi-buffer yank/paste | Yank a line from buffer A. Switch to buffer B. Paste with `p`. Buffer B gains the yanked line. Register contents persist across buffer switch. |
| INT-09 | core-state, render | Resize pipeline | With a rendered frame, trigger a resize from 80x24 to 40x12. A new snapshot is produced with updated `text_rows` and `text_cols`. The rendered frame fits the new geometry. |
| INT-10 | input, core-mode, core-state | Command-line execution pipeline | Input `:` transitions to Command mode. Typing `set number` and pressing `Enter` dispatches the command. Line numbers become visible in the next rendered frame. |

## Headless E2E scenarios

Headless E2E tests drive the full editor loop (input decode, core dispatch, state update, snapshot, render) without a real terminal. They use a synthetic input stream and assert against rendered frame buffers or persisted file state.

| Test ID | Scenario | Input sequence | Acceptance criterion |
|---|---|---|---|
| HE-01 | Create, edit, save | Open empty file, `i`, type `"test content"`, `Esc`, `:wq` | File on disk contains `"test content\n"`. Process exits cleanly. |
| HE-02 | Open, navigate, quit | Open a 200-line file, `50G`, `:q` | Cursor reaches line 50 before quit. No file modification occurs. |
| HE-03 | Search and replace | Open file with `"foo bar foo"`, `:%s/foo/baz/g`, `:wq` | File contains `"baz bar baz\n"`. |
| HE-04 | Visual block delete | Open 5-line file, `Ctrl-v`, `2j`, `3l`, `d` | A 3-row by 4-column block is deleted from the buffer. |
| HE-05 | Macro record and replay | `qa`, `dw`, `q`, `3@a` | Four words total are deleted (one from recording, three from replay). |
| HE-06 | Split and navigate | `:vsplit`, `Ctrl-w l`, `i`, type `"right"`, `Esc`, `Ctrl-w h` | Two windows exist. The right window buffer contains `"right"`. Focus returns to the left window. |
| HE-07 | Session save/load roundtrip | Open two files, split, set cursor positions, `:mksession sess.vim`, quit, reopen with session | After load, same two files are open in same split layout with cursors at saved positions. |
| HE-08 | CJK insert and cursor | Open empty file, `i`, type `"あいうえお"`, `Esc` | Buffer contains 5 graphemes. Cursor is at grapheme offset 4 (last CJK character). Display width of line is 10. |
| HE-09 | Undo to empty | Open empty file, `i`, type `"abc"`, `Esc`, `u` | Buffer is empty. Cursor is at offset 0. No crash. |

## PTY E2E scenarios

PTY E2E tests run the editor in a real pseudo-terminal, sending raw bytes and reading rendered output. These catch transport-layer bugs invisible to headless tests.

| Test ID | Scenario | Procedure | Acceptance criterion |
|---|---|---|---|
| PE-01 | `pty_append_eol_mode_churn` | Load `"aあb"`. Repeat 20 times: `A`, type `"x"`, `Esc`. | Cursor is on the last grapheme of the line. No half-cell cursor position appears in any rendered frame. Final line is `"aあb"` followed by 20 `"x"` characters. |
| PE-02 | `pty_wrap_long_cjk_line` | Load a single line of 500 CJK characters in an 80-column terminal. Press `$` then `0`. | `$` moves cursor to grapheme 499 (display column 998-999). `0` returns to grapheme 0 (display column 0). Viewport scrolls correctly in both directions. |
| PE-03 | `pty_leader_vs_ime_space` | Configure `<leader>` as `Space`. Begin IME composition for `"漢"`. Press `Space` to accept the IME candidate. | The committed character `"漢"` appears in the buffer. No leader mapping fires. Mode remains Insert. |
| PE-04 | `pty_tmux_detach_resume` | Start editor inside tmux. Insert text. Detach (`Ctrl-b d`). Re-attach. | Buffer content is intact. Cursor position is unchanged. Rendering is correct after re-attach. |
| PE-05 | `pty_resize_storm_with_wrap` | Load a 300-character CJK line. Send 50 rapid `SIGWINCH` signals alternating between 80x24 and 40x12. | After the storm, the viewport has correct `text_rows` and `text_cols` for the final geometry. Cursor is visible. No panic, no corrupted cell state. |
| PE-06 | `pty_terminal_emulator_basic` | Open `:terminal`. Run `echo "hello world"`. | Terminal screen buffer contains `"hello world"`. Editor remains responsive. Switching back to a buffer window with `Ctrl-w h` works. |
| PE-07 | `pty_multi_window_nav` | Open 3 files in vertical splits. Navigate through all windows with `Ctrl-w l` twice, then `Ctrl-w h` twice. | Focus visits each window in order and returns to the original. Each window retains its independent cursor position. |
| PE-08 | `pty_concurrent_terminal_edit` | Open a split: buffer on left, `:terminal` on right running `yes` (continuous output). Type in the buffer window. | Typing in the buffer window is not blocked or delayed by terminal output. All typed characters appear. Terminal output continues in the right window. |

## Boundary and edge-case tests

These tests target corner cases that commonly cause panics, rendering artifacts, or data loss. An AI implementor MUST write each test and verify it passes.

### Empty and minimal file operations

| Test ID | Scenario | Acceptance criterion |
|---|---|---|
| BD-01 | Open empty file, `:wq` | An empty file is written to disk (0 bytes or a single newline per policy). No panic. |
| BD-02 | Open empty file, `dd` | `dd` on the only (empty) line is a no-op or leaves a single empty line. No panic. |
| BD-03 | Open empty file, `p` with empty register | Paste from an empty register is a no-op. No panic. |
| BD-04 | Single-character file (`"x"`) | Open, `x` (delete char), `u` (undo). File returns to `"x"`. |
| BD-05 | Single newline file | Open a file containing only `"\n"`. `j` is a no-op (no next line). `dd` leaves an empty buffer. |

### Very long lines

| Test ID | Scenario | Acceptance criterion |
|---|---|---|
| BD-06 | 10,000 ASCII characters, `wrap=true` | Line wraps into `ceil(10000/text_cols)` display rows. Cursor navigation with `l` and `h` traverses all characters. |
| BD-07 | 10,000 CJK characters, `wrap=true`, 80-col terminal | Line wraps into `ceil(20000/80) = 250` display rows. No wide character is split across rows. Every wrap boundary has a padding cell if a width-2 grapheme would be split. |
| BD-08 | 10,000 CJK characters, `wrap=false` | Horizontal scrolling with `l` advances through all 10,000 graphemes. `left_col` updates to keep cursor visible. |
| BD-09 | Mixed ASCII/CJK 10,000-char line | `$` moves to the last grapheme. `0` returns to the first. Display width is computed correctly for the mixed line. |

### Rapid mode switching

| Test ID | Scenario | Acceptance criterion |
|---|---|---|
| BD-10 | 1000 `i`/`Esc` cycles | No memory leak (RSS stays within 2x of baseline after cycles). Mode is Normal at the end. Cursor is clamped. |
| BD-11 | `v`/`Esc` visual flicker | 500 `v`/`Esc` cycles produce no leftover selection state. Visual anchor is cleared on each `Esc`. |
| BD-12 | Insert/Replace alternation | `i`, type `"a"`, `Esc`, `R`, type `"b"`, `Esc`, repeated 200 times. Buffer contains the expected interleaved overwrites. |

### Concurrent terminal output with editing

| Test ID | Scenario | Acceptance criterion |
|---|---|---|
| BD-13 | Terminal `yes` flood | `:terminal` running `yes` for 2 seconds does not cause OOM. Scrollback is bounded to `terminal.scrollback_lines`. |
| BD-14 | Buffer edit during terminal flood | With `yes` running in a split terminal, edit a buffer in an adjacent window. All edits land correctly. No cross-window corruption. |
| BD-15 | Terminal close during output | Close a terminal window while `yes` is running. Child process is reaped within 1 second. No zombie process. |

### Resize storms

| Test ID | Scenario | Acceptance criterion |
|---|---|---|
| BD-16 | 100 resizes in 1 second, empty buffer | After the storm, `text_rows` and `text_cols` match the final terminal size. Cursor is at `(0, 0)`. |
| BD-17 | 100 resizes in 1 second, wrapped CJK buffer | After the storm, CJK wrapping is recomputed for the final size. No wide character is split. Cursor is visible. |
| BD-18 | Resize to 1 column | Shrinking to 1 column does not panic. Each width-2 grapheme either renders as a padding-only row or the implementation clamps to a minimum viable width. |
| BD-19 | Resize to 1 row | Shrinking to 1 row does not panic. The cursor line is the only visible line. Status line may be suppressed. |

### Session roundtrip tests

| Test ID | Scenario | Acceptance criterion |
|---|---|---|
| BD-20 | Session save/load with empty buffer | `:SessionSave`, quit, `:SessionLoad`. Empty buffer restored. Cursor at (0,0). |
| BD-21 | Session save/load with splits | Create 3-way split (`:vsplit`, `:split`), set distinct cursor positions. `:SessionSave`, `:SessionLoad`. Layout tree matches: 3 windows, same split types, cursor positions restored. |
| BD-22 | Session save/load with terminal windows | Create a split with one buffer, one terminal. `:SessionSave`, `:SessionLoad`. Layout restored with 2 windows. Terminal window creates a new shell (process state not restored). Buffer window has correct cursor. |
| BD-23 | Session save with CJK content | Buffer contains `"あいうえお"` with cursor on grapheme 3. `:SessionSave` writes JSON with `cursor_grapheme: 3`. `:SessionLoad` restores cursor on `"え"`. |
| BD-24 | Session load with missing file | Session references a deleted file. `:SessionLoad` opens empty buffer for the missing file, shows warning. Other buffers load normally. |

### CJK-specific editing tests

| Test ID | Scenario | Acceptance criterion |
|---|---|---|
| BD-25 | CJK word motion | `w` on `"あいう えお"` (CJK space at index 3) moves cursor from grapheme 0 to grapheme 4 (first grapheme of second CJK word). |
| BD-26 | CJK delete word | `dw` on `"あいう えお"` at grapheme 0 deletes `"あいう "` leaving `"えお"`. |
| BD-27 | CJK visual selection | `v`, `2l` on `"あいうえお"` selects graphemes 0-2 (`"あいう"`). `d` deletes them, leaving `"えお"`. |
| BD-28 | CJK yank and paste | `yy` on `"あいうえお"`, `p` on next line. Pasted line matches original. Display width of pasted line is 10. |
| BD-29 | CJK search | `/あ` on buffer containing `"テストあいう"` moves cursor to grapheme 3. Search highlight covers display columns 6-7. |
| BD-30 | CJK substitute | `:%s/あ/ア/g` on `"あいあう"` produces `"アいアう"`. Display width unchanged (all width-2). |
| BD-31 | CJK cursor at line end | On `"あいう"` (3 graphemes), `$` moves to grapheme 2. Cursor block spans display columns 4-5. `a` enters Insert with insertion point at grapheme 3 (after `"う"`). |
| BD-32 | Mixed ASCII/CJK append | On `"aあb"`, `$` places cursor on `"b"` (grapheme 2, display col 3). `A` enters Insert at grapheme 3 (past end). No half-cell state at any point. |

### Terminal emulator rendering tests

| Test ID | Scenario | Acceptance criterion |
|---|---|---|
| BD-33 | Terminal CJK output | `:terminal`, run `echo "あいう"`. Terminal screen buffer contains 3 graphemes occupying 6 cells. Continuation cells marked correctly. |
| BD-34 | Terminal color output | `:terminal`, run `printf "\033[38;2;255;0;0mRED\033[0m"`. Terminal cells for `R`, `E`, `D` have fg RGB(255,0,0). Following cells have default fg. |
| BD-35 | Terminal scrollback | `:terminal`, run `seq 100`. Terminal scrollback contains lines 1 through (100 - visible_rows). Scrollback navigation with `Ctrl-\ Ctrl-n` then `k` scrolls up. |
| BD-36 | Terminal alternate screen | `:terminal`, run program that enters alternate screen (e.g., `less`). Exiting `less` restores main screen content. |
| BD-37 | Terminal resize correctness | `:terminal` in a split. Resize the split with `Ctrl-w +`. Terminal grid rows increase. Child process receives SIGWINCH. |

### Wiring verification tests

| Test ID | Scenario | Acceptance criterion |
|---|---|---|
| BD-38 | All normal mode motions reachable | For each motion key (h/j/k/l/w/b/e/W/B/E/0/$/_/^/g_/gg/G/f/F/t/T/;/,/{/}/(/)/n/N/*/#/%/H/M/L), verify the key resolves to a handler and produces correct cursor movement. |
| BD-39 | All operators with motions | For each operator (d/c/y/>/</ =/gq/gw/gu/gU/g~), combine with at least 3 motions (w/$/j) and verify correct text mutation. |
| BD-40 | All ex commands reachable | For each essential command (:w/:q/:wq/:q!/:e/:sp/:vs/:b/:bn/:bp/:bd/:set/:map/:help/:terminal/:SessionSave/:SessionLoad), verify the command parser routes to a handler and produces the documented effect. |

## Test infrastructure requirements

| Requirement | Detail |
|---|---|
| PTY harness | A reusable test harness MUST spawn the editor in a PTY, send input bytes, and read output bytes with bounded timeouts. |
| Frame assertion helper | A helper MUST extract a cell grid from raw terminal output for cell-level assertions. |
| Resize helper | A helper MUST send `SIGWINCH` with specified dimensions to the editor process. |
| CJK assertion macros | Macros MUST verify: (1) no cell contains a split wide character, (2) cursor display column is even when on a CJK grapheme, (3) continuation cells are marked correctly. |

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Unit test requirements: [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Viewport management: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Terminal emulator: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Latency and ordering: [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)

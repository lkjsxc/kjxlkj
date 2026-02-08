# Unit Test Requirements Per Crate

Back: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

Each crate MUST ship the unit tests listed below before it is considered reconstruction-complete. Test IDs are stable references for traceability. An implementor MUST NOT mark a crate done until every listed test passes.

## kjxlkj-core-text

Responsible for the rope-backed text model, grapheme decomposition, and display width computation.

| Test ID | Area | Acceptance criterion |
|---|---|---|
| CT-01 | Grapheme decomposition | Decomposing `"cafe\u{0301}"` (e + combining acute) yields 4 grapheme clusters: `c`, `a`, `f`, `e\u{0301}`. |
| CT-02 | CJK display width | `display_width("あ")` returns 2. `display_width("a")` returns 1. `display_width("\t")` returns the configured `tabstop`. |
| CT-03 | Mixed-width line width | For `"aあbいc"`, total display width is 8 (1+2+1+2+1+1 if newline, or 7 without). Per-grapheme widths are `[1,2,1,2,1]`. |
| CT-04 | Emoji width | `display_width("👨‍👩‍👧‍👦")` (family emoji ZWJ sequence) returns 2 and counts as 1 grapheme cluster. |
| CT-05 | Rope insert | Insert `"xyz"` at byte offset 0 of a rope containing `"abc"`. Result is `"xyzabc"`. Grapheme count is 6. |
| CT-06 | Rope delete | Delete byte range `[1..3)` from `"abcde"`. Result is `"ade"`. |
| CT-07 | Rope split and join | Split a rope at the midpoint, then join. Content MUST be identical to original. |
| CT-08 | Large rope | Build a rope from 100,000 lines of 80 ASCII characters. Line-at-index lookup MUST complete in under 1 ms. |
| CT-09 | Empty rope | A newly constructed rope has 0 graphemes, 0 lines, and 0 bytes. Inserting then deleting all content returns to this state. |
| CT-10 | Line index mapping | For a 5-line rope, `line_to_byte(3)` and `byte_to_line(result)` round-trip to 3. |
| CT-11 | Combining mark width | A base character followed by 3 combining marks is 1 grapheme with display width 1. |

## kjxlkj-core-edit

Responsible for editing primitives: operators, text objects, and motions.

| Test ID | Area | Acceptance criterion |
|---|---|---|
| CE-01 | Delete word (`dw`) | On `"hello world"` with cursor at 0, `dw` leaves `"world"`. |
| CE-02 | Delete inner word (`diw`) | On `"hello world"` with cursor on `w`, `diw` leaves `"hello "`. |
| CE-03 | Change line (`cc`) | On a 3-line buffer with cursor on line 2, `cc` clears line 2 and enters Insert mode. Lines 1 and 3 remain. |
| CE-04 | Yank and put (`yy`, `p`) | Yank line 1, move to line 3, `p` pastes below line 3. Buffer gains one line. |
| CE-05 | Text object `i(` | With cursor inside `"fn(a, b)"`, `di(` removes `"a, b"` leaving `"fn()"`. |
| CE-06 | Text object `a"` | With cursor inside `'say "hi" now'`, `da"` removes `'"hi"'` including quotes. |
| CE-07 | Count prefix | `3dw` on `"one two three four"` leaves `"four"`. |
| CE-08 | Dot repeat | After `dw` on word 1, `.` on the next word deletes it. Buffer loses two words total. |
| CE-09 | Undo single | After `dw`, `u` restores the deleted word. Buffer matches original. |
| CE-10 | Undo group | An Insert-mode session (`i`, type text, `Esc`) is one undo group. A single `u` removes all typed text. |
| CE-11 | Indent operator (`>`) | `>j` on lines 1-2 adds one `shiftwidth` of indentation to both lines. |
| CE-12 | CJK text object | `diw` with cursor on a CJK word `"漢字"` surrounded by spaces deletes exactly `"漢字"`. |

## kjxlkj-core-mode

Responsible for modal state machines, transitions, and cursor clamping on transition.

| Test ID | Area | Acceptance criterion |
|---|---|---|
| CM-01 | Normal to Insert (`i`) | Starting in Normal, `i` transitions to Insert. Mode enum is `Insert`. |
| CM-02 | Insert to Normal (`Esc`) | From Insert, `Esc` transitions to Normal. Cursor clamps to end-exclusive range. |
| CM-03 | Normal to Visual (`v`) | `v` enters Visual char mode. Anchor is set to current cursor position. |
| CM-04 | Visual to Normal (operator) | Applying `d` in Visual mode deletes selection and returns to Normal. |
| CM-05 | Normal to Command (`:`) | `:` enters Command mode. The command buffer is empty and cursor is at position 0. |
| CM-06 | Command cancel (`Esc`) | `Esc` in Command mode returns to Normal without executing. |
| CM-07 | Replace mode (`R`) | `R` enters Replace mode. Typing overwrites characters in place. `Esc` returns to Normal. |
| CM-08 | Cursor clamp on exit | On a line with 5 graphemes and Insert cursor at offset 5, `Esc` clamps to offset 4 (end-exclusive). |
| CM-09 | Empty line clamp | On an empty line, `a` then `Esc` leaves cursor at offset 0. |
| CM-10 | Rapid mode churn | Sequence `i Esc i Esc i Esc` repeated 100 times. Final mode is Normal, cursor is clamped, no panic. |
| CM-11 | Visual block (`Ctrl-v`) | `Ctrl-v` enters Visual-Block mode. Selection is rectangular. |
| CM-12 | Operator-pending timeout | After pressing `d` in Normal mode without a motion, `Esc` cancels and returns to Normal. |

## kjxlkj-core-state

Responsible for editor state aggregation, viewport follow, snapshot production, and command dispatch.

| Test ID | Area | Acceptance criterion |
|---|---|---|
| CS-01 | Viewport vertical follow | Cursor at line 50 with `scrolloff=5` and `text_rows=20`. After moving to line 80, `top_line` adjusts so cursor row is within `[5, 14]`. |
| CS-02 | Viewport horizontal follow | `wrap=false`, `sidescrolloff=3`, `text_cols=40`. Cursor at display column 60 causes `left_col` to adjust so cursor is within margin. |
| CS-03 | Wrap follow with CJK | A 200-column CJK line in a 40-column window. Moving cursor to grapheme 30 scrolls viewport so the cursor's display row is visible. |
| CS-04 | Snapshot monotonicity | Two successive state updates produce snapshots with strictly increasing sequence numbers. |
| CS-05 | Command dispatch `:w` | Dispatching `:w` on a modified buffer triggers a write intent. Buffer dirty flag clears after write completes. |
| CS-06 | Command dispatch `:q!` | `:q!` on a dirty buffer exits without error and without writing. |
| CS-07 | Command dispatch `:set wrap` | `:set wrap` toggles the viewport `wrap` field. When `wrap` becomes true, `left_col` resets to 0. |
| CS-08 | Resize re-clamp | Shrinking `text_rows` from 40 to 10 while cursor is at line 35 forces `top_line` adjustment. Cursor remains visible. |
| CS-09 | Multi-window state | Two windows on the same buffer have independent viewport and cursor state. Moving cursor in window A does not change viewport in window B. |
| CS-10 | `zz` centering | After `zz`, cursor display row equals `floor(text_rows / 2)`. |

## kjxlkj-input

Responsible for terminal input decoding, key parsing, and mapping expansion.

| Test ID | Area | Acceptance criterion |
|---|---|---|
| KI-01 | ASCII key parse | Byte `0x61` decodes to `Key::Char('a')` with no modifiers. |
| KI-02 | Escape sequence | Bytes `\x1b[A` decode to `Key::Up`. |
| KI-03 | Ctrl modifier | Byte `0x01` decodes to `Key::Char('a')` with `Ctrl` modifier. |
| KI-04 | UTF-8 multi-byte | Bytes `\xe3\x81\x82` decode to `Key::Char('あ')`. |
| KI-05 | Ambiguous escape | A lone `\x1b` followed by a timeout decodes to `Key::Esc`, not a partial sequence. |
| KI-06 | Mapping expansion | With mapping `jk -> Esc`, typing `j` then `k` within timeout produces `Esc`. Typing `j` then timeout produces literal `j`. |
| KI-07 | Leader expansion | With `<leader>` set to `Space`, `Space e` expands to the mapped action for `<leader>e`. |
| KI-08 | Mapping vs IME | During IME composition, `Space` used for conversion MUST NOT trigger leader mappings. |
| KI-09 | Paste bracket | Bytes `\x1b[200~hello\x1b[201~` decode to a bracketed paste event containing `"hello"`. |
| KI-10 | Resize event | `SIGWINCH` produces a `Resize(cols, rows)` event. |
| KI-11 | Burst input ordering | 50 rapid keypresses decoded in sequence MUST produce 50 events in the same order. |

## kjxlkj-render

Responsible for converting snapshots to terminal frames, line wrapping, and cell rendering.

| Test ID | Area | Acceptance criterion |
|---|---|---|
| RR-01 | ASCII line render | A line `"hello"` in a 40-column window renders 5 cells with correct characters and 35 empty cells. |
| RR-02 | CJK line render | A line `"あいう"` renders as 6 occupied cells (each character spanning 2 cells, with continuation markers). |
| RR-03 | Wrap at boundary | A 41-column ASCII line in a 40-column window produces 2 display rows: 40 cells then 1 cell. |
| RR-04 | CJK wrap padding | In a 5-column window, the line `"aaああ"` (widths 1,1,2,2 = 6 cols) wraps: row 1 is `a`, `a`, `あ`(2 cols) = 5 cols; row 2 is `あ`(2 cols). No split wide character. |
| RR-05 | CJK wrap with 1 remaining col | In a 5-column window, `"aaaaあ"` (widths 1,1,1,1,2 = 6 cols) wraps: row 1 is `a`,`a`,`a`,`a`,`PAD`; row 2 starts with `あ`. The padding cell MUST appear, not a half-width artifact. |
| RR-06 | Cursor cell highlight | The cell under the cursor MUST have the cursor highlight attribute set. If the cursor is on a width-2 grapheme, both cells carry the highlight. |
| RR-07 | Diff rendering | Two consecutive frames with a single character change produce a diff that updates only the changed cell, not the entire screen. |
| RR-08 | Tab rendering | A tab character renders as `tabstop` cells filled with spaces. Cursor on a tab highlights all cells the tab occupies. |
| RR-09 | Empty buffer | An empty buffer renders a single empty line. The cursor is at row 0, column 0. Tilde lines fill remaining rows. |
| RR-10 | Status line | The status line renders mode, filename, line/column, and fits within one terminal row. |

## kjxlkj-service-terminal

Responsible for the integrated terminal emulator: escape sequence parsing, screen buffer, and PTY lifecycle.

| Test ID | Area | Acceptance criterion |
|---|---|---|
| ST-01 | CUP positioning | Escape `\x1b[5;10H` moves terminal cursor to row 4, column 9 (0-based). |
| ST-02 | SGR color | `\x1b[38;2;255;0;0m` sets foreground to RGB(255,0,0). Subsequent text cells carry this color. `\x1b[0m` resets. |
| ST-03 | ED erase display | `\x1b[2J` clears all cells to empty. |
| ST-04 | Scroll region | `\x1b[5;15r` sets scroll region rows 5-15. `\n` at row 15 scrolls only within the region. |
| ST-05 | Alternate screen | `\x1b[?1049h` switches to alternate screen. `\x1b[?1049l` restores main screen content. |
| ST-06 | Wide char in terminal | Writing `"あ"` to the terminal grid occupies 2 cells. The second cell is marked `is_wide_continuation = true`. |
| ST-07 | PTY spawn and read | Spawning `/bin/echo hello` via PTY produces output containing `"hello"` in the screen buffer. |
| ST-08 | PTY resize | After `ioctl(TIOCSWINSZ)` with new dimensions, the terminal grid resizes and `SIGWINCH` is delivered to the child. |
| ST-09 | PTY cleanup | Closing the terminal sends `SIGHUP`. After a bounded wait, the child process is reaped. No zombie remains. |
| ST-10 | Crash resilience | If the PTY child exits unexpectedly, the terminal window displays an error message and does not freeze or panic. |
| ST-11 | OSC title | `\x1b]2;My Title\x07` sets the terminal window title to `"My Title"`. |
| ST-12 | 256-color support | `\x1b[38;5;196m` sets foreground to color index 196. |

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- E2E and boundary tests: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Viewport management: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Terminal emulator: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Crate topology: [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)

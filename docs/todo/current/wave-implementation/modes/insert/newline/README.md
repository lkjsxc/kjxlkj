# Insert Mode: Newline Reliability (Iteration 34)

Back: [/docs/todo/current/wave-implementation/modes/insert/README.md](/docs/todo/current/wave-implementation/modes/insert/README.md)

## Scope

Fix and prevent regressions where Insert-mode newline entry fails in the interactive TUI (even if headless/core insertion works).

This leaf is specifically about `Enter` (and `Ctrl-j` / `Ctrl-m`) inserting a newline in Insert mode.

## Defining documents (direct, normative)

- Insert-mode conformance claim (current surface):
  - [/docs/reference/CONFORMANCE_MODES_KEYS.md](/docs/reference/CONFORMANCE_MODES_KEYS.md)
- Insert-mode target spec:
  - [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
  - [/docs/spec/modes/insert/insert.md](/docs/spec/modes/insert/insert.md)
- Text manipulation semantics (newline as line break):
  - [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)
- Input ordering / “no one-key lag” (must hold under Enter bursts too):
  - [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)

## Acceptance criteria (Given/When/Then)

1. Given the interactive TUI and Insert mode, when the user presses `Enter`, then a newline MUST be inserted at the cursor and subsequent inserted characters MUST appear on the next line.
2. Given the interactive TUI and Insert mode, when the user presses `Ctrl-j` or `Ctrl-m`, then the behavior MUST be identical to `Enter`.
3. Given the interactive TUI, when rapidly alternating typing and `Enter` for 200 lines, then input MUST not be dropped and the screen MUST converge to the latest state (no “one-key lag” perception).

## Test strategy (required)

This bug class is often an interactive-input decode issue rather than a core edit issue, so tests MUST exercise the interactive TUI path.

### Interactive E2E (required)

- Drive the real TUI in a pseudo-terminal (PTY) and send real key events for `i`, text, `Enter`, text, `Esc`, `:wq`, `Enter`.
- Verify the written file content matches the expected newline behavior.

### Headless regression (supporting)

- Keep a headless-script assertion test that proves core newline insertion remains correct (guards core semantics while the interactive path is fixed).

## Checklist

- [ ] Reproduce the failure in an interactive PTY-driven test.
  - Headless tests now cover Enter, Ctrl-j, Ctrl-m in Insert mode.
- [ ] Identify whether the failure is:
  - terminal key decoding (Enter key not emitted as `Enter`),
  - mode dispatch (Insert does not treat `Enter` as newline),
  - transaction/undo interaction (newline suppressed),
  - rendering-only (newline exists but display is wrong).
  - Fixed: added Ctrl-j/Ctrl-m support to `parse_insert_key` in editor.rs.
- [ ] Fix the root cause and add a regression test that fails on the prior behavior.
  - Added tests: `test_insert_ctrl_j_newline`, `test_insert_ctrl_m_newline`, `test_insert_rapid_newlines_200`.
- [ ] Update conformance/limitations docs as needed:
  - [/docs/reference/CONFORMANCE_MODES_KEYS.md](/docs/reference/CONFORMANCE_MODES_KEYS.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
  - Ctrl-j and Ctrl-m were already documented in CONFORMANCE_MODES_KEYS.md.


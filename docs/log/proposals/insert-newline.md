# Proposal: Interactive Insert-Mode Newline Reliability

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem statement

A reported rough edge is that the interactive TUI cannot reliably insert a newline in Insert mode when the user presses `Enter`.

Core/headless insertion semantics appear capable of inserting a newline, so this is suspected to be an interactive input-decoding and/or mode-dispatch issue rather than a text-model limitation.

## Defining documents

- TODO leaf:
  - [/docs/todo/current/wave-implementation/modes/insert/newline/README.md](/docs/todo/current/wave-implementation/modes/insert/newline/README.md)
- Conformance claim (current surface):
  - [/docs/reference/CONFORMANCE_MODES_KEYS.md](/docs/reference/CONFORMANCE_MODES_KEYS.md)
- Target spec:
  - [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
  - [/docs/spec/modes/insert/insert.md](/docs/spec/modes/insert/insert.md)

## Proposed approach

1. Make the bug reproducible with an interactive, PTY-driven E2E test.
2. Classify the failure into one of:
   - terminal decode: `Enter` does not arrive as an `Enter` key in the editor input layer
   - mode dispatch: Insert mode does not treat `Enter` as newline insertion
   - rendering-only: newline exists but the display path fails to show it
3. Fix the narrowest root cause, and keep the PTY E2E as a regression guard.

## Test plan (required)

### Interactive E2E (PTY)

Drive the real binary with a pseudo-terminal and verify newline insertion by writing a file:

- open editor on a temp file path
- send `i`, `line1`, `Enter`, `line2`, `Esc`, `:wq`, `Enter`
- read the written file and assert it contains `line1` followed by a newline then `line2`

This avoids needing internal state dumps and asserts the end-to-end interactive path.

### Supporting tests

- Keep a headless-script assertion for newline insertion to guard core semantics.

## Risks / open questions

- Terminals differ in how they encode `Enter` (CR vs LF vs key events); the input layer should normalize consistently.
- If the renderer is the issue, the PTY test must be designed to validate file output rather than screen capture to reduce flakiness.


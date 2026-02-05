# Editing: Append at End-of-Line Regression (Iteration 35)

Back: [/docs/todo/current/wave-implementation/editing/cursor/README.md](/docs/todo/current/wave-implementation/editing/cursor/README.md)

## Scope

Fix and prevent the “`a` inserts one character left at end-of-line” class of cursor/insert-point bugs.

This is a common off-by-one failure when mixing end-exclusive (Normal) and end-inclusive (Insert) cursor models.

## Defining documents (direct, normative)

- Cursor column model and append semantics:
  - [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Keybinding expectations:
  - [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)

## Checklist

### A. Deterministic headless regression

- [ ] Add a headless E2E test that:
  - positions the cursor on the last character of a non-empty line in Normal mode
  - presses `a`
  - asserts the Insert-mode cursor column is exactly `N` (true EOL), not `N-1`

### B. Interactive PTY regression

- [ ] Add a PTY E2E regression that:
  - opens a temp file
  - inserts `hello`, exits Insert, moves to EOL, presses `a`, types `X`, writes and quits
  - asserts the file ends with `helloX` (not `hellXo`)

### C. Conformance and limitations updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (if user-visible drift existed)

## Related

- PTY harness: [/docs/todo/current/wave-implementation/technical/testing/pty-e2e/README.md](/docs/todo/current/wave-implementation/technical/testing/pty-e2e/README.md)

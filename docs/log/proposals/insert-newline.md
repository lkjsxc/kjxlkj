# Proposal: Interactive Insert-mode Newline Reliability

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

The `Enter` key in Insert mode must reliably insert a newline at the cursor position without cursor drift or display artifacts.

## Defining specs

- Insert mode behavior: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Conformance claim

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) — Insert mode section

## Limitations

- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) — newline entry

## Implementation TODO

- [/docs/todo/current/wave-implementation/modes/insert/newline/README.md](/docs/todo/current/wave-implementation/modes/insert/newline/README.md)

## Test requirements

- Given a buffer with text, when `Enter` is pressed in Insert mode, then a newline is inserted at the cursor position.
- Given cursor at end of line, when `Enter` is pressed, then a new empty line is created below.
- Given cursor in the middle of a line, when `Enter` is pressed, then the line splits at the cursor.

## Status

Implemented and tested in kjxlkj-core-edit crate.

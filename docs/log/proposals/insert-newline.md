# Proposal: Insert Newline Reliability

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem

Interactive Insert-mode newline (`Enter` key) behavior must be reliable and consistent.

## Defining specs

- [/docs/spec/modes/README.md](/docs/spec/modes/README.md)
- [/docs/spec/editing/README.md](/docs/spec/editing/README.md)

## Status

Placeholder - to be refined in implementation wave.

## Acceptance criteria

- Given Insert mode, when pressing Enter, then a newline MUST be inserted at cursor.
- Given Insert mode at end of file, when pressing Enter, then a new line MUST be added.

## Test strategy

- Unit tests for newline insertion
- E2E test for Enter key in Insert mode

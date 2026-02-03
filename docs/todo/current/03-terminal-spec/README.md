# Integrated Terminal Spec (Iteration 32)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Objective

Ensure the integrated terminal feature is fully defined:

- PTY lifecycle and supervision expectations
- Input routing between editor and terminal pane
- Scrollback behavior and navigation
- Rendering constraints and performance expectations
- Exit behavior and persistence rules

## Tasks

### A. Clarify mode behavior

- Define which mode(s) apply when focus is in a terminal pane.
- Define how `Ctrl-\\ Ctrl-n` transitions terminal focus back to Normal.

### B. Define scrollback and viewport interaction

- Terminal panes MUST have deterministic scrollback.
- Terminal panes MUST follow the same viewport invariants as editor windows.

### C. Define failure and recovery behavior

- Terminal process crashes MUST surface visible diagnostics.
- Core editing MUST remain responsive under terminal output load.

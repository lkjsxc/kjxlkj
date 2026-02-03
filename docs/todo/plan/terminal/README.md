# Plan: Integrated Terminal Specification

Back: [/docs/todo/plan/README.md](/docs/todo/plan/README.md)

## Implementation Order

### 1. Scope and invariants

1. Define terminal pane types (split, float, tab)
2. Define focus and mode behavior within terminal panes
3. Define “no freeze editing” responsiveness invariant

### 2. PTY lifecycle

1. Define creation, shutdown, and restart behavior
2. Define process supervision and crash reporting requirements
3. Define persistence rules across toggles and layout changes

### 3. Rendering and scrollback

1. Define scrollback model and user navigation behavior
2. Define viewport clamping within terminal panes
3. Define performance constraints under high output

### 4. Input routing

1. Define how keystrokes are routed to terminal vs editor
2. Define escape sequences for returning to editor focus
3. Define paste behavior and bracketed paste support

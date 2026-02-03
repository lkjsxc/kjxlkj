# Cursor, Viewport, and Input Spec (Iteration 32)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Objective

Define cursor rendering, cursor movement, and viewport-follow behavior precisely enough to prevent:

- Cursor invisibility
- Incorrect cursor movement under boundary conditions
- Viewport/camera drifting off-screen (cursor must remain visible)
- One-key input lag
- Sluggish performance when typing quickly

## Deep-dive Areas

### A. Cursor rendering and visibility

- Cursor rendering MUST be unambiguous in all modes and themes.
- Cursor MUST remain visible across redraws, mode transitions, and overlays.
- Define how cursor rendering interacts with:
  - selection highlights
  - search highlights
  - diagnostics underlines
  - terminal-native cursor vs drawn cursor

### B. Cursor movement determinism

- Motions MUST clamp or no-op; never panic.
- Mode transitions MUST clamp cursor to the mode’s column model.

### C. Viewport follow algorithm

- Define explicit vertical and horizontal follow rules:
  - scroll offsets
  - centering commands
  - split-local viewports
  - wrap vs no-wrap behavior
  - smooth scrolling constraints (if supported)
- Cursor MUST remain within the visible viewport after any operation.

### D. Input ordering and latency

- Define ordering guarantees between:
  - input events
  - intent emission
  - core state mutation
  - snapshot creation
  - rendering
- Eliminate off-by-one perception by specifying when a render is required.

### E. Performance and throughput targets

- Define render coalescing and backpressure rules.
- Define “dirty region” expectations for reducing terminal writes.

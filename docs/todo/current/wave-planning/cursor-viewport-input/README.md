# Cursor, Viewport, and Input Spec (Iteration 34)

Back: [/docs/todo/current/wave-planning/README.md](/docs/todo/current/wave-planning/README.md)

## Objective

Define cursor rendering, cursor movement, and viewport-follow behavior precisely enough to prevent:

- Cursor invisibility
- Incorrect cursor movement under boundary conditions
- Viewport/camera drifting off-screen (cursor must remain visible)
- One-key input lag
- Sluggish performance when typing quickly

## Defining documents (direct, normative)

- Cursor semantics:
  - [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Viewport invariants:
  - [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Runtime ordering (input → core → snapshot → render):
  - [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- Window model (viewport is per-window):
  - [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Scrolling customization (when relevant):
  - [/docs/spec/features/ui/scroll-customization.md](/docs/spec/features/ui/scroll-customization.md)
- Cursor customization (when relevant):
  - [/docs/spec/features/ui/cursor-customization.md](/docs/spec/features/ui/cursor-customization.md)
- Latency targets (when relevant):
  - [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)

## Coverage traversal (to avoid missing docs)

- Cursor subtree:
  - [/docs/todo/doc-coverage/spec/editing/cursor/README.md](/docs/todo/doc-coverage/spec/editing/cursor/README.md)
- Viewport file:
  - [/docs/todo/doc-coverage/spec/features/ui/README.md](/docs/todo/doc-coverage/spec/features/ui/README.md)
- Runtime subtree:
  - [/docs/todo/doc-coverage/spec/architecture/README.md](/docs/todo/doc-coverage/spec/architecture/README.md)

## Outputs (required)

- A set of acceptance criteria (Given/When/Then) that can be implemented as tests:
  - [/docs/todo/current/wave-planning/testing-spec/README.md](/docs/todo/current/wave-planning/testing-spec/README.md)
- A set of implementation leaves under:
  - [/docs/todo/current/wave-implementation/ui/viewport/README.md](/docs/todo/current/wave-implementation/ui/viewport/README.md)
  - [/docs/todo/current/wave-implementation/ui/cursor/README.md](/docs/todo/current/wave-implementation/ui/cursor/README.md)
  - [/docs/todo/current/wave-implementation/architecture/runtime/README.md](/docs/todo/current/wave-implementation/architecture/runtime/README.md)

## Checklist

- [ ] Define cursor layering rules (cursor vs selection vs overlays) as explicit requirements.
- [ ] Define cursor movement determinism at boundaries (clamp/no-op; no panics).
- [ ] Define viewport follow behavior for both `wrap = false` and `wrap = true`, including margins.
- [ ] Define input ordering guarantees and “no one-key lag” observable criteria.
- [ ] Define throughput targets and performance constraints for typing/scroll/resize.

## Deep-dive Areas

### A. Cursor rendering and visibility

- Cursor rendering MUST be unambiguous in all modes and themes.
- Cursor MUST remain visible across redraws, mode transitions, and overlays.
- Define how cursor rendering interacts with:
  - selection highlights
  - search highlights
  - diagnostics underlines
  - terminal-native cursor vs drawn cursor
- Define which layer “wins” when cursor overlaps overlays.

### B. Cursor movement determinism

- Motions MUST clamp or no-op; never panic.
- Mode transitions MUST clamp cursor to the mode’s column model.
- Define how cursor column is preserved (or not) across vertical moves.

### C. Viewport follow algorithm

- Define explicit vertical and horizontal follow rules:
  - scroll offsets
  - centering commands
  - split-local viewports
  - wrap vs no-wrap behavior
  - smooth scrolling constraints (if supported)
- Cursor MUST remain within the visible viewport after any operation.
- Define the “minimum margin” rules (if any) that keep cursor away from edges.

### D. Input ordering and latency

- Define ordering guarantees between:
  - input events
  - intent emission
  - core state mutation
  - snapshot creation
  - rendering
- Eliminate off-by-one perception by specifying when a render is required.
- Define the rule for “a keypress is visible” (which frame must contain it).

### E. Performance and throughput targets

- Define render coalescing and backpressure rules.
- Define “dirty region” expectations for reducing terminal writes.
- Define resize-storm behavior (how many renders, how coalesced).

# UI: Viewport (Iteration 36)

Back: [/docs/todo/current/wave-implementation/ui/README.md](/docs/todo/current/wave-implementation/ui/README.md)

## Scope

Implement viewport model and cursor-follow invariants.

## Defining documents (direct, normative)

- Viewport:
  - [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)

## Checklist

### A. Viewport state and invariants (scaffolding)

- [x] Define viewport state per window.
- [x] Ensure viewport invariants and clamping rules match `/docs/spec/features/ui/viewport.md`.

### B. Cursor-follow rules (no-wrap + wrap)

- [x] Implement deterministic vertical follow for `wrap = false`.
- [x] Implement deterministic horizontal follow for `wrap = false`.
- [x] Implement deterministic follow for `wrap = true` (display-row model) and default wrap posture: — done: `viewport_wrap.rs` with DisplayRow, DisplayMap, compute_display_rows(), follow_cursor_wrap()
  - [wrap/README.md](wrap/README.md)

### C. Long-line stability (user-reported rough edge)

- [x] Ensure extremely long lines do not corrupt or break display. — done: `viewport_integrity.rs` (render) with DisplayCell (normal/wide/continuation), DisplayRow, wrap_line (unicode_width), is_long_line, truncate_line, validate_viewport
  - [long-lines/README.md](long-lines/README.md)

### D. Regression tests and documentation ledgers

- [x] Add regression tests for long lines, wrap/no-wrap, and resize storms.
- [x] Update conformance and limitations docs when behavior is user-visible: — done: batch 15 viewport follow conformance and limitations entries added
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

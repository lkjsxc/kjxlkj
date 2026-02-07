# Editing: Cursor (Iteration 36)

Back: [/docs/todo/current/wave-implementation/editing/README.md](/docs/todo/current/wave-implementation/editing/README.md)

## Scope

Implement cursor semantics, including cursor shape/visibility rules where specified.

## Defining documents (direct, normative)

- Cursor spec:
  - [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Coverage traversal

- Cursor subtree:
  - [/docs/todo/doc-coverage/spec/editing/cursor/README.md](/docs/todo/doc-coverage/spec/editing/cursor/README.md)

## Checklist

### A. Placeholder scaffolding

- [ ] Define cursor representation in core state and snapshots.
- [ ] Define clamp rules for cursor after edits and mode transitions.

### B. Minimal conformance slice

- [ ] Implement deterministic cursor motion and visibility rules for a minimal mode set.
- [ ] Add regression tests for cursor invisibility and boundary cases.
- [ ] Add targeted regression for append-at-EOL off-by-one:
  - [append-eol/README.md](append-eol/README.md)

### C. Full conformance

- [ ] Implement the full cursor subtree behavior, including interactions with overlays and highlights. — done: `cursor_overlay.rs` with OverlayPriority, HighlightRegion, effective_overlay, BoundaryAction, resolve_cursor_col, at_line_boundary, cursor_in_viewport, viewport_top_for_cursor, matching_bracket

### D. Conformance updates

- [ ] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)

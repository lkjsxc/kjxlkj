# UI: Long-Line Stability (Iteration 34)

Back: [/docs/todo/current/wave-implementation/ui/viewport/README.md](/docs/todo/current/wave-implementation/ui/viewport/README.md)

## Scope

Prevent display corruption, panics, and pathological per-frame cost when rendering extremely long lines.

This leaf covers both:

- `wrap = false` (horizontal scrolling via `left_col`)
- `wrap = true` (display-row model; no horizontal scrolling)

## Defining documents (direct, normative)

- Viewport rules (wrap/no-wrap, `left_col`, cursor visibility):
  - [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Large file + long line posture:
  - [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
  - [/docs/technical/large-files.md](/docs/technical/large-files.md)
- Unicode/grapheme constraints (display width correctness):
  - [/docs/technical/unicode.md](/docs/technical/unicode.md)

## Acceptance criteria (Given/When/Then)

1. Given a buffer containing a single line with 1,000,000 visible columns, when rendering the editor view, then rendering MUST NOT panic and MUST NOT corrupt the terminal display (no broken layout, no runaway cursor positioning).
2. Given `wrap = false`, when `left_col` is advanced across the line, then only the visible slice MUST be materialized for rendering (no O(line-length) per-frame scanning past the visible region).
3. Given `wrap = true`, when the cursor moves within a long line, then viewport follow MUST keep the cursor visible without requiring full-line reflow work outside the visible display-row slice.
4. Given a long line with wide graphemes, tabs, and combining marks, when rendering, then display width handling MUST remain deterministic and MUST NOT desync cursor position vs rendered cells.

## Test strategy (required)

### Unit / snapshot tests (required)

- Add tests around the “line → visible cells” path to ensure it slices by viewport width/offset rather than cloning full lines.
- Add tests for cursor visibility invariants under long-line cursor movement.

### Golden render tests (recommended)

- Add snapshot-to-frame stability tests for long-line rendering at multiple `left_col` offsets and terminal widths.

### Load/perf tests (recommended)

- Add a deterministic benchmark that renders N frames of a long-line viewport and asserts the time remains within an agreed budget (relative, not absolute, to reduce flakiness).

## Checklist

- [x] Add a minimal reproduction file fixture (long ASCII line; long Unicode-heavy line).
- [x] Add tests that fail if the renderer materializes full long lines per frame.
- [x] Add at least one regression test that exercises:
  - resize storms with a long line visible
  - rapid horizontal scrolling (no-wrap)
  - wrap toggling (if supported)
- [x] Fix root causes (slicing, width computation caching, virtualization) until acceptance criteria hold.
- [x] Update limitations if any degradation is intentionally applied for very long lines:
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)


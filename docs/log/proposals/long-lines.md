# Proposal: Long-Line Rendering Stability and Virtualization

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem statement

Reported rough edge: display can break or become unstable when a single line is extremely long.

This is a known stressor even when overall file size is modest: a renderer that materializes or width-measures the entire line per frame becomes O(line-length), causing slowdowns, high CPU, and potential terminal desynchronization.

## Defining documents

- TODO leaf:
  - [/docs/todo/current/wave-implementation/ui/viewport/long-lines/README.md](/docs/todo/current/wave-implementation/ui/viewport/long-lines/README.md)
- Viewport rules (wrap/no-wrap, `left_col`):
  - [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Large-file posture (long lines, idle CPU expectations):
  - [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- Unicode constraints:
  - [/docs/technical/unicode.md](/docs/technical/unicode.md)

## Proposed approach

### A. Virtualize by viewport, not by line length

For `wrap = false`:

- rendering MUST be able to produce the visible slice of a long line given `(left_col, text_cols)` without scanning from column 0 to column `left_col` on every frame
- width computation SHOULD be incremental/cached, and bounded to the visible range plus a small margin

For `wrap = true`:

- treat the line as display rows, but materialize only the display rows that intersect the viewport
- avoid full-line reflow unless explicitly required

### B. Make invariants testable

Introduce tests that fail when the implementation:

- clones/materializes the entire long line for rendering
- performs per-frame work proportional to total line length
- corrupts cursor visibility invariants under long-line navigation/resizes

## Test plan (required)

- Unit/snapshot tests that measure or assert the amount of text materialized for rendering is viewport-sized.
- Golden UI tests that render known long-line fixtures at multiple offsets and widths.
- A deterministic benchmark that renders multiple frames and fails on algorithmic regressions (O(N) vs O(viewport)).

## Risks / open questions

- Unicode display width correctness can conflict with aggressive slicing; width-aware slicing must remain deterministic.
- Horizontal slicing requires stable mapping between buffer indices and display columns (tabs and wide graphemes complicate this).


# UI: Default Soft-Wrap Behavior (Iteration 35)

Back: [/docs/todo/current/wave-implementation/ui/viewport/README.md](/docs/todo/current/wave-implementation/ui/viewport/README.md)

## Scope

Ensure long lines **wrap by default** and that wrap/no-wrap behavior is deterministic and testable.

Reported rough edge: long lines not wrapping in the interactive UI.

## Defining documents (direct, normative)

- Viewport defaults and wrap invariants:
  - [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Navigation on display lines (`g`-prefixed motions):
  - [/docs/spec/editing/motions/line-motions.md](/docs/spec/editing/motions/line-motions.md)

## Checklist

### A. Defaults

- [ ] Ensure `wrap = true` is the default for normal text windows.
- [ ] Ensure `wrap = true` implies `left_col = 0` (no horizontal scroll).
- [ ] Ensure switching `wrap` on/off re-clamps viewport deterministically and keeps cursor visible.

### B. Tests (required)

- [ ] Add headless tests verifying default wrap state and invariants.
- [ ] Add golden UI snapshot tests for the same buffer in:
  - wrap mode at multiple widths
  - no-wrap mode at multiple `left_col` offsets
- [ ] Add a PTY E2E regression that edits a long line and verifies the editor remains usable (cursor visible; no desync).

### C. Conformance and limitations updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)

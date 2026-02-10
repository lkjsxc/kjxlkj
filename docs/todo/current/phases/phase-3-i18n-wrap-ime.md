# Phase 3: I18N, Wrap, and IME

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Japanese input correctness, CJK width safety, and overflow-free rendering.

## Tasks

- [x] implement IME composition state model and routing priority
- [x] prevent leader leakage during IME candidate operations
- [x] guarantee no half-cell cursor state on width-2 graphemes
- [x] enforce wrap padding rule at width-2 boundary split risk
- [x] validate long-line rendering stays on-screen in all window types

## Required Spec Links

- [x] [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md)
- [x] [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- [x] [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- [x] [/docs/spec/ui/views.md](/docs/spec/ui/views.md)

## Required Tests

- [x] `JP-01`
- [x] `JP-02`
- [x] `JP-03`
- [x] `JP-04`
- [x] `JP-05`
- [x] `BD-01`
- [x] `BD-02`
- [x] `BD-10`
- [x] `PE-04`
- [x] `PE-06`

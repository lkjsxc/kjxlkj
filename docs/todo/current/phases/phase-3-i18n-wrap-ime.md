# Phase 3: I18N, Wrap, and IME

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Japanese input correctness, CJK width safety, and overflow-free rendering.

## Tasks

- [ ] implement IME composition state model and routing priority
- [ ] prevent leader leakage during IME candidate operations
- [ ] guarantee no half-cell cursor state on width-2 graphemes
- [ ] enforce wrap padding rule at width-2 boundary split risk
- [ ] validate long-line rendering stays on-screen in all window types

## Required Spec Links

- [ ] [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md)
- [ ] [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- [ ] [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- [ ] [/docs/spec/ui/views.md](/docs/spec/ui/views.md)

## Required Tests

- [ ] `JP-01`
- [ ] `JP-02`
- [ ] `JP-03`
- [ ] `JP-04`
- [ ] `JP-05`
- [ ] `BD-01`
- [ ] `BD-02`
- [ ] `BD-10`
- [ ] `PE-04`
- [ ] `PE-06`

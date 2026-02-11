# Phase 3: I18N, Wrap, and IME

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

IME isolation, CJK-safe wrapping, and cursor visibility under stress.

## Tasks

- [x] enforce IME-first key routing while composing
- [x] block leader leakage during candidate operations
- [x] guarantee width-2 wrap padding and no half-cell cursor state
- [x] verify mixed-window wrap safety for editor/explorer/terminal views
- [x] close `LIM-BLOCK-WRAP-02` and remaining IME verification gaps

## Required Spec Links

- [x] [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md)
- [x] [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- [x] [/docs/spec/features/ui/cursor-customization.md](/docs/spec/features/ui/cursor-customization.md)
- [x] [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Required Tests

- [x] `JP-03`
- [x] `JP-04`
- [x] `JP-06R`
- [x] `JP-07R`
- [x] `JP-09R`
- [x] `WRAP-11R`
- [x] `WRAP-14R`
- [x] `WRAP-16R`
- [x] `CUR-08R`
- [x] `CUR-10R`

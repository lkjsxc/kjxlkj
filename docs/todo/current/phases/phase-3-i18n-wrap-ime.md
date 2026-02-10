# Phase 3: I18N, Wrap, and IME

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

IME isolation, CJK-safe wrapping, and cursor visibility under stress.

## Tasks

- [ ] enforce IME-first key routing while composing
- [ ] block leader leakage during candidate operations
- [ ] guarantee width-2 wrap padding and no half-cell cursor state
- [ ] verify mixed-window wrap safety for editor/explorer/terminal views
- [ ] close `LIM-BLOCK-WRAP-02` and remaining IME verification gaps

## Required Spec Links

- [ ] [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md)
- [ ] [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- [ ] [/docs/spec/features/ui/cursor-customization.md](/docs/spec/features/ui/cursor-customization.md)
- [ ] [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Required Tests

- [ ] `JP-03`
- [ ] `JP-04`
- [ ] `JP-06R`
- [ ] `JP-07R`
- [ ] `JP-09R`
- [ ] `WRAP-11R`
- [ ] `WRAP-14R`
- [ ] `WRAP-16R`
- [ ] `CUR-08R`
- [ ] `CUR-10R`

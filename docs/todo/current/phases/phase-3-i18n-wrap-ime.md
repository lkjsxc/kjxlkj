# Phase 3: I18N, Wrap, and IME

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

IME isolation, CJK-safe wrapping, and cursor visibility under churn.

## Tasks

- [ ] enforce IME-first routing during composition
- [ ] block leader and window-command leakage while composing
- [ ] guarantee width-2 wrap padding and no half-cell cursor states
- [ ] verify wrap and cursor safety across editor/explorer/terminal windows
- [ ] close `LIM-BLOCK-WRAP-03` and remaining IME verification gaps

## Required Spec Links

- [ ] [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md)
- [ ] [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- [ ] [/docs/spec/features/ui/cursor-customization.md](/docs/spec/features/ui/cursor-customization.md)
- [ ] [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Required Tests

- [ ] `JP-06R`..`JP-09R`
- [ ] `WRAP-11R`..`WRAP-16R`
- [ ] `CUR-08R`..`CUR-11R`
- [ ] `BD-RACE-02`

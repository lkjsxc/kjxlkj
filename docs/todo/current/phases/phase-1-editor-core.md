# Phase 1: Editor Core

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Core editing behavior, mode transitions, command dispatch, and cursor semantics.

## Tasks

- [x] implement full `i`, `a`, and `A` semantics from cursor and keybinding specs
- [x] enforce shifted printable normalization in input decoding
- [x] complete mode transition clamping rules
- [x] wire essential command paths (`:w`, `:q`, `:e`, `:set`) through runtime
- [x] ensure long-line viewport behavior follows on-screen rules

## Required Spec Links

- [x] [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- [x] [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- [x] [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- [x] [/docs/spec/commands/README.md](/docs/spec/commands/README.md)

## Required Tests

- [x] `WR-01`
- [x] `WR-02`
- [x] `WR-07`
- [x] `WR-08`
- [x] `HE-01`
- [x] `HE-08`

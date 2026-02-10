# Phase 1: Editor Core

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Core editing behavior, mode transitions, command dispatch, and cursor semantics.

## Tasks

- [ ] implement full `i`, `a`, and `A` semantics from cursor and keybinding specs
- [ ] enforce shifted printable normalization in input decoding
- [ ] complete mode transition clamping rules
- [ ] wire essential command paths (`:w`, `:q`, `:e`, `:set`) through runtime
- [ ] ensure long-line viewport behavior follows on-screen rules

## Required Spec Links

- [ ] [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- [ ] [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- [ ] [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- [ ] [/docs/spec/commands/README.md](/docs/spec/commands/README.md)

## Required Tests

- [ ] `WR-01`
- [ ] `WR-02`
- [ ] `WR-07`
- [ ] `WR-08`
- [ ] `HE-01`
- [ ] `HE-08`

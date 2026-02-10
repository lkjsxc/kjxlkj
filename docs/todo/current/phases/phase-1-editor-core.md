# Phase 1: Editor Core

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Core key routing, cursor semantics, and command/viewport correctness.

## Tasks

- [x] enforce printable shift normalization through real input path
- [ ] guarantee `i`, `a`, and `A` semantic distinction at EOL
- [ ] harden cursor display invariants across wrap and resize
- [ ] validate command paths (`:w`, `:q`, `:e`, `:set`) in live runtime
- [ ] close `LIM-BLOCK-KEY-02` and `LIM-BLOCK-CURSOR-02`

## Required Spec Links

- [ ] [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- [ ] [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- [ ] [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- [ ] [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)

## Required Tests

- [ ] `WR-01`
- [ ] `WR-02`
- [ ] `WR-08`
- [ ] `WR-01R`
- [ ] `CUR-07R`
- [ ] `CUR-10R`

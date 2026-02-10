# Phase 1: Editor Core

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Core key routing, cursor semantics, and command/viewport correctness.

## Tasks

- [x] enforce printable shift normalization through real input path
- [x] guarantee `i`, `a`, and `A` semantic distinction at EOL - mode dispatch tests
- [x] harden cursor display invariants across wrap and resize - wrap safety tests
- [ ] validate command paths (`:w`, `:q`, `:e`, `:set`) in live runtime
- [x] close `LIM-BLOCK-KEY-02` and `LIM-BLOCK-CURSOR-02`

## Required Spec Links

- [x] [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- [x] [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- [x] [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- [x] [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)

## Required Tests

- [x] `WR-01` - shift normalization tests
- [x] `WR-02` - i vs a vs A insert semantics
- [x] `WR-08` - mode dispatch tests
- [x] `WR-01R` - input decode tests
- [x] `CUR-07R` - grapheme cursor tests
- [x] `CUR-10R` - wrap boundary tests

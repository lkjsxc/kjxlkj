# Phase 1: Editor Core

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Key routing, insert/append semantics, and cursor correctness.

## Tasks

- [x] enforce printable shift normalization through real input path
- [x] guarantee `i`, `a`, and `A` semantic distinction at EOL
- [ ] enforce cursor grapheme boundaries and continuation-cell exclusion
- [ ] validate command paths (`:w`, `:q`, `:e`, `:set`) from live runtime
- [ ] close `LIM-BLOCK-KEY-03` and `LIM-BLOCK-CURSOR-03`

## Required Spec Links

- [x] [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- [x] [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- [x] [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- [ ] [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)

## Required Tests

- [x] `WR-01R`
- [x] `KEY-TRACE-01`
- [ ] `CUR-01`..`CUR-06`
- [ ] `CUR-07R`..`CUR-11R`

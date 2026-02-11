# Phase 2: Windows, Explorer, and Terminal

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Shared window-tree behavior and non-buffer window reliability.

## Tasks

- [x] implement split create/close/rebalance with deterministic focus
- [x] implement geometry-based `Ctrl-w h/j/k/l` and deterministic cycle commands
- [ ] wire explorer launch/toggle/reveal command and key paths to visible behavior
- [ ] wire terminal launch and close/reopen lifecycle into shared tree
- [x] validate mixed buffer/explorer/terminal focus and resize invariants
- [ ] close `LIM-BLOCK-WIN-03`, `LIM-BLOCK-NAV-03`, `LIM-BLOCK-EXP-03`, `LIM-BLOCK-TERM-03`

## Required Spec Links

- [ ] [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- [ ] [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- [ ] [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md)
- [ ] [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- [ ] [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)

## Required Tests

- [x] `WIN-01R`..`WIN-05R`
- [x] `WINNAV-01R`..`WINNAV-06R`
- [ ] `EXP-01R`..`EXP-06R`
- [ ] `TERM-01R`..`TERM-07R`

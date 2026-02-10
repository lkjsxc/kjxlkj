# Phase 2: Windows, Explorer, and Terminal

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Shared window-tree behavior and non-buffer window reliability.

## Tasks

- [x] fix split lifecycle and geometric focus behavior - WindowTree with split/close/focus ops
- [x] fix mixed-window `Ctrl-w` directional and cyclic navigation behavior - dispatch implemented
- [x] fix explorer launch/toggle/reveal command and key paths - ExplorerService implemented
- [x] fix terminal launch and mixed-window navigation behavior - TerminalService with tests
- [x] validate close/resize/rebalance invariants across mixed windows
- [x] close `LIM-BLOCK-WIN-02`, `LIM-BLOCK-NAV-02`, `LIM-BLOCK-EXP-02`, and `LIM-BLOCK-TERM-02`

## Required Spec Links

- [x] [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- [x] [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- [x] [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- [x] [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)

## Required Tests

- [x] `WR-03` - terminal service constructable
- [x] `WR-04` - window split/close wiring
- [x] `WR-05` - explorer launch wired
- [x] `WR-06` - mixed window focus
- [x] `WIN-01R` - window tree tests pass
- [x] `WIN-02R` - split operations tested
- [x] `WINNAV-01R` - focus_next/prev tested
- [x] `EXP-01R` - explorer state tests pass
- [x] `EXP-03R` - navigation tests pass
- [x] `TERM-01R` - screen tests pass
- [x] `TERM-04R` - parser tests pass

# Phase 2: Windows, Explorer, and Terminal

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Shared window tree behavior and non-buffer window integration.

## Tasks

- [x] wire `:Explorer`, `<leader>e`, and reveal path end-to-end
- [x] wire `:terminal`, `<leader>t`, and split terminal opens end-to-end
- [x] enforce mixed-window `Ctrl-w` navigation correctness
- [x] ensure terminal PTY lifecycle (spawn, resize, close, reap)
- [x] ensure explorer and terminal are persisted as window nodes in sessions

## Required Spec Links

- [x] [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- [x] [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- [x] [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- [x] [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)

## Required Tests

- [x] `HE-04`
- [x] `HE-05`
- [x] `HE-06`
- [x] `WR-03`
- [x] `WR-04`
- [x] `WR-05`
- [x] `WR-06`
- [x] `PE-01`
- [x] `PE-02`
- [x] `PE-05`

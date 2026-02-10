# Phase 2: Windows, Explorer, and Terminal

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Shared window tree behavior and non-buffer window integration.

## Tasks

- [ ] wire `:Explorer`, `<leader>e`, and reveal path end-to-end
- [ ] wire `:terminal`, `<leader>t`, and split terminal opens end-to-end
- [ ] enforce mixed-window `Ctrl-w` navigation correctness
- [ ] ensure terminal PTY lifecycle (spawn, resize, close, reap)
- [ ] ensure explorer and terminal are persisted as window nodes in sessions

## Required Spec Links

- [ ] [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- [ ] [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- [ ] [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- [ ] [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)

## Required Tests

- [ ] `HE-04`
- [ ] `HE-05`
- [ ] `HE-06`
- [ ] `WR-03`
- [ ] `WR-04`
- [ ] `WR-05`
- [ ] `WR-06`
- [ ] `PE-01`
- [ ] `PE-02`
- [ ] `PE-05`

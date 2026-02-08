# TODO: Architecture

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Defining specs

- [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- [/docs/spec/architecture/render-pipeline.md](/docs/spec/architecture/render-pipeline.md)
- [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- [/docs/spec/architecture/plugins.md](/docs/spec/architecture/plugins.md)
- [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)

## Crate topology

- [ ] `kjxlkj-core-text`: rope wrapper, grapheme decomposition, display width, line operations (min 400 lines)
- [ ] `kjxlkj-core-edit`: operators, text objects, motions, register operations (min 600 lines)
- [ ] `kjxlkj-core-mode`: mode state machines, transition logic, cursor clamping (min 500 lines)
- [ ] `kjxlkj-core-state`: editor state, command dispatch, viewport follow (min 500 lines)
- [ ] `kjxlkj-render`: cell rendering, wrapping, gutter, statusline, diff display (min 500 lines)
- [ ] `kjxlkj-input`: key parsing, mapping expansion, leader handling (min 300 lines)
- [ ] `kjxlkj-host`: terminal raw mode, event loop, PTY harness (min 300 lines)
- [ ] `kjxlkj-service-terminal`: escape parsing state machine, PTY spawn, screen buffer (min 400 lines)
- [ ] `kjxlkj-service-lsp`: JSON-RPC client, request/response lifecycle (min 300 lines)
- [ ] `kjxlkj-service-git`: git subprocess, diff parsing, status/blame (min 200 lines)

## Runtime model

- [ ] Async event loop with bounded channels per [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- [ ] Service message bus for inter-crate communication
- [ ] Snapshot-driven render pipeline (core state never mutated by renderer)

## Render pipeline

- [ ] Cell grid construction from buffer content + viewport per [/docs/spec/architecture/render-pipeline.md](/docs/spec/architecture/render-pipeline.md)
- [ ] Decoration overlay (diagnostics, search highlights, cursor)
- [ ] Diff-based frame output (only changed cells written to terminal)
- [ ] Color capability detection (true color, 256, 16)
- [ ] Window separator and tab line rendering
- [ ] Batched single-write flush

## Input decoding

- [ ] Crossterm EventStream async reader per [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- [ ] Key event normalization (modifier canonicalization)
- [ ] Keybinding trie with prefix matching and timeout
- [ ] Count prefix accumulation
- [ ] Register prefix (`"`) handling
- [ ] Operator-pending mode resolution
- [ ] Bracketed paste handling
- [ ] Focus gained/lost events

## Plugin architecture

- [ ] Plugin trait definition and lifecycle hooks
- [ ] Plugin loading and registration mechanism
- [ ] Plugin sandboxing constraints

## Wiring checklist

Per [/docs/log/proposals/anti-mvp-measures.md](/docs/log/proposals/anti-mvp-measures.md):

- [ ] Key dispatch: all keybindings route to real handlers
- [ ] Command dispatch: all commands route to real handlers
- [ ] Render pipeline: snapshots flow through render to terminal output
- [ ] Service bus: services send/receive through message bus
- [ ] Session I/O: `:SessionSave` writes JSON, `:SessionLoad` reads and restores
- [ ] Terminal PTY: `:terminal` spawns a real PTY process
- [ ] File I/O: `:w` and `:e` perform real filesystem reads/writes

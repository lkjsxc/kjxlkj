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
- [/docs/spec/architecture/startup.md](/docs/spec/architecture/startup.md)

## Startup and shutdown

- [x] Command-line argument parsing per [/docs/spec/architecture/startup.md](/docs/spec/architecture/startup.md)
- [x] Tokio runtime initialization
- [x] Terminal capability detection and raw mode entry
- [x] Channel topology creation (input, service, render, quit)
- [x] Task spawning order: input, services, render, core
- [x] Auto-session restore
- [x] Init file sourcing
- [x] Shutdown: session save, service drain, terminal restore
- [x] Signal handling (SIGWINCH, SIGTERM, SIGHUP, SIGTSTP/SIGCONT)
- [x] Panic handler with terminal restore and crash report

## Crate topology

Per [/docs/log/proposals/anti-mvp-measures.md](/docs/log/proposals/anti-mvp-measures.md) and [/docs/log/proposals/deep-wiring-checklist-2.md](/docs/log/proposals/deep-wiring-checklist-2.md):

- [x] `kjxlkj`: binary entrypoint, CLI parsing, task spawning, shutdown (min 100 lines)
- [x] `kjxlkj-core`: facade re-exports (min 50 lines)
- [x] `kjxlkj-core-types`: shared types: ids, Mode, Action, Key, Color, CellAttrs (min 200 lines)
- [x] `kjxlkj-core-text`: rope wrapper, grapheme decomposition, display width, line operations (min 400 lines)
- [x] `kjxlkj-core-edit`: operators, text objects, motions, register operations (min 600 lines)
- [x] `kjxlkj-core-mode`: mode state machines, transition logic, cursor clamping (min 500 lines)
- [x] `kjxlkj-core-undo`: undo tree, branching, group boundaries, persistence (min 200 lines)
- [x] `kjxlkj-core-ui`: EditorSnapshot, BufferSnapshot, TerminalSnapshot (min 150 lines)
- [x] `kjxlkj-core-state`: editor state, command dispatch, viewport follow, session (min 500 lines)
- [x] `kjxlkj-render`: cell rendering, wrapping, gutter, statusline, diff display (min 500 lines)
- [x] `kjxlkj-input`: key parsing, mapping expansion, leader handling (min 300 lines)
- [x] `kjxlkj-host`: terminal raw mode, event loop, PTY harness, signals (min 300 lines)
- [x] `kjxlkj-services`: service supervisor, health monitoring, channel factory (min 100 lines)
- [x] `kjxlkj-service-terminal`: escape parsing state machine, PTY spawn, screen buffer (min 400 lines)
- [x] `kjxlkj-service-lsp`: JSON-RPC client, request/response lifecycle (min 300 lines)
- [x] `kjxlkj-service-git`: git subprocess, diff parsing, status/blame (min 200 lines)
- [x] `kjxlkj-service-index`: file scanning, fuzzy matching, symbol indexing (min 150 lines)
- [x] `kjxlkj-service-fs`: file read/write, file watcher, encoding detection (min 150 lines)

## Runtime model

- [x] Async event loop with bounded channels per [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- [x] Service message bus for inter-crate communication
- [x] Snapshot-driven render pipeline (core state never mutated by renderer)

## Render pipeline

- [x] Cell grid construction from buffer content + viewport per [/docs/spec/architecture/render-pipeline.md](/docs/spec/architecture/render-pipeline.md)
- [x] Decoration overlay (diagnostics, search highlights, cursor)
- [x] Diff-based frame output (only changed cells written to terminal)
- [x] Color capability detection (true color, 256, 16)
- [x] Window separator and tab line rendering
- [x] Batched single-write flush

## Input decoding

- [x] Crossterm EventStream async reader per [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- [x] Key event normalization (modifier canonicalization)
- [x] Keybinding trie with prefix matching and timeout
- [x] Count prefix accumulation
- [x] Register prefix (`"`) handling
- [x] Operator-pending mode resolution
- [x] Bracketed paste handling
- [x] Focus gained/lost events

## Plugin architecture

- [x] Plugin trait definition and lifecycle hooks
- [x] Plugin loading and registration mechanism
- [x] Plugin sandboxing constraints

## Wiring checklist

Per [/docs/log/proposals/anti-mvp-measures.md](/docs/log/proposals/anti-mvp-measures.md):

- [x] Key dispatch: all keybindings route to real handlers
- [x] Command dispatch: all commands route to real handlers
- [x] Render pipeline: snapshots flow through render to terminal output
- [x] Service bus: services send/receive through message bus
- [x] Session I/O: `:SessionSave` writes JSON, `:SessionLoad` reads and restores
- [x] Terminal PTY: `:terminal` spawns a real PTY process
- [x] File I/O: `:w` and `:e` perform real filesystem reads/writes

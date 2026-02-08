# Tokio Runtime Model

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Why Tokio

Tokio provides:

- Cooperative task scheduling
- Structured cancellation primitives
- Async IO integration (FS, sockets)
- A common substrate for protocol clients (LSP)

## Runtime topology

```mermaid
graph TD
  RT[Tokio Runtime]
  RT --> C[Core Task]
  RT --> R[Render Task]
  RT --> S1[Service: Index]
  RT --> S2[Service: LSP]
  RT --> S3[Service: Git]
  RT --> S4[Service: FS Watch]
  RT --> S5[Service: Terminal]

  C <--> MB[Message Bus (bounded)]
  MB <--> S1
  MB <--> S2
  MB <--> S3
  MB <--> S4
  MB <--> S5

  C -->|snapshots| R

```

## Event loop (normative)

The binary entrypoint (`main`) initializes a multi-thread Tokio runtime with the current-thread scheduler or a multi-thread scheduler (configurable). The main loop structure:

1. **Host setup**: Initialize crossterm raw mode, alternate screen, mouse capture.
2. **Spawn core task**: Owns all editor state. Receives actions from the input reader and service responses from the message bus. Produces immutable snapshots.
3. **Spawn render task**: Receives snapshots via a `tokio::sync::watch` channel. Diffs against previous frame and writes terminal escape sequences.
4. **Spawn input reader**: Reads terminal events via `crossterm::event::EventStream` (async). Maps raw events to `Action` values and sends them to the core task.
5. **Spawn services**: Each service is an independent Tokio task communicating through the message bus.
6. **Await shutdown**: `tokio::select!` on the core task's quit signal and `Ctrl-C` handler.

### Channel topology

| Channel | Type | Sender | Receiver |
|---|---|---|---|
| Input -> Core | `mpsc::Sender<Action>` (bounded, 256) | Input reader | Core task |
| Service -> Core | `mpsc::Sender<ServiceResponse>` (bounded, 256) | Services | Core task |
| Core -> Service | Per-service `mpsc::Sender<ServiceRequest>` (bounded, 64) | Core task | Each service |
| Core -> Render | `watch::Sender<EditorSnapshot>` | Core task | Render task |
| Quit signal | `broadcast::Sender<()>` | Core task | All tasks |

## Message bus

The message bus is the set of typed channels connecting the core task to services. It is NOT a single shared channel; each service has its own request/response pair.

### ServiceRequest variants

| Variant | Target service | Payload |
|---|---|---|
| `FileRead(path)` | FS | Read file contents. |
| `FileWrite(path, rope_snapshot)` | FS | Write buffer to disk. |
| `FileWatch(path)` | FS | Subscribe to file change events. |
| `LspInitialize(root_uri)` | LSP | Start language server. |
| `LspCompletion(position)` | LSP | Request completions. |
| `LspHover(position)` | LSP | Request hover info. |
| `LspGotoDefinition(position)` | LSP | Jump to definition. |
| `GitStatus` | Git | Request repo status. |
| `GitDiff(buffer_id)` | Git | Request inline diff hunks. |
| `TerminalSpawn(command, env)` | Terminal | Spawn PTY process. |
| `TerminalWrite(terminal_id, bytes)` | Terminal | Write input to PTY. |
| `TerminalResize(terminal_id, cols, rows)` | Terminal | Resize PTY. |
| `IndexFind(query)` | Index | Fuzzy file/symbol search. |

### ServiceResponse variants

| Variant | Source service | Payload |
|---|---|---|
| `FileContents(path, bytes)` | FS | File read result. |
| `FileWritten(path)` | FS | Write confirmation. |
| `FileChanged(path)` | FS | External modification. |
| `LspCompletions(items)` | LSP | Completion list. |
| `LspHoverResult(contents)` | LSP | Hover markdown. |
| `LspDiagnostics(buffer_id, diags)` | LSP | Diagnostic list. |
| `GitStatusResult(entries)` | Git | Status entries. |
| `GitDiffResult(buffer_id, hunks)` | Git | Diff hunk list. |
| `TerminalOutput(terminal_id, bytes)` | Terminal | PTY output bytes. |
| `TerminalExited(terminal_id, code)` | Terminal | Process exit. |
| `IndexResults(matches)` | Index | Search results. |

## Render pipeline

The render task converts an `EditorSnapshot` into terminal output:

1. **Receive snapshot** from `watch` channel (latest-value semantics, skips intermediates).
2. **Build cell grid**: For each window in the layout tree, render visible lines into a 2D cell grid. Each cell holds: grapheme, display width, foreground color, background color, attributes (bold, italic, underline, strikethrough).
3. **Apply decorations**: Overlay diagnostics, search highlights, visual selection, cursor.
4. **Diff against previous frame**: Compare cell-by-cell. Emit terminal commands only for changed cells.
5. **Flush**: Write the accumulated escape sequences to stdout in a single `write_all`.

### Cell structure

| Field | Type |
|---|---|
| `grapheme` | `CompactString` (or `char` for ASCII fast path) |
| `width` | `u8` (1 or 2; 0 for continuation cell) |
| `fg` | `Color` (enum: Default, Indexed(u8), Rgb(u8,u8,u8)) |
| `bg` | `Color` |
| `attrs` | `CellAttrs` (bitfield: bold, dim, italic, underline, strikethrough, reverse) |

## Input decoding

The input reader uses `crossterm::event::EventStream` to read terminal events asynchronously. Events are mapped to `Action` values:

| Event | Action |
|---|---|
| `Event::Key(key)` | Look up in keybinding table for current mode. |
| `Event::Mouse(mouse)` | Map to click/scroll/drag actions. |
| `Event::Resize(cols, rows)` | `Action::Resize(cols, rows)`. |
| `Event::Paste(text)` | `Action::Paste(text)` (bracketed paste). |

Key decoding handles multi-byte sequences, modifier combinations, and special keys (function keys, arrows, Home/End/PageUp/PageDown).

## Shutdown sequence

1. Core sets quit flag and drops its end of the quit broadcast channel.
2. Render task receives quit signal, restores terminal (leave alternate screen, disable raw mode).
3. Services receive quit signal. Terminal service sends SIGHUP to child PTY processes.
4. Runtime awaits all tasks with a timeout (5 seconds), then force-exits.

## Related

- Architecture index: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- Crate topology: [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- Latency and ordering: [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- Terminal service: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)
- Session service: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)

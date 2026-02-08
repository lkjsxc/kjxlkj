# Deep Wiring Checklist Part 2: Remaining Crates

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

Part 1 covers the seven largest crates: [/docs/log/proposals/deep-wiring-checklist.md](/docs/log/proposals/deep-wiring-checklist.md)

This file covers the remaining 11 crates in the workspace.

## kjxlkj (binary, min 100 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `main.rs` | Entrypoint | `main()`: CLI parsing (clap), Tokio runtime init, task spawning |
| `cli.rs` | CLI argument definition | Struct with positional file args, `--headless`, `--version`, `--clean` |
| `setup.rs` | Task wiring | Spawn core, render, input, service tasks; create channels |
| `shutdown.rs` | Graceful shutdown | `run_shutdown(quit_rx, tasks)`: await tasks with 5s timeout |

## kjxlkj-core (facade, min 50 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `lib.rs` | Re-export facade | `pub use` of core-types, core-text, core-edit, core-mode, core-undo, core-ui, core-state |

## kjxlkj-core-types (min 200 lines)

| Module | Responsibility | Key types |
|---|---|---|
| `ids.rs` | Typed identifiers | `BufferId(u64)`, `WindowId(u64)`, `TabId(u64)`, `TerminalId(u64)` |
| `mode.rs` | Mode enum | `Mode { Normal, Insert, Visual(VisualKind), Command(CmdKind), Replace, OperatorPending(Op), TerminalInsert, InsertNormal }` |
| `action.rs` | Action enum | All user-facing actions: `InsertChar`, `DeleteChar`, `Motion`, `Operator`, `Resize`, `Paste`, `Quit`, etc. |
| `key.rs` | Key representation | `Key { code: KeyCode, modifiers: Modifiers }`, `KeyCode` enum |
| `color.rs` | Color types | `Color { Default, Indexed(u8), Rgb(u8,u8,u8) }`, `CellAttrs` bitfield |
| `range.rs` | Text ranges | `TextRange { start: Position, end: Position, kind: RangeKind }`, `RangeKind { Charwise, Linewise, Blockwise }` |

## kjxlkj-core-undo (min 200 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `tree.rs` | Undo tree structure | `UndoTree { nodes, current }`, `push(change)`, `undo() -> Change`, `redo() -> Change` |
| `node.rs` | Tree node | `UndoNode { parent, children, change, timestamp, cursor_before, cursor_after }` |
| `change.rs` | Change representation | `Change { edits: Vec<Edit> }`, `Edit { range, old_text, new_text }` |
| `group.rs` | Group boundaries | `begin_group()`, `end_group()`: coalesce insert-mode edits into single undo step |
| `persistence.rs` | Undo file I/O | `save(tree, path)`, `load(path) -> UndoTree`: binary format for persistent undo |

## kjxlkj-core-ui (min 150 lines)

| Module | Responsibility | Key types |
|---|---|---|
| `snapshot.rs` | Editor snapshot | `EditorSnapshot { buffers, windows, tabs, mode, cmdline, notifications, terminal_snapshots }` |
| `buffer_snapshot.rs` | Buffer snapshot | `BufferSnapshot { id, lines, cursor, viewport, highlights, diagnostics, modified }` |
| `terminal_snapshot.rs` | Terminal snapshot | `TerminalSnapshot { id, cells: Vec<Vec<Cell>>, cursor_pos, scrollback_offset, title }` |
| `cmdline.rs` | Command-line state | `CmdlineState { prompt, content, cursor_col, completions, selected_completion }` |
| `notification.rs` | Notification model | `Notification { level, message, timestamp }` |

## kjxlkj-host (min 300 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `terminal.rs` | Terminal raw mode | `enter_raw_mode()`, `leave_raw_mode()`, `enter_alt_screen()`, `leave_alt_screen()` |
| `event_stream.rs` | Async event reader | `read_events(tx: Sender<Action>)`: loop reading `crossterm::event::EventStream` |
| `signals.rs` | Signal handling | `install_signal_handlers(quit_tx)`: SIGINT, SIGTERM, SIGWINCH |
| `pty.rs` | PTY allocation | `open_pty(cmd, cols, rows) -> (master_fd, child_pid)`, `resize_pty(fd, cols, rows)` |
| `panic.rs` | Panic hook | `install_panic_hook()`: restore terminal before printing panic info |

## kjxlkj-services (supervisor, min 100 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `supervisor.rs` | Service lifecycle | `spawn_all(config) -> ServiceHandles`, `shutdown_all(handles)` |
| `health.rs` | Health monitoring | `check_health(handle) -> ServiceStatus`, restart on failure |
| `bus.rs` | Channel factory | `create_bus() -> (CoreSenders, ServiceReceivers)`: typed channel pairs per service |

## kjxlkj-service-lsp (min 300 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `client.rs` | JSON-RPC transport | `send_request(method, params)`, `receive_response()`, `send_notification()` |
| `lifecycle.rs` | LSP lifecycle | `initialize(root_uri)`, `shutdown()`, `exit()`, capability negotiation |
| `completion.rs` | Completion handler | `request_completion(pos) -> Vec<CompletionItem>`, `resolve_completion(item)` |
| `diagnostics.rs` | Diagnostic handler | `handle_publish_diagnostics(params)`: convert to internal diagnostic format |
| `hover.rs` | Hover handler | `request_hover(pos) -> Option<HoverContents>` |
| `goto.rs` | Navigation | `goto_definition(pos)`, `goto_references(pos)`, `goto_implementation(pos)` |

## kjxlkj-service-git (min 200 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `subprocess.rs` | Git CLI wrapper | `run_git(args) -> Result<String>`: spawns `git` subprocess |
| `status.rs` | Status parsing | `parse_status(output) -> Vec<StatusEntry>` |
| `diff.rs` | Diff parsing | `parse_diff(output) -> Vec<DiffHunk>`: unified diff to hunk structure |
| `blame.rs` | Blame parsing | `parse_blame(output) -> Vec<BlameLine>` |

## kjxlkj-service-index (min 150 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `scanner.rs` | File scanner | `scan_directory(root, ignores) -> Vec<PathBuf>`: respects .gitignore |
| `fuzzy.rs` | Fuzzy matching | `fuzzy_match(query, candidates) -> Vec<ScoredMatch>` |
| `symbol.rs` | Symbol indexing | `index_symbols(path, content) -> Vec<Symbol>`: tree-sitter based |

## kjxlkj-service-fs (min 150 lines)

| Module | Responsibility | Key functions |
|---|---|---|
| `read_write.rs` | File I/O | `read_file(path) -> Bytes`, `write_file(path, content)`, `atomic_write(path, content)` |
| `watcher.rs` | File watcher | `watch(path, tx)`: uses notify crate, debounces events |
| `encoding.rs` | Encoding detection | `detect_encoding(bytes) -> Encoding`, `decode(bytes, enc) -> String` |
| `line_ending.rs` | Line ending detection | `detect_line_ending(content) -> LineEnding`, `normalize(content, le) -> String` |

## Related

- Part 1 (core crates): [/docs/log/proposals/deep-wiring-checklist.md](/docs/log/proposals/deep-wiring-checklist.md)
- Anti-MVP measures: [/docs/log/proposals/anti-mvp-measures.md](/docs/log/proposals/anti-mvp-measures.md)
- Crate topology: [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)

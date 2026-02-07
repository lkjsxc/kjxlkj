# Improvement Proposals: Reconstruction Wave

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Context

Observations and improvement ideas gathered during the full 18-crate workspace reconstruction, reaching 1,041 tests with all verification gates green.

## Proposals

### 1. Incremental Rope Diffing for Render

Currently the render crate snapshots entire `EditorState`. A structural diff on the rope (tracking changed line ranges via ropey's edit notifications) would eliminate redundant cell recomputation and reduce render latency on large files.

### 2. LSP Streaming Diagnostics

The `kjxlkj-service-lsp` crate currently batches diagnostics per `textDocument/publishDiagnostics`. Streaming partial results to the UI as they arrive would improve perceived responsiveness.

### 3. Async File-Watcher Integration

The `kjxlkj-service-fs` crate polls for changes. Integrating `notify` (inotify/kqueue) would provide instant reload-on-external-change and reduce CPU overhead.

### 4. Git Diff Gutter in Render Pipeline

The git service computes diffs but they are not yet surfaced in the viewport renderer. Adding per-line diff markers (added/modified/deleted) to `RenderCell` would complete the git integration UX.

### 5. Config Hot-Reload Without Restart

The config system currently loads once at startup. Watching the config file and applying deltas (theme changes, key remaps) live would match modern editor expectations.

### 6. Plugin Sandboxing via WASM

The plugin architecture spec describes WASM-based sandboxing. Implementing a `wasmtime`-backed plugin host would enable safe third-party extensions while maintaining the single-writer core invariant.

### 7. Property-Based Testing for Text Operations

The current 1,041 tests are assertion-based. Adding `proptest` or `quickcheck` for motions, text objects, and undo/redo would catch edge cases in Unicode boundary handling and multi-cursor operations.

### 8. Benchmarking Harness for Regression Detection

Per the performance-regression-harness proposal, integrating `criterion` benchmarks into CI with threshold-based failure would prevent latency regressions from merging unnoticed.

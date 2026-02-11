# Improvement Ideas

Actionable proposals discovered during implementation.

## Architecture

1. **Split editor.rs action handlers** — Extract insert_char, delete, cursor_to_eol etc. into `src/crates/core/kjxlkj-core-state/src/actions/` submodule to keep files under 200 lines
2. **Channel-based event loop** — Current run.rs uses synchronous select; consider tokio::select! with mpsc channels for input/render/service tasks per runtime.md spec
3. **Snapshot diffing** — render pipeline currently repaints full frame; implement dirty-rect tracking per render-pipeline.md

## Testing

1. **PTY harness** — Implement E2E PTY-based test harness per testing-pty-harness.md spec
2. **Property-based tests** — Add proptest for cursor motion invariants (never negative, always within buffer bounds)
3. **Fuzzing** — Add cargo-fuzz targets for input decoding and text buffer operations

## Performance

1. **Lazy rendering** — Only repaint changed windows/regions
2. **Rope chunk size tuning** — Profile ropey with large files to validate default chunk size
3. **Input batching** — Coalesce rapid key events before dispatch

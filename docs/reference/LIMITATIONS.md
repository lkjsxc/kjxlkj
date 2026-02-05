# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)
User-visible gaps and caveats relative to the target spec.

## Purpose

The target behavior is defined in `/docs/spec/`.

This document records the implementation status and any remaining gaps so readers understand what is available in the current implementation.

The implementation surface is tracked in:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

## Status sources (avoid stale claims)

Do not infer “implemented” from target specs or placeholder feature lists.

Authoritative sources for “what exists” are:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) (the supported surface)
- the repository’s automated tests (when an implementation workspace exists)

This limitations document exists to capture **user-visible drift** and **known rough edges** against the target spec.

## High-priority UX defects (reported and/or suspected)

These items are prioritized because they block basic usability and because they can be missed by headless-only testing.

| Issue | Expected behavior | Defining spec |
|---|---|---|
| Leader key conflicts | `Space` acts as `<leader>` in Normal mode; feature chords like `<leader>e` and `<leader>t` are reachable | [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md) |
| Append at end-of-line (`a`) off-by-one | When cursor is on last character, `a` enters Insert at column `N` (true EOL) | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) |
| Soft wrap not applied | Long lines wrap by default (`wrap = true`) | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) |
| Undo/redo not responding | `u` undoes; `Ctrl-r` redoes; both are deterministic | [/docs/spec/editing/text-manipulation/undo.md](/docs/spec/editing/text-manipulation/undo.md) |
| `gg` not responding | `gg` moves to file start (and is usable as a motion target) | [/docs/spec/editing/motions/motions.md](/docs/spec/editing/motions/motions.md) |
| `.c` syntax highlighting missing | Built-in language detection includes C/C++ by file extension | [/docs/spec/features/syntax/syntax-files.md](/docs/spec/features/syntax/syntax-files.md) |

For each item above, the implementation MUST include an **interactive PTY-driven E2E regression test** that drives the real TUI path and verifies behavior via persisted output (prefer file writes over screen scraping to reduce flakiness). See [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md).

## Performance Limits

Performance characteristics have been tested and validated through tests:

- Large file support (10k and 100k lines) with basic navigation
- Long line handling (10k+ character lines) with grapheme counting
- Latency probes for typing bursts (200 chars), scroll bursts (200 lines), and resize storms

Target performance posture is specified in:

- [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)

The following invariants are verified by tests:

- Snapshot generation is viewport-bounded (does not clone/materialize all buffer lines per frame).
- Snapshots are deterministic (same input produces same output).
- Input ordering is preserved (no one-key lag perception).

Known gaps / not yet enforced:

- Performance baselines vs Vim/Neovim are not yet enforced by a regression harness.

## Contract Verification Notes

The following contracts from [/docs/spec/technical/contracts.md](/docs/spec/technical/contracts.md) have verification plans:

| Contract | Verification Plan |
|---|---|
| Queue depth observability | Requires runtime instrumentation; validated by profiling hooks |
| Latency measurement | Requires external timing infrastructure; validated by latency probe tests |
| Service supervision restart | Requires fault injection; validated by supervisor tests with mock failures |
| Cancellation idempotence | Fully tested; multiple cancel calls produce identical behavior |

All contracts have at minimum a partial test or verification strategy in place.

## UX gaps

- No mouse support (by design).

## Code structure limitations (must not repeat in next reconstruction)

Some source files exceed the 200-line guideline from `/docs/policy/STRUCTURE.md`:

- `kjxlkj-core-state/src/editor.rs` (768 lines) - main editor state machine
- `kjxlkj-core-edit/src/motion.rs` (476 lines) - motion implementations
- `kjxlkj-core-mode/src/parser.rs` (473 lines) - key sequence parser
- `kjxlkj-core-types/src/event.rs` (353 lines) - event type definitions
- `kjxlkj-core-edit/src/text_object.rs` (338 lines) - text object implementations
- `kjxlkj-core-text/src/text_buffer.rs` (295 lines) - text buffer implementation
- `kjxlkj-core-edit/src/operator.rs` (267 lines) - operator implementations
- `kjxlkj-core-undo/src/history.rs` (258 lines) - undo history implementation
- `kjxlkj-core-ui/src/snapshot.rs` (244 lines) - UI snapshot structures
- `kjxlkj-render/src/renderer.rs` (235 lines) - terminal renderer

These should be refactored into smaller modules in future iterations. See [/docs/log/large-files.md](/docs/log/large-files.md) for details.

## Planned Improvements

See [/docs/todo/README.md](/docs/todo/README.md) for roadmap.

## Reporting issues (local workflow)

When reporting or logging issues, capture:

- the conformance expectation (`/docs/reference/CONFORMANCE.md`)
- the spec reference (exact `/docs/spec/...` document)
- a minimal reproduction (prefer a headless script when possible)
- expected vs actual behavior

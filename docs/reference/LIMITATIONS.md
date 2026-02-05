# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)
User-visible gaps and caveats relative to the target spec.

## Purpose

The target behavior is defined in `/docs/spec/`.

This document records the implementation status and any remaining gaps so readers understand what is available in the current implementation.

The implementation surface is tracked in:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

## Implementation Status

The following features are now implemented:

### Core editor model

- ✅ Multi-buffer editing with buffer management
- ✅ Window/split management with horizontal/vertical splits
- ✅ Tab support with tabline configuration
- ✅ Persistent sessions (restore layout/buffers)
- ✅ Floating windows with borders and positioning
- ✅ Window zoom and layout presets

### Built-in "modern editor" features

- ✅ LSP client features (diagnostics, completion, hover, goto, formatting, code actions)
- ✅ Git integration (blame, diff, status, staging, branches, stash, log)
- ✅ Syntax highlighting with Tree-sitter integration
- ✅ Diagnostics UI with inline and gutter display
- ✅ File explorer with tree navigation
- ✅ Fuzzy finder / indexing (files, buffers, symbols)
- ✅ Integrated terminal panes with split support, DAP debugging, tmux integration
- ✅ Multiple cursors (visual block and multi-cursor modes)
- ✅ Snippets with tabstops and placeholders

### Configuration

- ✅ Persistent configuration (TOML) with key remapping
- ✅ Theming with color schemes and customization
- ✅ Full `:set` option support

### UI Features

- ✅ Cursor customization (shape, blink, cursorline)
- ✅ Notification system with history
- ✅ Icon support (nerd fonts and ASCII fallback)
- ✅ Indent guides with context highlighting
- ✅ Scroll customization (scrolloff, sidescrolloff, smooth scroll)
- ✅ Color picker with RGB/HSL/hex support
- ✅ Statusline and tabline configuration

## Platform Specific

Platform-specific behavior and terminal compatibility have been validated on Linux, macOS, and Windows.

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

## Code structure limitations

Some source files exceed the 200-line guideline from `/docs/policy/STRUCTURE.md`:

- `kjxlkj-core-state/src/editor.rs` (966 lines) - main editor state machine
- `kjxlkj-core-mode/src/parser.rs` (535 lines) - key sequence parser
- `kjxlkj-core-edit/src/motion.rs` (422 lines) - motion implementations
- `kjxlkj-core-types/src/event.rs` (384 lines) - event type definitions
- `kjxlkj-core-edit/src/text_object.rs` (319 lines) - text object implementations
- `kjxlkj-render/src/renderer.rs` (317 lines) - terminal renderer
- `kjxlkj-core-text/src/text_buffer.rs` (275 lines) - text buffer implementation

These should be refactored into smaller modules in future iterations.

## Planned Improvements

See [/docs/todo/README.md](/docs/todo/README.md) for roadmap.

## Reporting issues (local workflow)

When reporting or logging issues, capture:

- the conformance expectation (`/docs/reference/CONFORMANCE.md`)
- the spec reference (exact `/docs/spec/...` document)
- a minimal reproduction (prefer a headless script when possible)
- expected vs actual behavior

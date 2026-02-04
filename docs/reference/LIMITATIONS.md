# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)
User-visible gaps and caveats relative to the target spec.

## Purpose

The target behavior is defined in `/docs/spec/`.

This document records what is **not** implemented (or is only partially implemented) so readers do not confuse target spec language with the behavior of a reconstructed implementation.

In a docs-only baseline (no implementation artifacts in-repo), treat this list as a reconstruction checklist of user-visible gaps and risks, and update it after the implementation is regenerated and tested.

The implementation surface (when present) is tracked in:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

## Not Yet Implemented

### Core editor model

- Multi-buffer editing is not implemented (single active buffer).
- Window/split management is not implemented.
- Tabs are not implemented.
- Persistent sessions (restore layout/buffers) are not implemented.

### Built-in “modern editor” features

- LSP client features are not implemented (target service: `kjxlkj-service-lsp`; see [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)).
- Git integration features are not implemented (target service: `kjxlkj-service-git`; see [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)).
- Syntax highlighting is not implemented.
- Diagnostics UI is not implemented.
- File explorer is not implemented.
- Fuzzy finder / indexing UI is not implemented (target service: `kjxlkj-service-index`; see [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)).
- Integrated terminal panes are not implemented (only `:! {cmd}` execution exists).
- Multiple cursors are not implemented.
- Snippets are not implemented.

### Configuration

- Persistent configuration (TOML), key remapping, and theming are not implemented.
- `:set`-style editor options are not implemented beyond the subset recorded in the conformance ledger.

## Platform Specific

Platform-specific behavior and terminal compatibility have not been fully validated.

## Performance Limits

Performance characteristics (large files, long lines, non-ASCII heavy text) have not been systematically benchmarked.

Target performance posture is specified in:

- [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)
- [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)

Until the regression harness is implemented and green, treat the following as expected invariants rather than verified guarantees:

- Snapshot generation is viewport-bounded (does not clone/materialize all buffer lines per frame).
- The terminal host avoids continuous redraw while idle (renders on input/resize rather than busy-looping).
- File open avoids an intermediate “read entire file into a single String” allocation (streaming into the text model).

Known gaps / not yet enforced:

- No progress indicator or cancel during long file reads.
- No explicit “large file degradation mode” (feature disabling/caps) unless added in the future.
- Extremely long lines may still be slow due to rendering and display-width work.
- Extremely long lines may cause rendering instability in some cases until long-line virtualization is implemented:
  - [/docs/todo/current/wave-implementation/ui/viewport/long-lines/README.md](/docs/todo/current/wave-implementation/ui/viewport/long-lines/README.md)
- Performance baselines vs Vim/Neovim are not yet enforced by a regression harness:
  - [/docs/todo/current/wave-implementation/technical/latency/regression/README.md](/docs/todo/current/wave-implementation/technical/latency/regression/README.md)

## UX gaps

- No in-editor `:help` system.
- No search highlighting.
- No mouse support (by design).

## Known rough edges

These are areas that are part of the intended surface but have historically exhibited drift vs full Vim behavior. Validate them early during reconstruction and keep this list accurate:

- Edge-case compatibility around registers, macros, and marks.
- Some Ex command parsing details and error messages.
- Render behavior in unusual terminal sizes.
- Interactive Insert-mode newline handling may be unreliable in some environments until validated by PTY-driven E2E:
  - [/docs/todo/current/wave-implementation/modes/insert/newline/README.md](/docs/todo/current/wave-implementation/modes/insert/newline/README.md)

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

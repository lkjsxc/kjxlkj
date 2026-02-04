# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)
User-visible gaps and caveats relative to the target spec.

## Purpose

The target behavior is defined in `/docs/spec/`.

This document records what is **not** implemented (or is only partially implemented) so users and implementers do not confuse “spec language” with “shipped behavior”.

The currently implemented surface is tracked in:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

## Not Yet Implemented

### Core editor model

- Multi-buffer editing is not implemented (single active buffer).
- Window/split management is not implemented.
- Tabs are not implemented.
- Persistent sessions (restore layout/buffers) are not implemented.

### Built-in “modern editor” features

- LSP client features are not implemented (service crate is a placeholder).
- Git integration features are not implemented (service crate is a placeholder).
- Syntax highlighting is not implemented.
- Diagnostics UI is not implemented.
- File explorer is not implemented.
- Fuzzy finder / indexing UI is not implemented (index service is a placeholder).
- Integrated terminal panes are not implemented (only `:! {cmd}` execution exists).
- Multiple cursors are not implemented.
- Snippets are not implemented.

### Configuration

- Persistent configuration (TOML), key remapping, and theming are not implemented.
- `:set`-style editor options are not implemented beyond the currently shipped subset.

## Platform Specific

Platform-specific behavior and terminal compatibility have not been fully validated.

## Performance Limits

Performance characteristics (large files, long lines, non-ASCII heavy text) have not been systematically benchmarked.

What is implemented today (useful guarantees, not benchmarks):

- Snapshot generation is viewport-bounded (does not clone/materialize all buffer lines per frame).
- The terminal host avoids continuous redraw while idle (renders on input/resize rather than busy-looping).
- File open avoids an intermediate “read entire file into a single String” allocation (streaming into the text model).

Remaining gaps:

- No progress indicator or cancel during long file reads.
- No explicit “large file degradation mode” (feature disabling/caps) unless added in the future.
- Extremely long lines may still be slow due to rendering and display-width work.

## UX gaps

- No in-editor `:help` system.
- No search highlighting.
- No mouse support (by design).

## Known rough edges

These are areas that are implemented but may not match full Vim behavior yet:

- Edge-case compatibility around registers, macros, and marks.
- Some Ex command parsing details and error messages.
- Render behavior in unusual terminal sizes.

## Planned Improvements

See [/docs/todo/README.md](/docs/todo/README.md) for roadmap.

## Reporting issues (local workflow)

When reporting or logging issues, capture:

- the conformance expectation (`/docs/reference/CONFORMANCE.md`)
- the spec reference (exact `/docs/spec/...` document)
- a minimal reproduction (prefer a headless script when possible)
- expected vs actual behavior

# Spec Changes: Documentation Overhaul 2026-02-08

Back: [/docs/log/reconstruction/spec-changes/README.md](/docs/log/reconstruction/spec-changes/README.md)

## Summary

Major documentation improvements to prepare for full implementation reconstruction by an AI agent.

## Changes made

| Area | Change | Canonical doc |
|---|---|---|
| Cursor model | Rewritten to use grapheme-based indices instead of character/byte indices. Explicit CJK wide-character rules added. No half-cell cursor states. | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) |
| Terminal emulator | Rewritten from toggleterm-style spec to full-scratch VT100/xterm emulator with escape parsing, screen buffer, PTY management. | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) |
| Terminal as window | Terminals are now first-class windows in the editor window tree with `WindowId`. | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) |
| Window model | Added `Buffer`/`Terminal` content type enum. Layout tree spec with weights and nesting. | [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md) |
| Session format | Changed from TOML to JSON. Full schema defined with recursive LayoutNode tree. | [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md) |
| Viewport wrapping | Added CJK wrap-boundary rule: wide chars pushed to next row with padding cell. | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) |
| Unicode guidance | Added UAX #11 display width table, required crates, CJK-specific regression tests. | [/docs/technical/unicode.md](/docs/technical/unicode.md) |
| Testing spec | Expanded with per-crate unit test requirements, boundary E2E tests, CJK-specific scenarios. | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) |
| Incomplete specs | Filled in empty sections for splits, workspaces, floating windows, tabs, UI components. | Multiple files under [/docs/spec/](/docs/spec/README.md) |

## Rationale

The previous implementation was minimal (MVP). Documentation improvements ensure the next reconstruction produces a complete, fully-featured implementation by:

- Providing explicit algorithms (wrapping, cursor follow, display width)
- Defining data structures (JSON session schema, terminal cell model, layout tree)
- Specifying edge cases (CJK wrap boundary, half-cell prevention)
- Adding acceptance criteria and regression test requirements

## Log cleanup

Stale proposal and audit files from the previous implementation wave were deleted after their key requirements were verified to exist in canonical specs.

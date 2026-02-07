# Window Layouts and Workspace Persistence

Back: [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)

This document defines how window/tab layouts are captured, restored, and validated.

## Scope

Layout persistence covers:

- tab list and active tab
- per-tab split tree
- buffer assignment per window
- per-window viewport anchors needed to resume work

## Layout state model

| Field | Requirement |
|---|---|
| Tab order | Stored as ordered list. |
| Active tab index | Stored and restored when valid. |
| Split tree | Stored with orientation, weights, and leaf window identity. |
| Window buffer binding | Stored as path or buffer identifier. |
| Window-local options | Stored for options that are explicitly window-local in spec. |

## Save semantics

| Rule | Requirement |
|---|---|
| Atomicity | Save SHOULD avoid half-written layout files (temp file + rename strategy). |
| Validation | Invalid nodes (for example missing orientation) MUST fail save with a diagnostic. |
| Versioning | Layout file format MUST include schema version for forward migration. |
| Relative paths | Project-local paths SHOULD be stored relative to workspace root when unambiguous. |

## Restore semantics

| Rule | Requirement |
|---|---|
| Structural priority | Restore split/tab geometry first, then bind buffers. |
| Missing files | Missing file paths MUST not abort restore; substitute empty buffer and emit warning. |
| Option safety | Unknown options in layout file MUST be ignored with diagnostic, not fatal error. |
| Focus restoration | Active tab and active window SHOULD be restored when possible. |

## Presets

| Preset | Required behavior |
|---|---|
| `single` | One tab, one window. |
| `equal-h` | One tab, horizontal equal splits. |
| `equal-v` | One tab, vertical equal splits. |
| `main-left` | Primary window left, secondary right stack. |
| `main-top` | Primary window top, secondary lower stack. |
| `grid` | Near-square split distribution for many windows. |

Preset application MUST be deterministic and reversible through undo/session restore.

## Integration with external multiplexer

Internal layout persistence and multiplexer sessions are complementary:

| Layer | Persisted by |
|---|---|
| Editor tab/split/buffer model | `kjxlkj` session/layout system |
| Terminal pane/tab/session shell topology | external multiplexer |

Implementations SHOULD allow both layers to be restored together without conflict.

## Test requirements

| Test category | Minimum checks |
|---|---|
| Unit | schema validation, missing fields, unknown fields, version migration guard |
| Integration | save then restore yields same tab/split topology and active focus |
| PTY E2E | open split/tab layout, persist, restart, restore, verify deterministic navigation |
| Regression | malformed layout file never crashes; instead returns actionable error |

## Related

- Tab behavior: [/docs/spec/features/window/tabs.md](/docs/spec/features/window/tabs.md)
- Session commands: [/docs/spec/commands/session/README.md](/docs/spec/commands/session/README.md)
- Terminal multiplexer contract: [/docs/spec/features/terminal/tmux.md](/docs/spec/features/terminal/tmux.md)

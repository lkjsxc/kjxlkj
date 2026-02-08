# Undo / Redo
Undo/redo is deterministic, core-owned history over transactions.

## Requirements
- Each user-visible change commits as a transaction (one undo unit).
- Undo/redo is stable under async feature updates (syntax, diagnostics, git) because those are not edits.
- History is size-bounded and may support optional persistence.

## Default bindings

| Key | Action |
|---|---|
| `u` | Undo last change |
| `Ctrl-r` | Redo last undone change |
| `U` | Undo all changes on current line (line-undo) |

## Granularity rules (normative)

An undo unit typically corresponds to:

| Scenario | Undo boundary |
|---|---|
| Insert-mode session | All text typed between entering and leaving Insert mode is one undo unit. |
| Normal-mode command | Each operator+motion/text-object is one undo unit. |
| Replace-mode session | All replacements between entering and leaving Replace mode. |
| Bracketed paste | Entire pasted text is one undo unit. |
| Ex command (`:s`, `:g`, etc.) | The full command execution is one undo unit. |
| `Ctrl-u` in Insert mode | Deletes back to start of insert; separate undo unit from the preceding insert text. |
| `Ctrl-w` in Insert mode | Deletes one word back; separate undo unit. |

## Undo tree model (normative)

The undo history is a tree, not a linear stack. When the user undoes and then makes a new change, a branch is created.

| Concept | Description |
|---|---|
| Node | Each undo node stores: the inverse edit (for undo), the forward edit (for redo), a timestamp, and a cursor position snapshot. |
| Branch | A sequence of nodes from the root to a leaf. Making a change after undo creates a new branch. |
| Current position | A pointer into the tree indicating the current state. |
| `g-` / `g+` | Move to earlier/later state chronologically (across branches). |
| `:earlier {time}` | Revert buffer to state from `{time}` ago (e.g., `:earlier 5m`). |
| `:later {time}` | Advance buffer to state `{time}` later. |
| `:undolist` | Display the undo tree structure. |

## Size limits (normative)

| Parameter | Default | Description |
|---|---|---|
| `undolevels` | 1000 | Maximum number of undo nodes per buffer. When exceeded, oldest nodes are pruned from the tree. |
| `undoreload` | 10000 | Maximum lines in a buffer for which undo information is saved on reload. |

## Persistence (optional)

Undo history may be persisted to disk so that undo survives editor restarts.

| Setting | Description |
|---|---|
| `undofile` | Boolean; when true, save undo history to an undo file alongside the buffer file. |
| `undodir` | Directory for undo files. Default: `~/.local/share/kjxlkj/undo/`. |
| Format | Binary format: tree serialized with node data, timestamps, cursor positions. |
| Security | Undo files MUST be created with `0600` permissions. |

## UI

Undo tree UI (visual tree browser) is specified at: [/docs/spec/features/session/undo_tree.md](/docs/spec/features/session/undo_tree.md)

## Related

- Operators: [/docs/spec/editing/operators/operators.md](/docs/spec/editing/operators/operators.md)
- Insert mode: [/docs/spec/modes/insert/insert.md](/docs/spec/modes/insert/insert.md)
- Session persistence: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)

# Undo / Redo
Undo/redo is deterministic, core-owned history over transactions.

## Requirements
- Each user-visible change commits as a transaction (one undo unit).
- Undo/redo is stable under async feature updates (syntax, diagnostics, git) because those are not edits.
- History is size-bounded and may support optional persistence.

## Default bindings

- `u` undo
- `Ctrl-r` redo

## Granularity rules (normative)

An undo unit typically corresponds to:

- One Insert-mode session
- One Normal-mode command
- One operator+motion/text-object application

## UI

Undo tree UI is specified at: [docs/spec/features/session/undo_tree.md](docs/spec/features/session/undo_tree.md)

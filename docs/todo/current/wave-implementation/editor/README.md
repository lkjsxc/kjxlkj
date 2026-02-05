# Implementation: Editor Core (Iteration 36)

Back: [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)

## Scope

Implement the canonical editor core model:

- buffers (text, metadata, file association)
- windows (viewports, splits/tabs, per-window options)
- cursor and selection anchoring at the core state level
- state mutation and snapshot production

## Entry points (recursive)

| Subarea | Checklist |
|---|---|
| Buffers | [buffers/README.md](buffers/README.md) |
| Windows | [windows/README.md](windows/README.md) |

## Read first (direct, normative)

- Editor core spec:
  - [/docs/spec/editor/README.md](/docs/spec/editor/README.md)
  - [/docs/spec/editor/buffers.md](/docs/spec/editor/buffers.md)
  - [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- UI viewport interaction:
  - [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Runtime model constraints:
  - [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)

## Coverage traversal

- Editor subtree:
  - [/docs/todo/doc-coverage/spec/editor/README.md](/docs/todo/doc-coverage/spec/editor/README.md)

## Acceptance criteria (placeholder)

- Given a single buffer in a single window, when the user applies motion keys, then cursor movement is deterministic and clamps safely at file boundaries.
- Given the cursor moves outside the visible viewport, when a snapshot is produced, then the viewport follow rules keep the cursor visible.
- Given edits are applied, when undo/redo is invoked, then buffer state transitions are deterministic and do not panic.

## Placeholder scaffolding (sub-wave)

- [ ] Define core state entities and their ownership boundaries.
- [ ] Define snapshot structures and update frequency rules.
- [ ] Define window-local viewport state (including wrap and scroll offsets).

## Minimal conformance slice (sub-wave)

- [ ] Implement a single-buffer, single-window editor that satisfies:
  - deterministic cursor movement and clamping
  - deterministic viewport follow rules
  - snapshot → render loop consistency
- [ ] Implement file open/edit/write flows as specified by the command and file specs.

## Full conformance (sub-wave)

- [ ] Implement multi-buffer behavior per spec (buffer listing, navigation, arglist).
- [ ] Implement multi-window behavior per spec (splits, tabs, window commands).
- [ ] Ensure all per-window options that affect rendering are reflected in snapshots.

## Tests (normative outputs)

- [ ] Add tests for:
  - buffer lifecycle and identity invariants
  - cursor/viewport invariants across edits and window changes
  - snapshot stability and correctness

## Conformance and limitations (required updates)

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)


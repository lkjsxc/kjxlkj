# Ex Commands: Session (Iteration 34)

Back: [/docs/todo/current/wave-implementation/commands/README.md](/docs/todo/current/wave-implementation/commands/README.md)

## Scope

Implement session-related Ex commands and persistence behaviors (swap, undo persistence, autosave, recent files).

## Defining documents (direct, normative)

- Session commands index:
  - [/docs/spec/commands/session/README.md](/docs/spec/commands/session/README.md)

## Coverage traversal

- Session commands subtree:
  - [/docs/todo/doc-coverage/spec/commands/session/README.md](/docs/todo/doc-coverage/spec/commands/session/README.md)

## Checklist

### A. Placeholder scaffolding

- [x] Define persistence boundaries and storage locations.
  - SwapFile with path_for() encoding, UndoFile with path_for() encoding
  - Session with name, working_dir, buffers, layout
  - Workspace with name and folders
- [x] Define recovery behavior and user-visible diagnostics.

### B. Minimal conformance slice

- [x] Implement one persistence mechanism end-to-end with tests (as specified).
  - :mksession / :mks — session save
  - :oldfiles / :ol — recent files listing

### C. Full conformance

- [x] Implement all session commands and persistence behaviors in the subtree. — done: `session_persistence.rs` with SessionState (marks/jumps/registers/history/buffer_positions), save/restore via JSON, serialize_session, filter_history

### D. Conformance updates

- [ ] Update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)


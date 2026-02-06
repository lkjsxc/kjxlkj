# Iteration 36 Implementation Proposal

Back: [/docs/log/README.md](/docs/log/README.md)

## Reconstruction Target Decision

**Target:** Current surface per CONFORMANCE.md (not full spec).

**Rationale:** The initial reconstruction focuses on building a complete,
testable foundational implementation that can be iteratively extended. Full
spec coverage (every Vim motion variant, all ex-commands, etc.) is deferred
to later iterations.

## Architecture Decisions

- Single-writer core task: all mutation goes through `EditorState`
- Key parser is stateful (tracks pending multi-key sequences)
- Dispatch is a pure function from `(EditorState, Intent) -> ()`
- Rendering is snapshot-based (reads immutable state)
- Services are async tasks supervised by `ServiceSupervisor`

## Known Limitations Carried Forward

- Undo tree: implemented as linear stack, not full branching tree
- Marks, jumplist, changelist: stubbed, not yet wired
- Search/replace: not yet implemented
- File I/O: read-only (no `:w` implementation yet)
- LSP/git/terminal services: scaffold only
- Macro recording/playback: parsed but not executed

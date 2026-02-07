# Features: Session (Iteration 34)

Back: [/docs/todo/current/wave-implementation/features/README.md](/docs/todo/current/wave-implementation/features/README.md)

## Scope

Implement built-in session and persistence features.

## Defining documents (direct, normative)

- Session features index:
  - [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)

## Coverage traversal

- Session subtree:
  - [/docs/todo/doc-coverage/spec/features/session/README.md](/docs/todo/doc-coverage/spec/features/session/README.md)

## Checklist

- [ ] Placeholder scaffolding: define persistence boundaries and recovery UX.
- [ ] Minimal slice: implement one persistence mechanism end-to-end with tests.
- [ ] Full conformance: implement all session feature documents.
  - [ ] Session persistence (Session, SessionLayout, SessionSplit, SplitDirection)
  - [ ] Auto-save (AutoSaveConfig, AutoSaveState with triggers and debounce)
  - [ ] Macros (Macro, MacroRecorder, KeyStroke, KeyModifiers)
    - MacroStore with store/get/remove/registers/clear
    - KeyStroke (code, ctrl, alt, shift) for serialization
    - Macro, MacroRecord, KeyModifiers types
  - [ ] Workspaces (Workspace, WorkspaceFolder)
  - [ ] Swap/Undo files (SwapFile, UndoFile with path encoding)
  - [ ] Recent files tracking (RecentFiles, RecentFile)
  - [ ] session_full.rs: SessionData, SessionBuffer, SessionWindow, SessionMark, serialize_session, parse_session_buffers
- [ ] Update conformance and limitations docs when user-visible. — done: conformance and limitations entries maintained with each batch


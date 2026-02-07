# Ex Commands: File (Iteration 34)

Back: [/docs/todo/current/wave-implementation/commands/README.md](/docs/todo/current/wave-implementation/commands/README.md)

## Scope

Implement file-related Ex commands and path expansion behaviors.

## Defining documents (direct, normative)

- File commands index:
  - [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

## Coverage traversal

- File subtree:
  - [/docs/todo/doc-coverage/spec/commands/file/README.md](/docs/todo/doc-coverage/spec/commands/file/README.md)

## Checklist

### A. Placeholder scaffolding

- [ ] Define path expansion rules and their security constraints.
- [ ] Define filesystem service interfaces required by file commands.

### B. Minimal conformance slice

- [ ] Implement minimal `:edit` and `:write` behaviors with deterministic tests.
- [ ] Implement safe error handling for invalid paths and permissions.

### C. Full conformance

- [ ] Implement all file commands and behaviors in the subtree.
- [ ] Ensure behavior matches session/swap/undo persistence specs where applicable. — done: `file_io_commands.rs` with FileCommand enum, parse_file_command, validate_write, expand_tilde, buffer_title, display_path; `buffer_lifecycle.rs` with SwapState, ModificationInfo, LifecycleStage, AutoSavePolicy

### D. Conformance updates

- [ ] Update: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)


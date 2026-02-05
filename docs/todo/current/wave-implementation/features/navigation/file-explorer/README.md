# File Explorer MVP (Iteration 34)

Back: [/docs/todo/current/wave-implementation/features/navigation/README.md](/docs/todo/current/wave-implementation/features/navigation/README.md)

## Scope

Implement a minimal built-in file explorer that satisfies the core user intent:

- open and navigate a project tree
- open a file into the editor without leaving the TUI

This is the first, test-gated slice toward the target spec; advanced operations (rename/delete/copy, git badges, filters, tabs/splits) may be deferred but MUST be tracked explicitly.

## Defining documents (direct, normative)

- File explorer target spec:
  - [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- View taxonomy and input/render contracts:
  - [/docs/spec/ui/views.md](/docs/spec/ui/views.md)
  - [/docs/spec/ui/components.md](/docs/spec/ui/components.md)
- Runtime/service model (FS service ownership, cancellation, ordering):
  - [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)

## MVP definition (explicit)

The MVP MUST include:

- a dedicated Explorer view that can be toggled (keybinding per spec or temporary conformance binding recorded)
- directory listing of a root path (workspace root or current file directory, as specified)
- keyboard navigation within the tree (`j`/`k`, expand/collapse, open)
- open file in the current editor window
- deterministic behavior under large directories (no freeze)

The MVP MAY defer (but must track as TODO leaves):

- create/rename/delete operations
- git status and diagnostics badges
- filters and search
- open in splits/tabs

## Acceptance criteria (Given/When/Then)

1. Given a workspace root with nested directories, when toggling the explorer, then the tree MUST render and the focused row MUST be visible.
2. Given a directory with 10,000 children, when expanding it, then input MUST remain responsive and the UI MUST not freeze (work must be incremental/cancellable if needed).
3. Given a file selected in the explorer, when the user activates open, then the editor MUST load the file in the current window and focus MUST return to the editor view.

## Test strategy (required)

### Unit tests (required)

- [ ] Explorer state transitions (expand/collapse, selection movement).
- [ ] Tree rendering to a list of display rows given a known filesystem snapshot.

### Integration tests (required)

- [ ] FS service directory listing (cancellation, ordering, error handling).

### Golden UI tests (recommended)

- [ ] Snapshot-to-frame tests for explorer view with a deterministic fake filesystem.

## Checklist

- [ ] Define the explorer core state model (tree nodes, expansion, selection).
- [ ] Define the service interface for directory listing (incremental + cancellable for large dirs).
- [ ] Implement explorer view rendering and input handling with deterministic tests.
- [ ] Implement open-file intent wiring into the editor core.
- [ ] Add at least one regression test for "10k children expand does not freeze".
- [ ] Update conformance and limitations docs:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)


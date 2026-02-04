# Proposal: File Explorer MVP (Built-in)

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Problem statement

The target spec includes a built-in file explorer, but the conformance/limitations ledger indicates it is not implemented yet.

This blocks basic “project navigation” workflows and forces users to rely on external shell navigation.

## Defining documents

- TODO leaf:
  - [/docs/todo/current/wave-implementation/features/navigation/file-explorer/README.md](/docs/todo/current/wave-implementation/features/navigation/file-explorer/README.md)
- Target spec:
  - [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- UI contracts:
  - [/docs/spec/ui/views.md](/docs/spec/ui/views.md)
  - [/docs/spec/ui/components.md](/docs/spec/ui/components.md)
- Runtime/service model:
  - [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)

## Proposed approach

1. Implement an Explorer view as a first-class view in the UI model.
2. Use the FS service for directory listing, with cancellation/ordering guarantees (large directories must not freeze input).
3. Wire “open file” as a core intent so the editor core remains the single writer of state.

## Test plan (required)

- Unit tests for explorer tree state (expand/collapse, selection).
- Integration tests for FS listing (ordering, cancellation, errors).
- Golden UI tests rendering a deterministic explorer snapshot into a frame.
- A regression test for the 10k-children scenario (no freeze).

## Risks / open questions

- Workspace root definition must be explicit (current directory vs current file directory vs configured root).
- Git status/diagnostics badges require additional services; defer behind explicit TODOs if not in MVP.

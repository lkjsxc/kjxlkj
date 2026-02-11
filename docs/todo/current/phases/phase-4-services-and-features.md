# Phase 4: Services and Feature Completion

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Service-backed features and cross-surface integration after core blockers are closed.

## Tasks

- [x] revalidate LSP, Git, index, and FS service behavior against updated contracts
- [x] ensure service features remain stable with new window and input fixes
- [x] add cross-surface E2E where service output affects editor/explorer/terminal views
- [x] close any new `M2` or `M4` rows found during blocker closure

## Required Spec Links

- [x] [/docs/spec/features/lsp/README.md](/docs/spec/features/lsp/README.md)
- [x] [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)
- [x] [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
- [x] [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)

## Required Tests

- [x] existing domain integration suites remain green
- [x] at least one live E2E workflow per service family
- [x] no service regression under `BD-RACE-01` conditions

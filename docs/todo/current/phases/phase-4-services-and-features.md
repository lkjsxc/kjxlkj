# Phase 4: Services and Feature Completion

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Service-backed features and cross-surface integration after core blocker fixes.

## Tasks

- [ ] revalidate LSP, Git, index, and FS service behavior against updated contracts
- [ ] ensure service output remains stable with new window/input fixes
- [ ] add cross-surface E2E where service output affects visible windows
- [ ] close newly discovered `M2` or `M4` rows before phase closure

## Required Spec Links

- [ ] [/docs/spec/features/lsp/README.md](/docs/spec/features/lsp/README.md)
- [ ] [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)
- [ ] [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
- [ ] [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)

## Required Tests

- [ ] service integration suites remain green
- [ ] at least one live E2E workflow per service family
- [ ] `BD-RACE-01` remains green after service integration

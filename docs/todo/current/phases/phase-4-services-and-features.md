# Phase 4: Services and Feature Completion

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Complete service-backed feature surface beyond core editing.

## Tasks

- [x] implement LSP service with completion and diagnostics
- [x] implement Git service with status and diff surfaces
- [x] implement indexing/finder service with deterministic queries
- [x] implement syntax/highlighting path per spec
- [x] close scaffold-only statuses in conformance for core feature domains

## Required Spec Links

- [x] [/docs/spec/features/lsp/README.md](/docs/spec/features/lsp/README.md)
- [x] [/docs/spec/features/git/README.md](/docs/spec/features/git/README.md)
- [x] [/docs/spec/features/navigation/finder.md](/docs/spec/features/navigation/finder.md)
- [x] [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)

## Required Tests

- [x] domain integration tests for each implemented service path
- [x] at least one E2E workflow per service feature family
- [x] regression tests for every limitation closed in this phase

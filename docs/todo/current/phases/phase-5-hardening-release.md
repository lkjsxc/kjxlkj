# Phase 5: Hardening and Release Readiness

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Stability, stress, topology compliance, and release gate closure.

## Tasks

- [x] run full boundary and PTY stress matrix
- [x] verify source topology constraints (around 12 children, file length <= 200)
- [x] verify no unresolved high-severity limitations remain
- [x] verify CI and release checklist requirements
- [x] finalize conformance summary with dated evidence

## Required Spec Links

- [x] [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [x] [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [x] [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [x] [/docs/reference/CI.md](/docs/reference/CI.md)
- [x] [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)

## Required Tests

- [x] `BD-03` through `BD-10`
- [x] `PE-01` through `PE-06`
- [x] release-profile full verification gate

# Phase 5: Hardening and Release Readiness

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Stability, stress, topology compliance, and release gate closure.

## Tasks

- [x] run full retained + `*R` live E2E matrix
- [x] verify source topology constraints (around 12 children, file length <= 200)
- [x] verify no high-severity limitations remain open
- [x] verify CI and release checklist requirements
- [x] finalize conformance summary with dated evidence

## Required Spec Links

- [x] [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [x] [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [x] [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [x] [/docs/reference/CI.md](/docs/reference/CI.md)
- [x] [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)

## Required Tests

- [x] all retained baseline tests from testing contract
- [x] all `*R` tests from [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [x] race/stress suite `BD-RACE-01` through `BD-RACE-04`

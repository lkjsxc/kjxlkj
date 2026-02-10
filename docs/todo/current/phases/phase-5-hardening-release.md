# Phase 5: Hardening and Release Readiness

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Stability, stress, topology compliance, and release gate closure.

## Tasks

- [ ] run full retained + `*R` live E2E matrix
- [ ] verify source topology constraints (around 12 children, file length <= 200)
- [ ] verify no high-severity limitations remain open
- [ ] verify CI and release checklist requirements
- [ ] finalize conformance summary with dated evidence

## Required Spec Links

- [ ] [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [ ] [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [ ] [/docs/reference/CI.md](/docs/reference/CI.md)
- [ ] [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)

## Required Tests

- [ ] all retained baseline tests from testing contract
- [ ] all `*R` tests from [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [ ] race/stress suite `BD-RACE-01` through `BD-RACE-04`

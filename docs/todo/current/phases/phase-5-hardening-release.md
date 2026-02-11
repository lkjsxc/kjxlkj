# Phase 5: Hardening and Release Readiness

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Stress, topology compliance, and release gate closure.

## Tasks

- [ ] run retained baseline plus full `*R` E2E matrix
- [ ] verify source topology constraints (around 12 children, files <=200 lines)
- [ ] verify no high-severity limitations remain open
- [ ] satisfy CI and release checklist requirements
- [ ] finalize conformance summary with dated evidence

## Required Spec Links

- [ ] [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [ ] [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [ ] [/docs/reference/CI.md](/docs/reference/CI.md)
- [ ] [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)

## Required Tests

- [ ] retained baseline tests from testing contract
- [ ] all mandatory `*R` tests
- [ ] race/stress suite `BD-RACE-01`..`BD-RACE-04`

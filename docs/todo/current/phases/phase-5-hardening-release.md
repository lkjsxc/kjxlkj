# Phase 5: Hardening and Release Readiness

Back: [/docs/todo/current/phases/README.md](/docs/todo/current/phases/README.md)

## Scope

Stability, stress, topology compliance, and release gate closure.

## Tasks

- [ ] run full boundary and PTY stress matrix
- [ ] verify source topology constraints (around 12 children, file length <= 200)
- [ ] verify no unresolved high-severity limitations remain
- [ ] verify CI and release checklist requirements
- [ ] finalize conformance summary with dated evidence

## Required Spec Links

- [ ] [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [ ] [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [ ] [/docs/reference/CI.md](/docs/reference/CI.md)
- [ ] [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)

## Required Tests

- [ ] `BD-03` through `BD-10`
- [ ] `PE-01` through `PE-06`
- [ ] release-profile full verification gate

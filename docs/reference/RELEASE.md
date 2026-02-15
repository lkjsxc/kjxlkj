# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)

Release is valid only for blocker-free reconstructed runtime state.

## Preconditions

1. `Release` CI profile is green.
2. all high-severity limitation rows are closed.
3. conformance claims are evidence-backed and synchronized.
4. drift matrix has no open high-severity `M1` or `M2` rows.
5. acceptance suites in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) pass.
6. typed gates in [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) pass.

## Current Gate (2026-02-15)

Release gate is partially satisfied.

Completed:

- canonical docs structure is synchronized
- TODO checklists are reset with direct doc links per step
- improvement backlog is captured in canonical docs
- final completion file map is explicit
- runtime source artifacts rebuilt from TODO wave program (Stages 00–08)
- HTTP and WebSocket runtime paths rebuilt
- frontend runtime rebuilt including librarian review UX and automation API
- CSRF client-side enforcement implemented
- Docker single-container deployment created (Dockerfile, docker-compose.yml, entrypoint.sh)
- CI workflow created with 4 profile jobs (docs-integrity, workspace-bootstrap, core-runtime, release-gate)
- type-safety gates pass: cargo check clean, tsc --noEmit clean, no handwritten JS
- final file structure matches spec
- 54 tests passing (8 domain + 31 acceptance stubs + 14 regression stubs + 1 WS automation)
- all source files ≤ 200 lines
- 0 M1 correctness rows, 0 M2 missing feature rows in drift matrix

Remaining for release:

- execute CI workflow in GitHub Actions (local profiles pass)
- execute acceptance tests with live database (acceptance stubs are structural)
- execute performance profiles (PERF-01/02/03) at target scale
- execute operations drills (backup restore, restart recovery)
- rebuild and run browser E2E tests for small-screen menu and Create New Note
- verify Docker container builds and runs end-to-end
- close 1 high-severity limitation (LIM-TEST-01: acceptance evidence absent)

## Release Steps

1. reconstruct runtime from canonical docs and `docs/todo/` waves
2. run required profiles and archive deterministic evidence
3. verify no contradictions remain between runtime and docs
4. synchronize ledgers and TODO closure
5. create release commit/tag

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- CI profiles: [/docs/reference/CI.md](/docs/reference/CI.md)

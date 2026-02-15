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

Release gate is blocked.

Completed:

- canonical docs structure is synchronized
- TODO checklists are reset with direct doc links per step
- improvement backlog is captured in canonical docs
- final completion file map is explicit

Remaining for release:

- rebuild runtime source artifacts from TODO wave program
- rebuild HTTP and WebSocket runtime paths
- rebuild frontend runtime including small-screen top-right menu close-on-select behavior
- rebuild and run regression test for Create New Note add/select flow
- execute required CI, acceptance, perf, and operations evidence suites

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

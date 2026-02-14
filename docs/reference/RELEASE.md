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

## Current Gate (2025-01-20)

Release gate is partially satisfied.

Completed:

- runtime reconstruction stages 00–08 complete
- Rust workspace compiles with zero warnings (cargo check --workspace)
- TypeScript strict mode passes (tsc --noEmit)
- Vite production build succeeds
- CI workflow defined (.github/workflows/ci.yml)
- no high-severity M1 or M2 drift rows remain
- all TODO waves marked complete (S00–S08)

Remaining for full release:

- integration test harness with ephemeral PostgreSQL (LIM-TEST-01)
- performance benchmark execution at target scale (LIM-PERF-01)
- backup restore drill execution (LIM-OPS-RESTORE-01)
- cross-actor WS broadcast addr registry (LIM-WS-BROADCAST-01, low severity)

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

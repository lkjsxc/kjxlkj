# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)

Release is valid only for blocker-free reconstructed runtime state.

## Preconditions

1. `Release` CI profile is green.
2. no open high-severity limitations.
3. drift matrix has no open `M1` or `M2` rows.
4. acceptance tests in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) pass.
5. type-safety gates in [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) pass.

## Current Gate (2026-02-16)

Release is partially unblocked.

Resolved:

- runtime source reconstructed (`src/`, `Cargo.toml`, `Cargo.lock`, `scripts/`)
- TODO wave checklists are marked complete
- `cargo build --workspace` and `cargo test --workspace` pass
- frontend strict type-check and production build pass

Remaining blockers:

- acceptance ID suite in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) is not fully automated/executed
- DB-backed integration profile is not yet reproducible in-repo
- browser E2E suite for required `E2E-*` IDs is not yet implemented

## Release Steps

1. execute TODO waves in order and satisfy each wave build/test gate
2. run CI profiles and archive evidence
3. close drift and limitation rows
4. synchronize ledgers and TODO completion
5. tag release

## Related

- Conformance: [CONFORMANCE.md](CONFORMANCE.md)
- Limitations: [LIMITATIONS.md](LIMITATIONS.md)
- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)

# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)

Release is valid only for blocker-free reconstructed runtime state.

## Preconditions

1. `Release` CI profile is green.
2. no open high-severity limitations.
3. drift matrix has no open `M1` or `M2` rows.
4. acceptance tests in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) pass.
5. type-safety gates in [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) pass.

## Current Gate (2026-02-15)

Release is partially unblocked.

Resolved:

- runtime source fully reconstructed (10 crates, clean compilation, 16 tests pass)
- hybrid search implemented (kjxlkj-search)
- editor redesign implemented (app-shell.ts)
- `kjxlkj-agent` runtime loop implemented (kjxlkj-automation)

Remaining blockers:

- CI workflow file not created (`.github/workflows/`)
- DB-backed integration test suite not yet run against live PostgreSQL
- browser E2E test suite not implemented

## Release Steps

1. execute TODO waves to rebuild runtime source
2. run CI profiles and archive evidence
3. close drift and limitation rows
4. synchronize ledgers and TODO completion
5. tag release

## Related

- Conformance: [CONFORMANCE.md](CONFORMANCE.md)
- Limitations: [LIMITATIONS.md](LIMITATIONS.md)
- Drift matrix: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)

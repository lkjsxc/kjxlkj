# Legacy Runtime Snapshot (Pre-Reset)

Back: [/docs/reference/README.md](/docs/reference/README.md)

Captured before docs-only reset execution on 2026-02-26.

## Scope

This snapshot records the last observed runtime footprint before deletion of reconstructable artifacts.

## Observed Runtime Paths

- `src/` (Rust crates + frontend app)
- `migrations/` (`001`..`008`)
- `Cargo.toml`, `Cargo.lock`
- `Dockerfile`, `docker-compose.yml`, `.dockerignore`
- `.github/`, `scripts/`, `target/`

## High-Risk Gaps Observed

1. communication-layer closure incomplete in HTTP/WS handlers
2. frontend communication/state architecture shallow vs spec requirements
3. frontend test coverage absent; acceptance/supplemental suites not evidenced
4. reference ledgers and root narrative were previously out of sync

## Recovery Rule

All runtime behavior from this snapshot is non-authoritative after reset. Reconstruction must proceed only via:

1. `/docs/spec`
2. `/docs/todo`
3. `/docs/reference` closure ledgers

## Related

- [CONFORMANCE.md](CONFORMANCE.md)
- [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- [TODO_TRACE_MATRIX.md](TODO_TRACE_MATRIX.md)
- [TEST_MATRIX.md](TEST_MATRIX.md)

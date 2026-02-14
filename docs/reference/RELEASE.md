# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)

Release is valid only for a blocker-free reconstructed state.

## Preconditions

1. `Release` CI profile is green.
2. all high-severity limitation rows are closed.
3. conformance claims are evidence-backed and synchronized.
4. drift matrix has no open high-severity `M1` or `M2` rows.
5. acceptance suites in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) pass.
6. `Librarian-small-model` CI profile is green when librarian feature is in scope.

## Current Gate (2026-02-13)

Release gate is green.

Evidence summary:

- `Release` profile command pass: `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --tests -- --nocapture`
- Librarian profile matrix and rerun stability pass (Wave 090 audit)
- `PERF-01`/`PERF-02`/`PERF-03` and `OPS-01`/`OPS-02` evidence archived (Wave 091 audit)
- final release-ledger sync and closure evidence archived (Wave 092 audit)
- limitations ledger has no open `high` severity rows

## Release Steps

1. reconstruct implementation from canonical docs
2. run `Release` profile and archive deterministic evidence
3. verify no contradictions remain between runtime and docs
4. create release commit and tag
5. publish artifacts
6. synchronize release evidence in reference ledgers

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)

Release is valid only for a blocker-free rebuilt runtime.

## Preconditions

1. `Release` CI profile is green.
2. all high-severity limitation rows are closed.
3. conformance claims are evidence-backed and synchronized.
4. drift matrix has no open high-severity `M1` or `M2` rows.
5. acceptance suites in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) pass.
6. single-container Docker Compose startup passes (`docker compose up --build` + `/api/readyz`).

## Current Gate (2026-02-13)

Release gate is blocked.

Blocking reasons:

- docs-only reset intentionally removed runtime/build artifacts
- no executable runtime/test harness exists in current baseline
- limitations include open high-severity `M2` and `M4` rows

## Release Steps

1. reconstruct implementation from canonical docs
2. implement `Dockerfile` + `docker-compose.yml` single-container startup path
3. run `Release` profile and archive deterministic evidence
4. verify no contradictions remain between runtime and docs
5. create release commit and tag
6. publish artifacts
7. synchronize release evidence in reference ledgers

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

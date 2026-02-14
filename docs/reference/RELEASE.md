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

## Current Gate (2026-02-14)

Release gate is not satisfied.

Blocking reasons:

- runtime source tree is intentionally absent after reset
- high-severity `M2` rows are open in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- high-severity limitations are open in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- acceptance and typed gates cannot currently pass

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

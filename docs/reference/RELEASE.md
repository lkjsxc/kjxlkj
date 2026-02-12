# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)

Release is valid only for a blocker-free reconstructed state.

## Preconditions

1. `Release` CI profile is green.
2. all high-severity limitation rows are closed.
3. conformance claims are evidence-backed and synchronized.
4. drift matrix has no open high-severity `M1` or `M2` rows.
5. acceptance suites in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) pass.

## Current Gate (2026-02-12)

Release is blocked.

Reasons:

- runtime implementation artifacts are not yet reconstructed
- high-severity `M2` rows remain open in limitations

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

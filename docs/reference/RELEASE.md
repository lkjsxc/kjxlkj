# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)

Release is valid only for a blocker-free reconstructed state.

## Preconditions

1. `Release` CI profile is green
2. all high-severity limitation rows are closed
3. conformance claims are evidence-backed and synchronized
4. drift matrix has no open `M1 correctness` high-severity rows
5. mandatory blocker `*R` tests pass with PTY frame assertions

## Current Gate (2026-02-12)

Release is blocked.

Reasons:

- known high-severity blocker rows remain open in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- fresh `T2` evidence for blocker closure is not complete

## Release Steps

1. reconstruct implementation from canonical docs
2. run `Release` profile and archive deterministic evidence
3. verify no contradictions remain between user behavior and test evidence
4. create release commit and tag
5. publish artifacts
6. synchronize release evidence in reference ledgers

## Post-Release

- open next TODO wave set
- update conformance and limitations snapshot
- remove promoted temporary log notes

# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)

Release is valid only for a reconstructed and blocker-free state.

## Preconditions

1. release CI profile is green
2. all high-severity limitations are closed
3. `CONFORMANCE` claims are evidence-backed and synchronized
4. `DRIFT_MATRIX` has no open `M1` high-severity rows
5. blocker `*R` cases pass with screen-state E2E assertions

## Current Gate (2026-02-11)

Release is blocked by open rows in
[/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md).

## Release Steps

1. freeze docs and implementation together
2. run release profile and capture deterministic evidence
3. confirm no contradiction remains between user reports and test evidence
4. create release commit and tag
5. publish artifacts
6. record release evidence links in reference ledgers

## Post-Release

- open next TODO wave
- update conformance and limitations for new baseline
- delete promoted temporary log notes

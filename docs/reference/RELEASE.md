# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)

Release is valid only for a reconstructed and blocker-free state.

## Preconditions

1. release CI profile is green
2. all high-severity limitations are closed
3. `CONFORMANCE` claims are evidence-backed and synchronized
4. `DRIFT_MATRIX` has no open `M1` high-severity rows

## Release Steps

1. freeze docs and implementation together
2. run release verification profile and capture evidence
3. create release commit and tag
4. publish artifacts
5. record release evidence links in reference ledger

## Post-Release

- reopen TODO plan for next wave
- update conformance and limitations for new baseline
- keep logs clean by deleting promoted temporary notes

# Wave 091: Conformance, Limitations, and Drift Closure

Back: [/docs/todo/waves/stage-09-ci-performance-release/README.md](/docs/todo/waves/stage-09-ci-performance-release/README.md)

## Relevant Documents

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/todo/README.md](/docs/todo/README.md)

## Restructure Steps

- [ ] restructure-step S09-W091-01: promote verified domains in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) using evidence from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/reference/CONFORMANCE.md)
- [ ] restructure-step S09-W091-02: close resolved rows and leave explicit open gaps in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) [doc-link](/docs/reference/LIMITATIONS.md)
- [ ] restructure-step S09-W091-03: close or reclassify mismatch rows in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/DRIFT_MATRIX.md)
- [ ] restructure-step S09-W091-04: ensure stage-evidence mapping remains current in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) [doc-link](/docs/reference/EVIDENCE_INDEX.md)
- [ ] restructure-step S09-W091-05: ensure TODO closure state in [/docs/todo/README.md](/docs/todo/README.md) matches ledger closure state [doc-link](/docs/todo/README.md)

## Verification Hooks

- [ ] restructure-step S09-W091-V01: run ledger consistency checks against acceptance outputs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [ ] restructure-step S09-W091-V02: verify no unresolved high-severity contradictions remain in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/DRIFT_MATRIX.md)

## Mandatory Build and Test Gate

- [ ] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [ ] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [ ] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

# Wave 092: Release Gate and Final Structure Validation

Back: [/docs/todo/waves/stage-09-ci-performance-release/README.md](/docs/todo/waves/stage-09-ci-performance-release/README.md)

## Relevant Documents

- [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
- [/docs/reference/CI.md](/docs/reference/CI.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md)
- [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Restructure Steps

- [ ] restructure-step S09-W092-01: satisfy release preconditions in [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) [doc-link](/docs/reference/RELEASE.md)
- [ ] restructure-step S09-W092-02: ensure required CI gates are green per [/docs/reference/CI.md](/docs/reference/CI.md) [doc-link](/docs/reference/CI.md)
- [ ] restructure-step S09-W092-03: ensure final repository tree matches [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) [doc-link](/docs/spec/architecture/final-file-structure.md)
- [ ] restructure-step S09-W092-04: ensure type-safety contract remains satisfied per [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) [doc-link](/docs/spec/technical/type-safety.md)
- [ ] restructure-step S09-W092-05: ensure all stage and wave checklists are complete in [/docs/todo/waves/README.md](/docs/todo/waves/README.md) [doc-link](/docs/todo/waves/README.md)

## Verification Hooks

- [ ] restructure-step S09-W092-V01: run final release profile and acceptance checks from [/docs/reference/CI.md](/docs/reference/CI.md) [doc-link](/docs/reference/CI.md)
- [ ] restructure-step S09-W092-V02: mark release closure state in [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) [doc-link](/docs/reference/RELEASE.md)

## Mandatory Build and Test Gate

- [ ] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [ ] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [ ] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

# Wave 090: CI Profiles and Type-Safety Closure

Back: [/docs/todo/waves/stage-09-ci-performance-release/README.md](/docs/todo/waves/stage-09-ci-performance-release/README.md)

## Relevant Documents

- [/docs/reference/CI.md](/docs/reference/CI.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md)
- [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
- [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

## Restructure Steps

- [x] restructure-step S09-W090-01: execute required CI profiles from [/docs/reference/CI.md](/docs/reference/CI.md) [doc-link](/docs/reference/CI.md)
- [x] restructure-step S09-W090-02: execute acceptance suites from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step S09-W090-03: execute Rust/TypeScript type gates from [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) [doc-link](/docs/spec/technical/type-safety.md)
- [x] restructure-step S09-W090-04: execute performance and operations profiles from [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) and [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) [doc-link](/docs/spec/technical/performance.md)
- [x] restructure-step S09-W090-05: map profile evidence to [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) [doc-link](/docs/reference/EVIDENCE_INDEX.md)

## Verification Hooks

- [x] restructure-step S09-W090-V01: repeat profile runs to detect nondeterminism using [/docs/reference/CI.md](/docs/reference/CI.md) [doc-link](/docs/reference/CI.md)
- [x] restructure-step S09-W090-V02: sync profile outcomes in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) [doc-link](/docs/reference/CONFORMANCE.md)

## Mandatory Build and Test Gate

- [x] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [x] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [x] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

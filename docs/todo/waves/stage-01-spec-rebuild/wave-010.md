# Wave 010: Runtime Topology and Typed Workspace Skeleton

Back: [/docs/todo/waves/stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md)

## Relevant Documents

- [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
- [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)

## Restructure Steps

- [ ] restructure-step S01-W010-01: scaffold crate topology required by [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md) [doc-link](/docs/spec/architecture/crates.md)
- [ ] restructure-step S01-W010-02: wire runtime supervision boundaries from [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) [doc-link](/docs/spec/architecture/runtime.md)
- [ ] restructure-step S01-W010-03: enforce workspace layout from [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) [doc-link](/docs/spec/architecture/source-layout.md)
- [ ] restructure-step S01-W010-04: enforce workspace manifest policy from [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md) [doc-link](/docs/spec/architecture/workspace-manifest.md)
- [ ] restructure-step S01-W010-05: enforce typed-language constraints from [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) [doc-link](/docs/spec/technical/type-safety.md)

## Verification Hooks

- [ ] restructure-step S01-W010-V01: run architecture/type checks required by [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [ ] restructure-step S01-W010-V02: verify deployability path remains compatible with [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) [doc-link](/docs/spec/architecture/deployment.md)

## Mandatory Build and Test Gate

- [ ] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [ ] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [ ] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

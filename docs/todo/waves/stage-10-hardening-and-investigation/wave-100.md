# Wave 100: Architecture and Operations Hardening

Back: [/docs/todo/waves/stage-10-hardening-and-investigation/README.md](/docs/todo/waves/stage-10-hardening-and-investigation/README.md)

## Relevant Documents

- [/docs/reference/IMPROVEMENT_BACKLOG.md](/docs/reference/IMPROVEMENT_BACKLOG.md)
- [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md)
- [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md)
- [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)

## Restructure Steps

- [x] restructure-step S10-W100-01: tune PostgreSQL pool sizing and timeout policy from [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) [doc-link](/docs/spec/technical/performance.md)
- [x] restructure-step S10-W100-02: enforce SQLx compile-time query checking strategy from [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md) [doc-link](/docs/spec/technical/migrations.md)
- [x] restructure-step S10-W100-03: implement cross-actor WebSocket broadcast registry semantics from [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) [doc-link](/docs/spec/api/websocket.md)
- [x] restructure-step S10-W100-04: automate backup/restore drill workflow from [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) [doc-link](/docs/spec/technical/operations.md)
- [x] restructure-step S10-W100-05: synchronize hardening outcomes in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) and [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/LIMITATIONS.md)

## Verification Hooks

- [x] restructure-step S10-W100-V01: run architecture and operations acceptance checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step S10-W100-V02: record evidence in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) [doc-link](/docs/reference/EVIDENCE_INDEX.md)

## Mandatory Build and Test Gate

- [x] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [x] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [x] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

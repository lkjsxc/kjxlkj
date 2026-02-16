# Wave 052: Performance and Recovery Baseline

Back: [/docs/todo/waves/stage-05-auth-and-security/README.md](/docs/todo/waves/stage-05-auth-and-security/README.md)

## Relevant Documents

- [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md)
- [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
- [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
- [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [/docs/spec/domain/export.md](/docs/spec/domain/export.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Restructure Steps

- [ ] restructure-step S05-W052-01: satisfy latency and throughput targets from [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) [doc-link](/docs/spec/technical/performance.md)
- [ ] restructure-step S05-W052-02: satisfy WS ordering/replay soak expectations from [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) [doc-link](/docs/spec/api/websocket.md)
- [ ] restructure-step S05-W052-03: satisfy backup/export/restart recovery expectations from [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) [doc-link](/docs/spec/technical/operations.md)
- [ ] restructure-step S05-W052-04: verify deployment health and shutdown behavior from [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) [doc-link](/docs/spec/architecture/deployment.md)
- [ ] restructure-step S05-W052-05: align ops job constraints with [/docs/spec/domain/export.md](/docs/spec/domain/export.md) [doc-link](/docs/spec/domain/export.md)

## Verification Hooks

- [ ] restructure-step S05-W052-V01: run `PERF-*` and `OPS-02` checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [ ] restructure-step S05-W052-V02: sync perf/ops status in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) [doc-link](/docs/reference/LIMITATIONS.md)

## Mandatory Build and Test Gate

- [ ] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [ ] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [ ] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

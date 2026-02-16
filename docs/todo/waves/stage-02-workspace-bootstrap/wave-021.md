# Wave 021: WebSocket Patch, Replay, and Idempotency

Back: [/docs/todo/waves/stage-02-workspace-bootstrap/README.md](/docs/todo/waves/stage-02-workspace-bootstrap/README.md)

## Relevant Documents

- [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- [/docs/spec/api/types.md](/docs/spec/api/types.md)
- [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Restructure Steps

- [x] restructure-step S02-W021-01: implement note/workspace subscribe flows from [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) [doc-link](/docs/spec/api/websocket.md)
- [x] restructure-step S02-W021-02: implement `apply_patch` semantics with version checks from [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) [doc-link](/docs/spec/ui/editor-flow.md)
- [x] restructure-step S02-W021-03: enforce idempotency replay semantics from [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) [doc-link](/docs/spec/api/websocket.md)
- [x] restructure-step S02-W021-04: enforce ack cursor replay and stale-cursor failures from [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) [doc-link](/docs/spec/api/websocket.md)
- [x] restructure-step S02-W021-05: enforce deterministic conflict/error codes from [/docs/spec/api/errors.md](/docs/spec/api/errors.md) [doc-link](/docs/spec/api/errors.md)

## Verification Hooks

- [x] restructure-step S02-W021-V01: run `WS-01..WS-05` checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step S02-W021-V02: update replay/idempotency status in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/DRIFT_MATRIX.md)

## Mandatory Build and Test Gate

- [x] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [x] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [x] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

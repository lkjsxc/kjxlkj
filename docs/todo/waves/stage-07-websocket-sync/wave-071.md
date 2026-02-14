# Wave 071: Ack Cursor, Replay, and Stale Cursor Handling

Back: [/docs/todo/waves/stage-07-websocket-sync/README.md](/docs/todo/waves/stage-07-websocket-sync/README.md)

## Relevant Documents

- [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

## Restructure Steps

- [ ] restructure-step S07-W071-01: enforce monotonic per-stream replay cursors from [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] restructure-step S07-W071-02: enforce reconnect replay-before-new-submit behavior from [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [ ] restructure-step S07-W071-03: enforce deterministic stale-cursor error payload from [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] restructure-step S07-W071-04: enforce duplicate idempotency replay behavior from [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] restructure-step S07-W071-05: align error envelope fields with [/docs/spec/api/errors.md](/docs/spec/api/errors.md)

## Verification Hooks

- [ ] restructure-step S07-W071-V01: run reconnect/replay/idempotency boundary checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] restructure-step S07-W071-V02: sync replay-closure status in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

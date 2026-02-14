# Wave 071: Replay Cursor and Idempotency for Librarian Events

Back: [/docs/todo/waves/stage-07-websocket-sync/README.md](/docs/todo/waves/stage-07-websocket-sync/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] integrate librarian events into ack-cursor replay protocol -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] enforce idempotent retransmit semantics for event emissions -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] reject stale cursors with deterministic error payloads -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)

## Verification Tasks

- [ ] run reconnect + replay boundary checks -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] run duplicate cursor and retransmit scenarios -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)

## Evidence Placeholder

- [ ] `Check:` `cargo test -p kjxlkj-server tests_ws_replay -- --nocapture` -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] `Result:` pass -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] `Proof:` `ws_flow`: reconnect replay cursor asserts for `note:{id}` and `workspace:{id}` passed; idempotent retransmit preserved commit identity; stale `ack` emitted deterministic `error` with `code=STALE_CURSOR` -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)

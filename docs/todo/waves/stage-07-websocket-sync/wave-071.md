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

- [ ] integrate librarian events into ack-cursor replay protocol
- [ ] enforce idempotent retransmit semantics for event emissions
- [ ] reject stale cursors with deterministic error payloads

## Verification Tasks

- [ ] run reconnect + replay boundary checks
- [ ] run duplicate cursor and retransmit scenarios

## Evidence Placeholder

- [ ] `Check:` `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test ws_flow ws_subscribe_patch_replay_and_conflict_flow -- --nocapture`
- [ ] `Result:` pass
- [ ] `Proof:` `ws_flow`: reconnect replay cursor asserts for `note:{id}` and `workspace:{id}` passed; idempotent retransmit preserved commit identity; stale `ack` emitted deterministic `error` with `code=STALE_CURSOR`

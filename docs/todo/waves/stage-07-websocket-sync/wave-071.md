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

- [ ] `Check:`
- [ ] `Result:`
- [ ] `Proof:`

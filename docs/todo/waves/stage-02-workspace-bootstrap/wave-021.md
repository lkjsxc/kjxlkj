# Wave 021: Realtime Patch and Replay Protocol

Back: [/docs/todo/waves/stage-02-workspace-bootstrap/README.md](/docs/todo/waves/stage-02-workspace-bootstrap/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] implement `subscribe_note` and `subscribe_workspace`
- [ ] implement `apply_patch` with replay-safe idempotency semantics
- [ ] implement ack-cursor replay and deterministic conflict rejection

## Verification Tasks

- [ ] run `WS-01..05`
- [ ] run reconnect and duplicate retransmit boundary cases

## Evidence Placeholder

- [ ] `Check: websocket subscribe/patch/replay/idempotency/conflict integration coverage`
- [ ] `Result: pass`
- [ ] `Proof: [/docs/log/audits/2026-02-13-stage-02-collaborative-notes-core.md](/docs/log/audits/2026-02-13-stage-02-collaborative-notes-core.md)`

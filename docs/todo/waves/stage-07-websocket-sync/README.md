# Stage 07: WebSocket Replay and Automation Events

Back: [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Relevant Documents

- [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

## Stage Objective

Close realtime event ordering/replay guarantees, including librarian automation
events and reconnect cursor determinism.

## Ordered Wave Checklist

- [ ] restructure-step S07-W070: complete workspace and librarian event surfaces in [wave-070.md](wave-070.md)
- [ ] restructure-step S07-W071: complete ack/replay/stale-cursor behavior in [wave-071.md](wave-071.md)
- [ ] restructure-step S07-W072: complete end-to-end WS acceptance closure in [wave-072.md](wave-072.md)

## Stage Exit Checklist

- [ ] restructure-step S07-EXIT-01: event types and order align with [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] restructure-step S07-EXIT-02: reconnect/idempotency behavior aligns with [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [ ] restructure-step S07-EXIT-03: WS acceptance evidence aligns with [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

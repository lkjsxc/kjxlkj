# Wave 070: Workspace and Librarian Event Surfaces

Back: [/docs/todo/waves/stage-07-websocket-sync/README.md](/docs/todo/waves/stage-07-websocket-sync/README.md)

## Relevant Documents

- [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [/docs/spec/domain/events.md](/docs/spec/domain/events.md)
- [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [/docs/spec/api/types.md](/docs/spec/api/types.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Restructure Steps

- [ ] restructure-step S07-W070-01: implement workspace stream event families from [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) [doc-link](/docs/spec/api/websocket.md)
- [ ] restructure-step S07-W070-02: emit librarian run lifecycle events from [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) [doc-link](/docs/spec/domain/automation.md)
- [ ] restructure-step S07-W070-03: emit operation review/apply/reject events compatible with [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) [doc-link](/docs/spec/ui/workspace-suite.md)
- [ ] restructure-step S07-W070-04: enforce event payload typing from [/docs/spec/api/types.md](/docs/spec/api/types.md) [doc-link](/docs/spec/api/types.md)
- [ ] restructure-step S07-W070-05: enforce unknown-event tolerance and continuity from [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) [doc-link](/docs/spec/api/websocket.md)

## Verification Hooks

- [ ] restructure-step S07-W070-V01: run automation-event stream checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [ ] restructure-step S07-W070-V02: sync WS event-surface status in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) [doc-link](/docs/reference/CONFORMANCE.md)

## Mandatory Build and Test Gate

- [ ] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [ ] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [ ] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

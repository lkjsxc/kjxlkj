# Wave 070: Librarian Event Types and Broadcast Wiring

Back: [/docs/todo/waves/stage-07-websocket-sync/README.md](/docs/todo/waves/stage-07-websocket-sync/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] add librarian run lifecycle events to workspace stream -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] add operation preview/apply/reject event payload definitions -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] enforce stable event code vocabulary for client automation UI -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)

## Verification Tasks

- [ ] run WS stream smoke tests for librarian event presence -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] run unknown-event compatibility checks -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)

## Evidence Placeholder

- [ ] `Check:` `cargo test -p kjxlkj-server tests_ws -- --nocapture` -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] `Result:` pass -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] `Proof:` `ws_flow`: `1 passed; 0 failed`; `automation_event` payload assertions include `operation_preview|operation_applied|operation_rejected`; unknown `future_workspace_event` replay assertion passed -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)

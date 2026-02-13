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

- [x] add librarian run lifecycle events to workspace stream
- [x] add operation preview/apply/reject event payload definitions
- [x] enforce stable event code vocabulary for client automation UI

## Verification Tasks

- [x] run WS stream smoke tests for librarian event presence
- [x] run unknown-event compatibility checks

## Evidence Placeholder

- [x] `Check:` `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test ws_flow ws_subscribe_patch_replay_and_conflict_flow -- --nocapture`
- [x] `Result:` pass
- [x] `Proof:` `ws_flow`: `1 passed; 0 failed`; `automation_event` payload assertions include `operation_preview|operation_applied|operation_rejected`; unknown `future_workspace_event` replay assertion passed

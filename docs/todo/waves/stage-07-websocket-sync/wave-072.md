# Wave 072: End-to-End WS Librarian Acceptance

Back: [/docs/todo/waves/stage-07-websocket-sync/README.md](/docs/todo/waves/stage-07-websocket-sync/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] complete `WS-06` acceptance scenarios -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] validate ordering guarantees with mixed note and librarian events -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] archive deterministic replay evidence for reference ledgers -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)

## Verification Tasks

- [ ] run `WS-06` -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] run WS soak scenario with librarian event bursts -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)

## Evidence Placeholder

- [ ] `Check:` `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test ws_flow ws_subscribe_patch_replay_and_conflict_flow -- --nocapture` and `for i in 1 2 3 4 5; do TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test ws_flow ws_subscribe_patch_replay_and_conflict_flow -- --nocapture; done` -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] `Result:` pass -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [ ] `Proof:` single-run and 5x repeated runs passed (`1 passed; 0 failed` each run); replay stream asserted commit-order chain `note_patched < automation_run_queued < automation_run_running < automation_run_succeeded` -> [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)

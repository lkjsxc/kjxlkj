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

- [x] complete `WS-06` acceptance scenarios
- [x] validate ordering guarantees with mixed note and librarian events
- [x] archive deterministic replay evidence for reference ledgers

## Verification Tasks

- [x] run `WS-06`
- [x] run WS soak scenario with librarian event bursts

## Evidence Placeholder

- [x] `Check:` `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test ws_flow ws_subscribe_patch_replay_and_conflict_flow -- --nocapture` and `for i in 1 2 3 4 5; do TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test ws_flow ws_subscribe_patch_replay_and_conflict_flow -- --nocapture; done`
- [x] `Result:` pass
- [x] `Proof:` single-run and 5x repeated runs passed (`1 passed; 0 failed` each run); replay stream asserted commit-order chain `note_patched < automation_run_queued < automation_run_running < automation_run_succeeded`

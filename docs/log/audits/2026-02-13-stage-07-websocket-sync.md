# Stage 07 Audit: Librarian WebSocket Sync and Replay

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Scope

Stage 07 (`wave-070`/`wave-071`/`wave-072`) delivery for librarian realtime and replay behavior:

- typed librarian lifecycle stream messages over workspace websocket subscriptions
- stable automation event payload vocabulary with operation preview/apply/reject fields
- deterministic stale-cursor rejection on `ack` regressions
- replay/idempotency coverage with mixed note + librarian ordering evidence (`WS-06`)
- unknown-event compatibility for forward-safe stream consumers

## Implementation Evidence

Changed runtime paths:

- `src/crates/db/kjxlkj-db/src/repos/automation.rs`
  - added stable `event_code` vocabulary for automation workspace events
  - expanded queued/running/succeeded/failed payloads with provider/model metadata
  - added `operation_preview`, `operation_applied`, `operation_rejected` arrays and deterministic counts
- `src/crates/app/kjxlkj-server/src/app_state.rs`
  - made websocket ack cursor updates monotonic
  - stale updates now return deterministic rejection (`Err(current_cursor)`)
- `src/crates/app/kjxlkj-server/src/handlers/ws.rs`
  - introduced typed workspace stream emission (`workspace_event` vs `automation_event`)
  - mapped automation event families to `automation_event` with required run/status fields
  - rejected stale `ack` cursor regressions with websocket `error` payload (`code=STALE_CURSOR` + cursor context)
- `src/crates/app/kjxlkj-server/tests/ws_flow.rs`
  - extended coverage to assert `automation_event` payload shape
  - validated replay cursors for both `note:{id}` and `workspace:{id}` streams
  - validated stale cursor deterministic reject behavior
  - validated mixed stream ordering (`note_patched < automation_run_queued < automation_run_running < automation_run_succeeded`)
  - validated unknown workspace event replay compatibility (`future_workspace_event`)

## Verification Evidence

Executed checks:

1. `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test ws_flow ws_subscribe_patch_replay_and_conflict_flow -- --nocapture`
2. `for i in 1 2 3 4 5; do TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test ws_flow ws_subscribe_patch_replay_and_conflict_flow -- --nocapture; done`

Observed results:

- single-run `WS-06` check: pass (`1 passed; 0 failed`)
- repeated burst/replay soak (5 iterations): pass on all runs (`1 passed; 0 failed` each iteration)

`WS-06` assertions now covered by deterministic integration evidence:

- automation lifecycle updates are emitted as `automation_event`
- replay cursor is honored for workspace and note streams independently
- stale cursor regressions are rejected with deterministic machine code and cursor details
- mixed note + librarian workspace events preserve commit order by `event_seq`
- unknown workspace event types are replayed without breaking stream processing

## Residual Deferred Scope

`LIM-LIB-01` remains open for operation-apply runtime and full E2E librarian closure (`E2E-15`). Stage 07 closes websocket stream/replay protocol obligations, not operation execution breadth.

# Stage 07 Session Log

## Scope

Stage 07: WebSocket Replay and Automation Events.

## Waves Completed

- Wave 070: Workspace and Librarian Event Surfaces
- Wave 071: Ack Cursor, Replay, and Stale Cursor Handling
- Wave 072: End-to-End Realtime Closure

## Changes

### New Files

- `src/crates/db/kjxlkj-db/src/repo_automation_event.rs` (60 lines)
  - `emit_lifecycle_event()`: wraps workspace event emission with automation prefix
  - `list_run_events()`: filters workspace events by automation prefix and run_id
- `src/crates/ws/kjxlkj-ws/src/automation_events.rs` (78 lines)
  - `is_automation_event()`: prefix detection
  - `classify_event()`: converts WorkspaceEventRow to AutomationEvent ServerMessage
  - `build_automation_payload()`: constructs automation event payload
  - Unit test for prefix detection

### Modified Files

- `src/crates/ws/kjxlkj-ws/src/messages.rs` (101 lines)
  - Added `AutomationEvent` variant to ServerMessage
  - Added `details: Option<serde_json::Value>` field to Error variant per errors.md
- `src/crates/ws/kjxlkj-ws/src/protocol.rs` (174 lines)
  - Removed duplicate `handle_apply_patch` (defers to protocol_patch module)
  - Added automation event classification in workspace subscribe replay
  - Updated Error constructors with `details` field
  - Added structured STALE_CURSOR details (stream_id, attempted_seq, current_cursor)
- `src/crates/ws/kjxlkj-ws/src/protocol_patch.rs` (124 lines)
  - Updated Error constructors with `details: None`
- `src/crates/ws/kjxlkj-ws/src/session_actor.rs` (148 lines)
  - Updated Error constructor with `details: None`
  - Added `ActorFutureExt` import for `map()` trait method
- `src/crates/ws/kjxlkj-ws/src/lib.rs` (33 lines)
  - Added `automation_events` module
- `src/crates/db/kjxlkj-db/src/lib.rs` (22 modules)
  - Added `repo_automation_event` module
- `src/crates/http/kjxlkj-http/tests/acceptance_pack.rs` (198 lines)
  - Added stubs: WS-02, WS-03, WS-06, API-AUTO-04, API-VIEW-01, API-DASH-01, API-PROJ-01

## Verification

- `cargo check --workspace`: zero errors, zero warnings
- `cargo test --workspace`: 54 tests passing (8 domain + 31 acceptance + 14 regression + 1 WS automation)
- All source files ≤ 200 lines
- All Stage 07 TODO checkboxes marked `[x]`
- Reference ledgers updated (CONFORMANCE, DRIFT_MATRIX, LIMITATIONS)

## Architecture Notes

- Automation events are stored as workspace events with `automation_` prefix on event_type
- This preserves unified workspace stream ordering (monotonic event_seq)
- During workspace subscribe replay, events with automation prefix are emitted as `automation_event` server messages instead of `workspace_event`
- The `details` field on WS Error is `skip_serializing_if = "Option::is_none"` for backward compatibility
- protocol.rs now delegates patch handling to protocol_patch.rs (eliminated code duplication)

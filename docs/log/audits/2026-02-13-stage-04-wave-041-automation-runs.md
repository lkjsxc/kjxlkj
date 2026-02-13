# Audit: Stage 04 Wave 041 Automation Run Engine and Events

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Date

2026-02-13

## Scope

Closure evidence for Wave 041:

- trigger-based automation run evaluation and run state machine
- idempotent run execution per `(rule_id, triggering_event_id)`
- workspace stream automation events and audit signals

## Implementation Summary

- added automation run queue/start/succeed/fail transition repository functions
- enforced run idempotency with unique `(rule_id, triggering_event_id)` handling
- integrated automation evaluation into note mutation paths and websocket patch paths
- emitted workspace events for automation run lifecycle (`queued/running/succeeded/failed`)
- emitted security audit events for automation run lifecycle transitions
- added integration test for run-idempotency, status retrieval, and workspace replay of automation events

## Deterministic Checks

### Check 1: compile baseline

```bash
cargo check --workspace --tests
```

Result: pass.

Proof:

```text
Checking kjxlkj-db v0.1.0
Checking kjxlkj-workspace v0.1.0
Checking kjxlkj-server v0.1.0
Finished `dev` profile [unoptimized + debuginfo]
```

### Check 2: API-AUTO-02 and run-idempotency boundaries

```bash
TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/kjxlkj_test cargo test -p kjxlkj-server --test automation_run_flow -- --nocapture
```

Result: pass.

Proof:

```text
test automation_run_idempotency_status_and_ws_event_replay ... ok
test result: ok. 1 passed; 0 failed
```

### Check 3: regression pack after run-engine integration

```bash
TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/kjxlkj_test cargo test -p kjxlkj-server --test ws_flow --test views_api --test ui_shell --test command_workflow --test automation_rules_api --test automation_run_flow -- --nocapture
```

Result: pass.

Proof:

```text
test automation_rule_crud_validation_and_forbidden_paths ... ok
test automation_run_idempotency_status_and_ws_event_replay ... ok
test command_actions_create_open_move_tag_and_run_rule_failure_path ... ok
test root_serves_workspace_shell_markup ... ok
test setup_lock_conflict_is_deterministic_for_login_only_switch ... ok
test saved_view_lifecycle_and_role_denial ... ok
test ws_subscribe_patch_replay_and_conflict_flow ... ok
```

## Conclusion

Wave 041 run-engine/idempotency/workspace-event objectives are implemented and evidence-backed. Wave 042 (export/backup and observability jobs) is the next ordered scope.
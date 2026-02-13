# Audit: Stage 05 Wave 050 Reliability Regression Guard Pack

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Date

2026-02-13

## Scope

Closure evidence for Wave 050:

- map `IMP-*` and `USR-*` findings to concrete regression tests
- add replay/idempotency/conflict boundary regression checks
- close finding-family reliability guard gap for implemented surfaces

## Mapping Ledger

| Finding | Regression Coverage |
|---|---|
| `IMP-001` | UI autosave path uses server-synced base (`state.currentMarkdown`) and command workflow patch regression (`command_workflow`) |
| `IMP-002` | HTTP idempotent retransmit guard (`reliability_regressions`) and WS idempotent retransmit guard (`ws_flow`) |
| `IMP-003` | metadata delete response `204` guard (`reliability_regressions`) |
| `IMP-004` | websocket replay cursor and reconnect behavior guard (`ws_flow`) |
| `IMP-005` | deferred to attachments implementation track under `LIM-API-02` (attachment APIs not yet implemented) |
| `USR-001` | pre-auth `GET /api/auth/session` returns deterministic `401` guard (`reliability_regressions`) |
| `USR-002` | fallback idempotency generator markers (`Date.now` + `Math.random`) guard (`ui_shell`) |
| `USR-003` | autosave/title patch flow regression coverage (`command_workflow`, `ui_shell`) |
| `USR-004` | setup-lock login-only switch guard (`ui_shell`) |
| `USR-005` | compact-screen nav collapse/restore marker guard (`ui_shell`) |
| `USR-006` | optional dashboard baseline maintained (no required dashboard shell controls) (`ui_shell`) |
| `USR-007` | same-cycle title propagation to list guard (`command_workflow`) |
| `USR-008` | minimal editor chrome guard (`ui_shell`) |

## Deterministic Checks

### Check 1: reliability-focused regression suite

```bash
TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/kjxlkj_test cargo test -p kjxlkj-server --test ws_flow --test automation_run_flow --test command_workflow --test reliability_regressions -- --nocapture
```

Result: pass.

Proof:

```text
test automation_run_idempotency_status_and_ws_event_replay ... ok
test command_actions_create_open_move_tag_and_run_rule_failure_path ... ok
test imp_002_imp_003_and_usr_001_regression_guards ... ok
test ws_subscribe_patch_replay_and_conflict_flow ... ok
```

### Check 2: non-flakiness verification (double run)

```bash
for run in 1 2; do TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/kjxlkj_test cargo test -p kjxlkj-server --test ws_flow --test automation_run_flow --test command_workflow --test reliability_regressions -- --nocapture; done
```

Result: pass.

Proof:

```text
run 1: all 4 tests passed
run 2: all 4 tests passed
```

### Check 3: UI regression markers for setup-lock/minimal chrome/idempotency fallback

```bash
TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/kjxlkj_test cargo test -p kjxlkj-server --test ui_shell -- --nocapture
```

Result: pass.

Proof:

```text
test root_serves_workspace_shell_markup ... ok
test setup_lock_conflict_is_deterministic_for_login_only_switch ... ok
```

## Conclusion

Wave 050 reliability guard objectives are implemented with deterministic evidence for active runtime surfaces. Attachment-specific continuity coverage remains deferred to the attachment API implementation track.
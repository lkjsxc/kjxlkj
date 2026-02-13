# Audit: Stage 06 Wave 061 Librarian Rule and Run Payload Contract

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Date

2026-02-13

## Scope

Closure evidence for Wave 061:

- validate `librarian_structure` action schema in automation routes
- persist parsed operation reports in run status payloads
- enforce scope/safety guards before operation application

## Implementation Summary

- extended librarian rule validation to require structured `plan` contract fields:
  - `goal`, `scope`, `taxonomy_json`, `style_profile`, `strict_mode`, `max_operations`
  - bounded `max_operations` (`1..=128`) and default-safe `allow_delete=false`
- introduced consolidated librarian action config parsing (`provider` + `plan`) and reused it for:
  - rule create/update validation path
  - run metadata extraction path
  - runtime execution path
- persisted structured operation-report payloads into automation run `result_json`:
  - `parsed_operations`
  - `applied_operations`
  - `rejected_operations`
  - `warnings`
- added pre-apply scope/safety guard evaluation on parsed candidate operations:
  - strict-mode kind allowlist enforcement
  - max-operations bound enforcement
  - confidence range guard
  - scope guard (`workspace`, `workspace:<id>`, `note:<id>`) with deterministic rejection reasons
- added deterministic XML operation extraction helper (non-repair baseline) to drive report payloads before full parser wave

## Deterministic Checks

### Check 1: API-AUTO-03 + malformed-payload boundaries + lifecycle regression

```bash
TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32771/kjxlkj_test cargo test -p kjxlkj-server --test automation_rules_api --test automation_provider_adapter --test automation_run_flow -- --nocapture
```

Result: pass.

Proof:

```text
test librarian_provider_modes_store_run_metadata_and_succeed ... ok
test librarian_operation_report_rejects_out_of_scope_operations ... ok
test librarian_provider_timeout_and_outage_fail_with_deterministic_codes ... ok
test automation_rule_crud_validation_and_forbidden_paths ... ok
test automation_run_idempotency_status_and_ws_event_replay ... ok
test result: ok. 5 passed; 0 failed
```

### Check 2: compile baseline

```bash
cargo test -p kjxlkj-server --tests --no-run
```

Result: pass.

Proof:

```text
Finished `test` profile [unoptimized + debuginfo]
Executable tests/automation_provider_adapter.rs
Executable tests/automation_rules_api.rs
Executable tests/automation_run_flow.rs
```

## Conclusion

Wave 061 librarian payload contract work is implemented and evidence-backed. Stage 06 remains in progress with Wave 062 (`API-AUTO-04`) pending for full `xml_attrless` parser repair-retry/failure-state semantics.

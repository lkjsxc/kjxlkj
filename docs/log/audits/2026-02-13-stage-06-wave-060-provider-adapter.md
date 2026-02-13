# Audit: Stage 06 Wave 060 Provider Adapter Baseline

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Date

2026-02-13

## Scope

Closure evidence for Wave 060:

- provider adapter baseline for `openrouter` and `lmstudio`
- deterministic timeout/retry/failure-classification path for librarian actions
- provider/model metadata persistence on automation run records

## Implementation Summary

- added migration `0006_stage06_librarian_provider.sql` to persist `provider_kind` and `model` on `automation_runs`
- extended `DbAutomationRun` and automation repo queries/mutations to include provider metadata fields
- extended automation queueing path to persist provider metadata at run creation time
- implemented librarian provider adapter execution path in automation handler:
  - OpenAI-compatible request path for `openrouter` and `lmstudio`
  - deterministic request timeout and bounded retry policy (`retry_limit` bounded to `<=2`)
  - deterministic failure-code classification (`LLM_PROVIDER_TIMEOUT`, `LLM_PROVIDER_UNREACHABLE`, `LLM_UPSTREAM_ERROR`)
- updated run-status payload to expose `provider_kind` and `model`
- added integration suite `automation_provider_adapter.rs` covering:
  - mode validation and success execution for both providers
  - outage/timeout boundary behavior and retry evidence
  - provider metadata persistence assertions for succeeded/failed runs

## Deterministic Checks

### Check 1: provider validation and outage/timeout boundaries

```bash
TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test automation_rules_api --test automation_provider_adapter -- --nocapture
```

Result: pass.

Proof:

```text
test librarian_provider_modes_store_run_metadata_and_succeed ... ok
test librarian_provider_timeout_and_outage_fail_with_deterministic_codes ... ok
test automation_rule_crud_validation_and_forbidden_paths ... ok
test result: ok. 3 passed; 0 failed
```

### Check 2: regression guard for prior automation run lifecycle behavior

```bash
TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32770/kjxlkj_test cargo test -p kjxlkj-server --test automation_run_flow -- --nocapture
```

Result: pass.

Proof:

```text
test automation_run_idempotency_status_and_ws_event_replay ... ok
test result: ok. 1 passed; 0 failed
```

### Check 3: compile baseline for all server tests

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

Wave 060 provider-adapter baseline is implemented and evidence-backed. Stage 06 remains in progress pending Wave 061 (`API-AUTO-03` schema/report guards) and Wave 062 (`API-AUTO-04` xml parser/retry semantics).

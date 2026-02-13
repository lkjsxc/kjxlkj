# Stage 06 Wave 062 Audit: XML Parser and Retry Loop

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Scope

Wave 062 delivery for librarian protocol runtime:

- deterministic `xml_attrless` parser with malformed-nesting rejection
- required-tag response validation and confidence-range enforcement
- bounded repair loop (initial + max two retries) with deterministic failure codes
- failed-run audit retention for raw model outputs and parse diagnostics

## Implementation Evidence

Changed runtime paths:

- `src/crates/app/kjxlkj-server/src/handlers/automation.rs`
  - replaced ad-hoc tag extraction with stack-based `xml_attrless` parser
  - added structured response parsing (`request_id`, `status`, `summary`, `operations`, `warnings`)
  - enforced operation required fields and confidence range
  - added repair prompt builder carrying only validation errors and original output
  - added bounded repair retry loop and diagnostics payload persistence on failed runs
  - preserved deterministic failure coding (`LIBRARIAN_PROTOCOL_INVALID`, `LIBRARIAN_PARSE_FAILED`)
- `src/crates/db/kjxlkj-db/src/repos/automation.rs`
  - expanded `mark_run_failed` to accept optional `result_json` diagnostics payload
- `src/crates/app/kjxlkj-server/tests/automation_provider_adapter.rs`
  - aligned provider-success fixtures to full response contract
  - added parse-failure diagnostics test (malformed nesting + missing required tag)
  - added overflow-operations rejection test (`MAX_OPERATIONS_EXCEEDED`)

## Verification Evidence

Executed checks:

1. `cargo test -p kjxlkj-server --no-run`
2. `cargo test -p kjxlkj-server --test automation_provider_adapter -- --nocapture`
3. `cargo test -p kjxlkj-server --test automation_rules_api -- --nocapture`
4. `cargo test -p kjxlkj-server --test automation_run_flow -- --nocapture`

Observed results:

- compile: pass
- `automation_provider_adapter`: pass (`5 passed; 0 failed`)
- `automation_rules_api`: pass (`1 passed; 0 failed`)
- `automation_run_flow`: pass (`1 passed; 0 failed`)

API-AUTO-04 assertions covered in runtime tests:

- malformed nesting fails run with `LIBRARIAN_PROTOCOL_INVALID`
- missing required tags fail run with `LIBRARIAN_PARSE_FAILED`
- failed runs persist `raw_model_outputs` and `parse_diagnostics` arrays
- overflow operations beyond `max_operations` are deterministically rejected

## Residual Deferred Scope

`LIM-LIB-01` remains open for operation-apply execution and downstream stream/E2E closure (`WS-06`, `E2E-15`). Wave 062 closes parser/retry/failure-state obligations only.

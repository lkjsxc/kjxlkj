# 2026-02-14 Automation Librarian Contract Sync

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Objective

Restore an executable automation/librarian baseline contract and synchronize
TODO/reference ledgers with deterministic evidence.

## Implementation Scope

- Implemented automation rule CRUD handlers with provider/protocol validation.
- Implemented automation run launch/list/get/review handlers.
- Enforced librarian provider kind validation (`openrouter`, `lmstudio`).
- Enforced protocol validation (`xml_attrless`) with deterministic `422` failures.
- Split automation handler source to keep runtime files within <=200 line policy.

## Deterministic Checks

Command:

`cargo test -p kjxlkj-server tests_automation -- --nocapture`

Result:

- pass (`tests_automation::automation_librarian_provider_validation_and_review`)

Command:

`cargo check --workspace`

Result:

- pass

Command:

`npm run typecheck`

Result:

- pass

Command:

`find src -type f \( -name '*.rs' -o -name '*.ts' -o -name '*.tsx' \) -exec sh -c 'lines=$(wc -l < "$1"); if [ "$lines" -gt 200 ]; then printf "%s:%s\n" "$1" "$lines"; fi' _ {} \;`

Result:

- no output (all runtime source files are <=200 lines)

## Ledger/TODO Impact

- top-level TODO row `Restore automation and librarian review/apply behavior` is checked.
- conformance now records automation/librarian contract as `partial` with
  deterministic evidence.
- limitations keep `LIM-AUTO-04` open as medium-severity until parser/retry/apply
  safety matrix closure.
- drift matrix includes explicit `R-AUTO-04` `M2` drift row.

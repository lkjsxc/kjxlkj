# Wave 061: Librarian Rule and Run Payload Contract

Back: [/docs/todo/waves/stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [x] validate `librarian_structure` action schema in automation routes
- [x] persist parsed operation reports in run status payloads
- [x] enforce scope and safety guards before operation application

## Verification Tasks

- [x] run `API-AUTO-03`
- [x] run malformed action payload rejection boundary tests

## Evidence Placeholder

- [x] `Check:` `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32771/kjxlkj_test cargo test -p kjxlkj-server --test automation_rules_api --test automation_provider_adapter --test automation_run_flow -- --nocapture`
- [x] `Result:` pass
- [x] `Proof:` `automation_provider_adapter`: `3 passed`; `automation_rules_api`: `1 passed`; `automation_run_flow`: `1 passed`

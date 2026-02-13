# Wave 090: CI Profile Integration and Stability

Back: [/docs/todo/waves/stage-09-ci-performance-release/README.md](/docs/todo/waves/stage-09-ci-performance-release/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [x] wire `Librarian-runtime` and `Librarian-small-model` profile execution
- [x] enforce deterministic fixture sets for parser and provider error paths
- [x] eliminate flaky signals in librarian test suites

## Verification Tasks

- [x] run CI profile matrix on clean environment
- [x] run rerun stability checks for nondeterminism

## Evidence Placeholder

- [x] `Check:` `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test automation_rules_api --test automation_provider_adapter --test automation_run_flow --test ws_flow -- --nocapture` and `for i in 1 2 3; do TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test automation_provider_adapter --test automation_run_flow --test ws_flow -- --nocapture; done`
- [x] `Result:` pass
- [x] `Proof:` profile matrix and 3x rerun stability checks passed with no nondeterministic failures (`automation_provider_adapter: 5 passed`, `automation_run_flow: 1 passed`, `ws_flow: 1 passed`)

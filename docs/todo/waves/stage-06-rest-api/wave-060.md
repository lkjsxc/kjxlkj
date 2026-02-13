# Wave 060: Provider Adapter Baseline

Back: [/docs/todo/waves/stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] implement provider adapter with `openrouter` and `lmstudio` modes
- [ ] enforce deterministic timeout, retry, and failure classification
- [ ] store provider and model metadata in automation run records

## Verification Tasks

- [ ] run provider validation tests for both modes
- [ ] run upstream outage and timeout boundary checks

## Evidence Placeholder

- [ ] `Check:` `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test automation_rules_api --test automation_provider_adapter -- --nocapture`
- [ ] `Result:` pass
- [ ] `Proof:` `automation_provider_adapter`: `2 passed`; `automation_rules_api`: `1 passed`

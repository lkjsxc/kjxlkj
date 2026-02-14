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

- [ ] validate `librarian_structure` action schema in automation routes -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] validate `prompt_pack` schema fields and manifest path presence -> [/docs/spec/api/types.md](/docs/spec/api/types.md)
- [ ] persist parsed operation reports in run status payloads -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] enforce scope and safety guards before operation application -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)

## Verification Tasks

- [ ] run `API-AUTO-03` -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] run `API-AUTO-05` -> [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] run malformed action payload rejection boundary tests -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)

## Evidence Placeholder

- [ ] `Check:` `cargo test -p kjxlkj-server tests_automation tests_feature_endpoints -- --nocapture` -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] `Result:` pass -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] `Proof:` `automation_provider_adapter`: `3 passed`; `automation_rules_api`: `1 passed`; `automation_run_flow`: `1 passed` -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)

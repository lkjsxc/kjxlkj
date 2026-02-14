# Wave 060: Provider Adapter Baseline

Back: [/docs/todo/waves/stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md)

## Implementation Tasks

- [ ] implement provider adapter with `openrouter` and `lmstudio` modes -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] enforce deterministic timeout, retry, and failure classification -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] store provider and model metadata in automation run records -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] load prompt stage mapping from `manifest.json` rather than embedded prompt literals -> [/docs/spec/technical/librarian-prompts/manifest.json](/docs/spec/technical/librarian-prompts/manifest.json)

## Verification Tasks

- [ ] run provider validation tests for both modes -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] run upstream outage and timeout boundary checks -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] run prompt-pack manifest parse and missing-file failure checks -> [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md)

## Evidence Placeholder

- [ ] `Check:` `cargo test -p kjxlkj-server tests_automation -- --nocapture` -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] `Result:` pass -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] `Proof:` provider adapter matrix and prompt-pack config validation checks pass deterministically -> [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md)

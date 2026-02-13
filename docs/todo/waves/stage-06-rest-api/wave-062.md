# Wave 062: Attribute-Less XML Parser and Retry Loop

Back: [/docs/todo/waves/stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] implement `xml_attrless` parser and required-tag validation
- [ ] implement bounded parser repair retries and deterministic failure states
- [ ] retain raw model output and parse diagnostics in run audit fields

## Verification Tasks

- [ ] run `API-AUTO-04`
- [ ] run malformed nesting, missing-tag, and overflow operation tests

## Evidence Placeholder

- [ ] `Check:` `cargo test -p kjxlkj-server --test automation_provider_adapter -- --nocapture`
- [ ] `Result:` pass (`5 passed; 0 failed`) including parser malformed nesting, missing-tag retry failure diagnostics, and overflow rejection assertions
- [ ] `Proof:` deterministic run records preserve `raw_model_outputs` + `parse_diagnostics`; malformed nesting fails with `LIBRARIAN_PROTOCOL_INVALID`; missing required tags fail with `LIBRARIAN_PARSE_FAILED`

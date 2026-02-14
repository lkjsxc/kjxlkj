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

- [ ] implement `xml_attrless` parser and required-tag validation -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] implement bounded parser repair retries and deterministic failure states -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] retain raw model output and parse diagnostics in run audit fields -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)

## Verification Tasks

- [ ] run `API-AUTO-04` -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] run malformed nesting, missing-tag, and overflow operation tests -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)

## Evidence Placeholder

- [ ] `Check:` `cargo test -p kjxlkj-server --test automation_provider_adapter -- --nocapture` -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] `Result:` pass (`5 passed; 0 failed`) including parser malformed nesting, missing-tag retry failure diagnostics, and overflow rejection assertions -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [ ] `Proof:` deterministic run records preserve `raw_model_outputs` + `parse_diagnostics`; malformed nesting fails with `LIBRARIAN_PROTOCOL_INVALID`; missing required tags fail with `LIBRARIAN_PARSE_FAILED` -> [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)

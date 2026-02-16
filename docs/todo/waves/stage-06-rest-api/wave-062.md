# Wave 062: xml_attrless Parse, Retry, and Apply Safety

Back: [/docs/todo/waves/stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md)

## Relevant Documents

- [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)
- [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)

## Restructure Steps

- [x] restructure-step S06-W062-01: implement attribute-less parser and required-tag validation from [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md) [doc-link](/docs/spec/api/librarian-xml.md)
- [x] restructure-step S06-W062-02: implement bounded repair retries and deterministic failure classes from [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) [doc-link](/docs/spec/technical/librarian-agent.md)
- [x] restructure-step S06-W062-03: enforce operation safety policy (scope/delete/review) from [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) [doc-link](/docs/spec/domain/automation.md)
- [x] restructure-step S06-W062-04: enforce protocol and provider error mapping from [/docs/spec/api/errors.md](/docs/spec/api/errors.md) [doc-link](/docs/spec/api/errors.md)
- [x] restructure-step S06-W062-05: preserve unresolved local drafts when apply runs touch active notes per [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) [doc-link](/docs/spec/ui/editor-flow.md)

## Verification Hooks

- [x] restructure-step S06-W062-V01: run `API-AUTO-04` and parser boundary checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step S06-W062-V02: sync parser/apply safety status in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/DRIFT_MATRIX.md)

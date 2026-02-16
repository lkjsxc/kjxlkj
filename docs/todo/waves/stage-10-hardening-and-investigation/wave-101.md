# Wave 101: Documentation and Verification Depth

Back: [/docs/todo/waves/stage-10-hardening-and-investigation/README.md](/docs/todo/waves/stage-10-hardening-and-investigation/README.md)

## Relevant Documents

- [/docs/reference/IMPROVEMENT_BACKLOG.md](/docs/reference/IMPROVEMENT_BACKLOG.md)
- [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [/docs/spec/api/openapi.md](/docs/spec/api/openapi.md)
- [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Restructure Steps

- [x] restructure-step S10-W101-01: add request-flow sequence diagrams in [/docs/spec/api/http.md](/docs/spec/api/http.md) and [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) [doc-link](/docs/spec/api/http.md)
- [x] restructure-step S10-W101-02: define JSON schema companion strategy for OpenAPI in [/docs/spec/api/openapi.md](/docs/spec/api/openapi.md) [doc-link](/docs/spec/api/openapi.md)
- [x] restructure-step S10-W101-03: codify document split strategy for high-line-count files in [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) [doc-link](/docs/policy/STRUCTURE.md)
- [x] restructure-step S10-W101-04: implement DB-backed integration test harness closure for `LIM-TEST-01` in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) [doc-link](/docs/reference/LIMITATIONS.md)
- [x] restructure-step S10-W101-05: add property-based and snapshot coverage requirements in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)

## Verification Hooks

- [x] restructure-step S10-W101-V01: run expanded test profiles listed in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step S10-W101-V02: synchronize verification status in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/DRIFT_MATRIX.md)

## Mandatory Build and Test Gate

- [x] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [x] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [x] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

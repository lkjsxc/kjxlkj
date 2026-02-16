# Wave 060: REST Surface Parity and OpenAPI Sync

Back: [/docs/todo/waves/stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md)

## Relevant Documents

- [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [/docs/spec/api/types.md](/docs/spec/api/types.md)
- [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- [/docs/spec/api/openapi.md](/docs/spec/api/openapi.md)
- [/docs/spec/api/openapi.yaml](/docs/spec/api/openapi.yaml)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Restructure Steps

- [x] restructure-step S06-W060-01: implement missing HTTP routes from [/docs/spec/api/http.md](/docs/spec/api/http.md) [doc-link](/docs/spec/api/http.md)
- [x] restructure-step S06-W060-02: enforce external payload schemas from [/docs/spec/api/types.md](/docs/spec/api/types.md) [doc-link](/docs/spec/api/types.md)
- [x] restructure-step S06-W060-03: enforce deterministic error envelopes from [/docs/spec/api/errors.md](/docs/spec/api/errors.md) [doc-link](/docs/spec/api/errors.md)
- [x] restructure-step S06-W060-04: synchronize route and schema examples in [/docs/spec/api/openapi.yaml](/docs/spec/api/openapi.yaml) [doc-link](/docs/spec/api/openapi.yaml)
- [x] restructure-step S06-W060-05: enforce OpenAPI change rules from [/docs/spec/api/openapi.md](/docs/spec/api/openapi.md) [doc-link](/docs/spec/api/openapi.md)

## Verification Hooks

- [x] restructure-step S06-W060-V01: run API acceptance checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step S06-W060-V02: sync API parity status in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/DRIFT_MATRIX.md)

## Mandatory Build and Test Gate

- [x] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [x] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [x] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

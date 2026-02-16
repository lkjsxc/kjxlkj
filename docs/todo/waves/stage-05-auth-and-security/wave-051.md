# Wave 051: Findings-Driven Regression Closure

Back: [/docs/todo/waves/stage-05-auth-and-security/README.md](/docs/todo/waves/stage-05-auth-and-security/README.md)

## Relevant Documents

- [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md)
- [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Restructure Steps

- [x] restructure-step S05-W051-01: implement regression guards for `IMP-*` findings from [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) [doc-link](/docs/spec/ui/findings-traceability.md)
- [x] restructure-step S05-W051-02: implement regression guards for `USR-*` findings from [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) [doc-link](/docs/spec/ui/findings-traceability.md)
- [x] restructure-step S05-W051-03: enforce editor replay/conflict regressions from [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) [doc-link](/docs/spec/ui/editor-flow.md)
- [x] restructure-step S05-W051-04: enforce API/WS boundary regressions from [/docs/spec/api/http.md](/docs/spec/api/http.md) and [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) [doc-link](/docs/spec/api/http.md)
- [x] restructure-step S05-W051-05: align regression IDs with acceptance catalog in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)

## Verification Hooks

- [x] restructure-step S05-W051-V01: run `REG-IMP-*`, `REG-USR-*`, and `REG-UX-003` packs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step S05-W051-V02: sync regression closure status in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/DRIFT_MATRIX.md)

## Mandatory Build and Test Gate

- [x] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [x] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [x] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

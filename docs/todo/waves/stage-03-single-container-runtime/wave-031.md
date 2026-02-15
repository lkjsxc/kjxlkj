# Wave 031: Editor State Model and Findings Regression

Back: [/docs/todo/waves/stage-03-single-container-runtime/README.md](/docs/todo/waves/stage-03-single-container-runtime/README.md)

## Relevant Documents

- [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md)
- [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md)
- [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Restructure Steps

- [ ] restructure-step S03-W031-01: enforce synced-snapshot and local-draft split from [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) [doc-link](/docs/spec/ui/editor-flow.md)
- [ ] restructure-step S03-W031-02: implement autosave/conflict/offline status transitions from [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) [doc-link](/docs/spec/ui/editor-flow.md)
- [ ] restructure-step S03-W031-03: enforce title propagation and minimal default chrome from [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) [doc-link](/docs/spec/ui/web-app.md)
- [ ] restructure-step S03-W031-04: enforce reconnect/idempotency behavior from [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) [doc-link](/docs/spec/api/websocket.md)
- [ ] restructure-step S03-W031-05: map each implemented fix to findings in [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) [doc-link](/docs/spec/ui/findings-traceability.md)
- [ ] restructure-step S03-W031-06: add regression test for Create New Note add/select behavior from [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) and [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/ui/web-app.md)

## Verification Hooks

- [ ] restructure-step S03-W031-V01: run `REG-IMP-*`, `REG-USR-*`, and `E2E-23` checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [ ] restructure-step S03-W031-V02: sync regression closure state in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/DRIFT_MATRIX.md)

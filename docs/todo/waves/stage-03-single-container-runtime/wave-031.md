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

- [x] restructure-step S03-W031-01: enforce synced-snapshot and local-draft split from [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [x] restructure-step S03-W031-02: implement autosave/conflict/offline status transitions from [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [x] restructure-step S03-W031-03: enforce title propagation and minimal default chrome from [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md)
- [x] restructure-step S03-W031-04: enforce reconnect/idempotency behavior from [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
- [x] restructure-step S03-W031-05: map each implemented fix to findings in [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md)

## Verification Hooks

- [x] restructure-step S03-W031-V01: run `REG-IMP-*` and `REG-USR-*` checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [x] restructure-step S03-W031-V02: sync regression closure state in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

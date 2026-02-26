# Wave 081: Librarian Review UX and Command Flows

Back: [/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md)

## Relevant Documents

- [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)
- [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)

## Restructure Steps

- [ ] restructure-step S08-W081-01: implement librarian run launch and review surfaces from [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) [doc-link](/docs/spec/ui/workspace-suite.md)
- [ ] restructure-step S08-W081-02: support per-operation accept/reject decisions from [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) [doc-link](/docs/spec/ui/reconstruction-ux-requirements.md)
- [ ] restructure-step S08-W081-03: preserve unresolved local drafts during apply flows from [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) [doc-link](/docs/spec/ui/editor-flow.md)
- [ ] restructure-step S08-W081-04: align review/apply API semantics with [/docs/spec/api/http.md](/docs/spec/api/http.md) [doc-link](/docs/spec/api/http.md)
- [ ] restructure-step S08-W081-05: align review progress events with [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) [doc-link](/docs/spec/api/websocket.md)

## Verification Hooks

- [ ] restructure-step S08-W081-V01: run `E2E-06`, `E2E-17`, `E2E-24`, `frontend_ws_replay_contract`, and `frontend_comm_degradation_e2e` checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and [/docs/reference/TEST_MATRIX.md](/docs/reference/TEST_MATRIX.md) [doc-link](/docs/spec/technical/testing.md)
- [ ] restructure-step S08-W081-V02: sync librarian UX status in [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/DRIFT_MATRIX.md)

## Mandatory Build and Test Gate

- [ ] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [ ] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [ ] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

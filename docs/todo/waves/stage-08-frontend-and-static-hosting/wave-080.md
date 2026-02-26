# Wave 080: Note-First Shell and Workspace Suite Baseline

Back: [/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md)

## Relevant Documents

- [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md)
- [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md)
- [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)

## Restructure Steps

- [ ] restructure-step S08-W080-01: implement note-first shell modules from [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) [doc-link](/docs/spec/ui/web-app.md)
- [ ] restructure-step S08-W080-02: keep dashboard/workspace switcher optional from [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) [doc-link](/docs/spec/ui/workspace-suite.md)
- [ ] restructure-step S08-W080-03: preserve editor-priority visual hierarchy from [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) [doc-link](/docs/spec/ui/layout-and-interaction.md)
- [ ] restructure-step S08-W080-04: align frontend session/notes data usage with [/docs/spec/api/http.md](/docs/spec/api/http.md) [doc-link](/docs/spec/api/http.md)
- [ ] restructure-step S08-W080-05: enforce strict TypeScript rules from [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) [doc-link](/docs/spec/technical/type-safety.md)

## Verification Hooks

- [ ] restructure-step S08-W080-V01: run `E2E-12`, `E2E-19`, `E2E-23`, `frontend_http_client_contract`, and `frontend_auth_session_rotation` checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and [/docs/reference/TEST_MATRIX.md](/docs/reference/TEST_MATRIX.md) [doc-link](/docs/spec/technical/testing.md)
- [ ] restructure-step S08-W080-V02: sync shell module status in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) [doc-link](/docs/reference/CONFORMANCE.md)

## Mandatory Build and Test Gate

- [ ] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [ ] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [ ] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

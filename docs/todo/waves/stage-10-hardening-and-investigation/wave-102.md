# Wave 102: Frontend and Security Hardening

Back: [/docs/todo/waves/stage-10-hardening-and-investigation/README.md](/docs/todo/waves/stage-10-hardening-and-investigation/README.md)

## Relevant Documents

- [/docs/reference/IMPROVEMENT_BACKLOG.md](/docs/reference/IMPROVEMENT_BACKLOG.md)
- [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md)
- [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [/docs/spec/security/auth.md](/docs/spec/security/auth.md)
- [/docs/spec/security/transport.md](/docs/spec/security/transport.md)
- [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md)

## Restructure Steps

- [x] restructure-step S10-W102-01: modularize monolithic frontend stylesheet strategy from [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) [doc-link](/docs/spec/ui/layout-and-interaction.md)
- [x] restructure-step S10-W102-02: add critical-flow browser E2E coverage from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step S10-W102-03: evaluate and apply lazy-loading boundaries for librarian review from [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) [doc-link](/docs/spec/ui/workspace-suite.md)
- [x] restructure-step S10-W102-04: implement CSP nonce and auth-route rate limiting controls from [/docs/spec/security/transport.md](/docs/spec/security/transport.md) and [/docs/spec/security/auth.md](/docs/spec/security/auth.md) [doc-link](/docs/spec/security/transport.md)
- [x] restructure-step S10-W102-05: enforce session-revocation broadcast on password change from [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md) [doc-link](/docs/spec/security/sessions.md)

## Verification Hooks

- [x] restructure-step S10-W102-V01: run frontend and security acceptance checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step S10-W102-V02: synchronize closure state in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) and [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) [doc-link](/docs/reference/CONFORMANCE.md)

## Mandatory Build and Test Gate

- [x] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [x] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [x] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

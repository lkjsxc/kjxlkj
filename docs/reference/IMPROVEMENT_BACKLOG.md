# Improvement Backlog

Back: [/docs/reference/README.md](/docs/reference/README.md)

Canonicalized backlog derived from prior implementation improvement notes.

## Governance

- This document replaces historical improvement logs.
- All items must be represented in TODO stage-10 checklists.
- Closure requires synchronized updates in:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
  - [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

## Backlog Matrix

| Backlog ID | Theme | Canonical Doc | TODO Step |
|---|---|---|---|
| `IMP-BACKLOG-ARCH-01` | DB pool tuning | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | `S10-W100-01` |
| `IMP-BACKLOG-ARCH-02` | SQLx compile-time query checking | [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md) | `S10-W100-02` |
| `IMP-BACKLOG-ARCH-03` | WS cross-actor broadcast registry | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `S10-W100-03` |
| `IMP-BACKLOG-ARCH-04` | Backup restore drill automation | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | `S10-W100-04` |
| `IMP-BACKLOG-DOC-01` | Request flow diagrams | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `S10-W101-01` |
| `IMP-BACKLOG-DOC-02` | JSON schema alongside OpenAPI | [/docs/spec/api/openapi.md](/docs/spec/api/openapi.md) | `S10-W101-02` |
| `IMP-BACKLOG-DOC-03` | Doc split strategy for large files | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | `S10-W101-03` |
| `IMP-BACKLOG-TEST-01` | Property-based patch tests | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `S10-W101-04` |
| `IMP-BACKLOG-TEST-02` | API snapshot coverage | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `S10-W101-05` |
| `IMP-BACKLOG-TEST-03` | DB-backed integration harness | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `S10-W101-04` |
| `IMP-BACKLOG-FE-01` | CSS modularization | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | `S10-W102-01` |
| `IMP-BACKLOG-FE-02` | Critical-flow E2E coverage | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `S10-W102-02` |
| `IMP-BACKLOG-FE-03` | React lazy-split review panel | [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) | `S10-W102-03` |
| `IMP-BACKLOG-SEC-01` | CSP nonce strategy | [/docs/spec/security/transport.md](/docs/spec/security/transport.md) | `S10-W102-04` |
| `IMP-BACKLOG-SEC-02` | Auth endpoint rate limiting | [/docs/spec/security/auth.md](/docs/spec/security/auth.md) | `S10-W102-04` |
| `IMP-BACKLOG-SEC-03` | Session revocation broadcast | [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md) | `S10-W102-05` |

## Related

- Stage-10 wave program: [/docs/todo/waves/stage-10-hardening-and-investigation/README.md](/docs/todo/waves/stage-10-hardening-and-investigation/README.md)
- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

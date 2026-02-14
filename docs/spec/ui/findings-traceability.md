# Findings Traceability

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

Canonical mapping from implementation/user findings to normative requirements.

## Source Findings

Primary source:

- [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md)

## Mapping Matrix

| Finding | Category | Normative Requirement | UX Requirement IDs | Canonical Spec Coverage | Required Acceptance IDs |
|---|---|---|---|---|---|
| `IMP-001` | editor correctness | synced server state and local draft state MUST be separate models | `UX-EDIT-01`, `UX-LIB-02` | [editor-flow.md](editor-flow.md), [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md) | `REG-IMP-001`, `WS-05`, `E2E-17` |
| `IMP-002` | replay safety | duplicate idempotency keys MUST replay existing commit identity | `UX-EDIT-07` | [editor-flow.md](editor-flow.md), [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `REG-IMP-002`, `WS-04` |
| `IMP-003` | API contract | metadata delete route MUST return `204` with no payload | `UX-FEEDBACK-02` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `REG-IMP-003`, `API-REC-01` |
| `IMP-004` | reconnect semantics | ack-cursor replay MUST be deterministic after reconnect | `UX-EDIT-06` | [editor-flow.md](editor-flow.md), [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `REG-IMP-004`, `WS-05`, `E2E-18` |
| `IMP-005` | data integrity | attachment stream MUST fail on chunk discontinuity | `UX-FEEDBACK-02` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | `REG-IMP-005`, `API-ATT-01` |
| `USR-001` | auth UX | pre-auth `401` on session probe MUST be treated as expected state | `UX-AUTH-01`, `UX-AUTH-03` | [web-app.md](web-app.md), [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md) | `REG-USR-001`, `E2E-10` |
| `USR-002` | compatibility | idempotency key generation MUST fallback when `crypto.randomUUID` is unavailable | `UX-EDIT-03` | [editor-flow.md](editor-flow.md), [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md) | `REG-USR-002`, `E2E-06` |
| `USR-003` | editing UX | autosave and markdown-native authoring are baseline, not optional | `UX-EDIT-02`, `UX-NAV-02`, `UX-FEEDBACK-01` | [editor-flow.md](editor-flow.md), [layout-and-interaction.md](layout-and-interaction.md), [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md) | `REG-USR-003`, `E2E-06`, `E2E-17` |
| `USR-004` | auth presentation | setup-locked state MUST render login-only UI with no setup-like visuals | `UX-AUTH-02` | [web-app.md](web-app.md), [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md) | `REG-USR-004`, `E2E-11` |
| `USR-005` | responsive usability | desktop split layout and constrained-screen collapse/restore MUST preserve editor-first flow with deterministic focus | `UX-LAYOUT-02`, `UX-LAYOUT-04`, `UX-LAYOUT-05`, `UX-LAYOUT-06`, `UX-A11Y-01` | [layout-and-interaction.md](layout-and-interaction.md), [web-app.md](web-app.md), [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md) | `REG-USR-005`, `E2E-12`, `E2E-19`, `E2E-23`, `E2E-24` |
| `USR-006` | cognitive load | dashboards/workspace switcher remain optional; baseline stays note-first | `UX-NAV-01` | [workspace-suite.md](workspace-suite.md), [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md) | `REG-USR-006`, `E2E-03`, `E2E-20` |
| `USR-007` | consistency | title rename MUST propagate in same cycle to lists/navigation surfaces | `UX-EDIT-04` | [web-app.md](web-app.md), [editor-flow.md](editor-flow.md), [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md) | `REG-USR-007`, `E2E-13` |
| `USR-008` | editor focus | default editor chrome omits inline version/save/delete controls | `UX-EDIT-05` | [web-app.md](web-app.md), [editor-flow.md](editor-flow.md), [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md) | `REG-USR-008`, `E2E-14` |

## Reconstruction Status Rule

For reconstruction baseline, all mapped findings are considered `open` until
runtime evidence is re-established.

## Closure Rule

A finding is considered closed only when all are true:

1. normative requirement exists in canonical spec
2. deterministic acceptance ID exists in testing contract
3. drift and limitations ledgers are synchronized for runtime status

## Related

- UX requirements: [reconstruction-ux-requirements.md](reconstruction-ux-requirements.md)
- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Limitations ledger: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

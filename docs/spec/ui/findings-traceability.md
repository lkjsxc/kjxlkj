# Findings Traceability

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

Canonical mapping from implementation/user findings to normative requirements.

## Source Findings

The baseline finding set is embedded in this document to keep the canonical
source self-contained in `/docs/spec/ui/`.

## Mapping Matrix

| Finding | Category | Normative Requirement | Canonical Spec Coverage | Required Acceptance IDs |
|---|---|---|---|---|
| `IMP-001` | editor correctness | synced server state and local draft state MUST be separate models | [editor-flow.md](editor-flow.md) | `REG-IMP-001`, `WS-05` |
| `IMP-002` | replay safety | duplicate idempotency keys MUST replay existing commit identity | [editor-flow.md](editor-flow.md), [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `REG-IMP-002`, `WS-04` |
| `IMP-003` | API contract | metadata delete route MUST return `204` with no payload | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `REG-IMP-003`, `API-REC-01` |
| `IMP-004` | reconnect semantics | ack-cursor replay MUST be deterministic after reconnect | [editor-flow.md](editor-flow.md), [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `REG-IMP-004`, `WS-05` |
| `IMP-005` | data integrity | attachment stream MUST fail on chunk discontinuity | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | `REG-IMP-005`, `API-ATT-01` |
| `USR-001` | auth UX | pre-auth `401` on session probe MUST be treated as expected state | [web-app.md](web-app.md) | `REG-USR-001`, `E2E-10` |
| `USR-002` | compatibility | idempotency key generation MUST fallback when `crypto.randomUUID` is unavailable | [editor-flow.md](editor-flow.md) | `REG-USR-002`, `E2E-06` |
| `USR-003` | editing UX | autosave and markdown-native authoring are baseline, not optional | [editor-flow.md](editor-flow.md), [layout-and-interaction.md](layout-and-interaction.md) | `REG-USR-003`, `E2E-06` |
| `USR-004` | auth presentation | setup-locked state MUST render login-only UI with no setup-like visuals | [web-app.md](web-app.md) | `REG-USR-004`, `E2E-11` |
| `USR-005` | mobile usability | constrained screens MUST allow collapsing navigation to expand editor | [layout-and-interaction.md](layout-and-interaction.md) | `REG-USR-005`, `E2E-12` |
| `USR-006` | cognitive load | dashboards/workspace switcher remain optional; baseline stays note-first | [workspace-suite.md](workspace-suite.md) | `REG-USR-006`, `E2E-03` |
| `USR-007` | consistency | title rename MUST propagate in same cycle to lists/navigation surfaces | [web-app.md](web-app.md), [editor-flow.md](editor-flow.md) | `REG-USR-007`, `E2E-13` |
| `USR-008` | editor focus | default editor chrome omits inline version/save/delete controls | [web-app.md](web-app.md), [editor-flow.md](editor-flow.md) | `REG-USR-008`, `E2E-14` |

## Closure Rule

A finding is considered closed only when all are true:

1. normative requirement exists in canonical spec
2. deterministic acceptance ID exists in testing contract
3. drift and limitations ledgers are synchronized for runtime status

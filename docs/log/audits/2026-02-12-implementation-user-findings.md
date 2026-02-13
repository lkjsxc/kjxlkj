# Implementation and User Findings (2026-02-12)

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

This audit captures defects and UX gaps discovered during hands-on implementation
and direct user feedback.

## Findings Ledger

| ID | Source | Finding | Impact | Root Cause | Required Spec/Build Action |
|---|---|---|---|---|---|
| `IMP-001` | implementation | note patch generation used mutable editor text as base | incorrect patch payload and conflict risk | editor state mixed server-synced and draft content | enforce synced/draft split model and regression tests |
| `IMP-002` | implementation | duplicate `idempotency_key` path was conflict-oriented instead of replay-safe | retransmit instability and duplicate-write ambiguity | idempotency behavior not fully wired to existing event lookup | require deterministic idempotent retransmit semantics and tests |
| `IMP-003` | implementation | metadata delete response shape diverged from HTTP contract | API contract drift | handler contract mismatch | lock `DELETE /notes/{id}/metadata/{key}` to strict `204` |
| `IMP-004` | implementation | WS replay cursor handling was incomplete | reconnect gaps and stale client state | `ack` cursor accepted but replay behavior under-specified | expand WS replay contract and acceptance tests |
| `IMP-005` | implementation | attachment download continuity checks were missing initially | possible partial/corrupt stream delivery | stream integrity rules not fully enforced | keep strict chunk continuity/integrity rules in attachment contract |
| `USR-001` | user report | `GET /api/auth/session` `401` surfaced as error in browser console | confusing first-load experience | unauthenticated state not treated as expected pre-login condition | treat pre-auth `401` as normal UX path |
| `USR-002` | user report | `crypto.randomUUID` unavailable in some client environments | save failure and blocked editing | compatibility assumption in key generation | require fallback idempotency key generation |
| `USR-003` | user report | explicit request for autosave and richer editor behavior | low editing confidence and extra friction | editor interaction contract too minimal | add autosave-first markdown-native UX requirements |
| `USR-004` | user report | setup-like visual shown when setup was already locked | misleading auth flow and onboarding confusion | setup/login presentation state not separated by lock signal | require login-only presentation after setup lock (`409`) |
| `USR-005` | user report | small screens gave insufficient note-editing space | editing friction on constrained displays | navigation/list regions were not collapsible in compact layouts | require menu-toggle collapse/restore behavior |
| `USR-006` | user report | dashboard/workspace/project-rail surfaces added unnecessary UI noise | reduced focus and cognitive overhead | non-essential modules treated as required baseline | keep dashboards/workspace switcher optional and note-first baseline |
| `USR-007` | user report | note title changes were not consistently reflected in list-related areas | stale context and wrong navigation labels | rename propagation rules were under-specified | require same-cycle title propagation across related surfaces |
| `USR-008` | user report | version/save/delete controls in editor were unnecessary for default flow | cluttered editing surface | editor chrome requirements overexposed secondary controls | hide secondary controls by default |

## Canonical Spec Closure Mapping

| Finding | Canonical spec evidence | Acceptance evidence contract |
|---|---|---|
| `IMP-001` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | `REG-IMP-001`, `WS-05` |
| `IMP-002` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md), [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `REG-IMP-002`, `WS-04` |
| `IMP-003` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `REG-IMP-003`, `API-REC-01` |
| `IMP-004` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md), [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `REG-IMP-004`, `WS-05` |
| `IMP-005` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | `REG-IMP-005`, `API-ATT-01` |
| `USR-001` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | `REG-USR-001`, `E2E-10` |
| `USR-002` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | `REG-USR-002`, `E2E-06` |
| `USR-003` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md), [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | `REG-USR-003`, `E2E-06` |
| `USR-004` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | `REG-USR-004`, `E2E-11` |
| `USR-005` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | `REG-USR-005`, `E2E-12` |
| `USR-006` | [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) | `REG-USR-006`, `E2E-03` |
| `USR-007` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md), [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | `REG-USR-007`, `E2E-13` |
| `USR-008` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md), [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | `REG-USR-008`, `E2E-14` |

## Reconstruction Implications

- Future runtime reconstruction MUST include deterministic coverage for all
  `REG-IMP-*` and `REG-USR-*` IDs.
- Any regression against this ledger is a release blocker until closed.

## Related

- Findings traceability: [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md)
- Drift ledger: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Limitations ledger: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

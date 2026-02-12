# Implementation and User Findings (2026-02-12)

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

This audit captures defects and UX gaps discovered during hands-on implementation and user feedback.

## Findings Ledger

| ID | Source | Finding | Impact | Root Cause | Required Spec/Build Action |
|---|---|---|---|---|---|
| `IMP-001` | implementation | note patch generation used mutable editor text as base | incorrect patch payload and conflict risk | editor state mixed server-synced and draft content | enforce separate synced/draft model in UI contract and tests |
| `IMP-002` | implementation | duplicate `idempotency_key` path was conflict-oriented instead of replay-safe | retransmit instability and duplicate-write ambiguity | idempotency behavior not fully wired to existing event lookup | require deterministic idempotent retransmit semantics and tests |
| `IMP-003` | implementation | metadata delete response shape diverged from HTTP contract | API contract drift | handler contract mismatch | lock `DELETE /notes/{id}/metadata/{key}` to `204` in specs/tests |
| `IMP-004` | implementation | WS replay cursor handling was incomplete | reconnect gaps and stale client state | `ack` cursor accepted but replay behavior under-specified | expand WS replay contract and acceptance tests |
| `IMP-005` | implementation | attachment download continuity checks were missing initially | possible partial/corrupt stream delivery | stream integrity rules not fully enforced | keep strict chunk continuity/integrity rules in attachment contract |
| `USR-001` | user report | `GET /api/auth/session` `401` surfaced as error in browser console | confusing first-load experience | unauthenticated state not treated as expected pre-login condition | UI/session contract MUST treat pre-auth `401` as normal path |
| `USR-002` | user report | `crypto.randomUUID` unavailable in some client environments | save failure and blocked editing | compatibility assumption in idempotency key generator | require fallback idempotency key generation in UI contract |
| `USR-003` | user report | explicit request for autosave and richer editor behavior | low editing confidence and extra friction | editor interaction contract too minimal | add autosave, markdown-native editing, and responsive UX requirements |
| `USR-004` | user report | setup-like visual was shown even when setup was already locked | misleading auth flow and onboarding confusion | setup/login presentation state not separated by deterministic setup-lock signal | require login-only presentation after setup lock (`409`) and add E2E coverage |
| `USR-005` | user report | small screens gave insufficient note-editing space | editing friction on constrained displays | navigation/list regions were not collapsible in compact layouts | require menu-toggle collapse/restore behavior for constrained screens |
| `USR-006` | user report | dashboard/workspace/project-rail surfaces added unnecessary UI noise | reduced focus and cognitive overhead in core notes flow | non-essential modules were treated as required baseline UI | keep dashboards/workspace switcher optional and remove `Project rail` from baseline UX assumptions |
| `USR-007` | user report | note title changes were not consistently reflected in list-related areas | stale context and wrong navigation labels | rename propagation rules were under-specified | require same-cycle title propagation across list and related surfaces |
| `USR-008` | user report | version/save/delete controls in editor were unnecessary for default flow | cluttered editing surface | editor chrome requirements overexposed secondary controls | make version/save/delete controls optional and hidden by default |

## Spec Closure Mapping (Docs Phase)

| Finding | Canonical spec evidence |
|---|---|
| `USR-004` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) setup/login split and login-only presentation in setup-locked state |
| `USR-005` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) constrained-screen menu-toggle collapse/restore behavior |
| `USR-006` | [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) optional dashboards/workspace switcher and no baseline project-navigation pane |
| `USR-007` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md), [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) title-rename propagation requirements |
| `USR-008` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md), [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) minimal default editor chrome |

## Reconstruction Implications

- These findings MUST be reflected in canonical `spec/`, `reference/`, and `todo/` artifacts.
- Any future runtime reconstruction MUST include acceptance tests that directly cover `IMP-*` and `USR-*` finding families.

## Related

- Drift ledger: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Limitations ledger: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

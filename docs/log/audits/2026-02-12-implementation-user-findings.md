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
| `USR-001` | user report | `GET /api/v1/auth/session` `401` surfaced as error in browser console | confusing first-load experience | unauthenticated state not treated as expected pre-login condition | UI/session contract MUST treat pre-auth `401` as normal path |
| `USR-002` | user report | `crypto.randomUUID` unavailable in some client environments | save failure and blocked editing | compatibility assumption in idempotency key generator | require fallback idempotency key generation in UI contract |
| `USR-003` | user report | explicit request for autosave and richer editor behavior | low editing confidence and extra friction | editor interaction contract too minimal | add autosave, markdown-native editing, and responsive UX requirements |

## Reconstruction Implications

- These findings MUST be reflected in canonical `spec/`, `reference/`, and `todo/` artifacts.
- Any future runtime reconstruction MUST include acceptance tests that directly cover `IMP-*` and `USR-*` finding families.

## Related

- Drift ledger: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Limitations ledger: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

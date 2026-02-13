# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-level mismatch tracking for reconstruction.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates canonical spec |
| `M2 missing feature` | required capability is absent |
| `M3 undocumented behavior` | behavior exists but is not canonically specified |
| `M4 verification gap` | deterministic evidence is insufficient |
| `M5 stale docs` | docs and stronger evidence contradict |

## Matrix

| Req ID | Canonical Document | Requirement | Observed Status | Mismatch Class | Action |
|---|---|---|---|---|---|
| `R-RUNTIME-03` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup and supervision path exists | spec-only | `M2` | implement |
| `R-API-03` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API endpoints are reachable | spec-only | `M2` | implement |
| `R-WS-03` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WebSocket protocol is reachable | spec-only | `M2` | implement |
| `R-UI-04` | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | note-first UI contract is user-reachable | spec-only | `M2` | implement |
| `R-RBAC-02` | [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) | role-based authorization is enforced | spec-only | `M2` | implement |
| `R-AUTO-02` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | deterministic automation rules/runs exist | spec-only | `M2` | implement |
| `R-LIB-02` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | autonomous librarian structuring runs are reachable and auditable | spec-only | `M2` | implement |
| `R-MEDIA-03` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | standalone media note uploads are supported | spec-only | `M2` | implement |
| `R-SEARCH-03` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | full-text and backlink search exists | spec-only | `M2` | implement |
| `R-ISSUE-03` | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | all `IMP-*`/`USR-*` findings are regression-guarded in runtime | partial | `M4` | test-add |
| `R-UX-03` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) | consolidated UX requirements are runtime-verified | partial | `M4` | implement + test-add |
| `R-PERF-03` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance evidence is current and archived | unverified | `M4` | test-add |
| `R-OPS-03` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup/restore/restart evidence is current and archived | unverified | `M4` | test-add |
| `R-DOC-SYNC-03` | [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) | reference ledgers accurately reflect docs-only reset state | aligned | closed | keep synchronized |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 9 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 4 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Reset audit: [/docs/log/audits/2026-02-13-reconstruction-reset-sync.md](/docs/log/audits/2026-02-13-reconstruction-reset-sync.md)

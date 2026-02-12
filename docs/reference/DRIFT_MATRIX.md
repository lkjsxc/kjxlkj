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
| `R-RUNTIME-02` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup and supervision path exists | partial | `M4` | test-add |
| `R-API-02` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API endpoints are reachable | partial | `M4` | test-add |
| `R-WS-02` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WebSocket protocol is reachable | partial | `M4` | test-add |
| `R-UI-03` | [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) | workspace modules are user-reachable in one responsive tree | partial | `M4` | test-add |
| `R-RBAC-01` | [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) | role-based authorization is enforced | partial | `M4` | test-add |
| `R-AUTO-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | deterministic automation rules and runs exist | partial | `M4` | test-add |
| `R-SEARCH-02` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | full-text and backlink search exists | partial | `M4` | test-add |
| `R-MEDIA-02` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | standalone media note uploads are supported | partial | `M4` | test-add |
| `R-ISSUE-02` | [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md) | historical findings are covered by regression tests | partial | `M4` | test-add |
| `R-PERF-02` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance evidence is archived | spec-only | `M4` | test-add |
| `R-OPS-02` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup/restore/restart recovery evidence is archived | spec-only | `M4` | test-add |
| `R-DOC-PIVOT-02` | [/docs/spec/README.md](/docs/spec/README.md) | hard-pivot docs are canonical | aligned | closed | keep synchronized |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 0 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 11 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

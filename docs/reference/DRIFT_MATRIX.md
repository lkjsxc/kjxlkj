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
| `R-RUNTIME-02` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup and supervision path exists | spec-only | `M2` | implement |
| `R-API-02` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API endpoints are reachable | spec-only | `M2` | implement |
| `R-WS-02` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WebSocket protocol is reachable | spec-only | `M2` | implement |
| `R-UI-03` | [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) | focused notes modules are user-reachable in one responsive tree | spec-only | `M2` | implement |
| `R-UI-LOCK-01` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | setup-locked state must render login-only UI with no setup-like appearance | aligned | closed | keep synchronized |
| `R-UI-SMALL-01` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | constrained screens must provide menu-toggle collapse/restore to expand editor area | aligned | closed | keep synchronized |
| `R-UI-MODULES-01` | [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) | dashboards/workspace switcher are optional and project-navigation pane is not baseline | aligned | closed | keep synchronized |
| `R-UI-RENAME-01` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | note title rename must propagate immediately to list/navigation surfaces | aligned | closed | keep synchronized |
| `R-UI-CHROME-01` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | default editor chrome omits required inline version/save/delete controls | aligned | closed | keep synchronized |
| `R-RBAC-01` | [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) | role-based authorization is enforced | spec-only | `M2` | implement |
| `R-AUTO-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | deterministic automation rules and runs exist | spec-only | `M2` | implement |
| `R-LIB-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | autonomous librarian structuring runs are reachable and auditable | spec-only | `M2` | implement |
| `R-LIB-PROTO-01` | [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md) | attribute-less XML-like protocol parsing and validation are enforced | spec-only | `M2` | implement |
| `R-SEARCH-02` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | full-text and backlink search exists | spec-only | `M2` | implement |
| `R-MEDIA-02` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | standalone media note uploads are supported | spec-only | `M2` | implement |
| `R-ISSUE-02` | [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md) | historical findings are covered by regression tests | partial | `M4` | test-add |
| `R-LIB-GUARD-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | librarian parser/retry and operation safety regressions are covered by tests | spec-only | `M4` | test-add |
| `R-PERF-02` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance evidence is archived | spec-only | `M4` | test-add |
| `R-OPS-02` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup/restore/restart recovery evidence is archived | spec-only | `M4` | test-add |
| `R-DOC-PIVOT-02` | [/docs/spec/README.md](/docs/spec/README.md) | hard-pivot docs are canonical | aligned | closed | keep synchronized |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 10 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 4 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

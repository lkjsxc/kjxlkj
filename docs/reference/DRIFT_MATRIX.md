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
| `R-RUNTIME-02` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup and supervision path exists | partial | `M2` | extend to full single-service supervision and worker topology |
| `R-API-02` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API endpoints are reachable | partial | `M2` | extend implemented route set from Stage 01-04 baseline (`notes/views/automation/admin-jobs`) to full canonical surface |
| `R-WS-02` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WebSocket protocol is reachable | partial | `M2` | extend Stage 02/04 note/workspace + automation replay baseline to full canonical message families |
| `R-UI-03` | [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) | focused notes modules are user-reachable in one responsive tree | partial | `M2` | harden shell behaviors and broaden E2E coverage to close remaining UX depth gaps |
| `R-UI-LOCK-01` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | setup-locked state must render login-only UI with no setup-like appearance | aligned | closed | keep synchronized |
| `R-UI-SMALL-01` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | constrained screens must provide menu-toggle collapse/restore to expand editor area | aligned | closed | keep synchronized |
| `R-UI-MODULES-01` | [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) | dashboards/workspace switcher are optional and project-navigation pane is not baseline | aligned | closed | keep synchronized |
| `R-UI-RENAME-01` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | note title rename must propagate immediately to list/navigation surfaces | aligned | closed | keep synchronized |
| `R-UI-CHROME-01` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | default editor chrome omits required inline version/save/delete controls | aligned | closed | keep synchronized |
| `R-RBAC-01` | [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) | role-based authorization is enforced | partial | `M2` | expand authz enforcement to all remaining mutation routes |
| `R-AUTO-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | deterministic automation rules and runs exist | partial | `M2` | extend Stage 04 rule/run baseline with Stage 06 parser/apply execution and broader trigger/action coverage |
| `R-LIB-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | autonomous librarian structuring runs are reachable and auditable | partial | `M2` | complete operation-apply execution and audit-stream closure beyond provider+payload-contract+xml parser baseline |
| `R-LIB-PROTO-01` | [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md) | attribute-less XML-like protocol parsing and validation are enforced | aligned | closed | keep synchronized |
| `R-SEARCH-02` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | full-text and backlink search exists | partial | `M2` | harden ranking/indexing behavior and broaden acceptance coverage |
| `R-MEDIA-02` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | standalone media note uploads are supported | spec-only | `M2` | implement |
| `R-ISSUE-02` | [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md) | historical findings are covered by regression tests | aligned | closed | keep synchronized and extend as new finding families land |
| `R-LIB-GUARD-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | librarian parser/retry and operation safety regressions are covered by tests | partial | `M4` | extend deterministic coverage from parser-retry boundaries to operation-apply + WS/E2E safety suites |
| `R-PERF-02` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance evidence is archived | partial | `M4` | expand perf smoke baseline to full target-scale envelope and archived resource telemetry |
| `R-OPS-02` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup/restore/restart recovery evidence is archived | partial | `M4` | expand job baseline evidence to include restore and restart-recovery drills |
| `R-DOC-PIVOT-02` | [/docs/spec/README.md](/docs/spec/README.md) | hard-pivot docs are canonical | aligned | closed | keep synchronized |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 9 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 3 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

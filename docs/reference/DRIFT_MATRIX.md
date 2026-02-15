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
| `R-DOC-PIVOT-02` | [/docs/spec/README.md](/docs/spec/README.md) | documentation is canonical product contract | aligned | closed | keep synchronized |
| `R-TODO-LINK-01` | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | every TODO checkbox item links directly to governing docs | aligned | closed | keep synchronized |
| `R-FILEMAP-01` | [/docs/spec/architecture/completion-file-map.md](/docs/spec/architecture/completion-file-map.md) | final completion file structure is fully specified | aligned | closed | keep synchronized |
| `R-CONFIG-01` | [/docs/spec/architecture/configuration.md](/docs/spec/architecture/configuration.md) | all non-secret runtime configuration lives in `data/config.json` | aligned | closed | keep synchronized |
| `R-SECRET-01` | [/docs/spec/architecture/configuration.md](/docs/spec/architecture/configuration.md) | secrets are sourced from `.env` only | aligned | closed | keep synchronized |
| `R-RUNTIME-02` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup and supervision path exists | partial | `M4` | run integration tests against live database |
| `R-API-02` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API endpoints are reachable | partial | `M4` | run acceptance tests for API-AUTH-* and CRUD routes |
| `R-WS-02` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WebSocket protocol is reachable | partial | `M4` | test WS connection and heartbeat cycle |
| `R-UI-SMALL-01` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | small-screen menu is top-right and closes on note select | spec-only | `M2` | rebuild responsive frontend behavior |
| `R-UI-CREATE-01` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | Create New Note creates and selects the new note | spec-only | `M2` | rebuild notes layout create/select flow |
| `R-TEST-NEWNOTE-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | deterministic test verifies Create New Note adds a note | spec-only | `M4` | implement test during frontend rebuild |
| `R-PERF-02` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance evidence is archived | partial | `M4` | execute PERF profiles and archive telemetry |
| `R-OPS-02` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup/restore/restart recovery evidence is archived | partial | `M4` | execute restore drill and record parity proof |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 2 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 3 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

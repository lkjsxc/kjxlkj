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
| `R-WS-02` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WebSocket protocol is reachable | partial | `M4` | test WS connection, heartbeat, subscribe/ack and apply-patch flow |
| `R-NOTE-LIFECYCLE-01` | [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md) | notes CRUD + rollback | partial | `M4` | run acceptance tests for note create/list/get/patch/delete/rollback |
| `R-METADATA-01` | [/docs/spec/domain/metadata.md](/docs/spec/domain/metadata.md) | metadata upsert/delete/list and tags | partial | `M4` | run acceptance tests for metadata and tag endpoints |
| `R-SEARCH-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | FTS search and backlink extraction | partial | `M4` | run acceptance tests for search and backlinks |
| `R-ATTACH-01` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | chunked attachment upload/download/delete | partial | `M4` | run acceptance tests for attachment lifecycle |
| `R-IDEMP-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS idempotency keys and snapshot store | partial | `M4` | verify idempotency dedup and snapshot every-100 behavior |
| `R-UI-SMALL-01` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | small-screen menu is top-right and closes on note select | partial | `M4` | run visual test at 320px and 1024px breakpoints |
| `R-UI-CREATE-01` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | Create New Note creates and selects the new note | partial | `M4` | run E2E-23 acceptance test |
| `R-TEST-NEWNOTE-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | deterministic test verifies Create New Note adds a note | spec-only | `M4` | implement test during frontend rebuild |
| `R-PERF-02` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance evidence is archived | partial | `M4` | execute PERF profiles and archive telemetry |
| `R-OPS-02` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup/restore/restart recovery evidence is archived | partial | `M4` | execute restore drill and record parity proof |
| `R-AUTO-RULE-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | automation rule CRUD with provider validation | partial | `M4` | run acceptance tests for rule CRUD |
| `R-AUTO-RUN-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | run state machine with idempotency | partial | `M4` | run acceptance tests for run lifecycle |
| `R-EXPORT-01` | [/docs/spec/domain/export.md](/docs/spec/domain/export.md) | export/backup job lifecycle | partial | `M4` | run acceptance tests for export jobs |
| `R-LIBRARIAN-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | librarian report and operation audit | partial | `M4` | run acceptance tests for librarian pipeline |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 0 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 14 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

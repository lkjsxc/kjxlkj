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
| `R-CSRF-01` | [/docs/spec/security/csrf.md](/docs/spec/security/csrf.md) | CSRF token validation on mutating requests | partial | `M4` | verify CSRF rejection with live requests |
| `R-TRANSPORT-01` | [/docs/spec/security/transport.md](/docs/spec/security/transport.md) | security response headers on all responses | partial | `M4` | verify response headers with live requests |
| `R-SESSION-01` | [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md) | session cookie and expiry semantics | partial | `M4` | verify session lifecycle with live requests |
| `R-REGRESSION-01` | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | regression test stubs for all findings | partial | `M4` | execute regression pack against live runtime |
| `R-ACCEPTANCE-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | acceptance test stubs for all API/WS/OPS/PERF areas | partial | `M4` | execute acceptance pack against live runtime |
| `R-VIEWS-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | saved views CRUD routes | partial | `M4` | run acceptance tests for views |
| `R-DASHBOARDS-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | dashboard widget routes (optional extension) | partial | `M4` | run acceptance tests for dashboards |
| `R-MEDIA-NOTE-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | media note creation route | partial | `M4` | run acceptance tests for media notes |
| `R-PROVIDER-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | provider adapter with fallback chain | partial | `M4` | test against live LLM endpoint |
| `R-PROMPT-PACK-01` | [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md) | JSON prompt loading and validation | partial | `M4` | test pack load with canonical prompts |
| `R-XML-PARSER-01` | [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md) | xml_attrless parser with tag validation | partial | `M4` | test against representative payloads |
| `R-PIPELINE-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | librarian run pipeline with bounded retry | partial | `M4` | end-to-end pipeline test |
| `R-WS-EVENT-FAMILY-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | workspace and automation event surface families | partial | `M4` | test workspace stream event families with live WS |
| `R-AUTOMATION-EVENT-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | automation lifecycle events in workspace stream | partial | `M4` | test automation event emission and replay |
| `R-WS-REPLAY-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | monotonic cursors and stale-cursor error | partial | `M4` | test cursor replay and stale-cursor rejection |
| `R-WS-ERROR-ENVELOPE-01` | [/docs/spec/api/errors.md](/docs/spec/api/errors.md) | WS error envelope with details field | partial | `M4` | test WS error payloads |
| `R-WS-ACCEPTANCE-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | WS-02/03/06 acceptance stubs | partial | `M4` | execute WS acceptance pack against live runtime |
| `R-FRONTEND-AUTO-API-01` | [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) | typed frontend automation API client | partial | `M4` | test API calls against live backend |
| `R-LIBRARIAN-UX-01` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) | librarian review per-operation accept/reject UX | partial | `M4` | test review component with live runs |
| `R-CSRF-CLIENT-01` | [/docs/spec/security/csrf.md](/docs/spec/security/csrf.md) | CSRF token sent on mutating frontend requests | partial | `M4` | verify token lifecycle with live requests |
| `R-DOCKER-DEPLOY-01` | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | single-container Docker deployment | partial | `M4` | build and test Docker image end-to-end |
| `R-TYPE-SAFETY-01` | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | strict TypeScript compilation with no any in domain | partial | `M4` | verify tsc --noEmit passes in CI |
| `R-ENTRYPOINT-01` | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | process supervisor with graceful shutdown | partial | `M4` | test entrypoint signal handling live |
| `R-CI-WORKFLOW-01` | [/docs/reference/CI.md](/docs/reference/CI.md) | CI workflow with 4 profile jobs | partial | `M4` | run in GitHub Actions |
| `R-FILE-STRUCTURE-01` | [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | repository tree matches spec | aligned | closed | keep synchronized |
| `R-TYPE-GATE-01` | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | backend + frontend type gates pass | aligned | closed | keep synchronized |
| `R-POOL-TUNE-01` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | DB pool sizing and statement_timeout | partial | `M4` | test pool under load with timeout policy |
| `R-BROADCAST-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | cross-actor WS broadcast registry | partial | `M4` | test broadcast with live WS sessions |
| `R-BACKUP-DRILL-01` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | automated backup/restore parity drill | partial | `M4` | execute drill against live PostgreSQL |
| `R-INTEGRATION-HARNESS-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | DB-backed integration test harness | partial | `M4` | run harness with DATABASE_URL |
| `R-CSS-MODULE-01` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | CSS module modularization | partial | `M4` | verify CSS isolation in browser |
| `R-LAZY-LOAD-01` | [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) | React.lazy code splitting | partial | `M4` | verify bundle split in production build |
| `R-CSP-01` | [/docs/spec/security/transport.md](/docs/spec/security/transport.md) | Content-Security-Policy header | partial | `M4` | verify CSP in live response headers |
| `R-RATE-LIMIT-01` | [/docs/spec/security/auth.md](/docs/spec/security/auth.md) | auth endpoint rate limiting | partial | `M4` | verify 429 responses under burst |
| `R-SESSION-REVOKE-01` | [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md) | session revocation broadcast on password change | partial | `M4` | test revoke_user_sessions with live sessions |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 0 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 49 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

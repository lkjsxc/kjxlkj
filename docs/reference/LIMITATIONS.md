# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-15)

- All in Docs governance is active.
- Stage 00 governance baseline complete.
- Stage 01 crate skeleton scaffolded and compiling.
- Stage 02 notes lifecycle, realtime WS, metadata, search, attachments compiled.
- Stage 03 frontend web app shell compiled.
- Stage 04 automation, export, librarian DB and HTTP routes compiled.
- Stage 05 CSRF and security headers middleware compiled.
- Stage 05 regression (14) and acceptance (24) test stubs compiled and passing.
- Stage 06 REST surface parity, provider adapters, prompt loader, xml parser, pipeline compiled.
- Stage 07 WS event surfaces, automation event classification, replay/cursor guarantees compiled.
- Stage 07 error envelope aligned with details field per errors.md spec.
- Stage 07 acceptance stubs: WS-02, WS-03, WS-06, API-AUTO-04, API-VIEW-01, API-DASH-01, API-PROJ-01 (31 total).
- Stage 08 frontend automation API, librarian review UX, CSRF client-side, Docker deployment compiled.
- Stage 08 TypeScript strict compilation clean. All frontend files ≤ 200 lines.
- TODO checklists for Stages 00–08 are fully checked.
- Improvement backlog is canonicalized in `IMPROVEMENT_BACKLOG.md`.

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-RUNTIME-REBUILD-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime skeleton compiled but not yet tested against live DB | `M4 verification gap` | medium | run integration tests with PostgreSQL |
| `LIM-API-REBUILD-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | HTTP route handlers compiled but not acceptance-tested | `M4 verification gap` | medium | run API acceptance tests |
| `LIM-UI-REBUILD-01` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | web app shell built; responsive layout pending visual verification | `M4 verification gap` | medium | run visual and E2E acceptance tests |
| `LIM-TEST-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | deterministic acceptance and integration evidence is absent in reset state | `M4 verification gap` | high | rebuild tests and run mandatory acceptance pack |
| `LIM-PERF-01` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance benchmarks not yet executed at target scale | `M4 verification gap` | medium | run PERF-01/02/03 and archive telemetry |
| `LIM-OPS-RESTORE-01` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup restore drill not yet executed | `M4 verification gap` | medium | execute restore drill and record parity proof |
| `LIM-SEARCH-FTS-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | search is PostgreSQL FTS only; no vector/semantic search | `M4 verification gap` | low | evaluate vector extension if needed post-release |
| `LIM-ATTACH-STREAM-01` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | attachment download streams from DB chunks, not object store | `M4 verification gap` | low | consider S3/object-store backend for large files |
| `LIM-WS-UNSAFE-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS actor uses unsafe raw pointer for &mut SubscriptionState across async boundary | `M3 undocumented behavior` | medium | refactor to safe actor message pattern |
| `LIM-AUTO-REBUILD-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | automation rule/run and librarian pipeline compiled but not acceptance-tested | `M4 verification gap` | medium | run automation acceptance tests |
| `LIM-EXPORT-REBUILD-01` | [/docs/spec/domain/export.md](/docs/spec/domain/export.md) | export/backup jobs compiled but not acceptance-tested | `M4 verification gap` | medium | run export acceptance tests |
| `LIM-CSRF-01` | [/docs/spec/security/csrf.md](/docs/spec/security/csrf.md) | CSRF middleware compiled but not tested against live mutating requests | `M4 verification gap` | medium | verify CSRF rejection with live requests |
| `LIM-TRANSPORT-01` | [/docs/spec/security/transport.md](/docs/spec/security/transport.md) | security response headers middleware compiled but not verified live | `M4 verification gap` | low | verify response headers with live requests |
| `LIM-REGRESSION-01` | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | regression stubs are structural placeholders; need assertion bodies | `M4 verification gap` | medium | fill assertion bodies during integration testing |
| `LIM-PROVIDER-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | provider adapters compiled but not tested against live LLM endpoints | `M4 verification gap` | medium | test with openrouter and lmstudio endpoints |
| `LIM-XML-PARSER-01` | [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md) | xml_attrless parser compiled but not tested against representative payloads | `M4 verification gap` | medium | add unit tests with sample librarian response payloads |
| `LIM-WS-AUTOMATION-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | automation event classification compiled but not tested with live WS stream | `M4 verification gap` | medium | test automation events via live WS connection |
| `LIM-DOCKER-01` | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | Dockerfile and entrypoint created but not built/tested live | `M4 verification gap` | medium | build Docker image and run container end-to-end |
| `LIM-LIBRARIAN-UX-01` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) | librarian review component exists but not tested with live runs | `M4 verification gap` | medium | test review UX with live automation runs |
| `LIM-FRONTEND-AUTO-API-01` | [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) | typed automation API client compiled but not tested against live backend | `M4 verification gap` | medium | test API calls against running server |

## Closure Rules

A limitation closes only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)

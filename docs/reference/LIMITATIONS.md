# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-15)

- All in Docs governance is active.
- Stage 00 governance baseline complete.
- Stage 01 crate skeleton scaffolded and compiling.
- Stage 02 notes lifecycle, realtime WS, metadata, search, attachments compiled.
- TODO checklists for Stages 00–02 are fully checked.
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

## Closure Rules

A limitation closes only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)

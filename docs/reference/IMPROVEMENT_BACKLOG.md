# Improvement Backlog

Back: [/docs/reference/README.md](/docs/reference/README.md)

Canonical backlog for full reconstruction from docs-only baseline.

## Governance

- This is the only durable source for improvement ideas.
- Entries below were harvested from the prior reconstruction logs.
- `docs/logs/` is non-canonical and MUST remain deleted.
- Each row MUST map to a TODO wave before implementation starts.

## Priority Focus

1. Communication-layer correctness and determinism.
2. Full `T0/T1/T2` test quality and evidence discipline.
3. Runtime regeneration from docs with no hidden assumptions.

## Backlog Matrix

| Backlog ID | Improvement | Canonical Docs | Proposed Wave | Status |
|---|---|---|---|---|
| `IMP-COMM-01` | Implement HTTP request-id propagation and deterministic error envelopes | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `S06/W060` | `planned` |
| `IMP-COMM-02` | Implement WebSocket replay semantics with explicit stale cursor contract | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `S07/W071` | `planned` |
| `IMP-COMM-03` | Implement full idempotency repository semantics (scope, TTL, deterministic replay) | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `S07/W072` | `planned` |
| `IMP-COMM-04` | Implement endpoint-level auth/csrf enforcement table exactly as specified | [/docs/spec/security/README.md](/docs/spec/security/README.md) | `S05/W051` | `planned` |
| `IMP-COMM-05` | Add deterministic 429 behavior with Retry-After on constrained routes | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `S05/W052` | `planned` |
| `IMP-DB-01` | Regenerate migrations from migration spec and revalidate ordering | [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md) | `S01/W012` | `planned` |
| `IMP-DB-02` | Replace in-memory stores with PostgreSQL-backed repositories | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `S03/W030` | `planned` |
| `IMP-SEARCH-01` | Implement semantic retrieval + lexical fallback diagnostics | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | `S02/W022` | `planned` |
| `IMP-AGENT-01` | Implement prompt JSON validation + strict agent mode handling | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `S04/W041` | `planned` |
| `IMP-AGENT-02` | Implement KV memory durability and mutation API | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `S04/W042` | `planned` |
| `IMP-TEST-01` | Build mandatory full `T0/T1/T2` matrix with communication dominance | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `S09/W091` | `planned` |
| `IMP-TEST-02` | Add fuzz/chaos communication tests (malformed WS, retry storms, reconnect churn) | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `S10/W101` | `planned` |
| `IMP-STRUCT-01` | Enforce <=200 line source files during rebuild | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | `S10/W100` | `planned` |
| `IMP-OPS-01` | Add structured tracing, metrics, and graceful shutdown behavior | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | `S10/W102` | `planned` |

## Imported Historical Notes

The following legacy notes were absorbed from deleted implementation logs and re-scoped as backlog inputs:

- in-memory repositories were used as temporary baseline
- embedding provider integration was partial and required degradation semantics
- session/cookie behavior needed stricter security treatment
- websocket reconnect/idempotency needed stronger deterministic guarantees
- large-file split targets required systematic enforcement

## Related

- TODO execution plan: [/docs/todo/README.md](/docs/todo/README.md)
- Drift ledger: [DRIFT_MATRIX.md](DRIFT_MATRIX.md)
- Test matrix: [TEST_MATRIX.md](TEST_MATRIX.md)

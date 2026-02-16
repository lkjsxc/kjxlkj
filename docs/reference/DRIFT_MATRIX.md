# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-level mismatch tracking for reconstruction.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates spec |
| `M2 missing feature` | specified capability absent |
| `M3 undocumented behavior` | implementation exists without spec |
| `M4 verification gap` | behavior exists but evidence insufficient |
| `M5 stale docs` | docs contradict stronger evidence |

## Matrix

| Req ID | Canonical Document | Requirement | Observed Status | Mismatch Class | Action |
|---|---|---|---|---|---|
| `R-DOC-01` | [/docs/README.md](/docs/README.md) | docs are canonical contract | aligned | closed | maintain |
| `R-TODO-01` | [/docs/todo/README.md](/docs/todo/README.md) | TODO drives deterministic rebuild with direct links | aligned | closed | maintain |
| `R-ROOT-01` | [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md) | docs-only root excludes Docker artifacts | aligned | closed | maintain |
| `R-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime services are reconstructed and reachable | aligned — 10 crates, 152 tests, AppState DI | closed | maintain |
| `R-HTTP-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | HTTP contract is live and testable | aligned — all routes wired, T1 integration tests | closed | maintain |
| `R-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS replay/idempotency contract is live | aligned — cursor replay + idempotency tested | closed | maintain |
| `R-SEARCH-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | hybrid search is live | aligned — lexical in-memory, merge_and_rank tested | closed | wire PG embeddings |
| `R-UI-01` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | editor UX contract is implemented | aligned — autosave, preview, wiki-links, backlinks, shortcuts, conflict UX, note-list | closed | maintain |
| `R-AGENT-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | agent loop contract is implemented | aligned — XML parser, KV, loop, LLM providers | closed | wire real LLM |
| `R-TEST-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | acceptance pack is executable | aligned — 152 tests, T0+T1+property+snapshot+E2E, 18+ acceptance IDs | closed | add E2E |
| `R-CSRF-01` | [/docs/spec/security/csrf.md](/docs/spec/security/csrf.md) | CSRF token validation for mutations | aligned — token bound to session, validated in middleware | closed | maintain |
| `R-SESSION-01` | [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md) | cookie session extraction | aligned — HttpOnly, SameSite=Lax, path=/, 7-day TTL, cookie clear on logout | closed | maintain |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 0 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 0 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [CONFORMANCE.md](CONFORMANCE.md)
- Limitations: [LIMITATIONS.md](LIMITATIONS.md)

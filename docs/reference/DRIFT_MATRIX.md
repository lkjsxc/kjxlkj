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
| `R-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime services are reconstructed and reachable | partial — 10 crates compile, stubs | `M2` | wire DB |
| `R-HTTP-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | HTTP contract is live and testable | partial — routes defined, stub handlers | `M2` | wire DB |
| `R-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS replay/idempotency contract is live | partial — types+handler stub | `M2` | implement replay |
| `R-SEARCH-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | hybrid search is live | partial — merge tested, no DB | `M2` | wire DB |
| `R-UI-01` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | editor UX contract is implemented | partial — TS shell scaffold | `M2` | rich editor |
| `R-AGENT-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | agent loop contract is implemented | partial — XML parser, KV, loop stub | `M2` | wire provider |
| `R-TEST-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | acceptance pack is executable | partial — 41 unit tests pass | `M4` | add integration |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 6 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 1 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [CONFORMANCE.md](CONFORMANCE.md)
- Limitations: [LIMITATIONS.md](LIMITATIONS.md)

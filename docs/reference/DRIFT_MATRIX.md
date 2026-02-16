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
| `R-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime services are reconstructed and reachable | spec-only | `M2` | implement |
| `R-HTTP-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | HTTP contract is live and testable | spec-only | `M2` | implement |
| `R-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS replay/idempotency contract is live | spec-only | `M2` | implement |
| `R-SEARCH-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | hybrid search is live | spec-only | `M2` | implement |
| `R-UI-01` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | editor UX contract is implemented | spec-only | `M2` | implement |
| `R-AGENT-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | agent loop contract is implemented | spec-only | `M2` | implement |
| `R-TEST-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | acceptance pack is executable | spec-only | `M4` | test-add |

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

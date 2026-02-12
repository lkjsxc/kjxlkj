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
| `R-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup and supervision path exists | aligned | closed | maintain coverage |
| `R-API-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API v1 endpoints are reachable | partial | `M4` | add full contract suite |
| `R-API-02` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | metadata delete returns `204` | aligned | closed | maintain |
| `R-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS conflict response returns expected/current version fields | aligned | closed | maintain |
| `R-WS-02` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | replay/recovery on reconnect is complete | partial | `M4` | add deterministic WS replay/reconnect suite |
| `R-EVENT-01` | [/docs/spec/domain/events.md](/docs/spec/domain/events.md) | event append + projection update are transactional | partial | `M4` | run DB integration evidence |
| `R-EVENT-02` | [/docs/spec/domain/events.md](/docs/spec/domain/events.md) | snapshot every 100 events | aligned | closed | maintain + test in DB profile |
| `R-ATT-01` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | chunk continuity and integrity checks on download | aligned | closed | maintain |
| `R-AUTH-01` | [/docs/spec/security/auth.md](/docs/spec/security/auth.md) | setup/login/session/csrf and auth rate limiting | partial | `M4` | expand abuse-path tests |
| `R-OPENAPI-01` | [/docs/spec/api/openapi.md](/docs/spec/api/openapi.md) | canonical OpenAPI doc validates in CI | partial | `M4` | add schema validation command/profile |
| `R-DOC-PIVOT-01` | [/docs/spec/README.md](/docs/spec/README.md) | web-server docs are canonical | aligned | closed | keep synchronized |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 0 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 5 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

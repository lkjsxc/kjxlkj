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
| `R-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup and supervision path exists | missing | `M2` | implement + test |
| `R-API-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API v1 endpoints are reachable | missing | `M2` | implement + test |
| `R-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS patch protocol is reachable | missing | `M2` | implement + test |
| `R-EVENT-01` | [/docs/spec/domain/events.md](/docs/spec/domain/events.md) | event append + projection update are transactional | missing | `M2` | implement + test |
| `R-ATT-01` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | 500MB chunked attachment path exists | missing | `M2` | implement + test |
| `R-AUTH-01` | [/docs/spec/security/auth.md](/docs/spec/security/auth.md) | first-run setup and session auth exist | missing | `M2` | implement + test |
| `R-OPENAPI-01` | [/docs/spec/api/openapi.md](/docs/spec/api/openapi.md) | canonical OpenAPI doc validates in CI | partial | `M4` | validate in CI |
| `R-DOC-PIVOT-01` | [/docs/spec/README.md](/docs/spec/README.md) | web-server docs are canonical | aligned | closed | keep synchronized |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 6 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 1 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

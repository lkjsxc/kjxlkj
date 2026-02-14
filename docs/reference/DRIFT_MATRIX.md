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
| `R-RUNTIME-05` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup and supervision path exists and is verified | spec-only | `M2` | implement |
| `R-API-05` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API endpoints are reachable and verified | spec-only | `M2` | implement |
| `R-WS-05` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WebSocket protocol is reachable and verified | spec-only | `M2` | implement |
| `R-UI-06` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | typed web app runtime is user-reachable | spec-only | `M2` | implement |
| `R-TYPE-03` | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | backend compile gate and frontend strict type gate pass | spec-only | `M2` | implement |
| `R-TYPE-04` | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | no direct JavaScript runtime source exists | aligned | closed | keep synchronized |
| `R-TODO-LINK-01` | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | every TODO checklist row links to relevant docs | aligned | closed | keep synchronized |
| `R-TODO-RESET-01` | [/docs/todo/README.md](/docs/todo/README.md) | TODO checkboxes are reset for fresh reconstruction | aligned | closed | keep synchronized |
| `R-WEB-TODO-01` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | web app docs are directly linked from Stage 08 TODOs | aligned | closed | keep synchronized |
| `R-FS-01` | [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | final completion file structure is fully documented | aligned | closed | keep synchronized |
| `R-PERF-05` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance evidence is current and archived | unverified | `M4` | test-add |
| `R-OPS-05` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | operations evidence is current and archived | unverified | `M4` | test-add |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 5 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 2 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

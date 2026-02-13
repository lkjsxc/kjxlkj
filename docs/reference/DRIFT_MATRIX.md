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
| `R-RUNTIME-04` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup and supervision path exists and is verified | spec-only | `M2` | implement |
| `R-API-04` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API endpoints are reachable and verified | spec-only | `M2` | implement |
| `R-WS-04` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WebSocket protocol is reachable and verified | spec-only | `M2` | implement |
| `R-UI-05` | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | note-first UI contract is user-reachable | spec-only | `M2` | implement |
| `R-TYPE-01` | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | backend compile gate and frontend strict type gate pass | spec-only | `M2` | implement |
| `R-TYPE-02` | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | no direct JavaScript runtime source exists | aligned | closed | keep synchronized |
| `R-ISSUE-04` | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | all `IMP-*`/`USR-*` findings are regression-guarded | partial | `M4` | test-add |
| `R-UX-04` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) | consolidated UX requirements are runtime-verified | partial | `M4` | implement + test-add |
| `R-PERF-04` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance evidence is current and archived | unverified | `M4` | test-add |
| `R-OPS-04` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | operations evidence is current and archived | unverified | `M4` | test-add |
| `R-DOC-SYNC-04` | [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) | ledgers reflect reset baseline truthfully | aligned | closed | keep synchronized |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 5 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 4 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

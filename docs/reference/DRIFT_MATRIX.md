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
| `R-DOCS-STATE-01` | [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md) | docs-only canonical root layout is valid and enforced | aligned | closed | keep synchronized |
| `R-TODO-LINK-01` | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | every TODO checklist row links to relevant docs | aligned | closed | keep synchronized |
| `R-PROMPT-JSON-01` | [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md) | every librarian-cycle prompt is configurable via JSON files | aligned | closed | keep synchronized |
| `R-UX-LAYOUT-01` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | desktop split and compact top-left menu behavior are explicitly specified | aligned | closed | keep synchronized |
| `R-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup and supervision path exists and is verified | spec-only | `M2` | implement |
| `R-API-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API endpoints are reachable and verified | spec-only | `M2` | implement |
| `R-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WebSocket protocol is reachable and verified | spec-only | `M2` | implement |
| `R-UI-01` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | typed web app runtime is user-reachable | spec-only | `M2` | implement |
| `R-AUTO-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | librarian automation cycle is executable and verified | spec-only | `M2` | implement |
| `R-DEPLOY-01` | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | deployment artifacts are present and validated in reconstruction mode | spec-only | `M2` | implement |
| `R-TEST-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | mandatory acceptance pack is executed with deterministic evidence | unverified | `M4` | test-add |
| `R-TYPE-01` | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | type gates pass in reconstructed runtime | spec-only | `M2` | implement |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 7 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 1 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

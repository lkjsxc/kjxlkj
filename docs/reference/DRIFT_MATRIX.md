# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-level mismatch tracking for rebuild.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates canonical spec |
| `M2 missing feature` | required capability is absent |
| `M3 undocumented behavior` | behavior exists but is not canonically specified |
| `M4 verification gap` | behavior exists but deterministic evidence is missing |
| `M5 stale docs` | docs and stronger evidence contradict |

## Matrix

| Req ID | Canonical Document | Requirement Statement | Code Path(s) | Test Path(s) | Observed Status | Mismatch Class | Action | Verification Evidence |
|---|---|---|---|---|---|---|---|---|
| `R-DOC-ISSUE-01` | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | implementation/user findings are canonically incorporated | n/a (docs baseline) | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `aligned` | `n/a` | `spec-update` | findings map includes `IMP-001..005`, `USR-001..008` |
| `R-DEPLOY-01` | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | single-container Docker Compose contract is defined | n/a (docs baseline) | [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md) | `aligned` | `n/a` | `spec-update` | architecture + guide include template and startup checks |
| `R-DEPLOY-02` | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | runnable `docker compose up --build` path exists | pending reconstruction | `OPS-01`, `E2E-01` | `spec-only` | `M2 missing feature` | `implement` | no runnable artifacts in docs-only baseline |
| `R-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime topology is implemented and reachable | pending reconstruction | `Core-runtime` | `spec-only` | `M2 missing feature` | `implement` | implementation tree intentionally absent |
| `R-API-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | HTTP surface is user-reachable | pending reconstruction | `API-*` | `spec-only` | `M2 missing feature` | `implement` | no current API runtime |
| `R-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS replay/idempotency contract is user-reachable | pending reconstruction | `WS-01..06` | `spec-only` | `M2 missing feature` | `implement` | no current WS runtime |
| `R-UI-01` | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | UX shell/editor flows are user-reachable | pending reconstruction | `E2E-03`, `E2E-06..15` | `spec-only` | `M2 missing feature` | `implement` | no current frontend runtime |
| `R-LIB-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | librarian provider/parser/review flows are reachable | pending reconstruction | `API-AUTO-03`, `API-AUTO-04`, `WS-06`, `E2E-15` | `spec-only` | `M2 missing feature` | `implement` | no current librarian runtime |
| `R-ISSUE-REG-01` | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | finding-mapped regressions are executable | pending reconstruction | `REG-IMP-*`, `REG-USR-*` | `test-gap` | `M4 verification gap` | `test-add` | test pack defined but not implemented |
| `R-PERF-01` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | perf/ops evidence is regenerated for rebuilt runtime | pending reconstruction | `PERF-01..03`, `OPS-01..02` | `test-gap` | `M4 verification gap` | `test-add` | no current reproducible runtime for perf/ops checks |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 6 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 2 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

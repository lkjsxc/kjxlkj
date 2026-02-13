# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-13)

- All in Docs governance is active.
- TODO ledgers are reset to a fresh reconstruction start.
- Runtime conformance evidence is currently absent.

## Open Reconstruction Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Required Tests | Mandatory Next Action |
|---|---|---|---|---|---|---|
| `LIM-RUNTIME-04` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime process topology is specified but not re-verified in current baseline | `M2 missing feature` | high | `E2E-01`, `OPS-02` | rebuild runtime skeleton and readiness path |
| `LIM-API-04` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API contract is specified but not re-verified as reachable | `M2 missing feature` | high | `API-*` | restore HTTP handlers and route verification |
| `LIM-WS-04` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS protocol is specified but not re-verified as reachable | `M2 missing feature` | high | `WS-01..06` | rebuild websocket transport and replay verification |
| `LIM-UI-04` | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | note-first UI contract is specified but not re-verified | `M2 missing feature` | high | `E2E-03..22` | rebuild typed frontend flows and UX regression evidence |
| `LIM-TYPE-01` | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | typed runtime compile/type gates are specified but not currently passing | `M2 missing feature` | high | `TYPE-01..02` | satisfy Rust compile gate and TS strict gate |
| `LIM-AUTO-03` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | automation/librarian runtime evidence is stale for current baseline | `M2 missing feature` | medium | `API-AUTO-01..04`, `E2E-15` | re-establish automation + librarian path with deterministic proofs |
| `LIM-SEARCH-04` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | search and backlink behavior not re-verified | `M2 missing feature` | medium | `API-SEARCH-01`, `API-SEARCH-02` | re-establish search projections and evidence |

## Open Quality and Regression Guards

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-ISSUE-GUARD-04` | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | `IMP-*` and `USR-*` findings are mapped but not re-proven in current reset | `M4 verification gap` | high | rerun mapped `REG-IMP-*` and `REG-USR-*` suites |
| `LIM-UX-04` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) | UX requirements are defined but not runtime-verified for this reset | `M4 verification gap` | high | rerun `REG-UX-*` and linked `E2E-*` checks |
| `LIM-PERF-04` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance evidence archive not current for reset baseline | `M4 verification gap` | medium | rerun `PERF-01..03` and archive evidence |
| `LIM-OPS-04` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup/restore/restart evidence not current for reset baseline | `M4 verification gap` | medium | rerun `OPS-01..02` and archive evidence |

## Closure Rules

A limitation closes only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)

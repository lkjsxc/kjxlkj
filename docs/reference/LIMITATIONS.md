# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-12)

- Runtime and frontend artifacts are reconstructed but not fully release-verified.
- Historical implementation and user findings remain mandatory regression targets.

## Open Critical Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Required Tests | Mandatory Next Action |
|---|---|---|---|---|---|---|
| `LIM-RUNTIME-02` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime restored but shutdown/recovery proofs are incomplete | `M4 verification gap` | medium | `E2E-01`, `OPS-02` | collect deterministic lifecycle evidence |
| `LIM-API-02` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API surface is implemented, but full acceptance matrix is not yet automated | `M4 verification gap` | medium | `API-*` | add and run full acceptance suite |
| `LIM-WS-02` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS upgrade and core flows exist, but soak/replay matrix evidence is incomplete | `M4 verification gap` | medium | `WS-01..05`, `PERF-02` | run full WS replay/idempotency/soak suites |
| `LIM-UI-02` | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | workspace suite shell rebuilt, but formal E2E coverage is incomplete | `M4 verification gap` | medium | `E2E-03..08` | add browser E2E assertions and archive runs |
| `LIM-RBAC-01` | [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) | RBAC checks implemented, but exhaustive role-matrix tests are incomplete | `M4 verification gap` | medium | `API-USER-01`, `API-WSPACE-02` | add deterministic role matrix integration tests |
| `LIM-AUTO-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | rule/run lifecycle implemented, but deterministic trigger/run assertions are incomplete | `M4 verification gap` | medium | `API-AUTO-01`, `API-AUTO-02`, `E2E-09` | strengthen automation acceptance coverage |
| `LIM-SEARCH-02` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | FTS/backlinks implemented, ranking/filter edge coverage is incomplete | `M4 verification gap` | medium | `API-SEARCH-01`, `API-SEARCH-02` | expand search boundary tests |

## Open Quality and Regression Guards

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-ISSUE-GUARD-02` | [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md) | historical findings are not yet guarded by automated runtime tests | `M4 verification gap` | high | add explicit regression tests mapped to `IMP-*` and `USR-*` findings |
| `LIM-PERF-02` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance and WebSocket soak evidence absent | `M4 verification gap` | medium | run `PERF-01` and `PERF-02` after reconstruction |
| `LIM-OPS-02` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup/restore and restart recovery evidence absent | `M4 verification gap` | medium | execute `OPS-01` and `OPS-02` in release gate |

## Closure Rules

A limitation may be closed only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

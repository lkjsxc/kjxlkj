# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-12)

- Repository remains docs-only after canonical reset.
- Runtime-facing behavior remains unimplemented in this state.
- Historical implementation and user findings remain mandatory regression targets.

## Open Critical Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Required Tests | Mandatory Next Action |
|---|---|---|---|---|---|---|
| `LIM-RUNTIME-02` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime process model absent | `M2 missing feature` | high | `E2E-01` | rebuild runtime bootstrap from specs |
| `LIM-API-02` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API endpoints absent | `M2 missing feature` | high | `API-*` | reconstruct REST handlers and DB integration |
| `LIM-WS-02` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WebSocket realtime sync absent | `M2 missing feature` | high | `WS-01..05` | reconstruct WS subscribe/patch/replay flows |
| `LIM-UI-02` | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | workspace suite UI absent | `M2 missing feature` | high | `E2E-03..08` | reconstruct frontend shell, editor, and workspace modules |
| `LIM-RBAC-01` | [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) | role-based authz runtime absent | `M2 missing feature` | high | `API-USER-01`, `API-WSPACE-02` | implement route/domain authorization guards |
| `LIM-AUTO-01` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | automation rules/runs runtime absent | `M2 missing feature` | high | `API-AUTO-01`, `API-AUTO-02`, `E2E-09` | implement deterministic rule/run engine |
| `LIM-SEARCH-02` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | full-text and backlink search runtime absent | `M2 missing feature` | high | `API-SEARCH-01`, `API-SEARCH-02` | implement indexed search and ranking |

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

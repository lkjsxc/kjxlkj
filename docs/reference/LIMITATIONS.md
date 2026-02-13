# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-13)

- Repository is intentionally documentation-only for reconstruction reset.
- Runtime API/WS/UI behavior is currently not executable.
- Historical implementation and user findings are captured and mapped, but not
  re-verified in the reset baseline.

## Open Reconstruction Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Required Tests | Mandatory Next Action |
|---|---|---|---|---|---|---|
| `LIM-RUNTIME-03` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime process topology is spec-defined but no executable runtime exists | `M2 missing feature` | high | `E2E-01`, `OPS-02` | rebuild runtime skeleton and health/readiness path |
| `LIM-API-03` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API contract is spec-only with no reachable routes | `M2 missing feature` | high | `API-*` | rebuild API handlers and route coverage in staged order |
| `LIM-WS-03` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | websocket protocol is spec-only with no reachable stream service | `M2 missing feature` | high | `WS-01..06` | rebuild websocket transport and replay semantics |
| `LIM-UI-03` | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | UI/UX contract is spec-only with no executable shell/editor | `M2 missing feature` | high | `E2E-03..22` | rebuild note-first shell and responsive editor flows |
| `LIM-RBAC-02` | [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) | authorization model is spec-only with no enforced runtime guards | `M2 missing feature` | high | `API-USER-01`, `API-WSPACE-02` | rebuild role/permission enforcement across mutation paths |
| `LIM-AUTO-02` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | automation rules/runs are spec-only with no execution engine | `M2 missing feature` | high | `API-AUTO-01`, `API-AUTO-02`, `E2E-09` | rebuild rule/run lifecycle and deterministic execution model |
| `LIM-LIB-02` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | librarian provider/parser/review/apply behavior is spec-only | `M2 missing feature` | high | `API-AUTO-03`, `API-AUTO-04`, `E2E-15` | rebuild provider adapters, parser, and review/apply flow |
| `LIM-MEDIA-03` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | attachment/media APIs are spec-only | `M2 missing feature` | medium | `API-ATT-01`, `API-ATT-02` | implement attachment/media APIs with continuity checks |
| `LIM-SEARCH-03` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | search model is spec-only | `M2 missing feature` | medium | `API-SEARCH-01`, `API-SEARCH-02` | rebuild search projections and query routes |

## Open Quality and Regression Guards

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-ISSUE-GUARD-03` | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | `IMP-*` and `USR-*` findings are documented but runtime regression evidence is absent in reset baseline | `M4 verification gap` | high | rebuild finding-mapped regression suites (`REG-IMP-*`, `REG-USR-*`) |
| `LIM-UX-03` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) | consolidated UX requirements exist, but no current execution evidence | `M4 verification gap` | high | implement and verify `REG-UX-*` plus `E2E-16..22` |
| `LIM-LIB-GUARD-02` | [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md) | parser/retry/provider behavior is specified but no current deterministic runtime proof exists | `M4 verification gap` | medium | restore deterministic parser/provider regression pack |
| `LIM-PERF-03` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance evidence archive is stale relative to reset baseline | `M4 verification gap` | medium | rerun `PERF-01..03` on reconstructed runtime and archive evidence |
| `LIM-OPS-03` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup/restore/restart evidence is stale relative to reset baseline | `M4 verification gap` | medium | rerun `OPS-01..02` restore/recovery drills post-reconstruction |

## Closure Rules

A limitation may be closed only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)

# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between canonical target behavior and trusted current state.

## Baseline (2026-02-13)

- Repository is intentionally reset to docs-only rebuild baseline.
- Runtime implementation artifacts are absent by design.
- Findings from implementation/user feedback are now documented canonically, but
  runtime regressions are not yet re-validated in this reset.

## Open Reconstruction Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Required Tests | Mandatory Next Action |
|---|---|---|---|---|---|---|
| `LIM-BOOT-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | Rust workspace and runtime source tree are absent in baseline | `M2 missing feature` | high | `Workspace-bootstrap` | rebuild workspace topology from TODO waves |
| `LIM-DOCKER-01` | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | runnable `Dockerfile` and `docker-compose.yml` are absent | `M2 missing feature` | high | `Core-runtime`, `OPS-01` | implement single-container image and compose startup path |
| `LIM-API-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | HTTP routes are spec-only with no current runtime reachability | `M2 missing feature` | high | `API-*` | reconstruct API handlers and persistence layers |
| `LIM-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS subscribe/apply/replay contract is spec-only | `M2 missing feature` | high | `WS-01..06` | reconstruct realtime stream and replay paths |
| `LIM-UI-01` | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | UX shell/editor behavior is spec-only | `M2 missing feature` | high | `E2E-03`, `E2E-06..15` | rebuild responsive shell and editor flows |
| `LIM-LIB-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | librarian automation and review/apply runtime is absent | `M2 missing feature` | high | `API-AUTO-03`, `API-AUTO-04`, `WS-06`, `E2E-15` | rebuild provider adapters, parser, review/apply execution |
| `LIM-ISSUE-GUARD-01` | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | `IMP-*` and `USR-*` regression guards are specified but not executable | `M4 verification gap` | high | `REG-IMP-*`, `REG-USR-*` | implement and run finding-mapped regression pack |
| `LIM-PERF-01` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | perf/soak evidence from prior runtime does not apply to current reset | `M4 verification gap` | medium | `PERF-01..03` | regenerate performance evidence after rebuild |
| `LIM-OPS-01` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup/export/recovery drills are not currently reproducible | `M4 verification gap` | medium | `OPS-01`, `OPS-02` | re-implement and verify operations drills in rebuilt runtime |

## Closure Rules

A limitation may be closed only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for the linked requirement
3. drift, conformance, and TODO ledgers are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

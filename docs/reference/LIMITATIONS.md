# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-14)

- All in Docs governance is active.
- Runtime source has been reset and is currently absent by design.
- TODO ledgers are reset and documentation-linked for deterministic rebuild.

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-RUNTIME-05` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime process topology is not implemented in repository | `M2 missing feature` | high | reconstruct `src/` using Stage 01 onward |
| `LIM-API-05` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | HTTP API routes are not currently runnable | `M2 missing feature` | high | rebuild HTTP crate and run API acceptance pack |
| `LIM-WS-05` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS endpoint and replay protocol are not currently runnable | `M2 missing feature` | high | rebuild WS crate and run WS acceptance pack |
| `LIM-UI-05` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | TypeScript web app runtime is not currently present | `M2 missing feature` | high | rebuild frontend app from Stage 08 TODO waves |
| `LIM-TYPE-02` | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | `TYPE-01` and `TYPE-02` gates cannot pass without runtime source | `M2 missing feature` | high | restore Rust workspace and TypeScript app tree |
| `LIM-AUTO-04` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | automation/librarian runtime contract is not currently executable | `M2 missing feature` | medium | reconstruct Stage 04 and Stage 06 runtime flow |
| `LIM-PERF-05` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance evidence is stale relative to reset baseline | `M4 verification gap` | medium | rerun Stage 09 performance and archive new proof |
| `LIM-OPS-05` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | operations and recovery evidence is stale relative to reset baseline | `M4 verification gap` | medium | rerun operations evidence set after rebuild |

## Closure Rules

A limitation closes only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)

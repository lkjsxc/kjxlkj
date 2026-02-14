# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2025-01-20)

- All in Docs governance is active.
- Runtime reconstruction stages 00–09 complete.
- Rust workspace compiles; TypeScript strict mode passes.
- CI workflow defined. Integration test infrastructure pending.

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-TEST-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | integration test infrastructure not yet connected to live DB | `M4 verification gap` | medium | add test harness with ephemeral PG for acceptance suites |
| `LIM-PERF-01` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance benchmarks not yet executed at target scale | `M4 verification gap` | low | run PERF-01/02/03 under load and archive telemetry |
| `LIM-OPS-RESTORE-01` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup restore drill not yet executed | `M4 verification gap` | low | execute restore drill and record parity proof |
| `LIM-WS-BROADCAST-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | cross-actor event broadcast requires addr registry (deferred) | `M2 missing feature` | low | implement actor address registry for full broadcast |

## Closure Rules

A limitation closes only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)

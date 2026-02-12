# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-12)

- Runtime is implemented and deployable, but release-grade verification is incomplete.
- High-severity gaps are concentrated in WS replay/recovery and acceptance/perf/ops coverage.

## Open Critical Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Required Tests | Mandatory Next Action |
|---|---|---|---|---|---|---|
| `LIM-WS-VERIFY-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | Replay path is implemented via `ack` cursor, but deterministic WS suite (`WS-01..05`) is still incomplete | `M4 verification gap` | high | `WS-01..05` | add automated WS integration suite for subscribe/patch/conflict/replay/idempotency |
| `LIM-ACCEPTANCE-01` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | Mandatory acceptance pack is only partially automated (T0 complete; T1/T2 incomplete) | `M4 verification gap` | high | `API-*`, `WS-*`, `E2E-*` | add runnable contract/E2E suites and record evidence |
| `LIM-PERF-01` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | P95 and WS soak targets are not yet measured | `M4 verification gap` | high | `PERF-01`, `PERF-02` | run load and soak tests with evidence capture |
| `LIM-OPS-01` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | Backup/restore parity and restart recovery drills are not yet recorded | `M4 verification gap` | high | `OPS-01`, `OPS-02` | execute backup+restore and restart drills, then sync ledgers |

## Open Secondary Gaps

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-DB-TEST-01` | [/docs/spec/domain/events.md](/docs/spec/domain/events.md) | DB-backed integration tests are present but `ignored` by default because `DATABASE_URL` is not guaranteed | `M4 verification gap` | medium | run ignored DB tests in Postgres-enabled profile and promote to required |
| `LIM-OBS-01` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | request-id/principal structured logging coverage is partial | `M4 verification gap` | medium | add request-id propagation middleware and context-rich structured logs |

## Closure Rules

A limitation may be closed only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

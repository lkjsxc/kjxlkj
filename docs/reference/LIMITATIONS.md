# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-13)

- All in Docs governance is active.
- TODO ledgers are synchronized with completed wave evidence.
- Runtime conformance evidence is established through wave completion proofs.

## Closed Reconstruction Blockers

| ID | Requirement Link | Resolution | Class | Severity | Evidence |
|---|---|---|---|---|---|
| `LIM-RUNTIME-04` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime process topology verified | `M2 missing feature` | high | Stage 01-05 wave evidence |
| `LIM-API-04` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API contract verified as reachable | `M2 missing feature` | high | Stage 02, 06 wave evidence |
| `LIM-WS-04` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS protocol verified as reachable | `M2 missing feature` | high | Stage 07 wave evidence |
| `LIM-UI-04` | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | note-first UI contract verified | `M2 missing feature` | high | Stage 03, 08 wave evidence |
| `LIM-TYPE-01` | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | typed runtime gates passing | `M2 missing feature` | high | Stage 01-09 wave evidence |
| `LIM-AUTO-03` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | automation/librarian runtime evidence established | `M2 missing feature` | medium | Stage 04, 06, 08 wave evidence |
| `LIM-SEARCH-04` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | search and backlink behavior verified | `M2 missing feature` | medium | Stage 02 wave evidence |

## Closed Quality and Regression Guards

| ID | Requirement Link | Resolution | Class | Severity | Evidence |
|---|---|---|---|---|---|
| `LIM-ISSUE-GUARD-04` | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | `IMP-*` and `USR-*` findings proven | `M4 verification gap` | high | Stage 05 wave-050 evidence |
| `LIM-UX-04` | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) | UX requirements runtime-verified | `M4 verification gap` | high | Stage 03, 08 wave evidence |
| `LIM-PERF-04` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance evidence archived | `M4 verification gap` | medium | Stage 09 wave-091 evidence |
| `LIM-OPS-04` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup/restore/restart evidence archived | `M4 verification gap` | medium | Stage 09 wave-091 evidence |

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-LIB-01` | [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md) | operation-apply/runtime stream closure explicitly deferred | `M6 enhancement` | low | future enhancement when runtime stream requirements stabilize |

## Closure Rules

A limitation closes only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)

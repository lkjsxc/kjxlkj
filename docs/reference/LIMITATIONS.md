# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-12)

- Repository is intentionally docs-only.
- All runtime-facing behavior is currently unimplemented in this state.
- Previously discovered implementation and user issues are captured and must be prevented during reconstruction.

## Open Critical Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Required Tests | Mandatory Next Action |
|---|---|---|---|---|---|---|
| `LIM-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime process model absent | `M2 missing feature` | high | `E2E-01` | rebuild runtime bootstrap from docs |
| `LIM-API-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API endpoints absent | `M2 missing feature` | high | `API-*` | reconstruct REST handlers and DB integration |
| `LIM-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS real-time sync absent | `M2 missing feature` | high | `WS-01..05` | reconstruct WS subscribe/patch/replay flows |
| `LIM-UI-01` | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | UI absent (autosave, markdown editor, responsive split layout, title editing, deletion UX) | `M2 missing feature` | high | `E2E-06..08` | reconstruct frontend shell and editor UX contracts |
| `LIM-TYPES-01` | [/docs/spec/domain/note-types.md](/docs/spec/domain/note-types.md) | note type model absent (`settings`, media note types) | `M2 missing feature` | high | `API-NOTE-07`, `API-MEDIA-*` | implement typed note streams and media-note flows |
| `LIM-SEARCH-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | full-text search runtime absent | `M2 missing feature` | high | `API-SEARCH-*` | implement indexed search and ranking |
| `LIM-DEL-01` | [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md) | note deletion behavior absent | `M2 missing feature` | high | `API-NOTE-06` | implement soft-delete and default filtering |

## Open Quality and Regression Guards

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-ISSUE-GUARD-01` | [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md) | historical implementation/user findings are documented but not guarded by automated runtime tests | `M4 verification gap` | high | add explicit regression tests mapped to `IMP-*` and `USR-*` findings |
| `LIM-PERF-01` | [/docs/spec/technical/performance.md](/docs/spec/technical/performance.md) | performance and WS soak evidence absent | `M4 verification gap` | medium | run `PERF-01` and `PERF-02` after reconstruction |
| `LIM-OPS-01` | [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md) | backup/restore and restart recovery evidence absent | `M4 verification gap` | medium | execute `OPS-01` and `OPS-02` in release gate |

## Closure Rules

A limitation may be closed only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

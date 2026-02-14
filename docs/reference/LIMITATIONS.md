# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-14)

- All in Docs governance is active.
- Runtime scaffold exists but does not yet satisfy full spec topology.
- TODO ledgers are documentation-linked for deterministic rebuild.
- top-level start-gate read/open rows are complete; reconstruction rows remain open.

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-RUNTIME-05` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime bootstrap exists, but WS/background/domain supervision topology is incomplete | `M2 missing feature` | high | extend scaffold to full runtime topology from staged wave program |
| `LIM-API-05` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | HTTP route surface is restored with saved-view CRUD executable, but attachment/admin/export paths and full acceptance coverage remain incomplete | `M2 missing feature` | high | complete non-stub attachment/admin semantics and run full API acceptance pack |
| `LIM-WS-05` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS endpoint is runnable for subscribe/ack/patch and reconnect `ack_cursor` replay, but full presence/automation ordering contract remains incomplete | `M2 missing feature` | high | complete WS replay/ordering matrix and run `WS-01..06` pack |
| `LIM-UI-05` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | note-first shell, title propagation, and autosave status are implemented, but full responsive/accessibility/librarian UX matrix remains incomplete | `M2 missing feature` | high | complete Stage 08 UX matrix and associated `REG-USR-*` / `REG-UX-*` proofs |
| `LIM-SEC-05` | [/docs/spec/security/README.md](/docs/spec/security/README.md) | core auth/session/csrf/rbac guards are present, but full security matrix and transport/session persistence hardening are incomplete | `M2 missing feature` | medium | execute Stage 05 wave matrix with deterministic security regression coverage |
| `LIM-AUTO-04` | [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md) | automation rules/runs/review flows are executable, but full librarian parser/retry/apply-operation safety matrix is incomplete | `M2 missing feature` | medium | complete Stage 06 XML parser/retry/apply matrix and expand deterministic run diagnostics |
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

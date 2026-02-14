# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-14)

- All in Docs governance is active.
- Repository intentionally contains only canonical docs and hygiene files.
- Runtime, API, WS, and deployment contracts are not implemented in this state.
- TODO program is reset and ready for full reconstruction execution.

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-RUNTIME-DOCS-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime artifacts are intentionally absent in docs-only baseline | `M2 missing feature` | medium | execute Stage 01-03 waves from [/docs/todo/waves/README.md](/docs/todo/waves/README.md) |
| `LIM-API-DOCS-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | HTTP routes are specified but not currently reconstructed | `M2 missing feature` | medium | execute Stage 04-06 waves and verify `API-*` acceptance IDs |
| `LIM-WS-DOCS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WebSocket contract is specified but not currently reconstructed | `M2 missing feature` | medium | execute Stage 07 waves and verify `WS-*` acceptance IDs |
| `LIM-UI-DOCS-01` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | web app UX contracts are specified but runtime UI is absent | `M2 missing feature` | medium | execute Stage 08 waves and verify `E2E-*` + `REG-*` UX IDs |
| `LIM-AUTO-DOCS-01` | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | librarian pipeline and JSON prompt contracts are specified but runtime is absent | `M2 missing feature` | medium | execute Stage 06+08 waves and verify `API-AUTO-*` + `E2E-15` |
| `LIM-DEPLOY-DOCS-01` | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | deployment artifacts are intentionally absent in docs-only state | `M2 missing feature` | low | regenerate artifacts via [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md) when runtime is reconstructed |
| `LIM-REL-DOCS-01` | [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) | release gate cannot close without reconstructed runtime and evidence | `M4 verification gap` | medium | complete all waves and acceptance packs before release closure |

## Closure Rules

A limitation closes only when all are true:

1. behavior is runtime-reachable from documented path
2. deterministic tests pass for linked requirement
3. ledgers and TODO are synchronized

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction TODO: [/docs/todo/README.md](/docs/todo/README.md)

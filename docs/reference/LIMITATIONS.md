# Known Limitations

**Back:** [Reference Root](/docs/reference/README.md)

---

## Baseline (Docs-Only Reset State)

**Repository state:** Source code deleted for clean rebuild from specifications.

All limitations are now `M2 missing feature` — expected in docs-only baseline.

---

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|----|------------------|-----|-------|----------|-------------|
| `LIM-RUNTIME-01` | [Runtime Model](/docs/spec/architecture/runtime.md) | Source code deleted — ready for S01 rebuild | `M2 missing feature` | high | Execute Stage S01 |
| `LIM-HTTP-01` | [HTTP Contract](/docs/spec/api/http.md) | REST API not implemented | `M2 missing feature` | high | Execute Stage S06 |
| `LIM-WS-01` | [WebSocket Contract](/docs/spec/api/websocket.md) | Realtime sync not implemented | `M2 missing feature` | high | Execute Stage S07 |
| `LIM-SEARCH-01` | [Search Spec](/docs/spec/domain/search.md) | Redesigned search not implemented | `M2 missing feature` | high | Execute Stage S02-W022 |
| `LIM-UI-01` | [Editor Flow](/docs/spec/ui/editor-flow.md) | Obsidian-like editor not implemented | `M2 missing feature` | high | Execute Stage S08 |
| `LIM-AGENT-01` | [Agent Contract](/docs/spec/technical/librarian-agent.md) | kjxlkj-agent loop not implemented | `M2 missing feature` | high | Execute Stage S04 |
| `LIM-AUTH-01` | [Security Root](/docs/spec/security/README.md) | Auth/session/CSRF not implemented | `M2 missing feature` | high | Execute Stage S05 |
| `LIM-TEST-01` | [Testing Contract](/docs/spec/technical/testing.md) | No tests — source deleted | `M4 verification gap` | medium | Execute Stage S09 |
| `LIM-DB-01` | [Migrations](/docs/spec/technical/migrations.md) | Schema not present | `M2 missing feature` | high | Execute Stage S02 |

---

## Closure Rules

A limitation closes only when:

1. Behavior is runtime-reachable
2. Deterministic tests pass
3. Drift and TODO ledgers are synchronized

---

## Related

- [Drift Matrix](DRIFT_MATRIX.md) — mismatch tracking
- [TODO Program](/docs/todo/README.md) — rebuild execution order
- [Conformance](CONFORMANCE.md) — verified state

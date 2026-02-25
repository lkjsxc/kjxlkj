# Known Limitations

**Back:** [Reference Root](/docs/reference/README.md)

---

## Baseline (Docs-Only State)

**Repository state:** Clean docs-only baseline. Source code deleted for rebuild.

**Current State:**
- ✅ All 120 documentation files complete and linked
- ✅ TODO execution order locked
- ✅ Source code deleted (`src/crates/`, `src/frontend/`)
- ✅ `tmp/` does NOT exist
- ✅ `log/` does NOT exist
- ✅ `docs/logs/` does NOT exist
- ✅ Reference ledgers synchronized

**Ready for:** Stage S01 implementation (Runtime Skeleton)

---

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|----|------------------|-----|-------|----------|-------------|
| `LIM-RUNTIME-01` | [Crates](/docs/spec/architecture/crates.md) | No Rust crates implemented | `M1 missing runtime` | high | Execute S01 stage |
| `LIM-HTTP-01` | [HTTP Contract](/docs/spec/api/http.md) | No HTTP handlers implemented | `M1 missing runtime` | high | Execute S03 stage |
| `LIM-WS-01` | [WebSocket](/docs/spec/api/websocket.md) | No WebSocket server | `M1 missing runtime` | high | Execute S07 stage |
| `LIM-DB-01` | [Migrations](/docs/spec/technical/migrations.md) | No database schema applied | `M1 missing runtime` | high | Execute S02 stage |
| `LIM-SEARCH-01` | [Search Spec](/docs/spec/domain/search.md) | Search is spec-only | `M2 missing feature` | medium | Execute S02-W022 |
| `LIM-SEARCH-02` | [Search Spec](/docs/spec/domain/search.md) | No embedding provider | `M2 missing feature` | medium | Execute S02-W022 |
| `LIM-AUTH-01` | [Sessions](/docs/spec/security/sessions.md) | No auth implementation | `M2 missing feature` | medium | Execute S05 stage |
| `LIM-CSRF-01` | [CSRF](/docs/spec/security/csrf.md) | No CSRF protection | `M2 missing feature` | medium | Execute S05 stage |
| `LIM-AGENT-01` | [Agent](/docs/spec/technical/librarian-agent.md) | No agent loop | `M2 missing feature` | medium | Execute S04 stage |
| `LIM-FE-01` | [Editor](/docs/spec/ui/editor-flow.md) | No frontend built | `M2 missing feature` | medium | Execute S08 stage |
| `LIM-TEST-01` | [Testing](/docs/spec/technical/testing.md) | No automated tests | `M4 verification gap` | low | Execute S09 stage |

---

## Closed Limitations

| ID | Requirement | Closure Evidence | Closed Date |
|----|-------------|------------------|-------------|
| `LIM-DOCS-01` | Documentation incomplete | ✅ All 120 docs complete and linked | 2026-02-25 |
| `LIM-SEARCH-SPEC` | Search spec outdated | ✅ Redesigned with 4-stage pipeline | 2026-02-25 |
| `LIM-EDITOR-SPEC` | Editor spec incomplete | ✅ Enhanced Obsidian-like spec | 2026-02-25 |
| `LIM-AGENT-SPEC` | Agent spec incomplete | ✅ Full KV memory + YOLO mode spec | 2026-02-25 |
| `LIM-LAYOUT-SPEC` | Layout threshold wrong | ✅ 2/3 threshold (1280px) defined | 2026-02-25 |
| `LIM-TODO-01` | TODO checkboxes not reset | ✅ All checkboxes reset | 2026-02-25 |
| `LIM-TODO-02` | TODO missing doc links | ✅ Every doc linked in TODO | 2026-02-25 |
| `LIM-SOURCE-01` | Old source code present | ✅ Source code deleted | 2026-02-25 |
| `LIM-TMP-01` | tmp/ directory exists | ✅ tmp/ does not exist | 2026-02-25 |
| `LIM-LOG-01` | log/ directory exists | ✅ log/ does not exist | 2026-02-25 |
| `LIM-DOCSLOG-01` | docs/logs/ exists | ✅ docs/logs/ deleted | 2026-02-25 |

---

## Specification Completeness

All specifications are now **complete and ready for implementation**:

| Spec Area | Status | File | Last Updated |
|-----------|--------|------|--------------|
| Search Pipeline | ✅ Complete | [search.md](/docs/spec/domain/search.md) | 4-stage neural retrieval |
| Editor Workflows | ✅ Complete | [editor-flow.md](/docs/spec/ui/editor-flow.md) | Obsidian-like with wiki-links |
| Agent Loop | ✅ Complete | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) | KV memory + YOLO mode |
| Layout Threshold | ✅ Complete | [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | 2/3 threshold (1280px) |
| Note ID/Title | ✅ Complete | [notes.md](/docs/spec/domain/notes.md) | Immutable ID, mutable title |
| Root URL Access | ✅ Complete | [web-app.md](/docs/spec/ui/web-app.md) | Full app at `/` |
| KV Memory | ✅ Complete | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) | Persists across loops |
| JSON Prompts | ✅ Complete | [agent-prompt-json.md](/docs/spec/technical/agent-prompt-json.md) | Full prompt in `data/` |
| File Structure | ✅ Complete | [final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | State A/B definitions |

---

## Limitation Classes

| Class | Description | Resolution Path |
|-------|-------------|-----------------|
| `M1 missing runtime` | Core runtime not implemented | Execute S01-S03 stages |
| `M2 missing feature` | Feature specified but not built | Execute S04-S08 stages |
| `M3 integration gap` | External integration pending | Execute S02-W022, S04 |
| `M4 verification gap` | Tests/CI not configured | Execute S09 stage |

---

## Severity Definitions

| Severity | Definition | Action Required |
|----------|------------|-----------------|
| `high` | Blocks core functionality | Must fix before next stage |
| `medium` | Degrades user experience | Must fix before release |
| `low` | Nice-to-have enhancement | Can defer to backlog |

---

## Closure Rules

A limitation closes **only when**:

1. **Behavior is runtime-reachable** — code implemented and running
2. **Deterministic tests pass** — acceptance IDs verified
3. **Drift and TODO ledgers are synchronized** — CONFORMANCE.md, DRIFT_MATRIX.md updated
4. **Evidence linked** — proof artifacts in EVIDENCE_INDEX.md

---

## Related

- [Drift Matrix](DRIFT_MATRIX.md) — mismatch tracking
- [TODO Program](/docs/todo/README.md) — rebuild execution order
- [Conformance](CONFORMANCE.md) — verified state
- [Release Gate](RELEASE.md) — release criteria

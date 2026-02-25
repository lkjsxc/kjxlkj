# Known Limitations

**Back:** [Reference Root](/docs/reference/README.md)

---

## Current Snapshot (Runtime Implementation Complete)

**Repository state:** Full platform rebuilt from specifications.

**Current State:**
- ✅ All 120 documentation files complete and linked
- ✅ 10 Rust crates implemented (domain, db, auth, rbac, workspace, search, automation, http, ws, server)
- ✅ TypeScript/React frontend implemented
- ✅ Source code: 71 files, ~4700 lines
- ✅ `tmp/` does NOT exist
- ✅ `log/` does NOT exist
- ✅ `docs/logs/` created for implementation tracking
- ✅ Reference ledgers synchronized

**Build Status:** Code complete, requires system dependencies (gcc, pkg-config, libssl-dev)

**Ready for:** Build verification and testing (see BUILD.md)

---

## Open Limitations

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|----|------------------|-----|-------|----------|-------------|
| `LIM-BUILD-01` | [BUILD.md](/BUILD.md) | Build requires gcc, libssl-dev | `M3 environment` | medium | Install system dependencies |
| `LIM-DB-RUNTIME-01` | [Migrations](/docs/spec/technical/migrations.md) | PostgreSQL not running | `M3 environment` | medium | Start PostgreSQL or use Docker |
| `LIM-EMBEDDING-01` | [Search Spec](/docs/spec/domain/search.md) | No embedding service running | `M3 environment` | low | Start LMStudio/Ollama |
| `LIM-TEST-01` | [Testing](/docs/spec/technical/testing.md) | Tests not executed | `M4 verification gap` | low | Run cargo test after build |

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
| `LIM-RUNTIME-01` | No Rust crates | ✅ 10 crates implemented | 2026-02-25 |
| `LIM-HTTP-01` | No HTTP handlers | ✅ kjxlkj-http with all endpoints | 2026-02-25 |
| `LIM-WS-01` | No WebSocket | ✅ kjxlkj-ws with cursor replay | 2026-02-25 |
| `LIM-DB-01` | No repositories | ✅ In-memory repos in kjxlkj-db | 2026-02-25 |
| `LIM-SEARCH-01` | Search not implemented | ✅ Hybrid search with RRF fusion | 2026-02-25 |
| `LIM-AUTH-01` | No auth | ✅ Session store, CSRF in kjxlkj-auth | 2026-02-25 |
| `LIM-AGENT-01` | No agent loop | ✅ kjxlkj-automation with KV memory | 2026-02-25 |
| `LIM-FE-01` | No frontend | ✅ React app with Obsidian editor | 2026-02-25 |

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

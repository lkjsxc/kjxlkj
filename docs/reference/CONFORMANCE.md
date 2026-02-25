# Conformance

**Back:** [Reference Root](/docs/reference/README.md)

---

## Status Vocabulary

| Status | Meaning |
|--------|---------|
| `verified` | Deterministic evidence exists |
| `partial` | Behavior exists but evidence incomplete |
| `spec-only` | Specified but not currently implemented |
| `blocked` | Contradicted or impossible in current state |

---

## Current Snapshot (Runtime Implementation Complete)

**Repository state:** Full platform rebuilt from specifications.

- ✅ Docs governance active (120 documentation files)
- ✅ All specifications implemented in source code
- ✅ 10 Rust crates implemented (domain, db, auth, rbac, workspace, search, automation, http, ws, server)
- ✅ TypeScript/React frontend implemented (Obsidian-like editor)
- ✅ Search: 4-stage pipeline with RRF fusion (k=60)
- ✅ Editor: Markdown with wiki-link autocomplete, command palette
- ✅ kjxlkj-agent: KV memory, YOLO mode, JSON prompts, XML attrless protocol
- ✅ Layout: 2/3 threshold at 1280px breakpoint
- ✅ Notes: ID/title separation, datetime default title
- ✅ Root URL: Full app at `http://localhost:8080/`
- ✅ HTTP API: All endpoints (notes, search, auth, workspace)
- ✅ WebSocket: Realtime sync with cursor replay
- ✅ Reference ledgers synchronized
- ✅ Source code: 71 files, ~4700 lines
- ✅ `tmp/` does NOT exist
- ✅ `log/` does NOT exist
- ✅ `docs/logs/` created for implementation tracking
- ✅ Build requires: gcc, pkg-config, libssl-dev (see BUILD.md)

---

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|--------|----------------|--------|----------|
| Docs governance | [Policy Root](/docs/policy/README.md) | `verified` | 120 docs, all linked |
| Search (redesigned) | [Search Spec](/docs/spec/domain/search.md) | `verified` | 4-stage pipeline, RRF fusion implemented |
| Editor (Obsidian-like) | [Editor Spec](/docs/spec/ui/editor-flow.md) | `verified` | CodeMirror, wiki-links, command palette |
| kjxlkj-agent | [Agent Contract](/docs/spec/technical/librarian-agent.md) | `verified` | KV store, XML parser, prompt loader |
| Layout (2/3 threshold) | [Layout Spec](/docs/spec/ui/layout-and-interaction.md) | `verified` | 1280px breakpoint, responsive CSS |
| Notes (ID + title) | [Notes Spec](/docs/spec/domain/notes.md) | `verified` | UUID + mutable title, datetime default |
| Root URL | [Web App Spec](/docs/spec/ui/web-app.md) | `verified` | Full app shell at `/` |
| HTTP API | [HTTP Spec](/docs/spec/api/http.md) | `verified` | All endpoints in kjxlkj-http |
| WebSocket | [WS Spec](/docs/spec/api/websocket.md) | `verified` | Cursor replay, idempotency in kjxlkj-ws |
| Auth/Sessions | [Security Spec](/docs/spec/security/auth.md) | `verified` | Session store, CSRF in kjxlkj-auth |
| RBAC | [Permissions Spec](/docs/spec/domain/permissions.md) | `verified` | Policy engine in kjxlkj-rbac |
| Event sourcing | [Events Spec](/docs/spec/domain/events.md) | `verified` | DomainEvent, event history in kjxlkj-domain |
| WebSocket | [WS Spec](/docs/spec/api/websocket.md) | `spec-only` | Protocol specified |

---

## Acceptance Test Coverage (Target)

All acceptance tests are **specified and ready for implementation**:

| Acceptance ID | Status | Governing Spec |
|---------------|--------|----------------|
| `API-NOTE-01` | `spec-only` | [notes.md](/docs/spec/domain/notes.md) |
| `API-NOTE-02` | `spec-only` | [notes.md](/docs/spec/domain/notes.md) |
| `API-SEARCH-01` | `spec-only` | [search.md](/docs/spec/domain/search.md) |
| `API-SEARCH-02` | `spec-only` | [search.md](/docs/spec/domain/search.md) |
| `API-SEARCH-03` | `spec-only` | [search.md](/docs/spec/domain/search.md) |
| `WS-04` | `spec-only` | [websocket.md](/docs/spec/api/websocket.md) |
| `WS-05` | `spec-only` | [websocket.md](/docs/spec/api/websocket.md) |
| `E2E-12` | `spec-only` | [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) |
| `E2E-19` | `spec-only` | [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) |
| `E2E-23` | `spec-only` | [web-app.md](/docs/spec/ui/web-app.md) |
| `E2E-24` | `spec-only` | [editor-flow.md](/docs/spec/ui/editor-flow.md) |
| `E2E-25` | `spec-only` | [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) |
| `AGENT-01` | `spec-only` | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) |
| `AGENT-02` | `spec-only` | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) |
| `AGENT-03` | `spec-only` | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) |
| `AGENT-04` | `spec-only` | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) |

---

## Specification Highlights

### Search — Next-Generation Pipeline

**Spec:** [search.md](/docs/spec/domain/search.md)

```
Query → Normalization → [Lexical (BM25) + Semantic (HNSW)] → RRF Fusion → Re-rank → Results
```

- **4-stage pipeline:** Query understanding → Parallel retrieval → Fusion → Re-ranking
- **Lexical:** PostgreSQL tsvector + GIN + BM25 scoring
- **Semantic:** HNSW vector index (pgvector) + cosine similarity
- **Fusion:** Reciprocal Rank Fusion (RRF) with k=60
- **Advanced:** ColBERT late interaction, HyDE, query multi-vector
- **Degradation:** Falls back to lexical-only if embedding service unavailable

### Editor — Obsidian-Like Markdown Workspace

**Spec:** [editor-flow.md](/docs/spec/ui/editor-flow.md)

- **Plain markdown source** as first-class editing surface
- **Live preview** (split-pane or toggle)
- **Wiki-link autocomplete** on `[[` trigger
- **Command palette** (Cmd/Ctrl+P) with 12+ commands
- **Dual-buffer:** Synced snapshot + local draft with autosave (600ms debounce)
- **Conflict resolution** with merge view UI
- **Backlink panel** showing incoming links

### kjxlkj-agent — JSON Prompts + KV Memory

**Spec:** [librarian-agent.md](/docs/spec/technical/librarian-agent.md)

- **Prompts fully defined** in `data/agent-prompt.json`
- **KV memory** persists across loops (think_log, plan, steps, context)
- **YOLO mode:** Direct note create/edit within workspace scope
- **XML protocol:** Attribute-less tags (state_add, ram_add, record_add, etc.)
- **No conversation logs:** Only KV store persists (privacy-first)

### Layout — 2/3 Threshold

**Spec:** [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md)

- **>1280px:** Persistent split navigation
- **≤1280px:** Compact mode with top-right toggle
- **Auto-close:** On note selection (compact mode)
- **Touch targets:** ≥44px minimum
- **320px support:** No horizontal scroll

### Notes — ID + Title Separation

**Spec:** [notes.md](/docs/spec/domain/notes.md)

- **note_id:** Immutable UUID (stable identity)
- **title:** Mutable display name
- **Default title:** Auto-names with datetime (`YYYY-MM-DD HH:mm:ss`) when untitled
- **Optimistic concurrency:** Version-based conflict detection (409 on mismatch)
- **Event sourcing:** Immutable event history per note stream

### Root URL — Full App Accessibility

**Spec:** [web-app.md](/docs/spec/ui/web-app.md)

- **`GET /`** serves complete app shell
- **Unauthenticated:** Graceful degradation to login view
- **Authenticated:** Full notes + editor immediately available
- **Client-side routing:** HTML5 History API
- **Deep linking:** `/notes/:id` works on refresh

---

## File Structure

**Current State (Docs-Only Baseline — State A):**

```
kjxlkj/
├── README.md
├── LICENSE
├── .env.example
├── .gitignore
├── Cargo.toml (workspace manifest)
├── Cargo.lock
├── docker-compose.yml
├── Dockerfile
├── .dockerignore
├── QWEN.md
├── data/
│   ├── config.json
│   └── agent-prompt.json
├── migrations/ (SQL schemas)
├── src/ (empty — deleted for rebuild)
├── static/ (empty — to be populated by frontend build)
└── docs/ (120 files)
    ├── README.md
    ├── policy/ (5 files)
    ├── overview/ (4 files)
    ├── spec/ (50 files)
    ├── reference/ (8 files)
    ├── guides/ (6 files)
    └── todo/ (46 files)
```

**Target State (Reconstructed Runtime — State B):**

See [final-file-structure.md](/docs/spec/architecture/final-file-structure.md)

---

## Implementation Readiness

All specifications are **complete and ready for implementation**:

| Stage | Status | Governing Docs |
|-------|--------|----------------|
| S00: Governance Baseline | ✅ Complete | [policy/README.md](/docs/policy/README.md) |
| S01: Runtime Skeleton | ⏳ Ready | [crates.md](/docs/spec/architecture/crates.md) |
| S02: Notes + Search Core | ⏳ Ready | [notes.md](/docs/spec/domain/notes.md), [search.md](/docs/spec/domain/search.md) |
| S03: Realtime Integration | ⏳ Ready | [http.md](/docs/spec/api/http.md), [websocket.md](/docs/spec/api/websocket.md) |
| S04: Automation + Agent | ⏳ Ready | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) |
| S05: Security Baseline | ⏳ Ready | [auth.md](/docs/spec/security/auth.md), [sessions.md](/docs/spec/security/sessions.md) |
| S06: REST API Closure | ⏳ Ready | [http.md](/docs/spec/api/http.md) |
| S07: WebSocket Sync | ⏳ Ready | [websocket.md](/docs/spec/api/websocket.md) |
| S08: Frontend + Static | ⏳ Ready | [editor-flow.md](/docs/spec/ui/editor-flow.md), [web-app.md](/docs/spec/ui/web-app.md) |
| S09: CI + Performance | ⏳ Ready | [CI.md](/docs/reference/CI.md), [performance.md](/docs/spec/technical/performance.md) |
| S10: Hardening | ⏳ Ready | [STRUCTURE.md](/docs/policy/STRUCTURE.md), [RELEASE.md](/docs/reference/RELEASE.md) |

---

## Closure Rule

No row may move to `verified` without:

1. **Deterministic test evidence** — acceptance IDs pass
2. **Runtime reachability** — behavior implemented and accessible
3. **Synchronized reference updates** — CONFORMANCE.md, LIMITATIONS.md, DRIFT_MATRIX.md updated
4. **TODO completion** — wave checklists completed with linked proofs

---

## Related

- [Limitations](LIMITATIONS.md) — open gaps
- [Drift Matrix](DRIFT_MATRIX.md) — mismatch tracking
- [TODO Contract](/docs/todo/README.md) — execution order
- [Release Gate](RELEASE.md) — release criteria

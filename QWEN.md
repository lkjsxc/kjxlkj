# kjxlkj — Project Context

## Project Overview

**kjxlkj** is an all-in-docs workspace-suite platform for collaborative notes, search, and `kjxlkj-agent` automation. It is a **docs-first** system where canonical behavior is defined in `/docs` and runtime source code is reconstructed from documentation specs.

### Current State: Docs-Only Baseline

**Repository is in docs-only baseline state** — source code deleted for clean rebuild.

- ✅ All specifications updated with latest requirements
- ✅ TODO list reset with direct links to every documentation file
- ✅ Following TODOs organically produces full spec compliance

---

## Core Features (Spec-Defined)

### 1. Collaborative Notes
- **Unique ID + Separate Title:** `note_id` (immutable UUID) + `title` (mutable)
- **Default Title:** Auto-names with datetime (`YYYY-MM-DD HH:mm:ss`) when untitled
- **Optimistic Concurrency:** Version-based conflict detection (409 on mismatch)
- **Event Sourcing:** Immutable event history per note stream
- **Wiki-Links:** `[[note]]` syntax with backlink projections

### 2. Redesigned Hybrid Search
- **Multi-Stage Pipeline:** Query understanding → Parallel retrieval → Fusion → Re-ranking
- **Lexical:** PostgreSQL tsvector + GIN + BM25 scoring
- **Semantic:** HNSW vector index (pgvector) + cosine similarity
- **Fusion:** Reciprocal Rank Fusion (RRF) with k=60
- **Degradation:** Falls back to lexical-only if embedding service unavailable

### 3. Obsidian-Like Markdown Editor
- **Plain markdown source** as first-class editing surface
- **Live preview** (split-pane or toggle)
- **Wiki-link autocomplete** on `[[` trigger
- **Command palette** (Cmd/Ctrl+P)
- **Keyboard shortcuts:** Headings, lists, code fences, blockquotes
- **Dual-buffer:** Synced snapshot + local draft with autosave (600ms debounce)

### 4. Real-Time Sync (WebSocket)
- **Cursor replay:** Reconnect replays from acknowledged event_seq
- **Idempotency:** Duplicate idempotency_key replays same commit
- **Patch protocol:** retain/insert/delete operations
- **Conflict UX:** Explicit resolution with merge view

### 5. kjxlkj-agent Automation
- **JSON-defined prompts:** Full prompt in `data/agent-prompt.json`
- **KV memory:** Persists across loops (think_log, plan, steps, context)
- **YOLO mode:** Direct note create/edit within workspace scope
- **XML protocol:** Attribute-less tags (state_add, ram_add, record_add, etc.)
- **No conversation logs:** Only KV store persists (privacy-first)

### 6. Responsive UI
- **2/3 Threshold:** Menu toggle activates at ≤1280px (two-thirds of 1920px)
- **Root URL accessible:** Full app at `http://localhost:8080/`
- **Mobile-first:** Touch targets ≥44px, no horizontal scroll at 320px

---

## Reconstruction TODO — Execution Contract

**Authority:** The TODO list in [`docs/todo/README.md`](docs/todo/README.md) is the ONLY allowed implementation order.

### Quick Start

```bash
# 1. Read the TODO
cat docs/todo/README.md

# 2. Execute stages in order (S00 → S10)
# Each stage links directly to governing specifications

# 3. For each wave:
#    - Read linked docs
#    - Implement behavior
#    - Run: cargo build --workspace && cargo test --workspace
#    - Sync ledgers (CONFORMANCE.md, LIMITATIONS.md, DRIFT_MATRIX.md)
#    - Check TODO box
```

### Stage Overview

| Stage | Objective | Wave Files |
|-------|-----------|------------|
| **S00** | Governance baseline | [stage-00](docs/todo/waves/stage-00-pivot-governance/README.md) |
| **S01** | Runtime skeleton | [stage-01](docs/todo/waves/stage-01-spec-rebuild/README.md) |
| **S02** | Notes + Search core | [stage-02](docs/todo/waves/stage-02-workspace-bootstrap/README.md) |
| **S03** | Realtime integration | [stage-03](docs/todo/waves/stage-03-runtime-integration/README.md) |
| **S04** | kjxlkj-agent | [stage-04](docs/todo/waves/stage-04-schema-and-projections/README.md) |
| **S05** | Security baseline | [stage-05](docs/todo/waves/stage-05-auth-and-security/README.md) |
| **S06** | REST API closure | [stage-06](docs/todo/waves/stage-06-rest-api/README.md) |
| **S07** | WebSocket sync | [stage-07](docs/todo/waves/stage-07-websocket-sync/README.md) |
| **S08** | Frontend + static | [stage-08](docs/todo/waves/stage-08-frontend-and-static-hosting/README.md) |
| **S09** | CI + performance | [stage-09](docs/todo/waves/stage-09-ci-performance-release/README.md) |
| **S10** | Hardening | [stage-10](docs/todo/waves/stage-10-hardening-and-investigation/README.md) |

---

## Documentation Structure (Complete Index)

### Policy Layer (Governance)
- [`docs/policy/README.md`](docs/policy/README.md) — Governance index
- [`INSTRUCT.md`](docs/policy/INSTRUCT.md) — Operating contract
- [`WORKFLOW.md`](docs/policy/WORKFLOW.md) — Gate sequence
- [`STRUCTURE.md`](docs/policy/STRUCTURE.md) — Directory/file constraints (≤12 items, ≤200 lines)
- [`ROOT_LAYOUT.md`](docs/policy/ROOT_LAYOUT.md) — Allowed root paths

### Specification Layer (Target Behavior)
- [`docs/spec/README.md`](docs/spec/README.md) — Specification index
- **Architecture:** [`docs/spec/architecture/`](docs/spec/architecture/README.md) — Runtime topology, crates, deployment
- **API:** [`docs/spec/api/`](docs/spec/api/README.md) — HTTP, WebSocket, types, errors, OpenAPI
- **Domain:** [`docs/spec/domain/`](docs/spec/domain/README.md) — Notes, search, automation, events, permissions
- **Security:** [`docs/spec/security/`](docs/spec/security/README.md) — Auth, sessions, CSRF, transport
- **Technical:** [`docs/spec/technical/`](docs/spec/technical/README.md) — Testing, performance, migrations, agent
- **UI:** [`docs/spec/ui/`](docs/spec/ui/README.md) — Editor, layout, web-app, UX requirements

### Reference Layer (Verified State)
- [`docs/reference/README.md`](docs/reference/README.md) — State index
- [`CONFORMANCE.md`](docs/reference/CONFORMANCE.md) — Verified behavior
- [`LIMITATIONS.md`](docs/reference/LIMITATIONS.md) — Open gaps
- [`DRIFT_MATRIX.md`](docs/reference/DRIFT_MATRIX.md) — Mismatch tracking
- [`CI.md`](docs/reference/CI.md) — Verification profiles
- [`RELEASE.md`](docs/reference/RELEASE.md) — Release gate
- [`EVIDENCE_INDEX.md`](docs/reference/EVIDENCE_INDEX.md) — Proof artifacts

### Overview Layer (Orientation)
- [`docs/overview/README.md`](docs/overview/README.md) — Orientation index
- [`all-in-docs.md`](docs/overview/all-in-docs.md) — Docs-first doctrine
- [`principles.md`](docs/overview/principles.md) — Design principles
- [`glossary.md`](docs/overview/glossary.md) — Shared terms

### Guides Layer (Operator Playbooks)
- [`docs/guides/README.md`](docs/guides/README.md) — Guides index
- [`QUICKSTART.md`](docs/guides/QUICKSTART.md) — First-run workflow
- [`DOCKER.md`](docs/guides/DOCKER.md) — Local orchestration
- [`API.md`](docs/guides/API.md) — API usage
- [`LIBRARIAN.md`](docs/guides/LIBRARIAN.md) — Agent operation

---

## Key Specifications (Redesigned)

### Search — Next-Gen Vectorization

**Spec:** [`docs/spec/domain/search.md`](docs/spec/domain/search.md)

```
Query → Normalization → [Lexical (BM25) + Semantic (HNSW)] → RRF Fusion → Re-rank → Results
```

- **Embedding:** 768-dim, OpenAI-compatible API (LMStudio/OpenRouter)
- **Index:** pgvector HNSW for ANN search
- **Fusion:** Reciprocal Rank Fusion with k=60
- **Fallback:** Lexical-only if embedding service unavailable

### Editor — Obsidian-Like

**Spec:** [`docs/spec/ui/editor-flow.md`](docs/spec/ui/editor-flow.md)

- Markdown-first with live preview
- Wiki-link autocomplete (`[[` trigger)
- Command palette (Cmd/Ctrl+P)
- Autosave with 600ms debounce
- Conflict resolution UI

### Layout — 2/3 Threshold

**Spec:** [`docs/spec/ui/layout-and-interaction.md`](docs/spec/ui/layout-and-interaction.md)

- **>1280px:** Persistent split navigation
- **≤1280px:** Compact mode with top-right toggle
- Auto-close on note selection (compact mode)

### Agent — JSON Prompts + KV Memory

**Spec:** [`docs/spec/technical/librarian-agent.md`](docs/spec/technical/librarian-agent.md)

- Prompts fully defined in `data/agent-prompt.json`
- KV store persists across loops
- YOLO mode for direct mutations
- XML attrless instruction protocol

---

## Building and Running (After Reconstruction)

### Prerequisites
- Rust toolchain (stable)
- PostgreSQL 16 with pgvector
- Docker & Docker Compose (optional)

### Quick Start (Post-Rebuild)

```bash
# 1. Configure environment
cp .env.example .env
# Edit .env to set DATABASE_URL

# 2. Configure runtime (non-secret)
# Edit data/config.json

# 3. Build
cargo build --workspace

# 4. Test
cargo test --workspace

# 5. Run (requires DATABASE_URL)
cargo run -p kjxlkj-server

# 6. Or use Docker Compose
docker compose up --build
```

---

## Development Conventions

### Docs-First Governance
- Canonical behavior defined in `/docs`
- Implementation traceable to spec
- TODO execution order is mandatory

### Testing Contract

**Spec:** [`docs/spec/technical/testing.md`](docs/spec/technical/testing.md)

| Tier | Purpose | Examples |
|------|---------|----------|
| **T0** | Unit/property | Deterministic invariants |
| **T1** | Integration | HTTP/WS/DB tests |
| **T2** | E2E | Browser + API assertions |

**Mandatory Acceptance IDs:**
- `API-NOTE-01/02`: Note creation, datetime title, ID stability
- `API-SEARCH-01/02/03`: Lexical/semantic/hybrid search
- `WS-04/05/06`: Idempotency, replay, event streaming
- `E2E-*`: Editor UX, layout responsiveness
- `AGENT-*`: Prompt loading, KV memory, YOLO mode

### Code Style
- **File limit:** Max 200 lines per `.rs`/`.ts` file
- **Determinism:** Bounded timeouts, explicit diagnostics
- **Async:** Tokio-based, non-blocking IO

---

## Repository Layout (Docs-Only Baseline)

```
.
├── README.md           # Project index
├── LICENSE             # MIT license
├── .env.example        # Secret template
├── .gitignore          # Repository hygiene
├── QWEN.md             # This file — project context
├── data/
│   ├── config.json     # Non-secret runtime config
│   └── agent-prompt.json  # kjxlkj-agent prompts
├── docs/               # Canonical contract (120+ files)
│   ├── policy/         # Governance
│   ├── spec/           # Target behavior
│   ├── reference/      # Verified state
│   ├── todo/           # Execution order
│   ├── overview/       # Orientation
│   └── guides/         # Playbooks
└── .github/            # CI automation
```

**After reconstruction (State B):**
- `Cargo.toml`, `Cargo.lock` — Workspace manifests
- `src/crates/` — 10 Rust crates
- `src/frontend/` — TypeScript app
- `migrations/` — PostgreSQL schemas
- `Dockerfile`, `docker-compose.yml` — Optional orchestration

---

## Related Documentation

- **TODO Execution:** [`docs/todo/README.md`](docs/todo/README.md)
- **Quickstart:** [`docs/guides/QUICKSTART.md`](docs/guides/QUICKSTART.md)
- **API Spec:** [`docs/spec/api/README.md`](docs/spec/api/README.md)
- **Architecture:** [`docs/spec/architecture/README.md`](docs/spec/architecture/README.md)
- **Testing:** [`docs/spec/technical/testing.md`](docs/spec/technical/testing.md)

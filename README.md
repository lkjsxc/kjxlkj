# kjxlkj

**All-in-docs workspace-suite platform** for collaborative notes, search, and `kjxlkj-agent` automation.

---

## Current State: Runtime Implementation Complete

**Status:** Full platform rebuilt from specifications.

- ✅ **120 documentation files** — Complete specification suite
- ✅ **Search redesigned** — Next-gen hybrid pipeline with RRF fusion
- ✅ **Editor spec enhanced** — Obsidian-like markdown workspace
- ✅ **Agent updated** — JSON prompts + KV memory (YOLO mode)
- ✅ **Layout specified** — 2/3 threshold (1280px breakpoint)
- ✅ **Runtime implemented** — 10 Rust crates + TypeScript frontend
- ✅ **Cleanup complete** — `tmp/`, `log/`, `docs/logs/` do not exist

---

## Quick Start

### Option 1: Docker (Recommended)

**Prerequisites:**
- Docker & Docker Compose

**Run with Docker Compose:**
```bash
docker compose up --build
```

Access the application at `http://localhost:8080`

See [Docker Guide](docs/guides/DOCKER.md) for detailed instructions.

### Option 2: Local Build

**1. Read the TODO**

```bash
cat docs/todo/README.md
```

**2. Build the Project**

**Prerequisites:**
- Rust toolchain (stable)
- Node.js 18+ (for frontend)
- PostgreSQL 16 with pgvector (optional, for persistence)

**Build Backend:**
```bash
cargo build --workspace
```

**Build Frontend:**
```bash
cd src/frontend/app
npm install
npm run build
```

**3. Run the Server**

```bash
cargo run -p kjxlkj-server
```

Server listens on `http://0.0.0.0:8080`

**4. Access the Application**

- **Web UI:** `http://localhost:8080/`
- **Health Check:** `http://localhost:8080/api/healthz`
- **API Docs:** `http://localhost:8080/api/docs`

---

## Key Specifications

### Search — Next-Generation Hybrid Pipeline

**Spec:** [docs/spec/domain/search.md](docs/spec/domain/search.md)

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

**Spec:** [docs/spec/ui/editor-flow.md](docs/spec/ui/editor-flow.md)

- Plain markdown source as first-class editing surface
- Live preview (split-pane or toggle)
- Wiki-link autocomplete on `[[` trigger
- Command palette (Cmd/Ctrl+P) with 12+ commands
- Dual-buffer: Synced snapshot + local draft with autosave (600ms debounce)
- Conflict resolution with merge view UI

### kjxlkj-agent — JSON Prompts + KV Memory

**Spec:** [docs/spec/technical/librarian-agent.md](docs/spec/technical/librarian-agent.md)

- Prompts fully defined in `data/agent-prompt.json`
- KV memory persists across loops (think_log, plan, steps, context)
- YOLO mode: Direct note create/edit within workspace scope
- XML protocol: Attribute-less tags (state_add, ram_add, record_add, etc.)
- No conversation logs: Only KV store persists (privacy-first)

### Layout — 2/3 Threshold

**Spec:** [docs/spec/ui/layout-and-interaction.md](docs/spec/ui/layout-and-interaction.md)

- **>1280px:** Persistent split navigation
- **≤1280px:** Compact mode with top-right toggle
- **Auto-close:** On note selection (compact mode)
- **Touch targets:** ≥44px minimum
- **320px support:** No horizontal scroll

### Notes — ID + Title Separation

**Spec:** [docs/spec/domain/notes.md](docs/spec/domain/notes.md)

- **note_id:** Immutable UUID (stable identity)
- **title:** Mutable display name
- **Default title:** Auto-names with datetime (`YYYY-MM-DD HH:mm:ss`) when untitled
- **Optimistic concurrency:** Version-based conflict detection (409 on mismatch)
- **Event sourcing:** Immutable event history per note stream

---

## Repository Layout

```
.
├── README.md                 # This file
├── LICENSE                   # MIT license
├── .env.example              # Secret template
├── .gitignore                # Repository hygiene
├── Cargo.toml                # Workspace manifest
├── docker-compose.yml        # Optional orchestration
├── Dockerfile                # Container build
├── QWEN.md                   # Project context
├── data/
│   ├── config.json           # Non-secret runtime config
│   └── agent-prompt.json     # kjxlkj-agent prompts
├── docs/                     # Canonical contract (114 files)
│   ├── policy/               # Governance (5 files)
│   ├── overview/             # Orientation (4 files)
│   ├── spec/                 # Target behavior (47 files)
│   ├── reference/            # Verified state (8 files)
│   ├── guides/               # Playbooks (6 files)
│   └── todo/                 # Execution order (44 files)
├── migrations/               # PostgreSQL schemas
├── src/                      # Empty (deleted for rebuild)
└── static/                   # Frontend assets
```

---

## Documentation Structure

| Layer | Directory | Files | Purpose |
|-------|-----------|-------|---------|
| **Policy** | `docs/policy/` | 5 | Governance invariants |
| **Overview** | `docs/overview/` | 4 | Orientation and glossary |
| **Specification** | `docs/spec/` | 50 | Target behavior |
| **Reference** | `docs/reference/` | 8 | Verified state |
| **Guides** | `docs/guides/` | 6 | Operator playbooks |
| **TODO** | `docs/todo/` | 46 | Execution order |
| docs/README.md | — | 1 | Documentation index |
| **Total** | — | **120** | — |

---

## Related Documentation

- **TODO Execution:** [docs/todo/README.md](docs/todo/README.md)
- **Quickstart:** [docs/guides/QUICKSTART.md](docs/guides/QUICKSTART.md)
- **API Spec:** [docs/spec/api/README.md](docs/spec/api/README.md)
- **Architecture:** [docs/spec/architecture/README.md](docs/spec/architecture/README.md)
- **Testing:** [docs/spec/technical/testing.md](docs/spec/technical/testing.md)

---

## License

MIT License — see [LICENSE](LICENSE) for details.

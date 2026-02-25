# Final File Structure Contract

**Back:** [Architecture Root](/docs/spec/architecture/README.md)

Defines canonical repository structure for:
1. **State A:** Docs-only baseline (current authoritative state)
2. **State B:** Reconstructed runtime target (completion target)

---

## State A: Docs-Only Baseline

```
kjxlkj/
├── README.md                    # Project index
├── LICENSE                      # MIT license
├── .gitignore                   # Repository hygiene
├── .env.example                 # Secret template
├── QWEN.md                      # Project context
├── Cargo.toml                   # Workspace manifests
├── Cargo.lock                   # Dependency lock
├── Dockerfile                   # Container build
├── docker-compose.yml           # Optional orchestration
├── .dockerignore                # Build context hygiene
├── data/
│   ├── config.json              # Non-secret runtime config
│   └── agent-prompt.json        # kjxlkj-agent prompts
└── docs/                        # Canonical contract (120 files)
    ├── README.md                # Documentation index
    ├── policy/                  # Governance (5 files)
    ├── overview/                # Orientation (4 files)
    ├── spec/                    # Target behavior (50 files)
    ├── reference/               # Verified state (8 files)
    ├── guides/                  # Operator playbooks (6 files)
    └── todo/                    # Execution order (46 files)
```

**Constraints:**
- `tmp/` MUST NOT exist
- `log/` MUST NOT exist
- `docs/logs/` MUST NOT exist
- `src/` MAY exist but MUST be empty or contain only reconstructible scaffolding

---

## State B: Reconstructed Runtime Target

```
kjxlkj/
├── README.md                    # Project index
├── LICENSE                      # MIT license
├── .gitignore                   # Repository hygiene
├── .env.example                 # Secret template
├── QWEN.md                      # Project context
├── Cargo.toml                   # Workspace manifests
├── Cargo.lock                   # Dependency lock
├── Dockerfile                   # Container build
├── docker-compose.yml           # Optional orchestration
├── .dockerignore                # Build context hygiene
├── data/
│   ├── config.json              # Non-secret runtime config
│   └── agent-prompt.json        # kjxlkj-agent prompts
├── migrations/                  # PostgreSQL schemas (8 files)
│   ├── 001_users_sessions.sql
│   ├── 002_workspaces.sql
│   ├── 003_projects.sql
│   ├── 004_notes_events.sql
│   ├── 005_search.sql
│   ├── 006_automation.sql
│   ├── 007_attachments.sql
│   └── 008_idempotency.sql
├── src/                         # Runtime source code
│   ├── crates/                  # 10 Rust crates
│   │   ├── app/
│   │   │   └── kjxlkj-server/
│   │   │       ├── Cargo.toml
│   │   │       └── src/
│   │   │           ├── main.rs
│   │   │           └── config.rs
│   │   ├── http/
│   │   │   └── kjxlkj-http/
│   │   │       ├── Cargo.toml
│   │   │       └── src/
│   │   │           ├── lib.rs
│   │   │           ├── routes.rs
│   │   │           ├── handlers/
│   │   │           │   ├── mod.rs
│   │   │           │   ├── note.rs
│   │   │           │   ├── workspace.rs
│   │   │           │   ├── search.rs
│   │   │           │   └── auth.rs
│   │   │           └── state.rs
│   │   ├── ws/
│   │   │   └── kjxlkj-ws/
│   │   │       ├── Cargo.toml
│   │   │       └── src/
│   │   │           ├── lib.rs
│   │   │           ├── handler.rs
│   │   │           ├── protocol.rs
│   │   │           └── session.rs
│   │   ├── domain/
│   │   │   └── kjxlkj-domain/
│   │   │       ├── Cargo.toml
│   │   │       └── src/
│   │   │           ├── lib.rs
│   │   │           ├── note.rs
│   │   │           ├── workspace.rs
│   │   │           ├── event.rs
│   │   │           ├── search.rs
│   │   │           └── automation.rs
│   │   ├── db/
│   │   │   └── kjxlkj-db/
│   │   │       ├── Cargo.toml
│   │   │       └── src/
│   │   │           ├── lib.rs
│   │   │           ├── config.rs
│   │   │           ├── error.rs
│   │   │           ├── repo/
│   │   │           │   ├── mod.rs
│   │   │           │   ├── note.rs
│   │   │           │   └── workspace.rs
│   │   │           └── pool.rs
│   │   ├── auth/
│   │   │   └── kjxlkj-auth/
│   │   │       ├── Cargo.toml
│   │   │       └── src/
│   │   │           ├── lib.rs
│   │   │           ├── session.rs
│   │   │           ├── user.rs
│   │   │           └── csrf.rs
│   │   ├── search/
│   │   │   └── kjxlkj-search/
│   │   │       ├── Cargo.toml
│   │   │       └── src/
│   │   │           ├── lib.rs
│   │   │           ├── lexical.rs
│   │   │           ├── semantic.rs
│   │   │           ├── fusion.rs
│   │   │           └── embedding.rs
│   │   ├── rbac/
│   │   │   └── kjxlkj-rbac/
│   │   │       ├── Cargo.toml
│   │   │       └── src/
│   │   │           ├── lib.rs
│   │   │           ├── permission.rs
│   │   │           └── policy.rs
│   │   ├── automation/
│   │   │   └── kjxlkj-automation/
│   │   │       ├── Cargo.toml
│   │   │       └── src/
│   │   │           ├── lib.rs
│   │   │           ├── agent.rs
│   │   │           ├── rule.rs
│   │   │           └── run.rs
│   │   └── workspace/
│   │       └── kjxlkj-workspace/
│   │           ├── Cargo.toml
│   │           └── src/
│   │               ├── lib.rs
│   │               ├── service.rs
│   │               └── project.rs
│   └── frontend/
│       └── app/
│           ├── package.json
│           ├── tsconfig.json
│           ├── vite.config.ts
│           ├── index.html
│           └── src/
│               ├── main.tsx
│               ├── App.tsx
│               ├── index.css
│               ├── api/
│               │   ├── client.ts
│               │   └── types.ts
│               ├── components/
│               │   ├── app-shell/
│               │   │   ├── AppShell.tsx
│               │   │   ├── Header.tsx
│               │   │   ├── Navigation.tsx
│               │   │   └── Overlay.tsx
│               │   ├── editor/
│               │   │   ├── MarkdownEditor.tsx
│               │   │   ├── TitleInput.tsx
│               │   │   ├── Toolbar.tsx
│               │   │   ├── Preview.tsx
│               │   │   └── WikiLinkAutocomplete.tsx
│               │   ├── command-palette/
│               │   │   ├── CommandPalette.tsx
│               │   │   └── CommandItem.tsx
│               │   ├── search/
│               │   │   ├── SearchBox.tsx
│               │   │   └── SearchResult.tsx
│               │   ├── backlinks/
│               │   │   └── BacklinkPanel.tsx
│               │   └── auth/
│               │       ├── LoginForm.tsx
│               │       ├── SetupForm.tsx
│               │       └── SessionProvider.tsx
│               ├── hooks/
│               │   ├── useAutosave.ts
│               │   ├── useWebSocket.ts
│               │   ├── useConflictResolution.ts
│               │   └── useResponsive.ts
│               ├── state/
│               │   ├── store.ts
│               │   ├── noteSlice.ts
│               │   └── uiSlice.ts
│               └── utils/
│                   ├── debounce.ts
│                   ├── diff.ts
│                   └── markdown.ts
├── static/                      # Built frontend assets
│   ├── index.html
│   ├── assets/
│   │   ├── index-[hash].js
│   │   └── index-[hash].css
│   └── manifest.json
└── docs/                        # Canonical contract (120 files)
    ├── README.md
    ├── policy/
    ├── overview/
    ├── spec/
    ├── reference/
    ├── guides/
    └── todo/
```

---

## File Size Constraints

Per [STRUCTURE.md](/docs/policy/STRUCTURE.md):

| Constraint | Limit | Action |
|------------|-------|--------|
| Source file (.rs, .ts, .tsx) | ≤200 lines | Split into submodules |
| Directory children | ≤12 items | Group by subdomain |
| Test files | ≤300 lines | Split by test scenario |

**Historical Split Targets:**

| Path | Prior Lines | Split Strategy |
|------|-------------|----------------|
| `src/frontend/app/src/components/app-shell.ts` | 422 | Split into `AppShell.tsx`, `Header.tsx`, `Navigation.tsx`, `Overlay.tsx` |
| `src/crates/http/kjxlkj-http/src/routes_note.rs` | 306 | Split into `handlers/note.rs`, `handlers/workspace.rs`, `handlers/search.rs` |
| `src/crates/db/kjxlkj-db/src/repo_note.rs` | 302 | Split into `repo/note.rs` with CRUD in separate impl blocks |
| `src/crates/ws/kjxlkj-ws/src/session.rs` | 229 | Split message handling into `protocol.rs` |
| `src/crates/db/kjxlkj-db/src/repo_automation.rs` | 205 | Split into `repo/automation.rs` with rule/run separation |

---

## Completion Interpretation

A completion claim is **valid only when**:

1. **Runtime tree matches State B** — all directories and files present
2. **Behavior conforms to `/docs/spec`** — all acceptance tests pass
3. **Ledgers in `/docs/reference/` show synchronized evidence** — CONFORMANCE.md, LIMITATIONS.md, DRIFT_MATRIX.md updated
4. **TODO checklists are completed with linked proofs** — all waves executed
5. **File size constraints enforced** — no file exceeds 200 lines
6. **Docker tooling remains optional** — not authoritative for semantics

---

## Prohibitions

The following MUST NOT exist in completion state:

| Path | Reason |
|------|--------|
| `tmp/` | Temporary files must be cleaned up |
| `log/` | Logs are runtime-generated, not committed |
| `docs/logs/` | Implementation logs are non-canonical |
| `*.log` | Log files are runtime-generated |
| `node_modules/` | Dependencies installed, not committed |
| `target/` | Build artifacts, not committed |

---

## Root URL Contract

Per [ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md) and [web-app.md](/docs/spec/ui/web-app.md):

**Allowed root paths:**

| Path | Purpose |
|------|---------|
| `/` | App shell (notes + editor) |
| `/setup` | First-run registration |
| `/login` | Login form |
| `/notes` | Note list view |
| `/notes/{id}` | Direct note link |
| `/search?q=...` | Search results view |
| `/api/*` | REST API endpoints |
| `/ws` | WebSocket endpoint |
| `/assets/*` | Static frontend assets |

**Forbidden root paths:**

| Path | Reason |
|------|--------|
| `/healthz` | Use `/api/healthz` |
| `/readyz` | Use `/api/readyz` |
| `/admin/*` | No admin UI in baseline |
| `/docs/*` | Documentation not served |

---

## Related

- Completion map: [completion-file-map.md](completion-file-map.md)
- Root layout policy: [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
- TODO execution: [/docs/todo/README.md](/docs/todo/README.md)
- Source layout: [source-layout.md](source-layout.md)
- Crates: [crates.md](crates.md)

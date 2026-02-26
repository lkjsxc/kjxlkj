# Final File Structure Contract

**Back:** [Architecture Root](/docs/spec/architecture/README.md)

Defines canonical repository structure for:
1. **State A:** docs-only reset baseline
2. **State B:** reconstructed runtime target

---

## State A: Docs-Only Baseline

```
kjxlkj/
├── README.md                    # Project index
├── LICENSE                      # License
├── .gitignore                   # Repository hygiene
├── .env.example                 # Secret template
├── data/
│   ├── config.json              # Non-secret runtime config
│   └── agent-prompt.json        # kjxlkj-agent prompts
└── docs/                        # Canonical contract
    ├── README.md                # Documentation index
    ├── policy/                  # Governance (5 files)
    ├── overview/                # Orientation (4 files)
    ├── spec/                    # Target behavior
    ├── reference/               # Verified state
    ├── guides/                  # Operator playbooks (6 files)
    └── todo/                    # Execution order
```

**Constraints:**
- `tmp/` MUST NOT exist
- `log/` MUST NOT exist
- `docs/logs/` MUST NOT exist
- `src/`, `migrations/`, `static/`, and `target/` MUST NOT exist in canonical docs-only commits

---

## State B: Reconstructed Runtime Target

```
kjxlkj/
├── README.md
├── LICENSE
├── .gitignore
├── .env.example
├── Cargo.toml                   # Regenerated from workspace-manifest spec
├── Cargo.lock
├── Dockerfile                   # Optional
├── docker-compose.yml           # Optional
├── .dockerignore                # Optional
├── data/
│   ├── config.json
│   └── agent-prompt.json
├── migrations/                  # Regenerated from migrations spec
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
│               ├── app/
│               │   ├── router.tsx
│               │   ├── providers.tsx
│               │   └── guards/
│               │       ├── RequireAuth.tsx
│               │       └── RequireSetupState.tsx
│               ├── api/
│               │   ├── client.ts
│               │   ├── types.ts
│               │   ├── errors.ts
│               │   └── contracts/
│               │       ├── auth.contract.ts
│               │       ├── notes.contract.ts
│               │       └── search.contract.ts
│               ├── communication/
│               │   ├── http/
│               │   │   ├── request-id.ts
│               │   │   ├── csrf.ts
│               │   │   ├── idempotency.ts
│               │   │   ├── retry-policy.ts
│               │   │   └── error-envelope.ts
│               │   ├── ws/
│               │   │   ├── client.ts
│               │   │   ├── protocol.ts
│               │   │   ├── replay.ts
│               │   │   ├── reconnect.ts
│               │   │   └── stale-cursor.ts
│               │   └── diagnostics/
│               │       ├── request-log.ts
│               │       └── transport-metrics.ts
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
│               ├── features/
│               │   ├── notes/
│               │   │   ├── list/
│               │   │   ├── create/
│               │   │   └── detail/
│               │   ├── editor/
│               │   │   ├── autosave/
│               │   │   ├── conflict/
│               │   │   └── wikilink/
│               │   ├── search/
│               │   └── automation/
│               ├── hooks/
│               │   ├── useAutosave.ts
│               │   ├── useWebSocket.ts
│               │   ├── useConflictResolution.ts
│               │   └── useResponsive.ts
│               ├── state/
│               │   ├── store.ts
│               │   ├── sessionSlice.ts
│               │   ├── noteSlice.ts
│               │   ├── editorSlice.ts
│               │   ├── communicationSlice.ts
│               │   └── uiSlice.ts
│               ├── utils/
│                   ├── debounce.ts
│                   ├── diff.ts
│                   └── markdown.ts
│               └── tests/
│                   ├── contract/
│                   │   ├── frontend_http_client_contract.test.ts
│                   │   ├── frontend_error_envelope_contract.test.ts
│                   │   └── frontend_ws_protocol_contract.test.ts
│                   ├── integration/
│                   │   ├── frontend_retry_idempotency.test.ts
│                   │   ├── frontend_stale_cursor_recovery.test.ts
│                   │   └── frontend_auth_session_rotation.test.ts
│                   └── e2e/
│                       ├── editor_autosave.e2e.ts
│                       ├── conflict_integrity.e2e.ts
│                       ├── compact_menu.e2e.ts
│                       └── mobile_320px.e2e.ts
├── static/                      # Built frontend assets (derived)
│   ├── index.html
│   ├── assets/
│   │   ├── index-[hash].js
│   │   └── index-[hash].css
│   └── manifest.json
└── docs/
```

---

## File Size Constraints

Per [STRUCTURE.md](/docs/policy/STRUCTURE.md):

| Constraint | Limit | Action |
|------------|-------|--------|
| Source file (.rs, .ts, .tsx) | <=200 lines | Split into submodules |
| Directory children | <=12 items | Group by subdomain |
| Test files | <=300 lines | Split by scenario |

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

A completion claim is valid only when:

1. runtime tree matches State B
2. behavior conforms to `/docs/spec`
3. mandatory `T0`, `T1`, and `T2` suites are green
4. TODO trace matrix is fully satisfied
5. evidence ledgers are synchronized
6. file size constraints are respected

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
- Build sequence: [BUILD_SEQUENCE.md](BUILD_SEQUENCE.md)

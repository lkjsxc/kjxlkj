# Completion File Map

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Normative map for the intended repository structure at completion.

## Root Paths

| Path | Required | Purpose | Governing Doc |
|---|---|---|---|
| `README.md` | yes | project entrypoint and current operating model | [/docs/README.md](/docs/README.md) |
| `LICENSE` | yes | license text | [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md) |
| `.gitignore` | yes | repository hygiene rules | [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md) |
| `.env.example` | yes | secret template for local runtime | [configuration.md](configuration.md) |
| `data/config.json` | yes | non-secret runtime config | [configuration.md](configuration.md) |
| `docs/` | yes | canonical policy/spec/reference/TODO contract | [/docs/README.md](/docs/README.md) |
| `src/` | yes | Rust + TypeScript runtime implementation | [source-layout.md](source-layout.md) |
| `Cargo.toml` | yes | Rust workspace manifest | [workspace-manifest.md](workspace-manifest.md) |
| `Cargo.lock` | yes | Rust dependency lockfile | [workspace-manifest.md](workspace-manifest.md) |
| `Dockerfile` | yes | single-container runtime image | [deployment.md](deployment.md) |
| `docker-compose.yml` | yes | single-service orchestration | [deployment.md](deployment.md) |
| `docker/entrypoint.sh` | yes | app+postgres supervisor script | [deployment.md](deployment.md) |

## Documentation Paths

| Path | Required | Purpose |
|---|---|---|
| `docs/policy/` | yes | governance, structure rules, precedence |
| `docs/spec/` | yes | normative system behavior |
| `docs/reference/` | yes | evidence, drift, limitations, release state |
| `docs/todo/` | yes | execution program and traceable checklists |
| `docs/guides/` | yes | operator workflows |
| `docs/overview/` | yes | conceptual orientation |

## Runtime Paths

| Path | Required | Purpose |
|---|---|---|
| `src/crates/app/kjxlkj-server/` | yes | process startup and route/service wiring |
| `src/crates/http/kjxlkj-http/` | yes | HTTP APIs and DTO contracts |
| `src/crates/ws/kjxlkj-ws/` | yes | WebSocket protocol and session management |
| `src/crates/domain/kjxlkj-domain/` | yes | core entities, events, patch semantics |
| `src/crates/db/kjxlkj-db/` | yes | SQL schema/migrations/repositories |
| `src/crates/auth/kjxlkj-auth/` | yes | auth/session/CSRF middleware |
| `src/crates/search/kjxlkj-search/` | yes | search and backlink logic |
| `src/crates/rbac/kjxlkj-rbac/` | yes | role/permission enforcement |
| `src/crates/automation/kjxlkj-automation/` | yes | librarian prompt/provider/execution |
| `src/crates/workspace/kjxlkj-workspace/` | yes | workspace/project/view services |
| `src/frontend/app/` | yes | React/Vite web app |

## Prohibited Paths

| Pattern | Reason |
|---|---|
| committed `.env` | secret leak |
| committed runtime secrets in `data/config.json` | non-secret config contract violation |
| handwritten runtime `.js` source | type-safety policy violation |
| `docs/logs/` | historical logs are kept in git history |

## Related

- Final tree: [final-file-structure.md](final-file-structure.md)
- Root allowlist: [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
- Reconstruction plan: [/docs/todo/README.md](/docs/todo/README.md)

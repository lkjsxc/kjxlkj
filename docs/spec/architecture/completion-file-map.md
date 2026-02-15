# Completion File Map

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Normative per-path map for completion.

## Root Paths

| Path | Required in Docs-Only | Required in Runtime | Purpose |
|---|---|---|---|
| `README.md` | yes | yes | project index |
| `LICENSE` | yes | yes | license |
| `.env.example` | yes | yes | secret template |
| `data/config.json` | yes | yes | non-secret runtime config |
| `data/agent-prompt.json` | yes | yes | full agent prompt JSON |
| `docs/` | yes | yes | canonical contract |
| `src/` | no | yes | runtime source tree |
| `Cargo.toml` | no | yes | workspace manifest |
| `Cargo.lock` | no | yes | lockfile |
| `Dockerfile` | no | yes | container build |
| `docker-compose.yml` | no | yes | runtime orchestration |

## Documentation Paths

| Path | Required | Purpose |
|---|---|---|
| `docs/policy/` | yes | governance and constraints |
| `docs/spec/` | yes | target behavior |
| `docs/reference/` | yes | verified current state |
| `docs/todo/` | yes | rebuild execution contract |
| `docs/guides/` | yes | operator workflows |
| `docs/overview/` | yes | orientation and glossary |

## Runtime Paths

| Path | Required | Purpose |
|---|---|---|
| `src/crates/search/kjxlkj-search/` | yes | hybrid lexical+semantic search |
| `src/crates/automation/kjxlkj-automation/` | yes | `kjxlkj-agent` runtime loop |
| `src/frontend/app/` | yes | Obsidian-like markdown UI |

## Forbidden Paths

| Path/Pattern | Reason |
|---|---|
| `tmp/` | temporary intake material only |
| `log/` | transient logs not canonical |
| `docs/logs/` | replaced by reference ledgers |
| committed secrets | policy violation |

## Related

- Final structure: [final-file-structure.md](final-file-structure.md)
- Root policy: [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)

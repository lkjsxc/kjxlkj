# Completion File Map

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Normative path classification for docs-only reset and runtime reconstruction.

## Classification Legend

| Class | Meaning |
|---|---|
| `A0 canonical` | MUST exist in every state |
| `A1 docs-only` | MUST exist in docs-only baseline |
| `B0 runtime-core` | MUST exist in reconstructed runtime |
| `B1 runtime-derived` | Generated during build/deploy, MUST NOT be committed |
| `OPT` | Optional helper |
| `FORBIDDEN` | Must never exist in canonical repo state |

## Root Map

| Path | Class | Notes |
|---|---|---|
| `README.md` | `A0 canonical` | project index |
| `LICENSE` | `A0 canonical` | licensing |
| `.gitignore` | `A0 canonical` | hygiene |
| `.env.example` | `A0 canonical` | secret template |
| `docs/` | `A0 canonical` | source of truth |
| `data/config.json` | `A1 docs-only` | runtime config contract |
| `data/agent-prompt.json` | `A1 docs-only` | agent prompt contract |
| `src/` | `B0 runtime-core` | runtime implementation tree |
| `migrations/` | `B0 runtime-core` | regenerated schema history |
| `Cargo.toml` | `B0 runtime-core` | workspace manifest |
| `Cargo.lock` | `B0 runtime-core` | dependency lock |
| `Dockerfile` | `OPT` | local image helper |
| `docker-compose.yml` | `OPT` | local stack helper |
| `.dockerignore` | `OPT` | Docker hygiene |
| `.github/` | `OPT` | CI workflows |
| `target/` | `B1 runtime-derived` | cargo build output |
| `static/` | `B1 runtime-derived` | built frontend assets |
| `node_modules/` | `B1 runtime-derived` | package cache |

## Documentation Subtrees

| Path | Class | Notes |
|---|---|---|
| `docs/policy/` | `A0 canonical` | governance |
| `docs/overview/` | `A0 canonical` | orientation |
| `docs/spec/` | `A0 canonical` | target behavior |
| `docs/reference/` | `A0 canonical` | evidence and status |
| `docs/guides/` | `A0 canonical` | operator procedures |
| `docs/todo/` | `A0 canonical` | implementation order |

## Prohibited Paths

| Path | Class | Reason |
|---|---|---|
| `tmp/` | `FORBIDDEN` | temp sprawl |
| `log/` | `FORBIDDEN` | non-canonical runtime logs |
| `docs/logs/` | `FORBIDDEN` | violates root policy |
| `*.log` | `FORBIDDEN` | runtime artifact leakage |
| committed secrets | `FORBIDDEN` | security violation |

## Gate Rules

- A docs-only reset is valid only when all `A0` and `A1` paths exist and no `FORBIDDEN` paths exist.
- Runtime completion is valid only when all `B0` paths are present and acceptance evidence is green.
- `B1` paths MAY exist during execution but MUST be removed before release commits.

## Related

- Final structure: [final-file-structure.md](final-file-structure.md)
- Root policy: [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
- Build sequence: [BUILD_SEQUENCE.md](BUILD_SEQUENCE.md)

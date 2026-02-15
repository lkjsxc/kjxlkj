# Runtime Configuration Contract

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Defines required split between non-secret runtime settings and secrets.

## Canonical Sources

| Source | Scope | Secret Allowed |
|---|---|---|
| `data/config.json` | non-secret runtime configuration | no |
| `data/agent-prompt.json` | full agent prompt definition | no |
| `.env` | credentials and tokens only | yes |

## Required `data/config.json` Categories

- logging
- HTTP/static hosting
- database pool/runtime tuning
- websocket runtime tuning
- editor/autosave settings
- search settings (lexical + embeddings)
- automation/agent runtime defaults
- storage paths and limits
- feature flags
- health endpoints

## Agent Configuration Requirements

`data/config.json` MUST include an `agent` section with:

- `name` = `kjxlkj-agent`
- `mode` (`reviewed` or `yolo`)
- `prompt_path` (defaults to `./data/agent-prompt.json`)
- `memory_store_path`
- `retain_full_conversation_logs` (default `false`)

## Secrets (`.env`)

| Variable | Purpose |
|---|---|
| `DATABASE_URL` | PostgreSQL connection string |
| `OPENROUTER_API_KEY` | OpenRouter provider key (if used) |

## Startup Rules

1. load `.env` (if present)
2. load `data/config.json`
3. load `data/agent-prompt.json`
4. validate JSON schemas and required fields
5. fail fast on invalid config or missing required secrets

## Related

- Agent prompt schema: [/docs/spec/technical/agent-prompt-json.md](/docs/spec/technical/agent-prompt-json.md)
- Runtime topology: [runtime.md](runtime.md)

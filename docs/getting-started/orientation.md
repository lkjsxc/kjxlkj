# Orientation

## What `kjxlkj` Is

- `kjxlkj` is a contract-first note service for LLM-operated workflows.
- The documentation under `docs/` is the only active canon for intended behavior.
- Operators use Docker Compose to run and verify the stack.
- Contributors change docs first, then realign code to match.

## Runtime Shape

- `postgres` stores notes, revisions, settings, search data, analytics, and sessions.
- `app` is the Rust web service that runs migrations and serves HTML plus JSON endpoints.
- `verify` runs Rust and docs quality gates inside Compose.
- `visual-verify` runs browser-rendered screenshot checks against the live `app` service.

## Fastest Reading Path

1. Read [compose-quickstart.md](compose-quickstart.md) to boot the service.
2. Read [first-session.md](first-session.md) to create the first admin and note.
3. Read [verification.md](verification.md) to run the full acceptance bundle.
4. Read [where-next.md](where-next.md) for deeper task-specific documentation.

## Key Defaults

- Host port default: `8080`
- Runtime bind host default: `0.0.0.0`
- Runtime bind port default: `8080`
- Untouched session timeout default: `1440` minutes
- Canonical deployment model: one host running Docker Compose

## Canonical Rules

- Use `docker compose`, not legacy `docker-compose`.
- Keep the tracked config template in `.env.example` and the active local file in `.env`.
- Treat PostgreSQL as the only required persistent runtime state store.
- Treat `tmp/visual-artifacts/` as disposable verification output.

# kjxlkj

Contract-first note service for LLM-operated workflows.

## Start Here

1. [docs/README.md](docs/README.md)
2. [docs/vision/purpose.md](docs/vision/purpose.md)
3. [docs/product/surface/routes.md](docs/product/surface/routes.md)
4. [docs/operations/verification/local-runbook.md](docs/operations/verification/local-runbook.md)
5. [docs/operations/quality/gates.md](docs/operations/quality/gates.md)

## Stack

- Rust runtime and CLI
- Actix Web server-rendered HTML
- PostgreSQL for notes, revisions, search, and sessions
- Docker Compose for build and verification

## Current Shape

- `/` is an auth-aware homepage
- `/admin` is the analytics and activity dashboard
- `/admin/settings` is the canonical settings workspace
- `/{ref}` serves notes by alias or Base32 ID

## Rule

If code and docs diverge, update the docs canon first and then realign code.

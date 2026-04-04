# kjxlkj

Contract-first note service for LLM-operated workflows.

## Start Here

1. [docs/README.md](docs/README.md)
2. [docs/vision/purpose.md](docs/vision/purpose.md)
3. [docs/operations/deployment/single-host-compose.md](docs/operations/deployment/single-host-compose.md)
4. [docs/operations/deployment/first-login-and-live-use.md](docs/operations/deployment/first-login-and-live-use.md)
5. [docs/product/surface/routes.md](docs/product/surface/routes.md)
6. [docs/operations/verification/local-runbook.md](docs/operations/verification/local-runbook.md)
7. [docs/operations/quality/gates.md](docs/operations/quality/gates.md)

## Stack

- Rust runtime and CLI
- Actix Web server-rendered HTML
- PostgreSQL for notes, revisions, search, and sessions
- Docker Compose for build and verification

## Current Shape

- `/` is an auth-aware homepage driven only by intro Markdown plus section settings
- `/admin` is the analytics and activity dashboard
- `/admin/settings` is the canonical settings workspace
- `/{ref}` serves notes by alias or Base32 ID
- Admin note editing uses a first-party Markdown textarea with on-demand preview

## Rule

If code and docs diverge, update the docs canon first and then realign code.

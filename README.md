# kjxlkj

`kjxlkj` is a Markdown note system for LLM-operated workflows. It serves notes at root URLs such as `/{alias}` or `/{id}`, gives one admin a live editing workspace with autosave and preview, keeps immutable saved-snapshot history, and exposes curated home, search, favorites, and analytics surfaces that are meant to be operated through AI agents.

Treat [docs/README.md](docs/README.md) as the only active canon for intended behavior, operations, and repository rules.

## Start Here

- Canonical documentation root: [docs/README.md](docs/README.md)
- Fastest local deployment path: [docs/getting-started/compose-quickstart.md](docs/getting-started/compose-quickstart.md)
- First boot and operator handoff: [docs/getting-started/first-session.md](docs/getting-started/first-session.md)
- Full compose verification path: [docs/getting-started/verification.md](docs/getting-started/verification.md)

## Reading Paths

- Mixed operator + contributor: [docs/getting-started/README.md](docs/getting-started/README.md)
- Product and behavior canon: [docs/product/README.md](docs/product/README.md)
- Architecture and data contracts: [docs/architecture/README.md](docs/architecture/README.md)
- Operations runbooks and quality gates: [docs/operations/README.md](docs/operations/README.md)
- Repository rules for LLM-driven changes: [docs/repository/README.md](docs/repository/README.md)

## Current Runtime Shape

- Rust runtime and CLI
- Actix Web server-rendered HTML
- PostgreSQL for notes, revisions, search, and sessions
- Docker Compose for runtime boot and verification

## Rule

If code and docs diverge, update the docs canon first and then realign code.

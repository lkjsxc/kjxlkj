# Runtime Architecture

## Stack

- Rust stable (latest)
- Actix Web + Tokio async runtime
- Server-rendered HTML + HTMX + small JavaScript
- PostgreSQL for admin/session state
- Filesystem for Markdown article source of truth

## Root Layout

- `src/bin/kjxlkj.rs`: binary entrypoint (`serve` + CLI commands)
- `src/app.rs`: startup wiring and route registration
- `src/core/`: pure domain logic
- `src/adapters/`: filesystem and PostgreSQL adapters
- `src/web/`: HTTP handlers and auth guards
- `src/cli/`: machine-friendly command modules
- `content/articles/`: Markdown articles
- `migrations/`: PostgreSQL schema migrations
- `docker-compose.yml`: canonical local/verification orchestration

## Request Paths

- Public routes: `/`, `/article/{slug}`
- Auth routes: `/setup`, `/login`, `/logout`
- Admin routes: `/admin/*`

## Design Principles

- Functional core, imperative shell.
- Async I/O boundaries.
- Deterministic outputs for automation.
- Small modules to satisfy the 200-line source limit.

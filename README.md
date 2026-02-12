# kjxlkj

A documentation-first web application platform for notes and records,
with an Obsidian-style knowledge workflow.

## Current Product Contract

- Runtime: Rust, Tokio, Actix Web.
- Persistence: PostgreSQL.
- Delivery: REST + WebSocket API and static SPA hosting.
- Deployment: single Docker Compose service container running app + database.

## Documentation

All canonical system definitions live in [`docs/`](docs/README.md).

Implementation artifacts are derived from canonical documentation.
When documentation and implementation diverge, documentation is updated first
and drift is recorded in reference ledgers.

## Reconstruction Start

1. Read [`docs/todo/README.md`](docs/todo/README.md).
2. Execute waves in [`docs/todo/waves/README.md`](docs/todo/waves/README.md).
3. Keep ledgers synchronized in [`docs/reference/`](docs/reference/README.md).

## Project Structure

| Path | Purpose |
|---|---|
| `docs/` | Canonical policy, spec, reference, and execution docs |
| `src/` | Reconstructed Rust workspace (derived) |
| `docker-compose.yml` | Single-service deployment entry (derived) |
| `README.md` | Top-level project entry |

## License

See [LICENSE](LICENSE).

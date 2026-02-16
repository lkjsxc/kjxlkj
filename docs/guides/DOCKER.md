# Docker Compose Guide

Back: [/docs/guides/README.md](/docs/guides/README.md)

## Scope

Optional local runtime orchestration for PostgreSQL + migrations + `kjxlkj-server`.

## Prerequisites

1. Docker Engine with Compose plugin
2. Repository root as current working directory

## Start

1. Build and start:
   - `docker compose up --build`
2. Wait until `migrate` exits successfully and `app` is healthy.
3. Open `http://127.0.0.1:8080/api/healthz`.

## Services

- `db`: PostgreSQL 16 with persistent `pg_data` volume
- `migrate`: applies all SQL files from `migrations/` in lexical order
- `app`: runs `kjxlkj-server` on port `8080`

## Stop and Cleanup

1. Stop:
   - `docker compose down`
2. Stop and remove database volume:
   - `docker compose down -v`

## Notes

- Default compose DB URL is `postgres://kjxlkj:password@db:5432/kjxlkj`.
- Deployment semantics remain host-process canonical per
  [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md).
- `.env` is optional for compose; set `OPENROUTER_API_KEY` in your shell or `.env`.

# Quickstart

Back: [/docs/guides/README.md](/docs/guides/README.md)

## Scope

Fast path for rebuilding and running the system with single-container Compose.

## Steps

1. Read [/docs/todo/README.md](/docs/todo/README.md) and execute rebuild waves in order.
2. Reconstruct runtime artifacts (`src/`, `Dockerfile`, `docker-compose.yml`).
3. Start stack: `docker compose up --build`
4. Verify readiness: `curl -fsS http://127.0.0.1:8080/api/readyz`
5. Validate key UX flows using acceptance IDs in
   [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Notes

- Current repository baseline is docs-only by design.
- Runtime startup is possible after reconstruction steps are completed.

## Related

- Docker guide: [DOCKER.md](DOCKER.md)
- Wave program: [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

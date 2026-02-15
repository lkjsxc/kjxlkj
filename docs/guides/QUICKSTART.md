# Quickstart

Back: [/docs/guides/README.md](/docs/guides/README.md)

## Scope

Fast path for rebuilding and running the system with single-container Compose.

## Steps

1. Read [/docs/todo/README.md](/docs/todo/README.md) and execute waves in order.
2. Configure non-secret settings in `data/config.json`.
3. Copy `.env.example` to `.env` and set secrets.
4. Start stack: `docker compose up --build`
5. Verify readiness: `curl -fsS http://127.0.0.1:8080/api/readyz`
6. Validate key UX flows using acceptance IDs in
   [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Notes

- Repository is currently in a docs-only reset state with runtime source removed.
- Runtime startup requires both `data/config.json` and `.env`.
- `.env` is ignored by git and must not be committed.

## Related

- Docker guide: [DOCKER.md](DOCKER.md)
- Wave program: [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

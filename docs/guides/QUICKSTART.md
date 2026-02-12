# Quickstart

Back: [/docs/guides/README.md](/docs/guides/README.md)

## Scope

Quick local startup for reconstructed runtime states.

## Steps

1. Ensure Docker and Docker Compose are available.
2. Provide required environment values.
3. Start stack: `docker compose up --build`.
4. Open app origin and perform first-run registration.
5. Verify readiness endpoint: `GET /api/v1/readyz`.

## Notes

- In docs-only baseline state, runtime artifacts may be absent.
- Use `/docs/todo/` waves to reconstruct missing artifacts.

## Related

- Docker guide: [DOCKER.md](DOCKER.md)
- TODO waves: [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

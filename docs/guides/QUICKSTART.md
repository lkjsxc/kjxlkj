# Quickstart

Back: [/docs/guides/README.md](/docs/guides/README.md)

## Scope

Fast path for All in Docs baseline workflow.

## Steps

1. Open canonical docs directly from repository `docs/`.
2. Read core contracts:
   - [/docs/todo/README.md](/docs/todo/README.md)
   - [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
   - [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
3. Execute wave plan in order to reconstruct typed runtime artifacts.
4. Re-verify acceptance IDs in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md).
5. Verify root Docker artifacts exist; regenerate any missing file via [DOCKER.md](DOCKER.md).

## Notes

- All in Docs governance is always active.
- After Docker launch, use `/api/healthz` and `/api/readyz` to confirm runtime startup.

## Related

- Docker guide: [DOCKER.md](DOCKER.md)
- Wave program: [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

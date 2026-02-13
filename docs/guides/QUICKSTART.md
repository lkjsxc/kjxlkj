# Quickstart

Back: [/docs/guides/README.md](/docs/guides/README.md)

## Scope

Fast path for docs-first reconstruction workflow.

## Steps

1. Launch documentation with one command: `docker compose up --build`
2. Read canonical contract start points:
   - [/docs/todo/README.md](/docs/todo/README.md)
   - [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
   - [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
3. Rebuild runtime artifacts from docs in wave order.
4. Re-verify acceptance IDs in
   [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md).

## Notes

- Current repository baseline is docs-only by design.
- Runtime startup (`/api/readyz`) is expected only after reconstruction.

## Related

- Docker guide: [DOCKER.md](DOCKER.md)
- Wave program: [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

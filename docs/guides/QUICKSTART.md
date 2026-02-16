# Quickstart

Back: [/docs/guides/README.md](/docs/guides/README.md)

## Scope

Fast path for rebuilding the runtime from the docs-only baseline.

## Steps

1. Read [/docs/todo/README.md](/docs/todo/README.md).
2. Execute waves in [/docs/todo/waves/README.md](/docs/todo/waves/README.md) in order.
3. Configure non-secret settings in `data/config.json`.
4. Copy `.env.example` to `.env` and set secrets.
5. For each wave, run build and test gates from [/docs/reference/CI.md](/docs/reference/CI.md).
6. Keep reference ledgers synchronized before checking any wave item.

## Notes

- Repository starts in docs-only reset state.
- Runtime startup is expected only after scaffold waves complete.
- `.env` is ignored by git and must not be committed.

## Related

- Wave program: [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

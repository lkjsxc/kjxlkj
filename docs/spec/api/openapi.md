# OpenAPI Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

## Source of Truth

The canonical OpenAPI document is:

- [openapi.yaml](openapi.yaml)

## Change Rules

- OpenAPI `info.version` MUST reflect the active contract revision.
- Breaking contract changes MUST update `openapi.yaml` and all linked specs.
- Contract additions MUST remain synchronized with `http.md` and `types.md`.

## Validation Gate

- OpenAPI document MUST pass schema validation in CI.
- Example payloads SHOULD match `types.md` contracts.
- Endpoint set MUST remain synchronized with `http.md`.

## Related

- HTTP contract: [http.md](http.md)
- Type contract: [types.md](types.md)
- CI gate: [/docs/reference/CI.md](/docs/reference/CI.md)

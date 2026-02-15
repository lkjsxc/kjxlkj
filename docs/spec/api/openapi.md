# OpenAPI Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

## Source of Truth

Canonical OpenAPI document:

- [openapi.yaml](openapi.yaml)

## Change Rules

- `openapi.yaml` MUST stay synchronized with `http.md` and `types.md`.
- Search contract changes MUST update `search_mode` and result schema.
- Agent contract changes MUST update `kjxlkj_agent` action schema.
- Breaking changes MUST bump `info.version`.

## Validation Gate

- OpenAPI file MUST pass schema validation.
- Example payloads SHOULD match runtime DTOs.

## Related

- HTTP contract: [http.md](http.md)
- Type contract: [types.md](types.md)

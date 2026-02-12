# OpenAPI Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

## Source of Truth

The canonical OpenAPI document is:

- [openapi.v1.yaml](openapi.v1.yaml)

## Versioning Rules

- OpenAPI `info.version` MUST match API major version (`v1`).
- Breaking changes MUST create `/api/v2` and a new OpenAPI document.
- Non-breaking additions MAY extend `v1` schemas and paths.

## Validation Gate

- OpenAPI document MUST pass schema validation in CI.
- Example payloads SHOULD match `types.md` contracts.
- Endpoint set MUST remain synchronized with `http.md`.

## Related

- HTTP contract: [http.md](http.md)
- Type contract: [types.md](types.md)
- CI gate: [/docs/reference/CI.md](/docs/reference/CI.md)

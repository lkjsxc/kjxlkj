# OpenAPI Contract

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

## Source of Truth

The canonical OpenAPI document is:

- [openapi.yaml](openapi.yaml)

## Change Rules

- OpenAPI `info.version` MUST reflect the active contract revision.
- Breaking contract changes MUST update `openapi.yaml` and all linked specs.
- Contract additions MUST remain synchronized with `http.md` and `types.md`.
- Librarian automation behavior MAY reuse existing automation paths, but
  `types.md` and `librarian-xml.md` MUST stay synchronized with any rule/run
  payload updates.

## Validation Gate

- OpenAPI document MUST pass schema validation in CI.
- Example payloads SHOULD match `types.md` contracts.
- Endpoint set MUST remain synchronized with `http.md`.
- If librarian payload contracts evolve, update OpenAPI examples and
  `librarian-xml.md` in the same change.

## Related

- HTTP contract: [http.md](http.md)
- Type contract: [types.md](types.md)
- Librarian protocol: [librarian-xml.md](librarian-xml.md)
- CI gate: [/docs/reference/CI.md](/docs/reference/CI.md)

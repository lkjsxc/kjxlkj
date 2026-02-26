# Wave 022: Metadata, Search, Backlinks, and Attachments

Back: [/docs/todo/waves/stage-02-workspace-bootstrap/README.md](/docs/todo/waves/stage-02-workspace-bootstrap/README.md)

## Relevant Documents

- [/docs/spec/domain/metadata.md](/docs/spec/domain/metadata.md)
- [/docs/spec/domain/search.md](/docs/spec/domain/search.md)
- [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md)
- [/docs/spec/domain/export.md](/docs/spec/domain/export.md)
- [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [/docs/spec/api/errors.md](/docs/spec/api/errors.md)

## Restructure Steps

- [ ] restructure-step S02-W022-01: implement metadata/tag upsert/delete semantics from [/docs/spec/domain/metadata.md](/docs/spec/domain/metadata.md) [doc-link](/docs/spec/domain/metadata.md)
- [ ] restructure-step S02-W022-02: implement backlink and search behavior from [/docs/spec/domain/search.md](/docs/spec/domain/search.md) [doc-link](/docs/spec/domain/search.md)
- [ ] restructure-step S02-W022-03: implement chunked attachment and media-note behavior from [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) [doc-link](/docs/spec/domain/attachments.md)
- [ ] restructure-step S02-W022-04: enforce attachment and metadata HTTP responses from [/docs/spec/api/http.md](/docs/spec/api/http.md) [doc-link](/docs/spec/api/http.md)
- [ ] restructure-step S02-W022-05: enforce boundary error codes from [/docs/spec/api/errors.md](/docs/spec/api/errors.md) [doc-link](/docs/spec/api/errors.md)

## Verification Hooks

- [ ] restructure-step S02-W022-V01: run `API-SEARCH-01`, `API-SEARCH-02`, and `API-SEARCH-03` checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [ ] restructure-step S02-W022-V02: sync search/attachment gaps in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) [doc-link](/docs/reference/LIMITATIONS.md)

## Mandatory Build and Test Gate

- [ ] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [ ] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [ ] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

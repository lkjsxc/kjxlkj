# Crates

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

The implementation is a Cargo workspace rooted at `src/crates/`.

## Topology Requirements

| Requirement | Value |
|---|---|
| Canonical crate set | 7 primary crates |
| Group roots | `app`, `http`, `ws`, `domain`, `db`, `auth`, `search` |
| Expansion rule | additional crates MAY be added only with spec justification |

## Canonical Workspace Members

| Group | Crate | Path |
|---|---|---|
| app | `kjxlkj-server` | `src/crates/app/kjxlkj-server` |
| http | `kjxlkj-http` | `src/crates/http/kjxlkj-http` |
| ws | `kjxlkj-ws` | `src/crates/ws/kjxlkj-ws` |
| domain | `kjxlkj-domain` | `src/crates/domain/kjxlkj-domain` |
| db | `kjxlkj-db` | `src/crates/db/kjxlkj-db` |
| auth | `kjxlkj-auth` | `src/crates/auth/kjxlkj-auth` |
| search | `kjxlkj-search` | `src/crates/search/kjxlkj-search` |

## Decomposition Rules

- Runtime wiring MUST stay in `app`.
- HTTP/WS transport code MUST stay outside domain core logic.
- DB repositories MUST be isolated from route-layer request types.
- Note editor and UX semantics MUST remain spec-driven and test-backed.

## Related

- Source layout: [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- Workspace manifest: [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)

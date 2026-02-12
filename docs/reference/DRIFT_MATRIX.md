# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-level mismatch tracking for reconstruction.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates canonical spec |
| `M2 missing feature` | required capability is absent |
| `M3 undocumented behavior` | behavior exists but is not canonically specified |
| `M4 verification gap` | deterministic evidence is insufficient |
| `M5 stale docs` | docs and stronger evidence contradict |

## Matrix

| Req ID | Canonical Document | Requirement | Observed Status | Mismatch Class | Action |
|---|---|---|---|---|---|
| `R-RUNTIME-01` | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | runtime startup and supervision path exists | spec-only | `M2` | implement |
| `R-API-01` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | API v1 endpoints are reachable | spec-only | `M2` | implement |
| `R-WS-01` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | WS patch/replay protocol is reachable | spec-only | `M2` | implement |
| `R-UI-01` | [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | independent scroll panes + responsive single-layout UX exists | spec-only | `M2` | implement |
| `R-UI-02` | [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md) | autosave markdown editor and editable title exist | spec-only | `M2` | implement |
| `R-DOMAIN-01` | [/docs/spec/domain/note-types.md](/docs/spec/domain/note-types.md) | note kinds (`settings`, media note types) are implemented | spec-only | `M2` | implement |
| `R-SEARCH-01` | [/docs/spec/domain/search.md](/docs/spec/domain/search.md) | full-text search and ranking exists | spec-only | `M2` | implement |
| `R-MEDIA-01` | [/docs/spec/domain/attachments.md](/docs/spec/domain/attachments.md) | standalone image/video note uploads are supported | spec-only | `M2` | implement |
| `R-DELETE-01` | [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md) | note deletion path is user-reachable and filtered by default | spec-only | `M2` | implement |
| `R-ISSUE-01` | [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md) | historical findings are covered by regression tests | partial | `M4` | test-add |
| `R-DOC-PIVOT-01` | [/docs/spec/README.md](/docs/spec/README.md) | web-server docs are canonical | aligned | closed | keep synchronized |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 0 |
| `M2 missing feature` | 9 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 1 |
| `M5 stale docs` | 0 |

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

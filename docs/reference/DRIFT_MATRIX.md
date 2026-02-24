# Drift Matrix

**Back:** [Reference Root](/docs/reference/README.md)

---

## Mismatch Classes

| Class | Meaning |
|-------|---------|
| `M1 correctness` | Runtime behavior violates spec |
| `M2 missing feature` | Specified capability absent |
| `M3 undocumented behavior` | Implementation exists without spec |
| `M4 verification gap` | Behavior exists but evidence insufficient |
| `M5 stale docs` | Docs contradict stronger evidence |

---

## Matrix (Docs-Only Baseline)

| Req ID | Canonical Document | Requirement | Observed Status | Mismatch Class | Action |
|--------|-------------------|-------------|-----------------|----------------|--------|
| `R-DOC-01` | [Docs Root](/docs/README.md) | Docs are canonical contract | Aligned — docs present | Closed | Maintain |
| `R-TODO-01` | [TODO Root](/docs/todo/README.md) | TODO drives rebuild with direct links | Aligned — all links present | Closed | Maintain |
| `R-ROOT-01` | [Root Layout](/docs/policy/ROOT_LAYOUT.md) | Root allowlist compliant | Aligned — no forbidden paths | Closed | Maintain |
| `R-RUNTIME-01` | [Runtime Model](/docs/spec/architecture/runtime.md) | Runtime services reachable | Source deleted — S01 pending | `M2` | Execute S01 |
| `R-HTTP-01` | [HTTP Contract](/docs/spec/api/http.md) | HTTP contract live | Not implemented — S06 pending | `M2` | Execute S06 |
| `R-WS-01` | [WebSocket](/docs/spec/api/websocket.md) | WS replay/idempotency live | Not implemented — S07 pending | `M2` | Execute S07 |
| `R-SEARCH-01` | [Search Spec](/docs/spec/domain/search.md) | Hybrid search live | Redesigned — S02-W022 pending | `M2` | Execute S02 |
| `R-UI-01` | [Editor Flow](/docs/spec/ui/editor-flow.md) | Editor UX implemented | Obsidian-like — S08 pending | `M2` | Execute S08 |
| `R-AGENT-01` | [Agent Contract](/docs/spec/technical/librarian-agent.md) | Agent loop implemented | JSON+KV spec — S04 pending | `M2` | Execute S04 |
| `R-TEST-01` | [Testing](/docs/spec/technical/testing.md) | Acceptance pack executable | Tests deleted — S09 pending | `M4` | Execute S09 |
| `R-AUTH-01` | [Security](/docs/spec/security/README.md) | Auth/session/CSRF live | Not implemented — S05 pending | `M2` | Execute S05 |

---

## Summary

| Class | Open |
|-------|------|
| `M1 correctness` | 0 |
| `M2 missing feature` | 8 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 1 |
| `M5 stale docs` | 0 |

**All open items are expected** — repository is in docs-only baseline state ready for reconstruction.

---

## Related

- [Conformance](CONFORMANCE.md) — verified state
- [Limitations](LIMITATIONS.md) — open gaps
- [TODO Contract](/docs/todo/README.md) — execution order

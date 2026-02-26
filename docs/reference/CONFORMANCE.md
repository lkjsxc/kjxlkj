# Conformance

**Back:** [Reference Root](/docs/reference/README.md)

---

## Status Vocabulary

| Status | Meaning |
|--------|---------|
| `verified` | Deterministic evidence exists |
| `partial` | Behavior exists but evidence incomplete |
| `spec-only` | Specified but not currently implemented |
| `blocked` | Contradicted or impossible in current state |

---

## Current Snapshot (Docs-Only Rebuild Baseline)

**Repository state:** documentation rewritten for reconstruction; runtime artifacts intentionally removable/rebuildable.

- ✅ governance and TODO sequencing remain canonical
- ✅ architecture includes explicit build sequence and cross-spec dependency map
- ✅ communication contracts rewritten with deterministic HTTP/WS/error semantics
- ✅ security contracts rewritten with explicit auth/session/csrf rules
- ✅ testing contract upgraded to mandatory full `T0/T1/T2`
- ✅ CI gate updated to enforce acceptance mapping and evidence discipline
- ✅ `docs/logs/` is prohibited by policy and removed
- ✅ reconstructable runtime/build artifacts were removed for docs-only reset
- ⏳ runtime implementation status intentionally deferred to reconstruction waves

---

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|--------|----------------|--------|----------|
| Docs governance | [Policy Root](/docs/policy/README.md) | `verified` | policy and TODO invariants |
| Architecture sequencing | [/docs/spec/architecture/BUILD_SEQUENCE.md](/docs/spec/architecture/BUILD_SEQUENCE.md) | `verified` | explicit dependency order |
| Spec interactions | [/docs/spec/architecture/SPEC_INTERACTIONS.md](/docs/spec/architecture/SPEC_INTERACTIONS.md) | `verified` | cross-spec ownership map |
| HTTP contract | [HTTP Spec](/docs/spec/api/http.md) | `verified` | endpoint-level auth/csrf/idempotency table |
| WebSocket contract | [WS Spec](/docs/spec/api/websocket.md) | `verified` | replay + stale cursor schema |
| Error model | [Error Spec](/docs/spec/api/errors.md) | `verified` | deterministic HTTP/WS code mapping |
| Auth/session/csrf | [/docs/spec/security/README.md](/docs/spec/security/README.md) | `verified` | deterministic security outcomes |
| Testing contract | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `verified` | mandatory full `T0/T1/T2` |
| Runtime implementation | [/docs/todo/README.md](/docs/todo/README.md) | `spec-only` | rebuilt via waves |

---

## Acceptance Test Coverage (Target)

All acceptance tests are specified and mapped. Runtime evidence is pending reconstruction.

| Acceptance ID | Status | Governing Spec |
|---------------|--------|----------------|
| `API-NOTE-01` | `spec-only` | [notes.md](/docs/spec/domain/notes.md) |
| `API-NOTE-02` | `spec-only` | [notes.md](/docs/spec/domain/notes.md) |
| `API-SEARCH-01` | `spec-only` | [search.md](/docs/spec/domain/search.md) |
| `API-SEARCH-02` | `spec-only` | [search.md](/docs/spec/domain/search.md) |
| `API-SEARCH-03` | `spec-only` | [search.md](/docs/spec/domain/search.md) |
| `WS-04` | `spec-only` | [websocket.md](/docs/spec/api/websocket.md) |
| `WS-05` | `spec-only` | [websocket.md](/docs/spec/api/websocket.md) |
| `E2E-12` | `spec-only` | [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) |
| `E2E-19` | `spec-only` | [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) |
| `E2E-23` | `spec-only` | [web-app.md](/docs/spec/ui/web-app.md) |
| `E2E-24` | `spec-only` | [editor-flow.md](/docs/spec/ui/editor-flow.md) |
| `E2E-25` | `spec-only` | [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) |
| `AGENT-01` | `spec-only` | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) |
| `AGENT-02` | `spec-only` | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) |
| `AGENT-03` | `spec-only` | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) |
| `AGENT-04` | `spec-only` | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) |

---

## Implementation Readiness

All specifications are ready for execution in wave order:

| Stage | Status | Governing Docs |
|-------|--------|----------------|
| S00: Governance Baseline | ✅ Complete | [policy/README.md](/docs/policy/README.md) |
| S01: Runtime Skeleton | ⏳ Ready | [crates.md](/docs/spec/architecture/crates.md) |
| S02: Notes + Search Core | ⏳ Ready | [notes.md](/docs/spec/domain/notes.md), [search.md](/docs/spec/domain/search.md) |
| S03: Realtime Integration | ⏳ Ready | [http.md](/docs/spec/api/http.md), [websocket.md](/docs/spec/api/websocket.md) |
| S04: Automation + Agent | ⏳ Ready | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) |
| S05: Security Baseline | ⏳ Ready | [auth.md](/docs/spec/security/auth.md), [sessions.md](/docs/spec/security/sessions.md) |
| S06: REST API Closure | ⏳ Ready | [http.md](/docs/spec/api/http.md) |
| S07: WebSocket Sync | ⏳ Ready | [websocket.md](/docs/spec/api/websocket.md) |
| S08: Frontend + Static | ⏳ Ready | [editor-flow.md](/docs/spec/ui/editor-flow.md), [web-app.md](/docs/spec/ui/web-app.md) |
| S09: CI + Performance | ⏳ Ready | [CI.md](/docs/reference/CI.md), [performance.md](/docs/spec/technical/performance.md) |
| S10: Hardening | ⏳ Ready | [STRUCTURE.md](/docs/policy/STRUCTURE.md), [RELEASE.md](/docs/reference/RELEASE.md) |

---

## Closure Rule

No row may move to `verified` without:

1. **Deterministic test evidence** — acceptance IDs pass
2. **Runtime reachability** — behavior implemented and accessible
3. **Synchronized reference updates** — CONFORMANCE.md, LIMITATIONS.md, DRIFT_MATRIX.md updated
4. **TODO completion** — wave checklists completed with linked proofs
5. **Trace matrix closure** — TODO and test matrices have no unresolved rows

---

## Related

- [Limitations](LIMITATIONS.md) — open gaps
- [Drift Matrix](DRIFT_MATRIX.md) — mismatch tracking
- [TODO Contract](/docs/todo/README.md) — execution order
- [Release Gate](RELEASE.md) — release criteria
- [TODO Trace Matrix](TODO_TRACE_MATRIX.md) — TODO-to-spec-to-artifact mapping

# Drift Matrix

**Back:** [Reference Root](/docs/reference/README.md)

---

## Purpose

Tracks mismatches between:

1. **Spec** — target behavior defined in `/docs/spec`
2. **Runtime** — active implementation artifacts
3. **Reference** — verified state in `/docs/reference`

---

## Current State (Docs-Only Baseline)

Active runtime drift rows are deferred until reconstruction recreates runtime paths.

| Mismatch Level | Description | Resolution |
|----------------|-------------|------------|
| `M0` | trivial wording/format issues | fix immediately |
| `M1` | missing runtime feature or missing test evidence | schedule in current/next wave |
| `M2` | runtime behavior diverges from normative spec | refactor runtime or tighten spec |
| `M3` | contradiction between authority layers | escalate to policy/reference immediately |

---

## Active Drifts

No active runtime rows in docs-only state.

---

## Legacy Drift Inputs (Carry-Forward)

Imported from pre-reset runtime snapshot and required for rebuild planning.

| ID | Spec | Legacy Theme | Carry Wave | Status |
|----|------|--------------|------------|--------|
| `DRIFT-001` | [search.md](/docs/spec/domain/search.md) | semantic degradation behavior | `S02/W022` | `carry` |
| `DRIFT-002` | [websocket.md](/docs/spec/api/websocket.md) | replay/idempotency ambiguity | `S07/W071` | `carry` |
| `DRIFT-003` | [auth.md](/docs/spec/security/auth.md) | auth/session stubbing risk | `S05/W050` | `carry` |
| `DRIFT-004` | [testing.md](/docs/spec/technical/testing.md) | evidence incompleteness | `S09/W091` | `carry` |
| `DRIFT-005` | [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md) | root/reference status contradiction | `S00/W002` | `carry` |
| `DRIFT-006` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | stale TODO verification IDs (normalized) | `S00/W002` | `resolved` |
| `DRIFT-007` | [/docs/spec/api/http.md](/docs/spec/api/http.md) | request-id/csrf/idempotency/rate-limit closure gaps | `S06/W060` | `carry` |
| `DRIFT-008` | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | replay/stale-cursor/idempotency closure gaps | `S07/W071` | `carry` |
| `DRIFT-009` | [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md) | frontend flow and communication conformance gaps | `S08/W080` | `carry` |
| `DRIFT-010` | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | acceptance and supplemental suite evidence gaps | `S09/W091` | `carry` |

---

## Minimum Reporting Rule

For every closed TODO wave after runtime reconstruction:

- list touched spec files
- list touched runtime paths
- classify drift as `M0`..`M3`
- attach evidence pointer in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
- update matrix rows in the same change

---

## Related

- [LEGACY_RUNTIME_SNAPSHOT.md](LEGACY_RUNTIME_SNAPSHOT.md) — archived pre-reset runtime state
- [Conformance](CONFORMANCE.md) — verified state
- [Limitations](LIMITATIONS.md) — open gaps
- [TODO Program](/docs/todo/README.md) — rebuild execution order

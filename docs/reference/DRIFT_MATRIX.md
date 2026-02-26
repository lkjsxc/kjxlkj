# Drift Matrix

**Back:** [Reference Root](/docs/reference/README.md)

---

## Purpose

Tracks mismatches between:
1. **Spec** — target behavior defined in `/docs/spec`
2. **Runtime** — actual implementation (currently absent by design)
3. **Reference** — verified state in `/docs/reference`

---

## Current State: Docs-Only Baseline

**Runtime:** Absent (source code deleted for clean rebuild)  
**Spec:** Complete and execution-ready  
**Drift:** Deferred until runtime reconstruction begins

---

## Drift Resolution Process

When runtime exists:

| Mismatch Level | Description | Resolution |
|----------------|-------------|------------|
| `M0` | Trivial (typos, formatting) | Fix immediately |
| `M1` | Missing runtime feature | Schedule in TODO |
| `M2` | Feature exists but diverges | Refactor or update spec |
| `M3` | Spec contradiction | Escalate to policy layer |

---

## Historical Drift Inputs (Imported)

These drift themes were imported from prior implementation notes and must be revalidated in the new rebuild:

| ID | Spec | Prior Runtime | Resolution |
|----|------|---------------|------------|
| `DRIFT-001` | [search.md](/docs/spec/domain/search.md) | semantic provider degradation behavior | carry into `S02/W022` |
| `DRIFT-002` | [websocket.md](/docs/spec/api/websocket.md) | replay/idempotency ambiguity | carry into `S07/W071` |
| `DRIFT-003` | [auth.md](/docs/spec/security/auth.md) | auth/session stubbing risk | carry into `S05/W050` |
| `DRIFT-004` | [testing.md](/docs/spec/technical/testing.md) | acceptance evidence incompleteness | carry into `S09/W091` |

---

## Active Drifts

No runtime drift rows yet. Runtime has not been rebuilt.

---

## Post-Rebuild Drift Tracking

After runtime reconstruction, this matrix will track:

| ID | Spec File | Runtime Path | Drift Type | Severity | Resolution Wave |
|----|-----------|--------------|------------|----------|-----------------|
| — | — | — | — | — | — |

## Minimum Reporting Rule

When runtime exists, every closed TODO wave MUST include:

- touched spec file list
- touched runtime paths
- drift classification (`M0`-`M3`)
- evidence pointer in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)

---

## Related

- [Conformance](CONFORMANCE.md) — verified state
- [Limitations](LIMITATIONS.md) — open gaps
- [TODO Program](/docs/todo/README.md) — rebuild execution order
- [TODO Trace Matrix](TODO_TRACE_MATRIX.md) — TODO-to-runtime mapping

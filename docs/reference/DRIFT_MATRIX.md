# Drift Matrix

**Back:** [Reference Root](/docs/reference/README.md)

---

## Purpose

Tracks mismatches between:
1. **Spec** — target behavior defined in `/docs/spec`
2. **Runtime** — actual implementation (currently absent)
3. **Reference** — verified state in `/docs/reference`

---

## Current State: Docs-Only Baseline

**Runtime:** Absent (source code deleted for clean rebuild)  
**Spec:** Complete (114 documentation files)  
**Drift:** N/A (no runtime to compare against)

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

## Historical Drift (Pre-Deletion)

These drifts were resolved by deleting source code and resetting:

| ID | Spec | Prior Runtime | Resolution |
|----|------|---------------|------------|
| `DRIFT-001` | [search.md](/docs/spec/domain/search.md) | Stub implementation | ✅ Deleted, spec redesigned |
| `DRIFT-002` | [editor-flow.md](/docs/spec/ui/editor-flow.md) | Basic editor | ✅ Deleted, spec enhanced |
| `DRIFT-003` | [librarian-agent.md](/docs/spec/technical/librarian-agent.md) | Partial agent | ✅ Deleted, spec updated |
| `DRIFT-004` | [layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) | Wrong threshold | ✅ Deleted, spec corrected |

---

## Active Drifts

**None** — No runtime exists to drift from spec.

---

## Post-Rebuild Drift Tracking

After runtime reconstruction, this matrix will track:

| ID | Spec File | Runtime Path | Drift Type | Severity | Resolution Wave |
|----|-----------|--------------|------------|----------|-----------------|
| — | — | — | — | — | — |

---

## Related

- [Conformance](CONFORMANCE.md) — verified state
- [Limitations](LIMITATIONS.md) — open gaps
- [TODO Program](/docs/todo/README.md) — rebuild execution order

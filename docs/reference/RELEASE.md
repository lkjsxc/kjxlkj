# Release Process

**Back:** [Reference Root](/docs/reference/README.md)

---

## Preconditions

1. `Release` CI profile is green
2. No open high-severity limitations
3. Drift matrix has no open `M1` or `M2` rows
4. Acceptance tests in [Testing](/docs/spec/technical/testing.md) pass
5. Type-safety gates in [Type Safety](/docs/spec/technical/type-safety.md) pass

---

## Current Gate (Docs-Only Baseline)

**Release is blocked.**

**Expected state:** Repository is in docs-only baseline — source code deleted for clean rebuild.

### Blocking Reasons

| Blocker | Resolution Stage |
|---------|------------------|
| Source code deleted | Stage S01 (runtime scaffold) |
| HTTP handlers not implemented | Stage S06 (REST API) |
| WebSocket replay not implemented | Stage S07 (WS sync) |
| Frontend not implemented | Stage S08 (frontend) |
| kjxlkj-agent not implemented | Stage S04 (automation) |
| Auth/session not implemented | Stage S05 (security) |
| Tests not present | Stage S09 (CI) |
| TODO waves not executed | All stages pending |

---

## Release Steps

1. Execute TODO waves in order (S00 → S10)
2. Satisfy each wave build/test gate
3. Run CI profiles and archive evidence
4. Close drift and limitation rows
5. Synchronize ledgers and TODO completion
6. Tag release

---

## Related

- [Conformance](CONFORMANCE.md) — verified state
- [Limitations](LIMITATIONS.md) — open gaps
- [Drift Matrix](DRIFT_MATRIX.md) — mismatch tracking
- [TODO Program](/docs/todo/README.md) — execution order
